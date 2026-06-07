// src/mobjects.rs
use pyo3::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use gmanim_core::mobjects::{SimpleLine, Rectangle, PolyLine, Arc, Dot, text::Text};
use gmanim_core::mobjects::object_3d::{Sphere3D, LineSegment3D, Arrow3D};
use gmanim_core::Color;
use crate::utils::build_draw_config;
use crate::scene::PyMobject;

#[pyclass(name = "Line", extends=PyMobject, skip_from_py_object)]
#[derive(Clone)]
pub struct PyLine {}

#[pymethods]
impl PyLine {
    #[new]
    #[pyo3(signature = (p0=None, p1=None, stroke_width=None, fill=None, color=None))]
    fn new(
        p0: Option<(f32, f32, f32)>,
        p1: Option<(f32, f32, f32)>,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let p0_val = p0.unwrap_or((-1.0, 0.0, 0.0));
        let p1_val = p1.unwrap_or((1.0, 0.0, 0.0));
        let draw_config = build_draw_config(stroke_width, fill, color);
        let core_line = Box::new(SimpleLine {
            p0: nalgebra::Point3::new(p0_val.0, p0_val.1, p0_val.2),
            p1: nalgebra::Point3::new(p1_val.0, p1_val.1, p1_val.2),
            draw_config,
            model_matrix: nalgebra::Matrix4::identity(),
        });
        (PyLine {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(gmanim_core::mobjects::scene_node::SceneNode::new(Some(core_line))))) })
    }
}

#[pyclass(name = "Rectangle", extends=PyMobject, skip_from_py_object)]
#[derive(Clone)]
pub struct PyRectangle {}

#[pymethods]
impl PyRectangle {
    #[new]
    #[pyo3(signature = (width=2.0, height=1.0, center=None, stroke_width=None, fill=None, color=None))]
    fn new(
        width: f32,
        height: f32,
        center: Option<(f32, f32, f32)>,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let draw_config = build_draw_config(stroke_width, fill, color);
        let c = center.unwrap_or((0.0, 0.0, 0.0));
        let hw = width / 2.0;
        let hh = height / 2.0;
        let core_rect = Box::new(Rectangle {
            p0: nalgebra::Point3::new(c.0 - hw, c.1 - hh, c.2),
            p1: nalgebra::Point3::new(c.0 + hw, c.1 - hh, c.2),
            p2: nalgebra::Point3::new(c.0 + hw, c.1 + hh, c.2),
            p3: nalgebra::Point3::new(c.0 - hw, c.1 + hh, c.2),
            color: Default::default(),
            draw_config,
            model_matrix: nalgebra::Matrix4::identity(),
        });
        (PyRectangle {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(gmanim_core::mobjects::scene_node::SceneNode::new(Some(core_rect))))) })
    }
}

#[pyclass(name = "PolyLine", extends=PyMobject, skip_from_py_object)]
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
        (PyPolyLine {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(gmanim_core::mobjects::scene_node::SceneNode::new(Some(core_poly))))) })
    }
}

#[pyclass(name = "Arc", extends=PyMobject, skip_from_py_object)]
#[derive(Clone)]
pub struct PyArc {}

#[pymethods]
impl PyArc {
    #[new]
    #[pyo3(signature = (center=None, start_angle=0.0, end_angle=3.14159, radius=1.0, stroke_width=None, fill=None, color=None))]
    fn new(
        center: Option<(f32, f32, f32)>,
        start_angle: f32,
        end_angle: f32,
        radius: f32,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let draw_config = build_draw_config(stroke_width, fill, color);
        let ct = center.unwrap_or((0.0, 0.0, 0.0));
        let mut arc = Arc::new(
            nalgebra::Point3::new(ct.0, ct.1, ct.2),
            start_angle,
            end_angle,
            radius,
        );
        arc.draw_config = draw_config;
        (PyArc {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(gmanim_core::mobjects::scene_node::SceneNode::new(Some(Box::new(arc)))))) })
    }
}

#[pyclass(name = "Dot", extends=PyMobject, skip_from_py_object)]
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
        (PyDot {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(gmanim_core::mobjects::scene_node::SceneNode::new(Some(core_dot))))) })
    }
}

#[pyclass(name = "Text", extends=PyMobject, skip_from_py_object)]
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
        (PyText {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(gmanim_core::mobjects::scene_node::SceneNode::new(Some(core_text))))) })
    }
}

#[pyclass(name = "Sphere3D", extends=PyMobject, skip_from_py_object)]
#[derive(Clone)]
pub struct PySphere3D {}

#[pymethods]
impl PySphere3D {
    #[new]
    #[pyo3(signature = (center=None, radius=1.0, color=None))]
    fn new(
        center: Option<(f32, f32, f32)>,
        radius: f32,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color.map(|c| Color::new(c.0, c.1, c.2, c.3)).unwrap_or_default();
        let ct = center.unwrap_or((0.0, 0.0, 0.0));
        let core_obj = Box::new(Sphere3D {
            center: nalgebra::Point3::new(0.0, 0.0, 0.0),
            radius,
            color: c,
                    model_matrix: nalgebra::Matrix4::identity(),
        });
        let mut node = gmanim_core::mobjects::scene_node::SceneNode::new(Some(core_obj));
        node.local_transform = nalgebra::Matrix4::new_translation(&nalgebra::Vector3::new(ct.0, ct.1, ct.2));
        (PySphere3D {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(node))) })
    }
}

#[pyclass(name = "LineSegment3D", extends=PyMobject, skip_from_py_object)]
#[derive(Clone)]
pub struct PyLineSegment3D {}

#[pymethods]
impl PyLineSegment3D {
    #[new]
    #[pyo3(signature = (a=None, b=None, radius=0.05, color=None))]
    fn new(
        a: Option<(f32, f32, f32)>,
        b: Option<(f32, f32, f32)>,
        radius: f32,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color.map(|c| Color::new(c.0, c.1, c.2, c.3)).unwrap_or_default();
        let pt_a = a.unwrap_or((-1.0, 0.0, 0.0));
        let pt_b = b.unwrap_or((1.0, 0.0, 0.0));
        let core_obj = Box::new(LineSegment3D {
            a: nalgebra::Point3::new(pt_a.0, pt_a.1, pt_a.2),
            b: nalgebra::Point3::new(pt_b.0, pt_b.1, pt_b.2),
            radius,
            color: c,
                    model_matrix: nalgebra::Matrix4::identity(),
        });
        (PyLineSegment3D {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(gmanim_core::mobjects::scene_node::SceneNode::new(Some(core_obj))))) })
    }
}

#[pyclass(name = "Arrow3D", extends=PyMobject, skip_from_py_object)]
#[derive(Clone)]
pub struct PyArrow3D {}

#[pymethods]
impl PyArrow3D {
    #[new]
    #[pyo3(signature = (start=None, end=None, shaft_radius=0.05, head_radius=0.1, head_length=0.3, color=None))]
    fn new(
        start: Option<(f32, f32, f32)>,
        end: Option<(f32, f32, f32)>,
        shaft_radius: f32,
        head_radius: f32,
        head_length: f32,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color.map(|c| Color::new(c.0, c.1, c.2, c.3)).unwrap_or_default();
        let pt_a = start.unwrap_or((-1.0, 0.0, 0.0));
        let pt_b = end.unwrap_or((1.0, 0.0, 0.0));
        let core_obj = Box::new(Arrow3D {
            start: nalgebra::Point3::new(pt_a.0, pt_a.1, pt_a.2),
            end: nalgebra::Point3::new(pt_b.0, pt_b.1, pt_b.2),
            shaft_radius,
            head_radius,
            head_length,
            color: c,
                    model_matrix: nalgebra::Matrix4::identity(),
        });
        (PyArrow3D {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(gmanim_core::mobjects::scene_node::SceneNode::new(Some(core_obj))))) })
    }
}

#[pyclass(name = "Box3DSdf", extends=PyMobject, skip_from_py_object)]
#[derive(Clone)]
pub struct PyBox3DSdf {}

#[pymethods]
impl PyBox3DSdf {
    #[new]
    #[pyo3(signature = (center=None, size=None, color=None))]
    fn new(
        center: Option<(f32, f32, f32)>,
        size: Option<(f32, f32, f32)>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color.map(|c| Color::new(c.0, c.1, c.2, c.3)).unwrap_or_default();
        let ct = center.unwrap_or((0.0, 0.0, 0.0));
        let sz = size.unwrap_or((1.0, 1.0, 1.0));
        let core_obj = Box::new(gmanim_core::mobjects::object_3d::Box3DSdf {
            center: nalgebra::Point3::new(0.0, 0.0, 0.0),
            size: nalgebra::Vector3::new(sz.0, sz.1, sz.2),
            x_axis: nalgebra::Vector3::new(1.0, 0.0, 0.0),
            y_axis: nalgebra::Vector3::new(0.0, 1.0, 0.0),
            z_axis: nalgebra::Vector3::new(0.0, 0.0, 1.0),
            color: c,
                    model_matrix: nalgebra::Matrix4::identity(),
        });
        let mut node = gmanim_core::mobjects::scene_node::SceneNode::new(Some(core_obj));
        node.local_transform = nalgebra::Matrix4::new_translation(&nalgebra::Vector3::new(ct.0, ct.1, ct.2));
        (PyBox3DSdf {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(node))) })
    }
}

#[pyclass(name = "Box3D", extends=PyMobject, skip_from_py_object)]
#[derive(Clone)]
pub struct PyBox3D {}

#[pymethods]
impl PyBox3D {
    #[new]
    #[pyo3(signature = (center=None, size=None, color=None))]
    fn new(
        center: Option<(f32, f32, f32)>,
        size: Option<(f32, f32, f32)>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color.map(|c| Color::new(c.0, c.1, c.2, c.3)).unwrap_or_default();
        let ct = center.unwrap_or((0.0, 0.0, 0.0));
        let sz = size.unwrap_or((1.0, 1.0, 1.0));
        let core_obj = Box::new(gmanim_core::mobjects::mesh_3d::TriangleMesh3D::box_mesh(
            nalgebra::Point3::new(0.0, 0.0, 0.0),
            nalgebra::Vector3::new(sz.0, sz.1, sz.2),
            c,
        ));
        let mut node = gmanim_core::mobjects::scene_node::SceneNode::new(Some(core_obj));
        node.local_transform = nalgebra::Matrix4::new_translation(&nalgebra::Vector3::new(ct.0, ct.1, ct.2));
        (PyBox3D {}, PyMobject { inner: Rc::new(RefCell::new(Box::new(node))) })
    }
}


#[pyclass(name = "TriangleMesh3D", extends=PyMobject, skip_from_py_object)]
#[derive(Clone)]
pub struct PyTriangleMesh3D;

#[pymethods]
impl PyTriangleMesh3D {
    #[new]
    #[pyo3(signature = (vertices, normals, colors, indices, color=(1.0, 1.0, 1.0, 1.0), model_matrix=None))]
    fn new(
        vertices: Vec<[f32; 3]>,
        normals: Vec<[f32; 3]>,
        colors: Vec<[f32; 4]>,
        indices: Vec<u32>,
        color: (f32, f32, f32, f32),
        model_matrix: Option<[[f32; 4]; 4]>,
    ) -> (Self, PyMobject) {
        let mut mesh_verts = Vec::new();
        for i in 0..vertices.len() {
            let p = vertices.get(i).copied().unwrap_or([0.0, 0.0, 0.0]);
            let n = normals.get(i).copied().unwrap_or([0.0, 0.0, 1.0]);
            let c = colors.get(i).copied().unwrap_or([color.0, color.1, color.2, color.3]);
            mesh_verts.push(gmanim_core::mobjects::mesh_3d::Vertex {
                position: p,
                normal: n,
                color: c,
            });
        }
        let mut mesh = gmanim_core::mobjects::mesh_3d::TriangleMesh3D::new(mesh_verts, indices);
        if let Some(mat) = model_matrix {
            let nalgebra_mat = nalgebra::Matrix4::from_row_slice(&[
                mat[0][0], mat[0][1], mat[0][2], mat[0][3],
                mat[1][0], mat[1][1], mat[1][2], mat[1][3],
                mat[2][0], mat[2][1], mat[2][2], mat[2][3],
                mat[3][0], mat[3][1], mat[3][2], mat[3][3],
            ]);
            gmanim_core::mobjects::Transform::set_model_matrix(&mut mesh, nalgebra_mat);
        }
        (
            PyTriangleMesh3D,
            PyMobject {
                inner: std::rc::Rc::new(std::cell::RefCell::new(Box::new(gmanim_core::mobjects::scene_node::SceneNode::new(Some(Box::new(mesh)))))),
            },
        )
    }
}
