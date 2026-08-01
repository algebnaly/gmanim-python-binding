use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};

use gmanim_core::{
    ClipRect, Color, Context, EnvironmentLight, PointLight, RendererConfig, Scene, SceneConfig,
    animation::{
        AaLevelProperty, CameraPoseProperty, CameraProjectionProperty, CompiledTimeline,
        EnvironmentLightProperty, LayerProperty, Move, PointLightProperty, PropertyWriteFrame,
        Rotate, TimelineBuilder, TransformProperty, ViewportProperty, VisibilityProperty, Wait,
    },
    camera::{CameraPose, OrthographicSetting, PerspectiveSetting, Projection},
    mobjects::MobjectId,
    video_backend::{
        ColorOrder, FfmpegPipeBackend, FfmpegPipeEncoder, VideoBackend, VideoBackendType,
        VideoConfig, vaapi::FfmpegVaapiBackend, vulkan_h264::AsyncVulkanH264Backend,
    },
    vulkan::{
        context::VulkanContext,
        renderer::{RenderOutputs, VulkanRenderer},
    },
};
use nalgebra::{Matrix4, Point3, Vector3};
use pyo3::{exceptions::*, prelude::*};

use crate::{
    PyVideoBackend,
    animations::{PyAnimation, PyAnimationSpec},
};

pub(crate) use crate::mobjects::ObjectState;

static NEXT_SCENE_TOKEN: AtomicU64 = AtomicU64::new(1);

fn runtime_error(error: impl ToString) -> PyErr {
    PyRuntimeError::new_err(error.to_string())
}

fn value_error(error: impl ToString) -> PyErr {
    PyValueError::new_err(error.to_string())
}

fn color(value: (u8, u8, u8, u8)) -> Color {
    Color::new(value.0, value.1, value.2, value.3)
}

#[pyclass(name = "Mobject", subclass, from_py_object)]
#[derive(Clone)]
pub struct PyMobject {
    pub(crate) state: Arc<Mutex<ObjectState>>,
}

#[pymethods]
impl PyMobject {
    fn set_position(&mut self, position: (f32, f32, f32)) -> PyResult<()> {
        let mut state = self.state.lock().unwrap();
        if state.attachment.is_some() {
            return Err(PyValueError::new_err(
                "attached mobjects are modified through Scene or SceneFrame",
            ));
        }
        state.transform[(0, 3)] = position.0;
        state.transform[(1, 3)] = position.1;
        state.transform[(2, 3)] = position.2;
        Ok(())
    }

    fn get_position(&self) -> PyResult<(f32, f32, f32)> {
        let state = self.state.lock().unwrap();
        if state.attachment.is_some() {
            return Err(PyValueError::new_err(
                "attached mobject state is read through Scene or SceneFrame",
            ));
        }
        Ok((
            state.transform[(0, 3)],
            state.transform[(1, 3)],
            state.transform[(2, 3)],
        ))
    }

    fn rotate_x(&mut self, angle: f32) -> PyResult<()> {
        self.apply_detached_transform(Matrix4::new_rotation(Vector3::x() * angle))
    }

    fn rotate_y(&mut self, angle: f32) -> PyResult<()> {
        self.apply_detached_transform(Matrix4::new_rotation(Vector3::y() * angle))
    }

    fn rotate_z(&mut self, angle: f32) -> PyResult<()> {
        self.apply_detached_transform(Matrix4::new_rotation(Vector3::z() * angle))
    }

    fn add(&self, child: PyMobject) -> PyResult<()> {
        self.add_detached_child(child)
    }

    fn remove(&self, child: PyMobject) -> PyResult<()> {
        self.remove_detached_child(&child)
    }

    #[getter]
    fn get_name(&self) -> String {
        self.state.lock().unwrap().name.clone()
    }

    #[setter]
    fn set_name(&mut self, name: String) -> PyResult<()> {
        let mut state = self.state.lock().unwrap();
        if state.attachment.is_some() {
            return Err(PyValueError::new_err(
                "an attached mobject name is immutable",
            ));
        }
        state.name = name;
        Ok(())
    }
}

impl PyMobject {
    fn apply_detached_transform(&mut self, transform: Matrix4<f32>) -> PyResult<()> {
        let current = self.state.lock().unwrap().transform;
        self.set_detached_transform(current * transform)
    }
}

enum SceneState {
    Building(TimelineBuilder),
    Compiled(CompiledTimeline),
}

#[pyclass(name = "Scene")]
pub struct PyScene {
    name: String,
    token: u64,
    state: Option<SceneState>,
}

#[pyclass(name = "PreviewSession", unsendable)]
pub struct PyPreviewSession {
    timeline: CompiledTimeline,
    renderer: VulkanRenderer,
    shmem: shared_memory::Shmem,
    layout: crate::ipc::PreviewLayout,
}

impl PyPreviewSession {
    fn new(timeline: CompiledTimeline, shm_id: &str) -> Result<Self, String> {
        let shmem = shared_memory::ShmemConf::new()
            .os_id(shm_id)
            .open()
            .map_err(|error| error.to_string())?;
        let header = unsafe { &*(shmem.as_ptr() as *const crate::ipc::PreviewShmHeader) };
        let layout = header.layout().map_err(|error| error.to_string())?;
        if shmem.len() < layout.total_size {
            return Err("preview shared memory is smaller than its declared layout".to_owned());
        }
        if layout.width != timeline.ctx.scene_config.output_width
            || layout.height != timeline.ctx.scene_config.output_height
            || layout.capacity != crate::ipc::PREVIEW_SLOT_COUNT
            || layout.pixel_format != crate::ipc::PreviewPixelFormat::Rgba8Unorm
        {
            return Err("preview layout does not match the scene output".to_owned());
        }

        let context = VulkanContext::new().map_err(|error| error.to_string())?;
        let renderer = VulkanRenderer::new(
            context,
            RendererConfig {
                msaa_samples: 8,
                ssaa_factor: 1,
                output_color_profile: Default::default(),
            },
        );
        Ok(Self {
            timeline,
            renderer,
            shmem,
            layout,
        })
    }
}

#[pymethods]
impl PyPreviewSession {
    fn render_frame(
        &mut self,
        py: Python<'_>,
        request_id: u64,
        frame: u32,
        slot: u32,
    ) -> PyResult<()> {
        if slot >= self.layout.capacity {
            return Err(PyValueError::new_err("preview slot is out of range"));
        }
        let rgba = py
            .detach(|| render_preview_frame(&mut self.timeline, &mut self.renderer, frame))
            .map_err(runtime_error)?;
        let destination = unsafe {
            self.shmem
                .as_ptr()
                .add(self.layout.frame_offset(u64::from(slot)))
        };
        let packed_stride = self.layout.width as usize * 4;
        for row in 0..self.layout.height as usize {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    rgba.as_ptr().add(row * packed_stride),
                    destination.add(row * self.layout.stride as usize),
                    packed_stride,
                );
            }
        }
        let header = unsafe { &*(self.shmem.as_ptr() as *const crate::ipc::PreviewShmHeader) };
        header.publish(slot, request_id, frame);
        Ok(())
    }
}

impl PyScene {
    fn builder_mut(&mut self) -> PyResult<&mut TimelineBuilder> {
        match self.state.as_mut() {
            Some(SceneState::Building(builder)) => Ok(builder),
            Some(SceneState::Compiled(_)) => Err(PyRuntimeError::new_err(
                "scene is already compiled and can no longer be modified",
            )),
            None => Err(PyRuntimeError::new_err("scene state is unavailable")),
        }
    }

    fn timeline_mut(&mut self) -> PyResult<&mut CompiledTimeline> {
        if matches!(self.state, Some(SceneState::Building(_))) {
            let Some(SceneState::Building(builder)) = self.state.take() else {
                unreachable!()
            };
            self.state = Some(SceneState::Compiled(builder.build()));
        }
        match self.state.as_mut() {
            Some(SceneState::Compiled(timeline)) => Ok(timeline),
            _ => Err(PyRuntimeError::new_err("scene state is unavailable")),
        }
    }

    fn object_id(&self, object: &PyMobject) -> PyResult<MobjectId> {
        Ok(object.attachment(self.token)?.id)
    }

    fn current_transform(&mut self, object: &PyMobject) -> PyResult<Matrix4<f32>> {
        let id = self.object_id(object)?;
        self.builder_mut()?
            .scene_view()
            .transform(id)
            .map_err(value_error)
    }

    fn set_camera_pose_inner(
        &mut self,
        position: Option<(f32, f32, f32)>,
        target: Option<(f32, f32, f32)>,
        direction: Option<(f32, f32, f32)>,
        up: Option<(f32, f32, f32)>,
    ) -> PyResult<()> {
        let mut pose = self.builder_mut()?.scene_view().camera_pose();
        if let Some(position) = position {
            pose.position = Point3::new(position.0, position.1, position.2);
        }
        if let Some(target) = target {
            pose.look_at = Point3::new(target.0, target.1, target.2) - pose.position;
        } else if let Some(direction) = direction {
            pose.look_at = Vector3::new(direction.0, direction.1, direction.2);
        }
        if pose.look_at.norm_squared() < 1e-6 {
            return Err(PyValueError::new_err("camera direction must be non-zero"));
        }
        pose.look_at.normalize_mut();
        if let Some(up) = up {
            pose.up_direction = Vector3::new(up.0, up.1, up.2);
        }
        if pose.up_direction.norm_squared() < 1e-6 {
            return Err(PyValueError::new_err("camera up vector must be non-zero"));
        }
        pose.up_direction.normalize_mut();
        self.builder_mut()?
            .set(CameraPoseProperty, pose)
            .map_err(value_error)
    }

    fn play_update_from_func(
        &mut self,
        py: Python<'_>,
        callback: &Py<PyAny>,
        frames: u32,
    ) -> PyResult<()> {
        let token = self.token;
        let builder = self.builder_mut()?;
        let mut recorder = builder.record_properties(frames).map_err(value_error)?;

        while recorder.next_frame().is_some() {
            let frame = recorder.begin_frame().map_err(value_error)?;
            let alpha = frame.alpha();
            let py_frame = Py::new(
                py,
                PySceneFrame {
                    scene_token: token,
                    inner: Some(frame),
                },
            )?;
            callback.call1(py, (py_frame.clone_ref(py), alpha))?;
            let frame = py_frame
                .borrow_mut(py)
                .inner
                .take()
                .ok_or_else(|| PyRuntimeError::new_err("SceneFrame was already consumed"))?;
            recorder.commit_frame(frame).map_err(value_error)?;
        }

        let clip = recorder.finish().map_err(value_error)?;
        builder.append_clip(clip).map_err(value_error)
    }
}

#[pymethods]
impl PyScene {
    #[new]
    #[pyo3(signature = (name, width=None, height=None, resolution=Some((1920, 1080)), scale_factor=None, fps=60))]
    fn new(
        name: String,
        width: Option<f32>,
        height: Option<f32>,
        resolution: Option<(u32, u32)>,
        scale_factor: Option<f32>,
        fps: u32,
    ) -> PyResult<Self> {
        if name.trim().is_empty() {
            return Err(PyValueError::new_err("scene name must not be empty"));
        }
        if fps == 0 {
            return Err(PyValueError::new_err("scene fps must be positive"));
        }
        let width = width.unwrap_or(16.0);
        let height = height.unwrap_or(9.0);
        let (output_width, output_height) = resolution.unwrap_or((1920, 1080));
        let scale_factor = scale_factor.unwrap_or(output_height as f32 / height);
        let context = Context {
            scene_config: SceneConfig {
                width,
                height,
                output_width,
                output_height,
                scale_factor,
                framerate: fps,
            },
        };
        Ok(Self {
            name,
            token: NEXT_SCENE_TOKEN.fetch_add(1, Ordering::Relaxed),
            state: Some(SceneState::Building(TimelineBuilder::new(
                Scene::new(),
                context,
            ))),
        })
    }

    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    #[getter]
    fn fps(&mut self) -> PyResult<u32> {
        Ok(match self.state.as_ref() {
            Some(SceneState::Building(builder)) => builder.scene_config().framerate,
            Some(SceneState::Compiled(timeline)) => timeline.ctx.scene_config.framerate,
            None => return Err(PyRuntimeError::new_err("scene state is unavailable")),
        })
    }

    fn add(&mut self, object: PyMobject) -> PyResult<PyMobject> {
        let (bundle, handles) = object.build_bundle_tree()?;
        let builder = self.builder_mut()?;
        let plan = builder.reserve_spawn(bundle, None).map_err(value_error)?;
        let ids = plan.ids();
        builder.spawn_now(plan).map_err(value_error)?;
        PyMobject::attach_tree(handles, ids, self.token)?;
        Ok(object)
    }

    fn remove(&mut self, argument: &Bound<'_, PyAny>) -> PyResult<()> {
        let id = if let Ok(index) = argument.extract::<usize>() {
            self.builder_mut()?
                .scene_view()
                .roots()
                .get(index)
                .copied()
                .ok_or_else(|| PyIndexError::new_err("root index is out of range"))?
        } else if let Ok(object) = argument.extract::<PyMobject>() {
            self.object_id(&object)?
        } else {
            return Err(PyTypeError::new_err(
                "expected a root index or Mobject handle",
            ));
        };
        self.builder_mut()?.remove(id).map_err(value_error)
    }

    fn set_position(&mut self, object: &PyMobject, position: (f32, f32, f32)) -> PyResult<()> {
        let id = self.object_id(object)?;
        let mut transform = self.current_transform(object)?;
        transform[(0, 3)] = position.0;
        transform[(1, 3)] = position.1;
        transform[(2, 3)] = position.2;
        self.builder_mut()?
            .set(TransformProperty::new(id), transform)
            .map_err(value_error)
    }

    fn get_position(&mut self, object: &PyMobject) -> PyResult<(f32, f32, f32)> {
        let transform = self.current_transform(object)?;
        Ok((transform[(0, 3)], transform[(1, 3)], transform[(2, 3)]))
    }

    fn set_visible(&mut self, object: &PyMobject, visible: bool) -> PyResult<()> {
        let id = self.object_id(object)?;
        self.builder_mut()?
            .set(VisibilityProperty::new(id), visible)
            .map_err(value_error)
    }

    fn set_background_color(&mut self, r: u8, g: u8, b: u8, a: u8) -> PyResult<()> {
        self.builder_mut()?
            .set_background_color(gmanim_core::Color::new(r, g, b, a));
        Ok(())
    }

    fn set_layer(&mut self, object: &PyMobject, layer: i32) -> PyResult<()> {
        let id = self.object_id(object)?;
        self.builder_mut()?
            .set(LayerProperty::new(id), layer)
            .map_err(value_error)
    }

    #[pyo3(signature = (position=None, target=None, direction=None, up=None))]
    fn set_camera(
        &mut self,
        position: Option<(f32, f32, f32)>,
        target: Option<(f32, f32, f32)>,
        direction: Option<(f32, f32, f32)>,
        up: Option<(f32, f32, f32)>,
    ) -> PyResult<()> {
        self.set_camera_pose_inner(position, target, direction, up)
    }

    #[pyo3(signature = (height=9.0, width=None, near=0.1, far=50.0))]
    fn set_orthographic_camera(
        &mut self,
        height: f32,
        width: Option<f32>,
        near: f32,
        far: f32,
    ) -> PyResult<()> {
        let width = width.unwrap_or(height * 16.0 / 9.0);
        let projection = Projection::Orthographic(OrthographicSetting::new(
            -width / 2.0,
            width / 2.0,
            -height / 2.0,
            height / 2.0,
            near,
            far,
        ));
        self.builder_mut()?
            .set(CameraProjectionProperty, projection)
            .map_err(value_error)
    }

    #[pyo3(signature = (fovy=std::f32::consts::PI / 2.0, aspect=None, near=0.1, far=50.0))]
    fn set_perspective_camera(
        &mut self,
        fovy: f32,
        aspect: Option<f32>,
        near: f32,
        far: f32,
    ) -> PyResult<()> {
        let projection = Projection::Perspective(PerspectiveSetting::new(
            aspect.unwrap_or(16.0 / 9.0),
            fovy,
            near,
            far,
        ));
        self.builder_mut()?
            .set(CameraProjectionProperty, projection)
            .map_err(value_error)
    }

    fn set_viewport(
        &mut self,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
    ) -> PyResult<()> {
        self.builder_mut()?
            .set(
                ViewportProperty,
                Some(ClipRect::Logical(center_x, center_y, width, height)),
            )
            .map_err(value_error)
    }

    fn set_pixel_viewport(&mut self, x: u32, y: u32, width: u32, height: u32) -> PyResult<()> {
        self.builder_mut()?
            .set(ViewportProperty, Some(ClipRect::Pixel(x, y, width, height)))
            .map_err(value_error)
    }

    fn clear_viewport(&mut self) -> PyResult<()> {
        self.builder_mut()?
            .set(ViewportProperty, None)
            .map_err(value_error)
    }

    fn set_anti_aliasing(&mut self, level: u32) -> PyResult<()> {
        self.builder_mut()?
            .set(AaLevelProperty, level)
            .map_err(value_error)
    }

    fn set_point_light(
        &mut self,
        position: (f32, f32, f32),
        color_value: (u8, u8, u8, u8),
        intensity: f32,
    ) -> PyResult<()> {
        self.builder_mut()?
            .set(
                PointLightProperty,
                PointLight {
                    position: Point3::new(position.0, position.1, position.2),
                    color: color(color_value),
                    intensity,
                },
            )
            .map_err(value_error)
    }

    #[pyo3(signature = (color_value, intensity, rotation_radians=0.0))]
    fn set_environment_light(
        &mut self,
        color_value: (u8, u8, u8, u8),
        intensity: f32,
        rotation_radians: f32,
    ) -> PyResult<()> {
        self.builder_mut()?
            .set(
                EnvironmentLightProperty,
                EnvironmentLight {
                    color: color(color_value),
                    intensity,
                    rotation_radians,
                },
            )
            .map_err(value_error)
    }

    fn play(&mut self, py: Python<'_>, animation: PyRef<'_, PyAnimation>) -> PyResult<()> {
        match &animation.spec {
            PyAnimationSpec::Move {
                target,
                displacement,
                frames,
            } => {
                let id = self.object_id(target)?;
                self.builder_mut()?
                    .play(Move::new(
                        id,
                        Vector3::new(displacement.0, displacement.1, displacement.2),
                        *frames,
                    ))
                    .map_err(value_error)
            }
            PyAnimationSpec::Rotate {
                target,
                axis,
                center,
                frames,
            } => {
                let id = self.object_id(target)?;
                self.builder_mut()?
                    .play(Rotate::new(
                        id,
                        Vector3::new(axis.0, axis.1, axis.2),
                        Point3::new(center.0, center.1, center.2),
                        *frames,
                    ))
                    .map_err(value_error)
            }
            PyAnimationSpec::Wait { frames } => self
                .builder_mut()?
                .play(Wait::new(*frames))
                .map_err(value_error),
            PyAnimationSpec::UpdateFromFunc { callback, frames } => {
                self.play_update_from_func(py, callback, *frames)
            }
        }
    }

    fn wait(&mut self, frames: u32) -> PyResult<()> {
        self.builder_mut()?
            .play(Wait::new(frames))
            .map_err(value_error)
    }

    #[pyo3(signature = (filename, backend=None, show_progress=true, bitrate=None, ssaa_factor=None, msaa_samples=None, vulkan_config=None))]
    #[allow(clippy::too_many_arguments)]
    fn render(
        &mut self,
        py: Python<'_>,
        filename: String,
        backend: Option<PyVideoBackend>,
        show_progress: bool,
        bitrate: Option<u64>,
        ssaa_factor: Option<u32>,
        msaa_samples: Option<u32>,
        vulkan_config: Option<crate::PyVulkanH264Config>,
    ) -> PyResult<()> {
        let timeline = self.timeline_mut()?;
        let scene_fps = timeline.ctx.scene_config.framerate;
        let options = RenderOptions {
            filename,
            fps: scene_fps,
            backend: backend.unwrap_or(PyVideoBackend::Ffmpeg),
            show_progress,
            bitrate,
            ssaa_factor: ssaa_factor.unwrap_or(2),
            msaa_samples: msaa_samples.unwrap_or(8),
            vulkan_config: vulkan_config.unwrap_or_default(),
        };
        py.detach(|| render_timeline(timeline, options))
            .map_err(runtime_error)
    }

    fn _get_render_info(&mut self) -> PyResult<(u32, u32, u32, u32)> {
        let timeline = self.timeline_mut()?;
        Ok((
            timeline.total_frames(),
            timeline.ctx.scene_config.output_width,
            timeline.ctx.scene_config.output_height,
            timeline.ctx.scene_config.framerate,
        ))
    }

    fn _open_preview(&mut self, shm_id: String) -> PyResult<PyPreviewSession> {
        self.timeline_mut()?;
        let Some(SceneState::Compiled(timeline)) = self.state.take() else {
            return Err(PyRuntimeError::new_err("scene timeline is unavailable"));
        };
        PyPreviewSession::new(timeline, &shm_id).map_err(runtime_error)
    }
}

#[pyclass(name = "SceneFrame")]
pub struct PySceneFrame {
    scene_token: u64,
    inner: Option<PropertyWriteFrame>,
}

impl PySceneFrame {
    fn frame_mut(&mut self) -> PyResult<&mut PropertyWriteFrame> {
        self.inner
            .as_mut()
            .ok_or_else(|| PyRuntimeError::new_err("SceneFrame is no longer active"))
    }

    fn id(&self, object: &PyMobject) -> PyResult<MobjectId> {
        Ok(object.attachment(self.scene_token)?.id)
    }
}

#[pymethods]
impl PySceneFrame {
    #[getter]
    fn frame(&self) -> PyResult<u32> {
        Ok(self
            .inner
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("SceneFrame is no longer active"))?
            .frame())
    }

    #[getter]
    fn alpha(&self) -> PyResult<f32> {
        Ok(self
            .inner
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("SceneFrame is no longer active"))?
            .alpha())
    }

    fn get_position(&self, object: &PyMobject) -> PyResult<(f32, f32, f32)> {
        let id = self.id(object)?;
        let position = self
            .inner
            .as_ref()
            .ok_or_else(|| PyRuntimeError::new_err("SceneFrame is no longer active"))?
            .view()
            .position(id)
            .map_err(value_error)?;
        Ok((position.x, position.y, position.z))
    }

    fn set_position(&mut self, object: &PyMobject, position: (f32, f32, f32)) -> PyResult<()> {
        let id = self.id(object)?;
        self.frame_mut()?
            .set_position(id, Point3::new(position.0, position.1, position.2))
            .map_err(value_error)
    }

    fn move_by(&mut self, object: &PyMobject, displacement: (f32, f32, f32)) -> PyResult<()> {
        let id = self.id(object)?;
        self.frame_mut()?
            .move_by(
                id,
                Vector3::new(displacement.0, displacement.1, displacement.2),
            )
            .map_err(value_error)
    }

    fn set_visible(&mut self, object: &PyMobject, visible: bool) -> PyResult<()> {
        let id = self.id(object)?;
        self.frame_mut()?
            .set_visible(id, visible)
            .map_err(value_error)
    }

    fn set_layer(&mut self, object: &PyMobject, layer: i32) -> PyResult<()> {
        let id = self.id(object)?;
        self.frame_mut()?.set_layer(id, layer).map_err(value_error)
    }

    fn set_rectangle_corners(
        &mut self,
        object: &PyMobject,
        corners: Vec<(f32, f32, f32)>,
    ) -> PyResult<()> {
        let id = self.id(object)?;
        let corners: [(f32, f32, f32); 4] = corners
            .try_into()
            .map_err(|_| PyValueError::new_err("corners must contain exactly four 3D points"))?;
        self.frame_mut()?
            .set_rectangle_corners(
                id,
                corners.map(|point| Point3::new(point.0, point.1, point.2)),
            )
            .map_err(value_error)
    }

    fn set_camera(
        &mut self,
        position: (f32, f32, f32),
        target: (f32, f32, f32),
        up: (f32, f32, f32),
    ) -> PyResult<()> {
        let position = Point3::new(position.0, position.1, position.2);
        let mut look_at = Point3::new(target.0, target.1, target.2) - position;
        let mut up_direction = Vector3::new(up.0, up.1, up.2);
        if look_at.norm_squared() < 1e-6 || up_direction.norm_squared() < 1e-6 {
            return Err(PyValueError::new_err(
                "camera direction and up vector must be non-zero",
            ));
        }
        look_at.normalize_mut();
        up_direction.normalize_mut();
        self.frame_mut()?
            .set_camera_pose(CameraPose {
                position,
                look_at,
                up_direction,
            })
            .map_err(value_error)
    }

    #[pyo3(signature = (height=9.0, width=None, near=0.1, far=50.0))]
    fn set_orthographic_camera(
        &mut self,
        height: f32,
        width: Option<f32>,
        near: f32,
        far: f32,
    ) -> PyResult<()> {
        let width = width.unwrap_or(height * 16.0 / 9.0);
        self.frame_mut()?
            .set_camera_projection(Projection::Orthographic(OrthographicSetting::new(
                -width / 2.0,
                width / 2.0,
                -height / 2.0,
                height / 2.0,
                near,
                far,
            )))
            .map_err(value_error)
    }

    #[pyo3(signature = (fovy=std::f32::consts::PI / 2.0, aspect=None, near=0.1, far=50.0))]
    fn set_perspective_camera(
        &mut self,
        fovy: f32,
        aspect: Option<f32>,
        near: f32,
        far: f32,
    ) -> PyResult<()> {
        self.frame_mut()?
            .set_camera_projection(Projection::Perspective(PerspectiveSetting::new(
                aspect.unwrap_or(16.0 / 9.0),
                fovy,
                near,
                far,
            )))
            .map_err(value_error)
    }

    fn set_viewport(
        &mut self,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
    ) -> PyResult<()> {
        self.frame_mut()?
            .set_viewport(Some(ClipRect::Logical(center_x, center_y, width, height)))
            .map_err(value_error)
    }

    fn set_pixel_viewport(&mut self, x: u32, y: u32, width: u32, height: u32) -> PyResult<()> {
        self.frame_mut()?
            .set_viewport(Some(ClipRect::Pixel(x, y, width, height)))
            .map_err(value_error)
    }

    fn clear_viewport(&mut self) -> PyResult<()> {
        self.frame_mut()?.set_viewport(None).map_err(value_error)
    }

    fn set_anti_aliasing(&mut self, level: u32) -> PyResult<()> {
        self.frame_mut()?.set_aa_level(level).map_err(value_error)
    }

    fn set_point_light(
        &mut self,
        position: (f32, f32, f32),
        color_value: (u8, u8, u8, u8),
        intensity: f32,
    ) -> PyResult<()> {
        self.frame_mut()?
            .set_point_light(PointLight {
                position: Point3::new(position.0, position.1, position.2),
                color: color(color_value),
                intensity,
            })
            .map_err(value_error)
    }

    #[pyo3(signature = (color_value, intensity, rotation_radians=0.0))]
    fn set_environment_light(
        &mut self,
        color_value: (u8, u8, u8, u8),
        intensity: f32,
        rotation_radians: f32,
    ) -> PyResult<()> {
        self.frame_mut()?
            .set_environment_light(EnvironmentLight {
                color: color(color_value),
                intensity,
                rotation_radians,
            })
            .map_err(value_error)
    }
}

struct RenderOptions {
    filename: String,
    fps: u32,
    backend: PyVideoBackend,
    show_progress: bool,
    bitrate: Option<u64>,
    ssaa_factor: u32,
    msaa_samples: u32,
    vulkan_config: crate::PyVulkanH264Config,
}

fn render_timeline(timeline: &mut CompiledTimeline, options: RenderOptions) -> Result<(), String> {
    timeline.seek(0).map_err(|error| error.to_string())?;
    let width = timeline.ctx.scene_config.output_width;
    let height = timeline.ctx.scene_config.output_height;
    let color_order = match options.backend {
        PyVideoBackend::Ffmpeg => ColorOrder::Yuv444p,
        PyVideoBackend::Vaapi | PyVideoBackend::Vulkan => ColorOrder::Nv12,
    };
    let video_config = VideoConfig {
        filename: options.filename,
        framerate: options.fps,
        output_width: width,
        output_height: height,
        color_order,
        bitrate: options.bitrate,
        output_color_profile: Default::default(),
    };

    let context = VulkanContext::new().map_err(|error| error.to_string())?;
    let mut renderer = VulkanRenderer::new(
        context.clone(),
        RendererConfig {
            msaa_samples: options.msaa_samples,
            ssaa_factor: options.ssaa_factor,
            output_color_profile: Default::default(),
        },
    );
    let mut backend = match options.backend {
        PyVideoBackend::Vaapi => VideoBackend {
            backend_type: VideoBackendType::Vaapi(FfmpegVaapiBackend::new(&video_config)),
        },
        PyVideoBackend::Ffmpeg => VideoBackend {
            backend_type: VideoBackendType::FfmpegPipe(FfmpegPipeBackend::new(
                &video_config,
                FfmpegPipeEncoder::Libx264,
                false,
            )),
        },
        PyVideoBackend::Vulkan => VideoBackend {
            backend_type: VideoBackendType::VulkanH264(
                AsyncVulkanH264Backend::try_new_with_encoder_config(
                    context,
                    &video_config,
                    options.vulkan_config.to_core(),
                )
                .map_err(|error| error.to_string())?,
            ),
        },
    };

    let progress = options.show_progress.then(|| {
        let progress = indicatif::ProgressBar::new(timeline.total_frames() as u64);
        progress.set_style(
            indicatif::ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} frames ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
        );
        progress
    });
    let mut frame_count = 0u64;

    if let VideoBackendType::VulkanH264(vulkan_backend) = &mut backend.backend_type {
        while timeline
            .advance_frame()
            .map_err(|error| error.to_string())?
        {
            renderer.render_scene_with_outputs(
                &timeline.scene,
                &timeline.ctx.scene_config,
                None,
                RenderOutputs::VULKAN_VIDEO_ONLY,
            );
            let frame = renderer
                .get_vulkan_video_frame()
                .ok_or_else(|| "renderer did not produce a Vulkan video frame".to_owned())?;
            vulkan_backend
                .submit_vulkan_frame(frame)
                .map_err(|error| error.to_string())?;
            frame_count += 1;
            if let Some(progress) = &progress {
                progress.set_position(frame_count);
            }
        }
    } else {
        let outputs = match options.backend {
            PyVideoBackend::Ffmpeg => RenderOutputs {
                cpu_nv12: false,
                vulkan_video: false,
                cpu_rgba: false,
                cpu_yuv444p: true,
            },
            PyVideoBackend::Vaapi => RenderOutputs::CPU_NV12_ONLY,
            PyVideoBackend::Vulkan => unreachable!(),
        };
        while timeline
            .advance_frame()
            .map_err(|error| error.to_string())?
        {
            renderer.render_scene_with_outputs(
                &timeline.scene,
                &timeline.ctx.scene_config,
                None,
                outputs,
            );
            let bytes = match options.backend {
                PyVideoBackend::Ffmpeg => renderer.get_yuv444p_bytes(),
                PyVideoBackend::Vaapi => renderer.get_nv12_bytes(),
                PyVideoBackend::Vulkan => unreachable!(),
            }
            .ok_or_else(|| "renderer did not produce the requested CPU frame".to_owned())?;
            let mut buffer = backend.acquire_buffer();
            buffer.as_mut_slice().copy_from_slice(bytes);
            backend.submit_frame(buffer);
            frame_count += 1;
            if let Some(progress) = &progress {
                progress.set_position(frame_count);
            }
        }
    }

    backend.close().map_err(|error| error.to_string())?;
    if let Some(progress) = progress {
        progress.finish_with_message("Render complete");
    }
    Ok(())
}

fn render_preview_frame(
    timeline: &mut CompiledTimeline,
    renderer: &mut VulkanRenderer,
    frame: u32,
) -> Result<Vec<u8>, String> {
    let total_frames = timeline.total_frames();
    if frame >= total_frames {
        return Err(format!(
            "preview frame {frame} is outside the timeline length {total_frames}"
        ));
    }
    timeline
        .seek(frame.saturating_add(1))
        .map_err(|error| error.to_string())?;
    renderer.render_scene_with_outputs(
        &timeline.scene,
        &timeline.ctx.scene_config,
        None,
        RenderOutputs::CPU_RGBA_ONLY,
    );
    renderer
        .get_rgba_bytes()
        .map(ToOwned::to_owned)
        .ok_or_else(|| "renderer did not produce an RGBA preview frame".to_owned())
}
