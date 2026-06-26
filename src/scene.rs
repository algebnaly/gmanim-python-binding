// src/scene.rs
use std::cell::RefCell;
use std::rc::Rc;
use pyo3::prelude::*;

use crate::animations::{PyMove, PyRotate, PyWait, PyUpdateFromFunc};

struct RustUpdateFromFunc {
    callback: pyo3::Py<pyo3::PyAny>,
    total_frames: u32,
    is_pure: bool,
}

impl gmanim_core::animation::Animation for RustUpdateFromFunc {
    fn total_frames(&self) -> u32 {
        self.total_frames
    }

    fn is_pure(&self) -> bool {
        self.is_pure
    }

    fn update(&mut self, alpha: gmanim_core::GMFloat, scene: &mut gmanim_core::Scene) {
        let _ = pyo3::Python::try_attach(|py| {
            let scene_ref = crate::scene::PySceneRef::borrow(scene);
            let py_scene_ref = pyo3::Bound::new(py, scene_ref).unwrap();
            let args = (py_scene_ref, alpha as f32);
            if let Err(e) = self.callback.call1(py, args) {
                e.print(py);
                panic!("Python callback failed in UpdateFromFunc");
            }
        });
    }
}




#[pyclass(name = "Mobject", unsendable, subclass, from_py_object)]
#[derive(Clone)]
pub struct PyMobject {
    pub inner: Rc<RefCell<Box<dyn gmanim_core::mobjects::Mobject>>>,
}

#[pymethods]
impl PyMobject {
    #[pyo3(signature = (position))]
    fn set_position(&mut self, position: (f32, f32, f32)) -> PyResult<()> {
        let pos = nalgebra::Point3::new(position.0, position.1, position.2);
        self.inner.borrow_mut().set_position(pos);
        Ok(())
    }

    fn get_position(&self) -> PyResult<(f32, f32, f32)> {
        let pos = self.inner.borrow().get_position();
        Ok((pos.x, pos.y, pos.z))
    }
    pub fn add(&mut self, child: &pyo3::Bound<'_, PyMobject>) {
        self.inner.borrow_mut().add_child(child.borrow().inner.clone());
    }

    pub fn remove(&mut self, child: &pyo3::Bound<'_, PyMobject>) {
        self.inner.borrow_mut().remove_child(&child.borrow().inner);
    }
    
    #[getter]
    fn get_name(&self) -> PyResult<Option<String>> {
        Ok(self.inner.borrow().get_name())
    }

    #[setter]
    fn set_name(&mut self, name: String) -> PyResult<()> {
        self.inner.borrow_mut().set_name(name);
        Ok(())
    }

}

#[pyclass(name = "Scene", unsendable)]
pub struct PyScene {
    pub inner: Option<gmanim_core::animation::Timeline>,
}

#[pymethods]
impl PyScene {
    #[new]
    #[pyo3(signature = (width=None, height=None, resolution=Some((1920, 1080)), scale_factor=None))]
    fn new(
        width: Option<f32>,
        height: Option<f32>,
        resolution: Option<(u32, u32)>,
        scale_factor: Option<f32>,
    ) -> PyResult<Self> {
        let w = width.unwrap_or(16.0);
        let h = height.unwrap_or(9.0);
        let (ow, oh) = resolution.unwrap_or((1920, 1080));
        let sf = scale_factor.unwrap_or(oh as f32 / h);

        let scene_config = gmanim_core::SceneConfig {
            width: w,
            height: h,
            output_width: ow,
            output_height: oh,
            scale_factor: sf,
        };

        let ctx = gmanim_core::Context {
            scene_config,
        };

        let scene_inner = gmanim_core::Scene::new();

        Ok(PyScene {
            inner: Some(gmanim_core::animation::Timeline::new(scene_inner, ctx)),
        })
    }

    fn add(&mut self, mobj: PyMobject) -> PyResult<PyMobject> {
        let timeline = self.inner.as_mut().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Scene internal error")
        })?;
        let scene = &mut timeline.scene;

        scene.add_ref(mobj.inner.clone());
        Ok(mobj)
    }

    fn remove(&mut self, arg: &Bound<'_, PyAny>) -> PyResult<()> {
        let timeline = self.inner.as_mut().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Scene internal error")
        })?;
        let scene = &mut timeline.scene;

        if let Ok(index) = arg.extract::<usize>() {
            if index < scene.mobjects.len() {
                scene.mobjects.remove(index);
                Ok(())
            } else {
                Err(pyo3::exceptions::PyIndexError::new_err("Index out of range"))
            }
        } else if let Ok(mobj) = arg.extract::<PyMobject>() {
            if let Some(pos) = scene.mobjects.iter().position(|m| Rc::ptr_eq(m, &mobj.inner)) {
                scene.mobjects.remove(pos);
                Ok(())
            } else {
                Err(pyo3::exceptions::PyValueError::new_err("Mobject not found in Scene"))
            }
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Expected an index (int) or a Mobject"))
        }
    }
    #[pyo3(signature = (position=None, target=None, direction=None, up=None))]
    fn set_camera(
        &mut self,
        position: Option<(f32, f32, f32)>,
        target: Option<(f32, f32, f32)>,
        direction: Option<(f32, f32, f32)>,
        up: Option<(f32, f32, f32)>,
    ) -> PyResult<()> {
        let timeline = self.inner.as_mut().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Scene internal error")
        })?;
        let scene = &mut timeline.scene;
        if let Some(pos) = position {
            scene.camera.position = nalgebra::Point3::new(pos.0, pos.1, pos.2);
        }
        
        if let Some(t) = target {
            let target_point = nalgebra::Point3::new(t.0, t.1, t.2);
            let mut dir = target_point - scene.camera.position;
            if dir.norm_squared() < 1e-6 {
                dir = nalgebra::Vector3::new(0.0, 0.0, -1.0);
            }
            scene.camera.set_look_at(dir);
        } else if let Some(d) = direction {
            let dir = nalgebra::Vector3::new(d.0, d.1, d.2);
            if dir.norm_squared() >= 1e-6 {
                scene.camera.set_look_at(dir);
            }
        }
        
        if let Some(u) = up {
            let up_dir = nalgebra::Vector3::new(u.0, u.1, u.2);
            if up_dir.norm_squared() >= 1e-6 {
                scene.camera.set_up_direction(up_dir);
            }
        }
        Ok(())
    }

    #[pyo3(signature = (height=9.0, width=None, near=0.1, far=50.0))]
    fn set_orthographic_camera(&mut self, height: f32, width: Option<f32>, near: f32, far: f32) -> PyResult<()> {
        let timeline = self.inner.as_mut().ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Timeline not found"))?;
        let scene = &mut timeline.scene;
        if scene.camera.position == nalgebra::Point3::new(0.0, 0.0, 0.0) {
            scene.camera.position = nalgebra::Point3::new(0.0, 0.0, 10.0);
        }
        let w = width.unwrap_or(height * (16.0 / 9.0));
        scene.camera.set_orthographic(-w / 2.0, w / 2.0, -height / 2.0, height / 2.0, near, far);
        Ok(())
    }

    #[pyo3(signature = (fovy=1.5707964, aspect=None, near=0.1, far=50.0))]
    fn set_perspective_camera(&mut self, fovy: f32, aspect: Option<f32>, near: f32, far: f32) -> PyResult<()> {
        let timeline = self.inner.as_mut().ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Timeline not found"))?;
        let scene = &mut timeline.scene;
        let a = aspect.unwrap_or(16.0 / 9.0);
        scene.camera.set_perspective(fovy, a, near, far);
        Ok(())
    }

    #[pyo3(signature = (center_x, center_y, width, height))]
    fn set_viewport(&mut self, center_x: f32, center_y: f32, width: f32, height: f32) -> PyResult<()> {
        let timeline = self.inner.as_mut().ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Timeline not found"))?;
        let scene = &mut timeline.scene;
        scene.clip_rect = Some(gmanim_core::ClipRect::Logical(center_x, center_y, width, height));
        Ok(())
    }

    #[pyo3(signature = (x, y, width, height))]
    fn set_pixel_viewport(&mut self, x: u32, y: u32, width: u32, height: u32) -> PyResult<()> {
        let timeline = self.inner.as_mut().ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Timeline not found"))?;
        let scene = &mut timeline.scene;
        scene.clip_rect = Some(gmanim_core::ClipRect::Pixel(x, y, width, height));
        Ok(())
    }

    #[pyo3(signature = (level))]
    fn set_anti_aliasing(&mut self, level: u32) -> PyResult<()> {
        let timeline = self.inner.as_mut().ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Timeline not found"))?;
        let scene = &mut timeline.scene;
        scene.aa_level = level;
        Ok(())
    }

    fn play(&mut self, anim: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(m) = anim.extract::<pyo3::PyRef<'_, PyMove>>() {
            let rust_move = gmanim_core::animation::Move::new(
                m.target.inner.clone(),
                nalgebra::Vector3::new(m.displacement.0, m.displacement.1, m.displacement.2),
                m.frames,
            );
            self.inner.as_mut().unwrap().play(rust_move);
            Ok(())
        } else if let Ok(r) = anim.extract::<pyo3::PyRef<'_, PyRotate>>() {
            let rust_rotate = gmanim_core::animation::Rotate::new(
                r.target.inner.clone(),
                nalgebra::Vector3::new(r.axis.0, r.axis.1, r.axis.2),
                nalgebra::Point3::new(r.center.0, r.center.1, r.center.2),
                r.frames,
            );
            self.inner.as_mut().unwrap().play(rust_rotate);
            Ok(())
        } else if let Ok(w) = anim.extract::<pyo3::PyRef<'_, PyWait>>() {
            let rust_wait = gmanim_core::animation::Wait::new(w.frames);
            self.inner.as_mut().unwrap().play(rust_wait);
            Ok(())
        } else if let Ok(update) = anim.extract::<pyo3::PyRef<'_, PyUpdateFromFunc>>() {
            let rust_update = RustUpdateFromFunc {
                callback: update.callback.clone_ref(anim.py()),
                total_frames: update.frames,
                is_pure: update.is_pure,
            };
            self.inner.as_mut().unwrap().play(rust_update);
            Ok(())
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Unsupported animation type for Scene.play()"))
        }
    }

    fn wait(&mut self, frames: u32) {
        self.inner.as_mut().unwrap().play(gmanim_core::animation::Wait::new(frames));
    }

    fn run(&mut self, callback: pyo3::Py<pyo3::PyAny>) {
        self.inner.as_mut().unwrap().run(move |scene: &mut gmanim_core::Scene| {
            if let Some(_) = pyo3::Python::try_attach(|py| {
                let scene_ref = PySceneRef::borrow(scene);
                let py_scene_ref = pyo3::Bound::new(py, scene_ref).unwrap();
                if let Err(e) = callback.call1(py, (py_scene_ref,)) {
                    e.print(py);
                    panic!("Python callback failed in Scene::run");
                }
            }) {}
        });
    }

    #[pyo3(signature = (filename, fps=60, backend="ffmpeg", show_progress=true))]
    fn render(&mut self, filename: &str, fps: u32, backend: &str, show_progress: bool) -> PyResult<()> {
        let timeline = self.inner.as_mut().unwrap();
        let ow = timeline.ctx.scene_config.output_width;
        let oh = timeline.ctx.scene_config.output_height;
        let video_config = gmanim_core::video_backend::VideoConfig {
            filename: filename.to_owned(),
            framerate: fps,
            output_width: ow,
            output_height: oh,
            color_order: gmanim_core::video_backend::ColorOrder::Nv12,
        };

        let total_frames = timeline.total_frames() as u64;
        let pb = if show_progress {
            let p = indicatif::ProgressBar::new(total_frames);
            p.set_style(
                indicatif::ProgressStyle::with_template(
                    "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} frames ({eta})"
                ).unwrap().progress_chars("#>-")
            );
            Some(p)
        } else {
            None
        };

        let mut video_backend = if backend == "vaapi" {
            gmanim_core::video_backend::VideoBackend {
                backend_type: gmanim_core::video_backend::VideoBackendType::Vaapi(
                    gmanim_core::video_backend::vaapi::FfmpegVaapiBackend::new(&video_config)
                )
            }
        } else if backend == "ffmpeg" {
            gmanim_core::video_backend::VideoBackend {
                backend_type: gmanim_core::video_backend::VideoBackendType::FfmpegPipe(
                    gmanim_core::video_backend::FfmpegPipeBackend::new(
                        &video_config,
                        gmanim_core::video_backend::FfmpegPipeEncoder::Libx264,
                        false,
                    )
                )
            }
        } else {
            return Err(pyo3::exceptions::PyValueError::new_err(format!("Unsupported backend: {}", backend)));
        };

        let mut frame_count: u64 = 0;
        while timeline.step_frame() {
            let mut buf = video_backend.acquire_buffer();
            if let Some(nv12_bytes) = timeline.nv12_image_bytes() {
                buf.as_mut_slice().copy_from_slice(nv12_bytes);
            } else {
                buf.as_mut_slice().fill(0);
            }
            video_backend.submit_frame(buf);
            
            if let Some(ref p) = pb {
                frame_count += 1;
                if frame_count % 60 == 0 {
                    p.inc(60);
                }
            }
        }

        if let Some(ref p) = pb {
            let remainder = frame_count % 60;
            if remainder != 0 {
                p.inc(remainder);
            }
        }

        video_backend.close().map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))?;

        if let Some(p) = pb {
            p.finish_with_message("Render Complete");
        }

        Ok(())
    }
}

#[pyclass(name = "SceneRef", unsendable)]
pub struct PySceneRef {
    ptr: *mut gmanim_core::Scene,
}

impl PySceneRef {
    pub fn borrow(scene: &mut gmanim_core::Scene) -> Self {
        Self { ptr: scene as *mut gmanim_core::Scene }
    }
}

#[pymethods]
impl PySceneRef {
    fn add(&mut self, mobj: PyMobject) -> PyResult<PyMobject> {
        let scene = unsafe { &mut *self.ptr };
        scene.add_ref(mobj.inner.clone());
        Ok(mobj)
    }

    fn remove(&mut self, arg: &Bound<'_, PyAny>) -> PyResult<()> {
        let scene = unsafe { &mut *self.ptr };
        if let Ok(index) = arg.extract::<usize>() {
            if index < scene.mobjects.len() {
                scene.mobjects.remove(index);
                Ok(())
            } else {
                Err(pyo3::exceptions::PyIndexError::new_err("Index out of range"))
            }
        } else if let Ok(mobj) = arg.extract::<PyMobject>() {
            if let Some(pos) = scene.mobjects.iter().position(|m| Rc::ptr_eq(m, &mobj.inner)) {
                scene.mobjects.remove(pos);
                Ok(())
            } else {
                Err(pyo3::exceptions::PyValueError::new_err("Mobject not found in Scene"))
            }
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Expected an index (int) or a Mobject"))
        }
    }
    #[pyo3(signature = (position=None, target=None, direction=None, up=None))]
    fn set_camera(
        &mut self,
        position: Option<(f32, f32, f32)>,
        target: Option<(f32, f32, f32)>,
        direction: Option<(f32, f32, f32)>,
        up: Option<(f32, f32, f32)>,
    ) -> PyResult<()> {
        let scene = unsafe { &mut *self.ptr };
        if let Some(pos) = position {
            scene.camera.position = nalgebra::Point3::new(pos.0, pos.1, pos.2);
        }
        
        if let Some(t) = target {
            let target_point = nalgebra::Point3::new(t.0, t.1, t.2);
            let mut dir = target_point - scene.camera.position;
            if dir.norm_squared() < 1e-6 {
                dir = nalgebra::Vector3::new(0.0, 0.0, -1.0);
            }
            scene.camera.set_look_at(dir);
        } else if let Some(d) = direction {
            let dir = nalgebra::Vector3::new(d.0, d.1, d.2);
            if dir.norm_squared() >= 1e-6 {
                scene.camera.set_look_at(dir);
            }
        }
        
        if let Some(u) = up {
            let up_dir = nalgebra::Vector3::new(u.0, u.1, u.2);
            if up_dir.norm_squared() >= 1e-6 {
                scene.camera.set_up_direction(up_dir);
            }
        }
        Ok(())
    }

    #[pyo3(signature = (height=9.0, width=None, near=0.1, far=50.0))]
    fn set_orthographic_camera(&mut self, height: f32, width: Option<f32>, near: f32, far: f32) -> PyResult<()> {
        let scene = unsafe { &mut *self.ptr };
        if scene.camera.position == nalgebra::Point3::new(0.0, 0.0, 0.0) {
            scene.camera.position = nalgebra::Point3::new(0.0, 0.0, 10.0);
        }
        let w = width.unwrap_or(height * (16.0 / 9.0));
        scene.camera.set_orthographic(-w / 2.0, w / 2.0, -height / 2.0, height / 2.0, near, far);
        Ok(())
    }

    #[pyo3(signature = (fovy=std::f32::consts::PI/2.0, aspect=None, near=0.1, far=50.0))]
    fn set_perspective_camera(&mut self, fovy: f32, aspect: Option<f32>, near: f32, far: f32) -> PyResult<()> {
        let scene = unsafe { &mut *self.ptr };
        let a = aspect.unwrap_or(16.0 / 9.0);
        scene.camera.set_perspective(fovy, a, near, far);
        Ok(())
    }

    #[pyo3(signature = (level))]
    fn set_anti_aliasing(&mut self, level: u32) -> PyResult<()> {
        let scene = unsafe { &mut *self.ptr };
        scene.aa_level = level;
        Ok(())
    }

    #[pyo3(signature = (center_x, center_y, width, height))]
    fn set_viewport(&mut self, center_x: f32, center_y: f32, width: f32, height: f32) -> PyResult<()> {
        let scene = unsafe { &mut *self.ptr };
        scene.clip_rect = Some(gmanim_core::ClipRect::Logical(center_x, center_y, width, height));
        Ok(())
    }

    #[pyo3(signature = (x, y, width, height))]
    fn set_pixel_viewport(&mut self, x: u32, y: u32, width: u32, height: u32) -> PyResult<()> {
        let scene = unsafe { &mut *self.ptr };
        scene.clip_rect = Some(gmanim_core::ClipRect::Pixel(x, y, width, height));
        Ok(())
    }
}
