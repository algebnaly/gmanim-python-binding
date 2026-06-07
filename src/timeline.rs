// src/timeline.rs
use pyo3::prelude::*;
use crate::scene::{PyScene, PySceneRef};
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

#[pyclass(name = "Timeline", unsendable)]
pub struct PyTimeline {
    inner: gmanim_core::animation::Timeline,
}

#[pymethods]
impl PyTimeline {
    #[new]
    #[pyo3(signature = (scene, width=None, height=None, resolution=Some((1920, 1080)), scale_factor=None))]
    fn new(
        scene: &Bound<'_, PyScene>,
        width: Option<f32>,
        height: Option<f32>,
        resolution: Option<(u32, u32)>,
        scale_factor: Option<f32>,
    ) -> PyResult<Self> {
        let mut scene_borrow = scene.try_borrow_mut()?;
        let scene_inner = scene_borrow.inner.take().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Scene has already been consumed by a Timeline")
        })?;

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

        let pixmap = tiny_skia::Pixmap::new(ow, oh)
            .ok_or_else(|| pyo3::exceptions::PyValueError::new_err("Failed to create tiny-skia Pixmap"))?;

        let ctx = gmanim_core::Context {
            pixmap,
            scene_config,
        };

        Ok(PyTimeline {
            inner: gmanim_core::animation::Timeline::new(scene_inner, ctx),
        })
    }

    fn play(&mut self, anim: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(m) = anim.extract::<pyo3::PyRef<'_, PyMove>>() {
            let rust_move = gmanim_core::animation::Move::new(
                m.target.inner.clone(),
                nalgebra::Vector3::new(m.displacement.0, m.displacement.1, m.displacement.2),
                m.frames,
            );
            self.inner.play(rust_move);
            Ok(())
        } else if let Ok(r) = anim.extract::<pyo3::PyRef<'_, PyRotate>>() {
            let rust_rotate = gmanim_core::animation::Rotate::new(
                r.target.inner.clone(),
                nalgebra::Vector3::new(r.axis.0, r.axis.1, r.axis.2),
                nalgebra::Point3::new(r.center.0, r.center.1, r.center.2),
                r.frames,
            );
            self.inner.play(rust_rotate);
            Ok(())
        } else if let Ok(w) = anim.extract::<pyo3::PyRef<'_, PyWait>>() {
            let rust_wait = gmanim_core::animation::Wait::new(w.frames);
            self.inner.play(rust_wait);
            Ok(())
        } else if let Ok(update) = anim.extract::<pyo3::PyRef<'_, PyUpdateFromFunc>>() {
            let rust_update = RustUpdateFromFunc {
                callback: update.callback.clone_ref(anim.py()),
                total_frames: update.frames,
                is_pure: update.is_pure,
            };
            self.inner.play(rust_update);
            Ok(())
        } else {
            Err(pyo3::exceptions::PyTypeError::new_err("Unsupported animation type for Timeline.play()"))
        }
    }

    fn wait(&mut self, frames: u32) {
        self.inner.play(gmanim_core::animation::Wait::new(frames));
    }

    fn run(&mut self, callback: pyo3::Py<pyo3::PyAny>) {
        self.inner.run(move |scene: &mut gmanim_core::Scene| {
            if let Some(_) = pyo3::Python::try_attach(|py| {
                let scene_ref = PySceneRef::borrow(scene);
                let py_scene_ref = pyo3::Bound::new(py, scene_ref).unwrap();
                if let Err(e) = callback.call1(py, (py_scene_ref,)) {
                    e.print(py);
                    panic!("Python callback failed in Timeline::run");
                }
            }) {}
        });
    }

    #[pyo3(signature = (filename, fps=60, backend="vaapi", show_progress=true))]
    fn render(&mut self, filename: &str, fps: u32, backend: &str, show_progress: bool) -> PyResult<()> {
        let ow = self.inner.ctx.scene_config.output_width;
        let oh = self.inner.ctx.scene_config.output_height;
        let video_config = gmanim_core::video_backend::VideoConfig {
            filename: filename.to_owned(),
            framerate: fps,
            output_width: ow,
            output_height: oh,
            color_order: gmanim_core::video_backend::ColorOrder::Rgba,
        };

        let total_frames = self.inner.total_frames() as u64;
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
        self.inner.render(|ctx| {
            let mut buf = video_backend.acquire_buffer();
            ctx.copy_image_into(buf.as_mut_slice());
            video_backend.submit_frame(buf);
            
            if let Some(ref p) = pb {
                frame_count += 1;
                if frame_count % 60 == 0 {
                    p.inc(60);
                }
            }
        });

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
