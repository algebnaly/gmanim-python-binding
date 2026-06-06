// src/scene.rs
use std::cell::RefCell;
use std::rc::Rc;
use pyo3::prelude::*;
use crate::mobjects::{PyLine, PyRectangle, PyPolyLine, PyArc, PyDot, PyText};

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
}

#[pyclass(name = "Scene", unsendable)]
pub struct PyScene {
    pub inner: Option<gmanim_core::Scene>,
}

#[pymethods]
impl PyScene {
    #[new]
    fn new() -> Self {
        PyScene {
            inner: Some(gmanim_core::Scene::new()),
        }
    }

    fn add(&mut self, mobj: PyMobject) -> PyResult<PyMobject> {
        let scene = self.inner.as_mut().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Scene has been consumed by a Timeline")
        })?;

        scene.add_ref(mobj.inner.clone());
        Ok(mobj)
    }

    fn remove(&mut self, arg: &Bound<'_, PyAny>) -> PyResult<()> {
        let scene = self.inner.as_mut().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Scene has been consumed by a Timeline")
        })?;

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
        let scene = self.inner.as_mut().unwrap();
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
        let scene = self.inner.as_mut().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Scene has been consumed by a Timeline")
        })?;
        if scene.camera.position == nalgebra::Point3::new(0.0, 0.0, 0.0) {
            scene.camera.position = nalgebra::Point3::new(0.0, 0.0, 10.0);
        }
        let w = width.unwrap_or(height * (16.0 / 9.0));
        scene.camera.set_orthographic(-w / 2.0, w / 2.0, -height / 2.0, height / 2.0, near, far);
        Ok(())
    }

    #[pyo3(signature = (fovy=1.5707964, aspect=None, near=0.1, far=50.0))]
    fn set_perspective_camera(&mut self, fovy: f32, aspect: Option<f32>, near: f32, far: f32) -> PyResult<()> {
        let scene = self.inner.as_mut().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Scene has been consumed by a Timeline")
        })?;
        let a = aspect.unwrap_or(16.0 / 9.0);
        scene.camera.set_perspective(fovy, a, near, far);
        Ok(())
    }

    #[pyo3(signature = (center_x, center_y, width, height))]
    fn set_viewport(&mut self, center_x: f32, center_y: f32, width: f32, height: f32) -> PyResult<()> {
        let scene = self.inner.as_mut().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Scene has been consumed by a Timeline")
        })?;
        scene.clip_rect = Some(gmanim_core::ClipRect::Logical(center_x, center_y, width, height));
        Ok(())
    }

    #[pyo3(signature = (x, y, width, height))]
    fn set_pixel_viewport(&mut self, x: u32, y: u32, width: u32, height: u32) -> PyResult<()> {
        let scene = self.inner.as_mut().ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Scene has been consumed by a Timeline")
        })?;
        scene.clip_rect = Some(gmanim_core::ClipRect::Pixel(x, y, width, height));
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
