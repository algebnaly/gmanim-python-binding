// src/animations.rs
use pyo3::prelude::*;
use crate::scene::PyMobject;

#[pyclass(name = "Animation", subclass)]
pub struct PyAnimation {}

#[pymethods]
impl PyAnimation {
    #[new]
    fn new() -> Self {
        PyAnimation {}
    }
}

#[pyclass(name = "Move", extends=PyAnimation, skip_from_py_object, unsendable)]
#[derive(Clone)]
pub struct PyMove {
    pub target: PyMobject,
    pub displacement: (f32, f32, f32),
    pub frames: u32,
}

#[pymethods]
impl PyMove {
    #[new]
    #[pyo3(signature = (target, displacement, frames=60))]
    fn new(target: PyMobject, displacement: (f32, f32, f32), frames: u32) -> (Self, PyAnimation) {
        (PyMove {
            target,
            displacement,
            frames,
        }, PyAnimation {})
    }
}

#[pyclass(name = "Rotate", extends=PyAnimation, skip_from_py_object, unsendable)]
#[derive(Clone)]
pub struct PyRotate {
    pub target: PyMobject,
    pub axis: (f32, f32, f32),
    pub center: (f32, f32, f32),
    pub frames: u32,
}

#[pymethods]
impl PyRotate {
    #[new]
    #[pyo3(signature = (target, axis, center, frames=60))]
    fn new(target: PyMobject, axis: (f32, f32, f32), center: (f32, f32, f32), frames: u32) -> (Self, PyAnimation) {
        (PyRotate {
            target,
            axis,
            center,
            frames,
        }, PyAnimation {})
    }
}

#[pyclass(name = "Wait", extends=PyAnimation, skip_from_py_object)]
#[derive(Clone)]
pub struct PyWait {
    pub frames: u32,
}

#[pymethods]
impl PyWait {
    #[new]
    #[pyo3(signature = (frames=60))]
    fn new(frames: u32) -> (Self, PyAnimation) {
        (PyWait { frames }, PyAnimation {})
    }
}

#[pyclass(name = "UpdateFromFunc", extends=PyAnimation, skip_from_py_object, unsendable)]
pub struct PyUpdateFromFunc {
    pub callback: pyo3::Py<pyo3::PyAny>,
    pub frames: u32,
    pub is_pure: bool,
}

#[pymethods]
impl PyUpdateFromFunc {
    #[new]
    #[pyo3(signature = (callback, frames=60, is_pure=None))]
    fn new(py: pyo3::Python<'_>, callback: pyo3::Py<pyo3::PyAny>, frames: u32, is_pure: Option<bool>) -> (Self, PyAnimation) {
        let is_pure = is_pure.unwrap_or_else(|| {
            let is_incremental = callback.bind(py).getattr("__incremental__").map(|x| x.is_truthy().unwrap_or(false)).unwrap_or(false);
            !is_incremental
        });
        (PyUpdateFromFunc {
            callback,
            frames,
            is_pure,
        }, PyAnimation {})
    }
}
