// src/lib.rs
mod utils;
mod scene;
mod mobjects;
mod animations;
mod timeline;

use pyo3::prelude::*;

use scene::{PyScene, PySceneRef, PyMobject};
use mobjects::{PyLine, PyRectangle, PyPolyLine, PyArc, PyDot, PyText, PySphere3D, PyLineSegment3D, PyArrow3D, PyBox3D};
use animations::{PyAnimation, PyMove, PyRotate, PyWait, PyUpdateFromFunc};
use timeline::PyTimeline;

#[pymodule]
fn gmanim(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyScene>()?;
    m.add_class::<PySceneRef>()?;
    m.add_class::<PyTimeline>()?;
    m.add_class::<PyMobject>()?;
    m.add_class::<PyLine>()?;
    m.add_class::<PyRectangle>()?;
    m.add_class::<PyPolyLine>()?;
    m.add_class::<PyArc>()?;
    m.add_class::<PyDot>()?;
    m.add_class::<PyText>()?;
    m.add_class::<PySphere3D>()?;
    m.add_class::<PyLineSegment3D>()?;
    m.add_class::<PyArrow3D>()?;
    m.add_class::<PyBox3D>()?;
    m.add_class::<PyAnimation>()?;
    m.add_class::<PyMove>()?;
    m.add_class::<PyRotate>()?;
    m.add_class::<PyWait>()?;
    m.add_class::<PyUpdateFromFunc>()?;
    Ok(())
}
