pub mod animations;
pub mod ipc;
pub mod mobjects;
pub mod scene;
pub mod utils;

use pyo3::prelude::*;

use animations::{PyAnimation, PyMove, PyRotate, PyUpdateFromFunc, PyWait};
use mobjects::{
    PyArc, PyArrow3D, PyBox3D, PyBox3DSdf, PyDot, PyLine, PyLineSegment3D, PyMesh2DIn3D,
    PyPolyLine, PyRectangle, PySphere3D, PyText,
};
use scene::{PyMobject, PyScene, PySceneRef};

#[pyclass(eq, eq_int, from_py_object)]
#[derive(PartialEq, Clone, Debug)]
pub enum PyVideoBackend {
    Ffmpeg,
    Vaapi,
    Vulkan,
}

#[pymodule]
pub fn gmanim(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let code = c"
registry = {}
def scene(name):
    def decorator(func):
        registry[name] = func
        return func
    return decorator

def incremental(func):
    func.__incremental__ = True
    return func
";
    let temp_module = pyo3::types::PyModule::from_code(py, code, c"", c"")?;
    m.setattr("registry", temp_module.getattr("registry")?)?;
    m.setattr("scene", temp_module.getattr("scene")?)?;
    m.setattr("incremental", temp_module.getattr("incremental")?)?;

    if let Ok(all) = m.getattr("__all__") {
        if let Ok(all_list) = all.cast::<pyo3::types::PyList>() {
            let _ = all_list.append("registry");
            let _ = all_list.append("scene");
            let _ = all_list.append("incremental");
        }
    }

    m.add_class::<PyScene>()?;
    m.add_class::<PySceneRef>()?;
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
    m.add_class::<PyBox3DSdf>()?;
    m.add_class::<PyMesh2DIn3D>()?;
    m.add_class::<mobjects::PyTriangleMesh3D>()?;
    m.add_class::<mobjects::PyCylinder3D>()?;
    m.add_class::<mobjects::PyCone3D>()?;
    m.add_class::<mobjects::PyGroup>()?;
    m.add_class::<PyAnimation>()?;
    m.add_class::<PyMove>()?;
    m.add_class::<PyRotate>()?;
    m.add_class::<PyWait>()?;
    m.add_class::<PyUpdateFromFunc>()?;
    m.add_class::<PyVideoBackend>()?;
    Ok(())
}
