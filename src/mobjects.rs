// src/mobjects.rs
use crate::scene::PyMobject;
use crate::utils::build_draw_config;
use gmanim_core::Color;
use gmanim_core::mobjects::object_3d::{Arrow3D, LineSegment3D, Sphere3D};
use gmanim_core::mobjects::wrapper_3d::Wrapper2DIn3D;
use gmanim_core::mobjects::{Arc, Dot, PolyLine, Rectangle, SimpleLine, text::Text};
use pyo3::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[pyclass(name = "Line", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyLine {
    pub concrete: Rc<RefCell<SimpleLine>>,
}

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
        let mut core_line = SimpleLine {
            base: gmanim_core::mobjects::MobjectBase::new("Line"),
            p0: nalgebra::Point3::new(p0_val.0, p0_val.1, p0_val.2),
            p1: nalgebra::Point3::new(p1_val.0, p1_val.1, p1_val.2),
            draw_config,
            mesh: Default::default(),
        };
        core_line.update_mesh();
        let concrete = Rc::new(RefCell::new(core_line));
        (
            PyLine {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Rectangle", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyRectangle {
    pub concrete: Rc<RefCell<Rectangle>>,
}

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
        let w = width / 2.0;
        let h = height / 2.0;
        let mut core_rect = Rectangle {
            base: gmanim_core::mobjects::MobjectBase::new("Rectangle"),
            p0: nalgebra::Point3::new(c.0 - w, c.1 + h, c.2),
            p1: nalgebra::Point3::new(c.0 - w, c.1 - h, c.2),
            p2: nalgebra::Point3::new(c.0 + w, c.1 - h, c.2),
            p3: nalgebra::Point3::new(c.0 + w, c.1 + h, c.2),
            color: Color::default(),
            draw_config,
            mesh: Default::default(),
        };
        core_rect.update_mesh();
        let concrete = Rc::new(RefCell::new(core_rect));
        (
            PyRectangle {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "PolyLine", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyPolyLine {
    pub concrete: Rc<RefCell<PolyLine>>,
}

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
        let pts: Vec<_> = points
            .iter()
            .map(|p| nalgebra::Point3::new(p.0, p.1, p.2))
            .collect();
        let draw_config = build_draw_config(stroke_width, fill, color);
        let mut core_pline = PolyLine {
            base: gmanim_core::mobjects::MobjectBase::new("PolyLine"),
            points: pts,
            draw_config,
            mesh: Default::default(),
        };
        core_pline.update_mesh();
        let concrete = Rc::new(RefCell::new(core_pline));
        (
            PyPolyLine {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Arc", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyArc {
    pub concrete: Rc<RefCell<Arc>>,
}

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
        let concrete = Rc::new(RefCell::new(arc));
        (
            PyArc {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Dot", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyDot {
    pub concrete: Rc<RefCell<Dot>>,
}

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
        let c = color
            .map(|c| Color::new(c.0, c.1, c.2, c.3))
            .unwrap_or_default();
        let core_dot = Dot::new(
            nalgebra::Point3::new(position.0, position.1, position.2),
            radius,
            c,
            draw_config,
        );
        let concrete = Rc::new(RefCell::new(core_dot));
        (
            PyDot {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Text", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyText {
    pub concrete: Rc<RefCell<Text>>,
}

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
        let core_text = Text::new(
            text,
            nalgebra::Point3::new(position.0, position.1, position.2),
            font_size,
            draw_config,
        );
        let concrete = Rc::new(RefCell::new(core_text));
        (
            PyText {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Mesh2DIn3D", extends=PyMobject, subclass, skip_from_py_object, unsendable)]
pub struct PyMesh2DIn3D {
    pub concrete: Rc<RefCell<Wrapper2DIn3D>>,
}

#[pymethods]
impl PyMesh2DIn3D {
    #[new]
    fn new(inner: &pyo3::Bound<'_, PyMobject>) -> (Self, PyMobject) {
        let inner_mobj = inner.borrow().inner.clone();
        let wrapper = Wrapper2DIn3D::new("Mesh2DIn3D", inner_mobj);
        let concrete = Rc::new(RefCell::new(wrapper));
        (
            PyMesh2DIn3D {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Sphere3D", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PySphere3D {
    pub concrete: Rc<RefCell<Sphere3D>>,
}

#[pymethods]
impl PySphere3D {
    #[new]
    #[pyo3(signature = (center=None, radius=1.0, color=None))]
    fn new(
        center: Option<(f32, f32, f32)>,
        radius: f32,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color
            .map(|c| Color::new(c.0, c.1, c.2, c.3))
            .unwrap_or_default();
        let ct = center.unwrap_or((0.0, 0.0, 0.0));
        let core_obj = Sphere3D {
            base: gmanim_core::mobjects::MobjectBase::new("Sphere3D"),
            radius,
            color: c,
        };
        let concrete = Rc::new(RefCell::new(core_obj));
        {
            use gmanim_core::mobjects::Transform;
            concrete
                .borrow_mut()
                .apply_transform(nalgebra::Matrix4::new_translation(&nalgebra::Vector3::new(
                    ct.0, ct.1, ct.2,
                )));
        }
        (
            PySphere3D {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "LineSegment3D", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyLineSegment3D {
    pub concrete: Rc<RefCell<LineSegment3D>>,
}

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
        let c = color
            .map(|c| Color::new(c.0, c.1, c.2, c.3))
            .unwrap_or_default();
        let pt_a = a.unwrap_or((-1.0, 0.0, 0.0));
        let pt_b = b.unwrap_or((1.0, 0.0, 0.0));
        let core_obj = LineSegment3D {
            base: gmanim_core::mobjects::MobjectBase::new("LineSegment3D"),
            a: nalgebra::Point3::new(pt_a.0, pt_a.1, pt_a.2),
            b: nalgebra::Point3::new(pt_b.0, pt_b.1, pt_b.2),
            radius,
            color: c,
        };
        let concrete = Rc::new(RefCell::new(core_obj));
        (
            PyLineSegment3D {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Arrow3D", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyArrow3D {
    pub concrete: Rc<RefCell<Arrow3D>>,
}

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
        let c = color
            .map(|c| Color::new(c.0, c.1, c.2, c.3))
            .unwrap_or_default();
        let pt_a = start.unwrap_or((-1.0, 0.0, 0.0));
        let pt_b = end.unwrap_or((1.0, 0.0, 0.0));
        let core_obj = Arrow3D {
            base: gmanim_core::mobjects::MobjectBase::new("Arrow3D"),
            start: nalgebra::Point3::new(pt_a.0, pt_a.1, pt_a.2),
            end: nalgebra::Point3::new(pt_b.0, pt_b.1, pt_b.2),
            shaft_radius,
            head_radius,
            head_length,
            color: c,
        };
        let concrete = Rc::new(RefCell::new(core_obj));
        (
            PyArrow3D {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Box3DSdf", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyBox3DSdf {
    pub concrete: Rc<RefCell<gmanim_core::mobjects::object_3d::Box3DSdf>>,
}

#[pymethods]
impl PyBox3DSdf {
    #[new]
    #[pyo3(signature = (center=None, size=None, color=None))]
    fn new(
        center: Option<(f32, f32, f32)>,
        size: Option<(f32, f32, f32)>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color
            .map(|c| Color::new(c.0, c.1, c.2, c.3))
            .unwrap_or_default();
        let ct = center.unwrap_or((0.0, 0.0, 0.0));
        let sz = size.unwrap_or((1.0, 1.0, 1.0));
        let core_obj = gmanim_core::mobjects::object_3d::Box3DSdf {
            base: gmanim_core::mobjects::MobjectBase::new("Box3DSdf"),
            size: nalgebra::Vector3::new(sz.0, sz.1, sz.2),
            x_axis: nalgebra::Vector3::new(1.0, 0.0, 0.0),
            y_axis: nalgebra::Vector3::new(0.0, 1.0, 0.0),
            z_axis: nalgebra::Vector3::new(0.0, 0.0, 1.0),
            color: c,
        };
        let concrete = Rc::new(RefCell::new(core_obj));
        {
            use gmanim_core::mobjects::Transform;
            concrete
                .borrow_mut()
                .apply_transform(nalgebra::Matrix4::new_translation(&nalgebra::Vector3::new(
                    ct.0, ct.1, ct.2,
                )));
        }
        (
            PyBox3DSdf {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Box3D", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyBox3D {
    pub concrete: Rc<RefCell<gmanim_core::mobjects::mesh_3d::TriangleMesh3D>>,
}

#[pymethods]
impl PyBox3D {
    #[new]
    #[pyo3(signature = (center=None, size=None, color=None))]
    fn new(
        center: Option<(f32, f32, f32)>,
        size: Option<(f32, f32, f32)>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color
            .map(|c| Color::new(c.0, c.1, c.2, c.3))
            .unwrap_or_default();
        let ct = center.unwrap_or((0.0, 0.0, 0.0));
        let sz = size.unwrap_or((1.0, 1.0, 1.0));
        let mut core_obj = gmanim_core::mobjects::mesh_3d::TriangleMesh3D::box_mesh(
            nalgebra::Point3::new(0.0, 0.0, 0.0),
            nalgebra::Vector3::new(sz.0, sz.1, sz.2),
            c,
        );
        {
            use gmanim_core::mobjects::Transform;
            core_obj.apply_transform(nalgebra::Matrix4::new_translation(&nalgebra::Vector3::new(
                ct.0, ct.1, ct.2,
            )));
        }
        let concrete = Rc::new(RefCell::new(core_obj));
        (
            PyBox3D {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "TriangleMesh3D", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyTriangleMesh3D {
    pub concrete: Rc<RefCell<gmanim_core::mobjects::mesh_3d::TriangleMesh3D>>,
}

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
            let c = colors
                .get(i)
                .copied()
                .unwrap_or([color.0, color.1, color.2, color.3]);
            mesh_verts.push(gmanim_core::mobjects::mesh_3d::Vertex {
                position: p,
                normal: n,
                color: c,
            });
        }
        let mut mesh = gmanim_core::mobjects::mesh_3d::TriangleMesh3D::new(mesh_verts, indices);
        if let Some(mat) = model_matrix {
            let nalgebra_mat = nalgebra::Matrix4::from_row_slice(&[
                mat[0][0], mat[0][1], mat[0][2], mat[0][3], mat[1][0], mat[1][1], mat[1][2],
                mat[1][3], mat[2][0], mat[2][1], mat[2][2], mat[2][3], mat[3][0], mat[3][1],
                mat[3][2], mat[3][3],
            ]);
            use gmanim_core::mobjects::Mobject;
            mesh.set_model_matrix(nalgebra_mat);
        }
        let concrete = Rc::new(RefCell::new(mesh));
        (
            PyTriangleMesh3D {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Cylinder3D", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyCylinder3D {
    pub concrete: Rc<RefCell<gmanim_core::mobjects::mesh_3d::TriangleMesh3D>>,
}

#[pymethods]
impl PyCylinder3D {
    #[new]
    #[pyo3(signature = (start=None, end=None, radius=1.0, segments=32, color=None))]
    fn new(
        start: Option<(f32, f32, f32)>,
        end: Option<(f32, f32, f32)>,
        radius: f32,
        segments: u32,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color
            .map(|c| Color::new(c.0, c.1, c.2, c.3))
            .unwrap_or_default();
        let s = start.unwrap_or((0.0, 0.0, 0.0));
        let e = end.unwrap_or((0.0, 1.0, 0.0));

        let core_obj = gmanim_core::mobjects::mesh_3d::TriangleMesh3D::cylinder(
            nalgebra::Point3::new(s.0, s.1, s.2),
            nalgebra::Point3::new(e.0, e.1, e.2),
            radius,
            segments,
            c,
        );

        let concrete = Rc::new(RefCell::new(core_obj));
        (
            PyCylinder3D {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Cone3D", extends=PyMobject, skip_from_py_object, unsendable)]
pub struct PyCone3D {
    pub concrete: Rc<RefCell<gmanim_core::mobjects::mesh_3d::TriangleMesh3D>>,
}

#[pymethods]
impl PyCone3D {
    #[new]
    #[pyo3(signature = (base_center=None, tip=None, radius=1.0, segments=32, color=None))]
    fn new(
        base_center: Option<(f32, f32, f32)>,
        tip: Option<(f32, f32, f32)>,
        radius: f32,
        segments: u32,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let c = color
            .map(|c| Color::new(c.0, c.1, c.2, c.3))
            .unwrap_or_default();
        let b = base_center.unwrap_or((0.0, 0.0, 0.0));
        let t = tip.unwrap_or((0.0, 1.0, 0.0));

        let core_obj = gmanim_core::mobjects::mesh_3d::TriangleMesh3D::cone(
            nalgebra::Point3::new(b.0, b.1, b.2),
            nalgebra::Point3::new(t.0, t.1, t.2),
            radius,
            segments,
            c,
        );

        let concrete = Rc::new(RefCell::new(core_obj));
        (
            PyCone3D {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }
}

#[pyclass(name = "Group", extends=PyMobject, subclass, skip_from_py_object, unsendable)]
pub struct PyGroup {
    pub concrete: Rc<RefCell<gmanim_core::mobjects::group::MobjectGroup>>,
}

#[pymethods]
impl PyGroup {
    #[new]
    #[pyo3(signature = (*args, **_kwargs))]
    fn new(
        args: &pyo3::Bound<'_, pyo3::types::PyTuple>,
        _kwargs: Option<&pyo3::Bound<'_, pyo3::types::PyDict>>,
    ) -> (Self, PyMobject) {
        let group = gmanim_core::mobjects::group::MobjectGroup::new();
        let concrete = Rc::new(RefCell::new(group));

        for arg in args.iter() {
            if let Ok(py_mobj) = arg.extract::<pyo3::PyRef<PyMobject>>() {
                use gmanim_core::mobjects::Mobject;
                concrete.borrow_mut().add_child(py_mobj.inner.clone());
            }
        }

        (
            PyGroup {
                concrete: concrete.clone(),
            },
            PyMobject { inner: concrete },
        )
    }

    fn add(&self, obj: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<()> {
        if let Ok(py_mobj) = obj.extract::<pyo3::PyRef<PyMobject>>() {
            use gmanim_core::mobjects::Mobject;
            self.concrete.borrow_mut().add_child(py_mobj.inner.clone());
        }
        Ok(())
    }
}
