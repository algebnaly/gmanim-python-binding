use pyo3::prelude::*;

use crate::scene::PyMobject;

pub enum PyAnimationSpec {
    Move {
        target: PyMobject,
        displacement: (f32, f32, f32),
        frames: u32,
    },
    Rotate {
        target: PyMobject,
        axis: (f32, f32, f32),
        center: (f32, f32, f32),
        frames: u32,
    },
    Wait {
        frames: u32,
    },
    UpdateFromFunc {
        callback: Py<PyAny>,
        frames: u32,
    },
}

#[pyclass(name = "Animation", subclass)]
pub struct PyAnimation {
    pub spec: PyAnimationSpec,
}

#[pyclass(name = "Move", extends=PyAnimation, skip_from_py_object)]
pub struct PyMove;

#[pymethods]
impl PyMove {
    #[new]
    #[pyo3(signature = (target, displacement, frames=60))]
    fn new(target: PyMobject, displacement: (f32, f32, f32), frames: u32) -> (Self, PyAnimation) {
        (
            Self,
            PyAnimation {
                spec: PyAnimationSpec::Move {
                    target,
                    displacement,
                    frames,
                },
            },
        )
    }
}

#[pyclass(name = "Rotate", extends=PyAnimation, skip_from_py_object)]
pub struct PyRotate;

#[pymethods]
impl PyRotate {
    #[new]
    #[pyo3(signature = (target, axis, center, frames=60))]
    fn new(
        target: PyMobject,
        axis: (f32, f32, f32),
        center: (f32, f32, f32),
        frames: u32,
    ) -> (Self, PyAnimation) {
        (
            Self,
            PyAnimation {
                spec: PyAnimationSpec::Rotate {
                    target,
                    axis,
                    center,
                    frames,
                },
            },
        )
    }
}

#[pyclass(name = "Wait", extends=PyAnimation, skip_from_py_object)]
pub struct PyWait;

#[pymethods]
impl PyWait {
    #[new]
    #[pyo3(signature = (frames=60))]
    fn new(frames: u32) -> (Self, PyAnimation) {
        (
            Self,
            PyAnimation {
                spec: PyAnimationSpec::Wait { frames },
            },
        )
    }
}

#[pyclass(name = "UpdateFromFunc", extends=PyAnimation, skip_from_py_object)]
pub struct PyUpdateFromFunc;

#[pymethods]
impl PyUpdateFromFunc {
    #[new]
    #[pyo3(signature = (callback, frames=60))]
    fn new(callback: Py<PyAny>, frames: u32) -> (Self, PyAnimation) {
        (
            Self,
            PyAnimation {
                spec: PyAnimationSpec::UpdateFromFunc { callback, frames },
            },
        )
    }
}
