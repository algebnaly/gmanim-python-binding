from collections.abc import Callable, Sequence
from enum import Enum
from typing import Optional, TypeVar, Union

# Stub-only aliases; runtime Point2/Point3/Color live in gmanim.types.
_Point2 = tuple[float, float]
_Point3 = tuple[float, float, float]
_Color = tuple[int, int, int, int]

_TMesh = TypeVar("_TMesh", bound="Mesh2DIn3D")

class PyVideoBackend(Enum):
    Vaapi: PyVideoBackend
    Ffmpeg: PyVideoBackend
    Vulkan: PyVideoBackend

class H264RateControl(Enum):
    Vbr: H264RateControl
    Cbr: H264RateControl
    Disabled: H264RateControl

class VulkanH264Config:
    use_p_frames: bool
    gop_size: int
    rate_control: H264RateControl
    def __init__(
        self,
        use_p_frames: bool = True,
        gop_size: int = 60,
        rate_control: H264RateControl = H264RateControl.Vbr,
    ) -> None: ...

class Mobject:
    name: str
    def set_position(self, position: _Point3) -> None: ...
    def get_position(self) -> _Point3: ...
    def rotate_x(self, angle: float) -> None: ...
    def rotate_y(self, angle: float) -> None: ...
    def rotate_z(self, angle: float) -> None: ...
    def add(self, child: Mobject) -> None: ...
    def remove(self, child: Mobject) -> None: ...

class Scene:
    def __init__(
        self,
        name: str,
        width: Optional[float] = None,
        height: Optional[float] = None,
        resolution: Optional[tuple[int, int]] = (1920, 1080),
        scale_factor: Optional[float] = None,
        fps: int = 60,
    ) -> None: ...
    name: str
    fps: int
    def add(self, obj: Mobject) -> Mobject: ...
    def remove(self, arg: Union[int, Mobject]) -> None: ...
    def set_position(self, obj: Mobject, position: _Point3) -> None: ...
    def get_position(self, obj: Mobject) -> _Point3: ...
    def set_visible(self, obj: Mobject, visible: bool) -> None: ...
    def set_background_color(self, r: int, g: int, b: int, a: int) -> None: ...
    def set_layer(self, obj: Mobject, layer: int) -> None: ...
    def set_camera(
        self,
        position: Optional[_Point3] = None,
        target: Optional[_Point3] = None,
        direction: Optional[_Point3] = None,
        up: Optional[_Point3] = None,
    ) -> None: ...
    def set_orthographic_camera(
        self,
        height: float = 9.0,
        width: Optional[float] = None,
        near: float = 0.1,
        far: float = 50.0,
    ) -> None: ...
    def set_perspective_camera(
        self,
        fovy: float = 1.5707964,
        aspect: Optional[float] = None,
        near: float = 0.1,
        far: float = 50.0,
    ) -> None: ...
    def set_viewport(
        self, center_x: float, center_y: float, width: float, height: float
    ) -> None: ...
    def set_pixel_viewport(
        self, x: int, y: int, width: int, height: int
    ) -> None: ...
    def clear_viewport(self) -> None: ...
    def set_anti_aliasing(self, level: int) -> None: ...
    def set_point_light(
        self, position: _Point3, color_value: _Color, intensity: float
    ) -> None: ...
    def set_environment_light(
        self,
        color_value: _Color,
        intensity: float,
        rotation_radians: float = 0.0,
    ) -> None: ...
    def play(self, animation: Animation) -> None: ...
    def wait(self, frames: int) -> None: ...
    def render(
        self,
        filename: str,
        backend: Optional[PyVideoBackend] = None,
        show_progress: bool = True,
        bitrate: Optional[int] = None,
        ssaa_factor: Optional[int] = None,
        msaa_samples: Optional[int] = None,
        vulkan_config: Optional[VulkanH264Config] = None,
    ) -> None: ...
    def _get_render_info(self) -> tuple[int, int, int, int]: ...
    def _open_preview(self, shm_id: str) -> PreviewSession: ...

class PreviewSession:
    def render_frame(self, request_id: int, frame: int, slot: int) -> None: ...

class SceneFrame:
    frame: int
    alpha: float
    def get_position(self, obj: Mobject) -> _Point3: ...
    def set_position(self, obj: Mobject, position: _Point3) -> None: ...
    def move_by(self, obj: Mobject, displacement: _Point3) -> None: ...
    def set_visible(self, obj: Mobject, visible: bool) -> None: ...
    def set_layer(self, obj: Mobject, layer: int) -> None: ...
    def set_rectangle_corners(
        self, obj: Mobject, corners: Sequence[_Point3]
    ) -> None: ...
    def set_camera(self, position: _Point3, target: _Point3, up: _Point3) -> None: ...
    def set_orthographic_camera(
        self,
        height: float = 9.0,
        width: Optional[float] = None,
        near: float = 0.1,
        far: float = 50.0,
    ) -> None: ...
    def set_perspective_camera(
        self,
        fovy: float = 1.5707964,
        aspect: Optional[float] = None,
        near: float = 0.1,
        far: float = 50.0,
    ) -> None: ...
    def set_viewport(
        self, center_x: float, center_y: float, width: float, height: float
    ) -> None: ...
    def set_pixel_viewport(
        self, x: int, y: int, width: int, height: int
    ) -> None: ...
    def clear_viewport(self) -> None: ...
    def set_anti_aliasing(self, level: int) -> None: ...
    def set_point_light(
        self, position: _Point3, color_value: _Color, intensity: float
    ) -> None: ...
    def set_environment_light(
        self,
        color_value: _Color,
        intensity: float,
        rotation_radians: float = 0.0,
    ) -> None: ...

class Line(Mobject):
    def __init__(
        self,
        p0: Optional[_Point3] = None,
        p1: Optional[_Point3] = None,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[_Color] = None,
        position: Optional[_Point3] = None,
        rotation: Optional[_Point3] = None,
    ) -> None: ...

class Rectangle(Mobject):
    def __init__(
        self,
        width: float = 2.0,
        height: float = 1.0,
        center: Optional[_Point3] = None,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[_Color] = None,
        corners: Optional[Sequence[_Point3]] = None,
        position: Optional[_Point3] = None,
        rotation: Optional[_Point3] = None,
    ) -> None: ...

class PolyLine(Mobject):
    def __init__(
        self,
        points: Sequence[_Point3],
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[_Color] = None,
        position: Optional[_Point3] = None,
        rotation: Optional[_Point3] = None,
    ) -> None: ...

class Arc(Mobject):
    def __init__(
        self,
        center: Optional[_Point3] = None,
        start_angle: float = 0.0,
        end_angle: float = 3.1415927,
        radius: float = 1.0,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[_Color] = None,
        position: Optional[_Point3] = None,
        rotation: Optional[_Point3] = None,
    ) -> None: ...

class QuadraticBezier(Mobject):
    def __init__(
        self,
        a: Optional[_Point2] = None,
        b: Optional[_Point2] = None,
        c: Optional[_Point2] = None,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[_Color] = None,
        position: Optional[_Point3] = None,
        rotation: Optional[_Point3] = None,
    ) -> None: ...

class Dot(Mobject):
    def __init__(
        self,
        position: _Point3 = (0.0, 0.0, 0.0),
        radius: float = 0.05,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[_Color] = None,
    ) -> None: ...

class Text(Mobject):
    def __init__(
        self,
        text: str,
        position: _Point3 = (0.0, 0.0, 0.0),
        font_size: float = 32.0,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[_Color] = None,
    ) -> None: ...

class Sphere3D(Mobject):
    def __init__(
        self,
        center: Optional[_Point3] = None,
        radius: float = 1.0,
        color: Optional[_Color] = None,
        unlit: bool = False,
        flat_shading: bool = False,
    ) -> None: ...

class LineSegment3D(Mobject):
    def __init__(
        self,
        a: Optional[_Point3] = None,
        b: Optional[_Point3] = None,
        radius: float = 0.05,
        color: Optional[_Color] = None,
        unlit: bool = False,
        flat_shading: bool = False,
    ) -> None: ...

class QuadraticBezier3D(Mobject):
    def __init__(
        self,
        a: _Point3 = (0.0, 0.0, 0.0),
        b: _Point3 = (0.0, 0.0, 0.0),
        c: _Point3 = (0.0, 0.0, 0.0),
        radius: float = 0.05,
        color: Optional[_Color] = None,
        unlit: bool = False,
        flat_shading: bool = False,
    ) -> None: ...

class Arrow3D(Mobject):
    def __init__(
        self,
        start: Optional[_Point3] = None,
        end: Optional[_Point3] = None,
        shaft_radius: float = 0.05,
        head_radius: float = 0.1,
        head_length: float = 0.3,
        color: Optional[_Color] = None,
        unlit: bool = False,
        flat_shading: bool = False,
    ) -> None: ...

class Box3DSdf(Mobject):
    def __init__(
        self,
        center: Optional[_Point3] = None,
        size: Optional[_Point3] = None,
        color: Optional[_Color] = None,
        unlit: bool = False,
        flat_shading: bool = False,
    ) -> None: ...

class Box3D(Mobject):
    def __init__(
        self,
        center: Optional[_Point3] = None,
        size: Optional[_Point3] = None,
        color: Optional[_Color] = None,
        unlit: bool = False,
        flat_shading: bool = False,
    ) -> None: ...

class TriangleMesh3D(Mobject):
    def __init__(
        self,
        vertices: Sequence[_Point3],
        normals: Sequence[_Point3],
        colors: Sequence[tuple[float, float, float, float]],
        indices: Sequence[int],
        color: tuple[float, float, float, float] = (1.0, 1.0, 1.0, 1.0),
        model_matrix: Optional[Sequence[Sequence[float]]] = None,
        unlit: bool = False,
        flat_shading: bool = False,
    ) -> None: ...

class Mesh2DIn3D(Mobject):
    def __new__(cls: type[_TMesh], obj: Mobject) -> _TMesh: ...

class Cylinder3D(Mobject):
    def __init__(
        self,
        start: Optional[_Point3] = None,
        end: Optional[_Point3] = None,
        radius: float = 1.0,
        segments: int = 32,
        color: Optional[_Color] = None,
        unlit: bool = False,
        flat_shading: bool = False,
    ) -> None: ...

class Cone3D(Mobject):
    def __init__(
        self,
        base_center: Optional[_Point3] = None,
        tip: Optional[_Point3] = None,
        radius: float = 1.0,
        segments: int = 32,
        color: Optional[_Color] = None,
        unlit: bool = False,
        flat_shading: bool = False,
    ) -> None: ...

class Group(Mobject):
    def __init__(self, *args: Mobject) -> None: ...

class Animation: ...

class Move(Animation):
    def __init__(
        self, target: Mobject, displacement: _Point3, frames: int = 60
    ) -> None: ...

class Rotate(Animation):
    def __init__(
        self,
        target: Mobject,
        axis: _Point3,
        center: _Point3,
        frames: int = 60,
    ) -> None: ...

class Wait(Animation):
    def __init__(self, frames: int = 60) -> None: ...

class UpdateFromFunc(Animation):
    def __init__(
        self, callback: Callable[[SceneFrame, float], None], frames: int = 60
    ) -> None: ...

__all__ = [
    "Scene",
    "PreviewSession",
    "SceneFrame",
    "Mobject",
    "Line",
    "Rectangle",
    "PolyLine",
    "Arc",
    "QuadraticBezier",
    "Dot",
    "Text",
    "Sphere3D",
    "LineSegment3D",
    "QuadraticBezier3D",
    "Arrow3D",
    "Box3D",
    "Box3DSdf",
    "Mesh2DIn3D",
    "TriangleMesh3D",
    "Cylinder3D",
    "Cone3D",
    "Group",
    "Animation",
    "Move",
    "Rotate",
    "Wait",
    "UpdateFromFunc",
    "PyVideoBackend",
    "H264RateControl",
    "VulkanH264Config",
]
__doc__: Optional[str]
