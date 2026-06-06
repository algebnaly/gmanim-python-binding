// src/mobjects.rs
use pyo3::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use gmanim_core::mobjects::{SimpleLine, Rectangle, PolyLine, Arc, Dot, text::Text};
use gmanim_core::mobjects::object_3d::{Sphere3D, LineSegment3D, Arrow3D, Box3D};
use gmanim_core::Color;
use crate::utils::build_draw_config;
use crate::scene::PyMobject;

#[pyclass(name = "Line", extends=PyMobject)]
#[derive(Clone)]
pub struct PyLine {}

#[pymethods]
impl PyLine {
    #[new]
    #[pyo3(signature = (p0, p1, stroke_width=None, fill=None, color=None))]
    fn new(
        p0: (f32, f32, f32),
        p1: (f32, f32, f32),
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let draw_config = build_draw_config(stroke_width, fill, color);
        let core_line = Box::new(SimpleLine {
            p0: nalgebra::Point3::new(p0.0, p0.1, p0.2),
            p1: nalgebra::Point3::new(p1.0, p1.1, p1.2),
            draw_config,
                    model_matrix: nalgebra::Matrix4::identity(),
        });
        (PyLine {}, PyMobject { inner: Rc::new(RefCell::new(core_line)) })
    }
}

#[pyclass(name = "Rectangle", extends=PyMobject)]
#[derive(Clone)]
pub struct PyRectangle {}

#[pymethods]
impl PyRectangle {
    #[new]
    #[pyo3(signature = (p0, p1, p2, p3, stroke_width=None, fill=None, color=None))]
    fn new(
        p0: (f32, f32, f32),
        p1: (f32, f32, f32),
        p2: (f32, f32, f32),
        p3: (f32, f32, f32),
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let draw_config = build_draw_config(stroke_width, fill, color);
        let core_rect = Box::new(Rectangle {
            p0: nalgebra::Point3::new(p0.0, p0.1, p0.2),
            p1: nalgebra::Point3::new(p1.0, p1.1, p1.2),
            p2: nalgebra::Point3::new(p2.0, p2.1, p2.2),
            p3: nalgebra::Point3::new(p3.0, p3.1, p3.2),
            color: Default::default(),
            draw_config,
            model_matrix: nalgebra::Matrix4::identity(),
        });
        (PyRectangle {}, PyMobject { inner: Rc::new(RefCell::new(core_rect)) })
    }
}

#[pyclass(name = "PolyLine", extends=PyMobject)]
#[derive(Clone)]
pub struct PyPolyLine {}

#[pymethods]
impl PyPolyLine {
    #[new]
    #[pyo3(signature = (points, stroke_width=None, fill=None, color=None))]
    fn new(
        points: Vec<(f32, f32, f32)>,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let draw_config = build_draw_config(stroke_width, fill, color);
        let core_poly = Box::new(PolyLine {
            points: points.iter().map(|p| nalgebra::Point3::new(p.0, p.1, p.2)).collect(),
            draw_config,
                    model_matrix: nalgebra::Matrix4::identity(),
        });
        (PyPolyLine {}, PyMobject { inner: Rc::new(RefCell::new(core_poly)) })
    }
}

#[pyclass(name = "Arc", extends=PyMobject)]
#[derive(Clone)]
pub struct PyArc {}

#[pymethods]
impl PyArc {
    #[new]
    #[pyo3(signature = (center, start_angle, end_angle, radius, stroke_width=None, fill=None, color=None))]
    fn new(
        center: (f32, f32, f32),
        start_angle: f32,
        end_angle: f32,
        radius: f32,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let draw_config = build_draw_config(stroke_width, fill, color);
        let mut arc = Arc::new(
            nalgebra::Point3::new(center.0, center.1, center.2),
            start_angle,
            end_angle,
            radius,
        );
        arc.draw_config = draw_config;
        (PyArc {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(arc))) })
    }
}

#[pyclass(name = "Dot", extends=PyMobject)]
#[derive(Clone)]
pub struct PyDot {}

#[pymethods]
impl PyDot {
    #[new]
    #[pyo3(signature = (position=(0.0,0.0,0.0), radius=0.05, stroke_width=None, fill=None, color=None))]
    fn new(
        position: (f32, f32, f32),
        radius: f32,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let draw_config = build_draw_config(stroke_width, fill, color);
        let c = color.map(|c| Color::new(c.0, c.1, c.2, c.3)).unwrap_or_default();
        let core_dot = Box::new(Dot::new(
            nalgebra::Point3::new(position.0, position.1, position.2),
            radius,
            c,
            draw_config,
        ));
        (PyDot {}, PyMobject { inner: Rc::new(RefCell::new(core_dot)) })
    }
}

#[pyclass(name = "Text", extends=PyMobject)]
#[derive(Clone)]
pub struct PyText {}

#[pymethods]
impl PyText {
    #[new]
    #[pyo3(signature = (text, position=(0.0,0.0,0.0), font_size=32.0, stroke_width=None, fill=None, color=None))]
    fn new(
        text: String,
        position: (f32, f32, f32),
        font_size: f32,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let draw_config = build_draw_config(stroke_width, fill, color);
        let core_text = Box::new(Text::new(
            text,
            nalgebra::Point3::new(position.0, position.1, position.2),
            font_size,
            draw_config,
        ));
        (PyText {}, PyMobject { inner: Rc::new(RefCell::new(core_text)) })
    }
}

#[pyclass(name = "Sphere3D", extends=PyMobject)]
#[derive(Clone)]
pub struct PySphere3D {}

#[pymethods]
impl PySphere3D {
    #[new]
    #[pyo3(signature = (center, radius, color=None))]
    fn new(
        center: (f32, f32, f32),
        radius: f32,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color.map(|c| Color::new(c.0, c.1, c.2, c.3)).unwrap_or_default();
        let core_obj = Box::new(Sphere3D {
            center: nalgebra::Point3::new(center.0, center.1, center.2),
            radius,
            color: c,
                    model_matrix: nalgebra::Matrix4::identity(),
        });
        (PySphere3D {}, PyMobject { inner: Rc::new(RefCell::new(core_obj)) })
    }
}

#[pyclass(name = "LineSegment3D", extends=PyMobject)]
#[derive(Clone)]
pub struct PyLineSegment3D {}

#[pymethods]
impl PyLineSegment3D {
    #[new]
    #[pyo3(signature = (a, b, radius, color=None))]
    fn new(
        a: (f32, f32, f32),
        b: (f32, f32, f32),
        radius: f32,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color.map(|c| Color::new(c.0, c.1, c.2, c.3)).unwrap_or_default();
        let core_obj = Box::new(LineSegment3D {
            a: nalgebra::Point3::new(a.0, a.1, a.2),
            b: nalgebra::Point3::new(b.0, b.1, b.2),
            radius,
            color: c,
                    model_matrix: nalgebra::Matrix4::identity(),
        });
        (PyLineSegment3D {}, PyMobject { inner: Rc::new(RefCell::new(core_obj)) })
    }
}

#[pyclass(name = "Arrow3D", extends=PyMobject)]
#[derive(Clone)]
pub struct PyArrow3D {}

#[pymethods]
impl PyArrow3D {
    #[new]
    #[pyo3(signature = (start, end, shaft_radius, head_radius, head_length, color=None))]
    fn new(
        start: (f32, f32, f32),
        end: (f32, f32, f32),
        shaft_radius: f32,
        head_radius: f32,
        head_length: f32,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color.map(|c| Color::new(c.0, c.1, c.2, c.3)).unwrap_or_default();
        let core_obj = Box::new(Arrow3D {
            start: nalgebra::Point3::new(start.0, start.1, start.2),
            end: nalgebra::Point3::new(end.0, end.1, end.2),
            shaft_radius,
            head_radius,
            head_length,
            color: c,
                    model_matrix: nalgebra::Matrix4::identity(),
        });
        (PyArrow3D {}, PyMobject { inner: Rc::new(RefCell::new(core_obj)) })
    }
}

#[pyclass(name = "Box3D", extends=PyMobject)]
#[derive(Clone)]
pub struct PyBox3D {}

#[pymethods]
impl PyBox3D {
    #[new]
    #[pyo3(signature = (center, size, color=None))]
    fn new(
        center: (f32, f32, f32),
        size: (f32, f32, f32),
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color.map(|c| Color::new(c.0, c.1, c.2, c.3)).unwrap_or_default();
        let core_obj = Box::new(Box3D {
            center: nalgebra::Point3::new(center.0, center.1, center.2),
            size: nalgebra::Vector3::new(size.0, size.1, size.2),
            x_axis: nalgebra::Vector3::new(1.0, 0.0, 0.0),
            y_axis: nalgebra::Vector3::new(0.0, 1.0, 0.0),
            z_axis: nalgebra::Vector3::new(0.0, 0.0, 1.0),
            color: c,
                    model_matrix: nalgebra::Matrix4::identity(),
        });
        (PyBox3D {}, PyMobject { inner: Rc::new(RefCell::new(core_obj)) })
    }
}
