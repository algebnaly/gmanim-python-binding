use std::{
    collections::HashSet,
    sync::{Arc as Shared, Mutex},
};

use gmanim_core::{
    Color,
    mobjects::{
        Arc, Dot, Draw, Mobject, MobjectId, NodeBundle, NodeVisual, PolyLine, Rectangle,
        RenderVisitor, SimpleLine,
        mesh_2d::TriangleMesh2D,
        mesh_3d::{SurfaceMaterial, TriangleMesh3D, Vertex},
        object_3d::{Arrow3D, Box3DSdf, LineSegment3D, Sphere3D},
        text::Text,
        wrapper_3d::Wrapper2DIn3D,
    },
};
use nalgebra::{Matrix4, Point3, Vector3};
use pyo3::{exceptions::PyValueError, prelude::*, types::PyTuple};

use crate::{scene::PyMobject, utils::build_draw_config};

fn point(value: (f32, f32, f32)) -> Point3<f32> {
    Point3::new(value.0, value.1, value.2)
}

fn core_color(value: Option<(u8, u8, u8, u8)>) -> Color {
    value
        .map(|value| Color::new(value.0, value.1, value.2, value.3))
        .unwrap_or_default()
}

fn material(value: Option<(u8, u8, u8, u8)>, unlit: bool, flat_shading: bool) -> SurfaceMaterial {
    let color = core_color(value);
    SurfaceMaterial {
        base_color: [
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
            color.a as f32 / 255.0,
        ],
        unlit,
        flat_shading,
        ..Default::default()
    }
}

fn apply_mesh_shading(mesh: &mut TriangleMesh3D, unlit: bool, flat_shading: bool) {
    mesh.material.unlit = unlit;
    mesh.material.flat_shading = flat_shading;
}

fn mobject(name: &str, visual: NodeVisual) -> PyMobject {
    PyMobject::detached(name, visual)
}
fn apply_initial_transform(
    mut mobj: PyMobject,
    position: Option<(f32, f32, f32)>,
    rotation: Option<(f32, f32, f32)>,
) -> PyMobject {
    let mut transform = nalgebra::Matrix4::identity();
    if let Some(rot) = rotation {
        transform = transform * nalgebra::Matrix4::new_rotation(nalgebra::Vector3::z() * rot.2);
        transform = transform * nalgebra::Matrix4::new_rotation(nalgebra::Vector3::y() * rot.1);
        transform = transform * nalgebra::Matrix4::new_rotation(nalgebra::Vector3::x() * rot.0);
    }
    if let Some(pos) = position {
        transform[(0, 3)] = pos.0;
        transform[(1, 3)] = pos.1;
        transform[(2, 3)] = pos.2;
    }
    mobj.set_detached_transform(transform).unwrap();
    mobj
}


struct StaticMesh2D(TriangleMesh2D);

impl Draw for StaticMesh2D {
    fn draw(&self, _ctx: &mut gmanim_core::Context, _parent_matrix: Matrix4<f32>) {}
}

impl Mobject for StaticMesh2D {
    fn default_name(&self) -> &'static str {
        "StaticMesh2D"
    }

    fn submit_to_renderer(&self, visitor: &mut dyn RenderVisitor, transform: Matrix4<f32>) {
        visitor.push_mesh_2d(&self.0, transform);
    }
}

#[pyclass(name = "Line", extends=PyMobject, skip_from_py_object)]
pub struct PyLine;

#[pymethods]
impl PyLine {
    #[new]
    #[pyo3(signature = (p0=None, p1=None, stroke_width=None, fill=None, color=None, position=None, rotation=None))]
    fn new(
        p0: Option<(f32, f32, f32)>,
        p1: Option<(f32, f32, f32)>,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
        position: Option<(f32, f32, f32)>,
        rotation: Option<(f32, f32, f32)>,
    ) -> (Self, PyMobject) {
        let mut line = SimpleLine::new(
            point(p0.unwrap_or((-1.0, 0.0, 0.0))),
            point(p1.unwrap_or((1.0, 0.0, 0.0))),
        );
        line.draw_config = build_draw_config(stroke_width, fill, color);
        line.update_mesh();
        (
            Self,
            apply_initial_transform(mobject("Line", NodeVisual::Renderable(Shared::new(line))), position, rotation),
        )
    }
}

#[pyclass(name = "Rectangle", extends=PyMobject, skip_from_py_object)]
pub struct PyRectangle;

#[pymethods]
impl PyRectangle {
    #[new]
    #[pyo3(signature = (width=2.0, height=1.0, center=None, stroke_width=None, fill=None, color=None, corners=None, position=None, rotation=None))]
    fn new(
        width: f32,
        height: f32,
        center: Option<(f32, f32, f32)>,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
        corners: Option<Vec<(f32, f32, f32)>>,
        position: Option<(f32, f32, f32)>,
        rotation: Option<(f32, f32, f32)>,
    ) -> PyResult<(Self, PyMobject)> {
        let corners = if let Some(corners) = corners {
            let corners: [(f32, f32, f32); 4] = corners.try_into().map_err(|_| {
                PyValueError::new_err("corners must contain exactly four 3D points")
            })?;
            corners.map(point)
        } else {
            let center = center.unwrap_or((0.0, 0.0, 0.0));
            let half_width = width / 2.0;
            let half_height = height / 2.0;
            [
                Point3::new(center.0 - half_width, center.1 + half_height, center.2),
                Point3::new(center.0 - half_width, center.1 - half_height, center.2),
                Point3::new(center.0 + half_width, center.1 - half_height, center.2),
                Point3::new(center.0 + half_width, center.1 + half_height, center.2),
            ]
        };
        let rectangle = Rectangle {
            p0: corners[0],
            p1: corners[1],
            p2: corners[2],
            p3: corners[3],
            color: core_color(color),
            draw_config: build_draw_config(stroke_width, fill, color),
        };
        Ok((Self, apply_initial_transform(mobject("Rectangle", NodeVisual::Rectangle(rectangle)), position, rotation)))
    }
}

#[pyclass(name = "PolyLine", extends=PyMobject, skip_from_py_object)]
pub struct PyPolyLine;

#[pymethods]
impl PyPolyLine {
    #[new]
    #[pyo3(signature = (points, stroke_width=None, fill=None, color=None, position=None, rotation=None))]
    fn new(
        points: Vec<(f32, f32, f32)>,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
        position: Option<(f32, f32, f32)>,
        rotation: Option<(f32, f32, f32)>,
    ) -> (Self, PyMobject) {
        let mut polyline = PolyLine::new(points.into_iter().map(point).collect());
        polyline.draw_config = build_draw_config(stroke_width, fill, color);
        polyline.update_mesh();
        (
            Self,
            apply_initial_transform(mobject("PolyLine", NodeVisual::Renderable(Shared::new(polyline))), position, rotation),
        )
    }
}

#[pyclass(name = "Arc", extends=PyMobject, skip_from_py_object)]
pub struct PyArc;

#[pymethods]
impl PyArc {
    #[new]
    #[pyo3(signature = (center=None, start_angle=0.0, end_angle=std::f32::consts::PI, radius=1.0, stroke_width=None, fill=None, color=None, position=None, rotation=None))]
    fn new(
        center: Option<(f32, f32, f32)>,
        start_angle: f32,
        end_angle: f32,
        radius: f32,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
        position: Option<(f32, f32, f32)>,
        rotation: Option<(f32, f32, f32)>,
    ) -> (Self, PyMobject) {
        let mut arc = Arc::new(
            point(center.unwrap_or((0.0, 0.0, 0.0))),
            start_angle,
            end_angle,
            radius,
        );
        arc.draw_config = build_draw_config(stroke_width, fill, color);
        arc.update_mesh();
        (
            Self,
            apply_initial_transform(mobject("Arc", NodeVisual::Renderable(Shared::new(arc))), position, rotation),
        )
    }
}

#[pyclass(name = "Dot", extends=PyMobject, skip_from_py_object)]
pub struct PyDot;

#[pymethods]
impl PyDot {
    #[new]
    #[pyo3(signature = (position=(0.0, 0.0, 0.0), radius=0.05, stroke_width=None, fill=None, color=None))]
    fn new(
        position: (f32, f32, f32),
        radius: f32,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let dot = Dot::new(
            point(position),
            radius,
            core_color(color),
            build_draw_config(stroke_width, fill, color),
        );
        (
            Self,
            mobject("Dot", NodeVisual::Renderable(Shared::new(dot))),
        )
    }
}

#[pyclass(name = "Text", extends=PyMobject, skip_from_py_object)]
pub struct PyText;

#[pymethods]
impl PyText {
    #[new]
    #[pyo3(signature = (text, position=(0.0, 0.0, 0.0), font_size=32.0, stroke_width=None, fill=None, color=None))]
    fn new(
        text: String,
        position: (f32, f32, f32),
        font_size: f32,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
    ) -> (Self, PyMobject) {
        let text = Text::new(
            text,
            point(position),
            font_size,
            build_draw_config(stroke_width, fill, color),
        );
        (
            Self,
            mobject("Text", NodeVisual::Renderable(Shared::new(text))),
        )
    }
}

#[pyclass(name = "Mesh2DIn3D", extends=PyMobject, subclass, skip_from_py_object)]
pub struct PyMesh2DIn3D;

#[pymethods]
impl PyMesh2DIn3D {
    #[new]
    fn new(inner: PyMobject) -> PyResult<(Self, PyMobject)> {
        let visual = inner.detached_visual()?;
        let wrapper = match visual {
            NodeVisual::Renderable(renderable) => Wrapper2DIn3D { inner: renderable },
            NodeVisual::Rectangle(rectangle) => {
                Wrapper2DIn3D::new(StaticMesh2D(rectangle.tessellate()))
            }
            NodeVisual::None => {
                return Err(PyValueError::new_err(
                    "a renderless group cannot be wrapped as Mesh2DIn3D",
                ));
            }
        };
        Ok((
            Self,
            mobject("Mesh2DIn3D", NodeVisual::Renderable(Shared::new(wrapper))),
        ))
    }
}

#[pyclass(name = "Sphere3D", extends=PyMobject, skip_from_py_object)]
pub struct PySphere3D;

#[pymethods]
impl PySphere3D {
    #[new]
    #[pyo3(signature = (center=None, radius=1.0, color=None, unlit=false, flat_shading=false))]
    fn new(
        center: Option<(f32, f32, f32)>,
        radius: f32,
        color: Option<(u8, u8, u8, u8)>,
        unlit: bool,
        flat_shading: bool,
    ) -> (Self, PyMobject) {
        let mut object = mobject(
            "Sphere3D",
            NodeVisual::Renderable(Shared::new(Sphere3D {
                radius,
                material: material(color, unlit, flat_shading),
            })),
        );
        object
            .set_detached_transform(Matrix4::new_translation(
                &point(center.unwrap_or((0.0, 0.0, 0.0))).coords,
            ))
            .expect("new object is detached");
        (Self, object)
    }
}

#[pyclass(name = "LineSegment3D", extends=PyMobject, skip_from_py_object)]
pub struct PyLineSegment3D;

#[pymethods]
impl PyLineSegment3D {
    #[new]
    #[pyo3(signature = (a=None, b=None, radius=0.05, color=None, unlit=false, flat_shading=false))]
    fn new(
        a: Option<(f32, f32, f32)>,
        b: Option<(f32, f32, f32)>,
        radius: f32,
        color: Option<(u8, u8, u8, u8)>,
        unlit: bool,
        flat_shading: bool,
    ) -> (Self, PyMobject) {
        let object = LineSegment3D {
            a: point(a.unwrap_or((-1.0, 0.0, 0.0))),
            b: point(b.unwrap_or((1.0, 0.0, 0.0))),
            radius,
            material: material(color, unlit, flat_shading),
        };
        (
            Self,
            mobject("LineSegment3D", NodeVisual::Renderable(Shared::new(object))),
        )
    }
}

#[pyclass(name = "Arrow3D", extends=PyMobject, skip_from_py_object)]
pub struct PyArrow3D;

#[pymethods]
impl PyArrow3D {
    #[new]
    #[pyo3(signature = (start=None, end=None, shaft_radius=0.05, head_radius=0.1, head_length=0.3, color=None, unlit=false, flat_shading=false))]
    fn new(
        start: Option<(f32, f32, f32)>,
        end: Option<(f32, f32, f32)>,
        shaft_radius: f32,
        head_radius: f32,
        head_length: f32,
        color: Option<(u8, u8, u8, u8)>,
        unlit: bool,
        flat_shading: bool,
    ) -> (Self, PyMobject) {
        let object = Arrow3D {
            start: point(start.unwrap_or((-1.0, 0.0, 0.0))),
            end: point(end.unwrap_or((1.0, 0.0, 0.0))),
            shaft_radius,
            head_radius,
            head_length,
            material: material(color, unlit, flat_shading),
        };
        (
            Self,
            mobject("Arrow3D", NodeVisual::Renderable(Shared::new(object))),
        )
    }
}

#[pyclass(name = "Box3DSdf", extends=PyMobject, skip_from_py_object)]
pub struct PyBox3DSdf;

#[pymethods]
impl PyBox3DSdf {
    #[new]
    #[pyo3(signature = (center=None, size=None, color=None, unlit=false, flat_shading=false))]
    fn new(
        center: Option<(f32, f32, f32)>,
        size: Option<(f32, f32, f32)>,
        color: Option<(u8, u8, u8, u8)>,
        unlit: bool,
        flat_shading: bool,
    ) -> (Self, PyMobject) {
        let size = size.unwrap_or((1.0, 1.0, 1.0));
        let mut object = mobject(
            "Box3DSdf",
            NodeVisual::Renderable(Shared::new(Box3DSdf {
                size: Vector3::new(size.0, size.1, size.2),
                x_axis: Vector3::x(),
                y_axis: Vector3::y(),
                z_axis: Vector3::z(),
                material: material(color, unlit, flat_shading),
            })),
        );
        object
            .set_detached_transform(Matrix4::new_translation(
                &point(center.unwrap_or((0.0, 0.0, 0.0))).coords,
            ))
            .expect("new object is detached");
        (Self, object)
    }
}

#[pyclass(name = "Box3D", extends=PyMobject, skip_from_py_object)]
pub struct PyBox3D;

#[pymethods]
impl PyBox3D {
    #[new]
    #[pyo3(signature = (center=None, size=None, color=None, unlit=false, flat_shading=false))]
    fn new(
        center: Option<(f32, f32, f32)>,
        size: Option<(f32, f32, f32)>,
        color: Option<(u8, u8, u8, u8)>,
        unlit: bool,
        flat_shading: bool,
    ) -> (Self, PyMobject) {
        let center = point(center.unwrap_or((0.0, 0.0, 0.0)));
        let size = size.unwrap_or((1.0, 1.0, 1.0));
        let mut mesh = TriangleMesh3D::box_mesh(
            center,
            Vector3::new(size.0, size.1, size.2),
            core_color(color),
        );
        apply_mesh_shading(&mut mesh, unlit, flat_shading);
        (
            Self,
            mobject("Box3D", NodeVisual::Renderable(Shared::new(mesh))),
        )
    }
}

#[pyclass(name = "TriangleMesh3D", extends=PyMobject, skip_from_py_object)]
pub struct PyTriangleMesh3D;

#[pymethods]
impl PyTriangleMesh3D {
    #[new]
    #[pyo3(signature = (vertices, normals, colors, indices, color=(1.0, 1.0, 1.0, 1.0), model_matrix=None, unlit=false, flat_shading=false))]
    fn new(
        vertices: Vec<[f32; 3]>,
        normals: Vec<[f32; 3]>,
        colors: Vec<[f32; 4]>,
        indices: Vec<u32>,
        color: (f32, f32, f32, f32),
        model_matrix: Option<[[f32; 4]; 4]>,
        unlit: bool,
        flat_shading: bool,
    ) -> (Self, PyMobject) {
        let vertices = vertices
            .into_iter()
            .enumerate()
            .map(|(index, position)| {
                let normal = normals.get(index).copied().unwrap_or([0.0, 0.0, 1.0]);
                let vertex_color = colors
                    .get(index)
                    .copied()
                    .unwrap_or([color.0, color.1, color.2, color.3]);
                Vertex {
                    position,
                    normal,
                    color: vertex_color,
                    surface_coord: normal,
                }
            })
            .collect();
        let mut mesh = TriangleMesh3D::new(vertices, indices);
        apply_mesh_shading(&mut mesh, unlit, flat_shading);
        let mut object = mobject("TriangleMesh3D", NodeVisual::Renderable(Shared::new(mesh)));
        if let Some(matrix) = model_matrix {
            object
                .set_detached_transform(Matrix4::from_row_slice(&matrix.concat()))
                .expect("new object is detached");
        }
        (Self, object)
    }
}

#[pyclass(name = "Cylinder3D", extends=PyMobject, skip_from_py_object)]
pub struct PyCylinder3D;

#[pymethods]
impl PyCylinder3D {
    #[new]
    #[pyo3(signature = (start=None, end=None, radius=1.0, segments=32, color=None, unlit=false, flat_shading=false))]
    fn new(
        start: Option<(f32, f32, f32)>,
        end: Option<(f32, f32, f32)>,
        radius: f32,
        segments: u32,
        color: Option<(u8, u8, u8, u8)>,
        unlit: bool,
        flat_shading: bool,
    ) -> (Self, PyMobject) {
        let mut mesh = TriangleMesh3D::cylinder(
            point(start.unwrap_or((0.0, 0.0, 0.0))),
            point(end.unwrap_or((0.0, 1.0, 0.0))),
            radius,
            segments,
            core_color(color),
        );
        apply_mesh_shading(&mut mesh, unlit, flat_shading);
        (
            Self,
            mobject("Cylinder3D", NodeVisual::Renderable(Shared::new(mesh))),
        )
    }
}

#[pyclass(name = "Cone3D", extends=PyMobject, skip_from_py_object)]
pub struct PyCone3D;

#[pymethods]
impl PyCone3D {
    #[new]
    #[pyo3(signature = (base_center=None, tip=None, radius=1.0, segments=32, color=None, unlit=false, flat_shading=false))]
    fn new(
        base_center: Option<(f32, f32, f32)>,
        tip: Option<(f32, f32, f32)>,
        radius: f32,
        segments: u32,
        color: Option<(u8, u8, u8, u8)>,
        unlit: bool,
        flat_shading: bool,
    ) -> (Self, PyMobject) {
        let mut mesh = TriangleMesh3D::cone(
            point(base_center.unwrap_or((0.0, 0.0, 0.0))),
            point(tip.unwrap_or((0.0, 1.0, 0.0))),
            radius,
            segments,
            core_color(color),
        );
        apply_mesh_shading(&mut mesh, unlit, flat_shading);
        (
            Self,
            mobject("Cone3D", NodeVisual::Renderable(Shared::new(mesh))),
        )
    }
}

#[pyclass(name = "Group", extends=PyMobject, subclass, skip_from_py_object)]
pub struct PyGroup {
    state: Shared<Mutex<crate::scene::ObjectState>>,
}

#[pymethods]
impl PyGroup {
    #[new]
    #[pyo3(signature = (*args, **_kwargs))]
    fn new(
        args: &Bound<'_, PyTuple>,
        _kwargs: Option<&Bound<'_, pyo3::types::PyDict>>,
    ) -> PyResult<(Self, PyMobject)> {
        let base = mobject("Group", NodeVisual::None);
        for argument in args.iter() {
            base.add_detached_child(argument.extract::<PyMobject>()?)?;
        }
        Ok((
            Self {
                state: base.state.clone(),
            },
            base,
        ))
    }

    fn add(&self, object: PyMobject) -> PyResult<()> {
        PyMobject {
            state: self.state.clone(),
        }
        .add_detached_child(object)
    }

    fn remove(&self, object: PyMobject) -> PyResult<()> {
        PyMobject {
            state: self.state.clone(),
        }
        .remove_detached_child(&object)
    }
}

pub(crate) struct ObjectState {
    pub(crate) name: String,
    pub(crate) visual: NodeVisual,
    pub(crate) transform: Matrix4<f32>,
    pub(crate) children: Vec<PyMobject>,
    pub(crate) attachment: Option<Attachment>,
}

#[derive(Clone, Copy)]
pub(crate) struct Attachment {
    pub scene_token: u64,
    pub id: MobjectId,
}

impl PyMobject {
    pub(crate) fn detached(name: impl Into<String>, visual: NodeVisual) -> Self {
        Self {
            state: Shared::new(Mutex::new(ObjectState {
                name: name.into(),
                visual,
                transform: Matrix4::identity(),
                children: Vec::new(),
                attachment: None,
            })),
        }
    }

    pub(crate) fn attachment(&self, scene_token: u64) -> PyResult<Attachment> {
        let attachment = self.state.lock().unwrap().attachment.ok_or_else(|| {
            PyValueError::new_err("mobject must be added to a scene before it can be animated")
        })?;
        if attachment.scene_token != scene_token {
            return Err(PyValueError::new_err(
                "mobject belongs to a different scene",
            ));
        }
        Ok(attachment)
    }

    pub(crate) fn detached_visual(&self) -> PyResult<NodeVisual> {
        let state = self.state.lock().unwrap();
        if state.attachment.is_some() {
            return Err(PyValueError::new_err(
                "operation requires a detached mobject blueprint",
            ));
        }
        Ok(state.visual.clone())
    }

    pub(crate) fn set_detached_transform(&mut self, transform: Matrix4<f32>) -> PyResult<()> {
        let mut state = self.state.lock().unwrap();
        if state.attachment.is_some() {
            return Err(PyValueError::new_err(
                "attached mobjects are modified through Scene or SceneFrame",
            ));
        }
        state.transform = transform;
        Ok(())
    }

    pub(crate) fn add_detached_child(&self, child: PyMobject) -> PyResult<()> {
        if Shared::ptr_eq(&self.state, &child.state) {
            return Err(PyValueError::new_err("a mobject cannot contain itself"));
        }
        let mut state = self.state.lock().unwrap();
        if state.attachment.is_some() || child.state.lock().unwrap().attachment.is_some() {
            return Err(PyValueError::new_err(
                "group composition must be completed before adding it to a scene",
            ));
        }
        if state
            .children
            .iter()
            .any(|existing| Shared::ptr_eq(&existing.state, &child.state))
        {
            return Err(PyValueError::new_err(
                "mobject is already a direct child of this group",
            ));
        }
        state.children.push(child);
        Ok(())
    }

    pub(crate) fn remove_detached_child(&self, child: &PyMobject) -> PyResult<()> {
        let mut state = self.state.lock().unwrap();
        if state.attachment.is_some() {
            return Err(PyValueError::new_err(
                "attached hierarchy is modified through Scene",
            ));
        }
        let index = state
            .children
            .iter()
            .position(|existing| Shared::ptr_eq(&existing.state, &child.state))
            .ok_or_else(|| PyValueError::new_err("mobject is not a child of this group"))?;
        state.children.remove(index);
        Ok(())
    }

    pub(crate) fn build_bundle_tree(&self) -> PyResult<(NodeBundle, Vec<PyMobject>)> {
        fn build(
            object: &PyMobject,
            visiting: &mut HashSet<usize>,
            handles: &mut Vec<PyMobject>,
        ) -> PyResult<NodeBundle> {
            let key = Shared::as_ptr(&object.state) as usize;
            if !visiting.insert(key) {
                return Err(PyValueError::new_err(
                    "mobject hierarchy contains a cycle or repeated node",
                ));
            }
            let state = object.state.lock().unwrap();
            if state.attachment.is_some() {
                return Err(PyValueError::new_err(
                    "mobject has already been added to a scene",
                ));
            }
            let name = state.name.clone();
            let transform = state.transform;
            let visual = state.visual.clone();
            let children = state.children.clone();
            drop(state);

            handles.push(object.clone());
            let mut bundle = NodeBundle {
                name,
                transform,
                visual,
                children: Vec::with_capacity(children.len()),
            };
            for child in children {
                bundle.children.push(build(&child, visiting, handles)?);
            }
            Ok(bundle)
        }

        let mut handles = Vec::new();
        let bundle = build(self, &mut HashSet::new(), &mut handles)?;
        Ok((bundle, handles))
    }

    pub(crate) fn attach_tree(
        handles: Vec<PyMobject>,
        ids: Vec<MobjectId>,
        scene_token: u64,
    ) -> PyResult<()> {
        if handles.len() != ids.len() {
            return Err(PyValueError::new_err(
                "core returned an inconsistent mobject tree",
            ));
        }
        for (handle, id) in handles.into_iter().zip(ids) {
            handle.state.lock().unwrap().attachment = Some(Attachment { scene_token, id });
        }
        Ok(())
    }
}

#[pyclass(name = "QuadraticBezier", extends=PyMobject, skip_from_py_object)]
pub struct PyQuadraticBezier;

#[pymethods]
impl PyQuadraticBezier {
    #[new]
    #[pyo3(signature = (a=None, b=None, c=None, stroke_width=None, fill=None, color=None, position=None, rotation=None))]
    fn new(
        a: Option<(f32, f32)>,
        b: Option<(f32, f32)>,
        c: Option<(f32, f32)>,
        stroke_width: Option<f32>,
        fill: Option<bool>,
        color: Option<(u8, u8, u8, u8)>,
        position: Option<(f32, f32, f32)>,
        rotation: Option<(f32, f32, f32)>,
    ) -> (Self, PyMobject) {
        let a = a.unwrap_or((-1.0, 0.0));
        let b = b.unwrap_or((0.0, 1.0));
        let c = c.unwrap_or((1.0, 0.0));
        let mut qb = gmanim_core::mobjects::QuadraticBezier::new(
            point((a.0, a.1, 0.0)),
            point((b.0, b.1, 0.0)),
            point((c.0, c.1, 0.0)),
        );
        qb.draw_config = build_draw_config(stroke_width, fill, color);
        qb.update_mesh();
        (
            Self,
            apply_initial_transform(mobject("QuadraticBezier", NodeVisual::Renderable(Shared::new(qb))), position, rotation),
        )
    }
}

#[pyclass(name = "QuadraticBezier3D", extends=PyMobject, skip_from_py_object)]
pub struct PyQuadraticBezier3D;

#[pymethods]
impl PyQuadraticBezier3D {
    #[new]
    #[pyo3(signature = (a=(0.0, 0.0, 0.0), b=(0.0, 0.0, 0.0), c=(0.0, 0.0, 0.0), radius=0.05, color=None, unlit=false, flat_shading=false))]
    fn new(
        a: (f32, f32, f32),
        b: (f32, f32, f32),
        c: (f32, f32, f32),
        radius: f32,
        color: Option<(u8, u8, u8, u8)>,
        unlit: bool,
        flat_shading: bool,
    ) -> (Self, PyMobject) {
        let qb = gmanim_core::mobjects::object_3d::QuadraticBezier3D {
            a: point(a),
            b: point(b),
            c: point(c),
            radius,
            material: material(color, unlit, flat_shading),
        };
        (
            Self,
            mobject("QuadraticBezier3D", NodeVisual::Renderable(Shared::new(qb))),
        )
    }
}
