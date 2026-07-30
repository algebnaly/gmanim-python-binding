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
use scene::{PyMobject, PyPreviewSession, PyScene, PySceneFrame};

#[pyclass(eq, eq_int, from_py_object)]
#[derive(PartialEq, Clone, Debug)]
pub enum PyVideoBackend {
    Ffmpeg,
    Vaapi,
    Vulkan,
}

#[pyclass(name = "H264RateControl", eq, eq_int, from_py_object)]
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub enum PyH264RateControl {
    #[default]
    Vbr,
    Cbr,
    Disabled,
}

#[pyclass(name = "VulkanH264Config", get_all, from_py_object)]
#[derive(Clone, Debug)]
pub struct PyVulkanH264Config {
    pub use_p_frames: bool,
    pub gop_size: u32,
    pub rate_control: PyH264RateControl,
}

impl Default for PyVulkanH264Config {
    fn default() -> Self {
        Self {
            use_p_frames: true,
            gop_size: 60,
            rate_control: PyH264RateControl::Vbr,
        }
    }
}

#[pymethods]
impl PyVulkanH264Config {
    #[new]
    #[pyo3(signature = (use_p_frames=true, gop_size=60, rate_control=PyH264RateControl::Vbr))]
    fn new(use_p_frames: bool, gop_size: u32, rate_control: PyH264RateControl) -> Self {
        Self {
            use_p_frames,
            gop_size,
            rate_control,
        }
    }
}

impl PyVulkanH264Config {
    pub fn to_core(&self) -> gmanim_core::video_backend::vulkan_h264::VulkanH264EncoderConfig {
        use gmanim_core::video_backend::vulkan_h264::{
            H264RateControlPolicy, VulkanH264EncoderConfig,
        };

        VulkanH264EncoderConfig {
            use_p_frames: self.use_p_frames,
            gop_size: self.gop_size,
            rate_control: match self.rate_control {
                PyH264RateControl::Vbr => H264RateControlPolicy::Vbr,
                PyH264RateControl::Cbr => H264RateControlPolicy::Cbr,
                PyH264RateControl::Disabled => H264RateControlPolicy::Disabled,
            },
        }
    }
}

#[pymodule]
pub fn gmanim(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyScene>()?;
    m.add_class::<PyPreviewSession>()?;
    m.add_class::<PySceneFrame>()?;
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
    m.add_class::<PyH264RateControl>()?;
    m.add_class::<PyVulkanH264Config>()?;
    Ok(())
}
