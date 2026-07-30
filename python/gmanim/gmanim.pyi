from collections.abc import Callable, Sequence
from enum import Enum
from typing import Optional, TypeVar, Union

Point3 = tuple[float, float, float]
Color = tuple[int, int, int, int]

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
    def set_position(self, position: Point3) -> None: ...
    def get_position(self) -> Point3: ...
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
    def set_position(self, obj: Mobject, position: Point3) -> None: ...
    def get_position(self, obj: Mobject) -> Point3: ...
    def set_visible(self, obj: Mobject, visible: bool) -> None: ...
    def set_layer(self, obj: Mobject, layer: int) -> None: ...
    def set_camera(
        self,
        position: Optional[Point3] = None,
        target: Optional[Point3] = None,
        direction: Optional[Point3] = None,
        up: Optional[Point3] = None,
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
        self, position: Point3, color_value: Color, intensity: float
    ) -> None: ...
    def set_environment_light(
        self,
        color_value: Color,
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
    def get_position(self, obj: Mobject) -> Point3: ...
    def set_position(self, obj: Mobject, position: Point3) -> None: ...
    def move_by(self, obj: Mobject, displacement: Point3) -> None: ...
    def set_visible(self, obj: Mobject, visible: bool) -> None: ...
    def set_layer(self, obj: Mobject, layer: int) -> None: ...
    def set_rectangle_corners(
        self, obj: Mobject, corners: Sequence[Point3]
    ) -> None: ...
    def set_camera(self, position: Point3, target: Point3, up: Point3) -> None: ...
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
        self, position: Point3, color_value: Color, intensity: float
    ) -> None: ...
    def set_environment_light(
        self,
        color_value: Color,
        intensity: float,
        rotation_radians: float = 0.0,
    ) -> None: ...

class Line(Mobject):
    def __init__(
        self,
        p0: Optional[Point3] = None,
        p1: Optional[Point3] = None,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Color] = None,
    ) -> None: ...

class Rectangle(Mobject):
    def __init__(
        self,
        width: float = 2.0,
        height: float = 1.0,
        center: Optional[Point3] = None,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Color] = None,
        corners: Optional[Sequence[Point3]] = None,
    ) -> None: ...

class PolyLine(Mobject):
    def __init__(
        self,
        points: Sequence[Point3],
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Color] = None,
    ) -> None: ...

class Arc(Mobject):
    def __init__(
        self,
        center: Optional[Point3] = None,
        start_angle: float = 0.0,
        end_angle: float = 3.1415927,
        radius: float = 1.0,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Color] = None,
    ) -> None: ...

class Dot(Mobject):
    def __init__(
        self,
        position: Point3 = (0.0, 0.0, 0.0),
        radius: float = 0.05,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Color] = None,
    ) -> None: ...

class Text(Mobject):
    def __init__(
        self,
        text: str,
        position: Point3 = (0.0, 0.0, 0.0),
        font_size: float = 32.0,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Color] = None,
    ) -> None: ...

class Sphere3D(Mobject):
    def __init__(
        self,
        center: Optional[Point3] = None,
        radius: float = 1.0,
        color: Optional[Color] = None,
    ) -> None: ...

class LineSegment3D(Mobject):
    def __init__(
        self,
        a: Optional[Point3] = None,
        b: Optional[Point3] = None,
        radius: float = 0.05,
        color: Optional[Color] = None,
    ) -> None: ...

class Arrow3D(Mobject):
    def __init__(
        self,
        start: Optional[Point3] = None,
        end: Optional[Point3] = None,
        shaft_radius: float = 0.05,
        head_radius: float = 0.1,
        head_length: float = 0.3,
        color: Optional[Color] = None,
    ) -> None: ...

class Box3DSdf(Mobject):
    def __init__(
        self,
        center: Optional[Point3] = None,
        size: Optional[Point3] = None,
        color: Optional[Color] = None,
    ) -> None: ...

class Box3D(Mobject):
    def __init__(
        self,
        center: Optional[Point3] = None,
        size: Optional[Point3] = None,
        color: Optional[Color] = None,
    ) -> None: ...

class TriangleMesh3D(Mobject):
    def __init__(
        self,
        vertices: Sequence[Point3],
        normals: Sequence[Point3],
        colors: Sequence[tuple[float, float, float, float]],
        indices: Sequence[int],
        color: tuple[float, float, float, float] = (1.0, 1.0, 1.0, 1.0),
        model_matrix: Optional[Sequence[Sequence[float]]] = None,
    ) -> None: ...

class Mesh2DIn3D(Mobject):
    def __new__(cls: type[_TMesh], obj: Mobject) -> _TMesh: ...

class Cylinder3D(Mobject):
    def __init__(
        self,
        start: Optional[Point3] = None,
        end: Optional[Point3] = None,
        radius: float = 1.0,
        segments: int = 32,
        color: Optional[Color] = None,
    ) -> None: ...

class Cone3D(Mobject):
    def __init__(
        self,
        base_center: Optional[Point3] = None,
        tip: Optional[Point3] = None,
        radius: float = 1.0,
        segments: int = 32,
        color: Optional[Color] = None,
    ) -> None: ...

class Group(Mobject):
    def __init__(self, *args: Mobject) -> None: ...

class Animation: ...

class Move(Animation):
    def __init__(
        self, target: Mobject, displacement: Point3, frames: int = 60
    ) -> None: ...

class Rotate(Animation):
    def __init__(
        self,
        target: Mobject,
        axis: Point3,
        center: Point3,
        frames: int = 60,
    ) -> None: ...

class Wait(Animation):
    def __init__(self, frames: int = 60) -> None: ...

class UpdateFromFunc(Animation):
    def __init__(
        self, callback: Callable[[SceneFrame, float], None], frames: int = 60
    ) -> None: ...

__all__ = [
    "Animation",
    "Arc",
    "Arrow3D",
    "Box3D",
    "Box3DSdf",
    "Cone3D",
    "Cylinder3D",
    "Dot",
    "Group",
    "H264RateControl",
    "Line",
    "LineSegment3D",
    "Mesh2DIn3D",
    "Mobject",
    "Move",
    "PolyLine",
    "PyVideoBackend",
    "Rectangle",
    "Rotate",
    "Scene",
    "SceneFrame",
    "Sphere3D",
    "Text",
    "TriangleMesh3D",
    "UpdateFromFunc",
    "VulkanH264Config",
    "Wait",
    "scene",
]
__doc__: Optional[str]
