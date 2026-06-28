from typing import List, Tuple, Union, Optional, Callable
from enum import Enum

class PyVideoBackend(Enum):
    Vaapi = ...
    Ffmpeg = ...
    Vulkan = ...

class Mobject:
    def set_position(self, position: Tuple[float, float, float]) -> None: ...
    def get_position(self) -> Tuple[float, float, float]: ...

class Scene:
    def __init__(
        self,
        width: Optional[float] = None,
        height: Optional[float] = None,
        resolution: Optional[Tuple[int, int]] = (1920, 1080),
        scale_factor: Optional[float] = None,
    ) -> None: ...
    def add(
        self,
        obj: Union[
            "Line",
            "Rectangle",
            "PolyLine",
            "Arc",
            "Dot",
            "Text",
            "Sphere3D",
            "LineSegment3D",
            "Arrow3D",
            "Box3D",
            "TriangleMesh3D",
            "Cylinder3D",
            "Cone3D",
            "Mobject",
        ],
    ) -> Mobject: ...
    def remove(self, arg: Union[int, Mobject]) -> None: ...
    def set_camera(
        self,
        position: Optional[Tuple[float, float, float]] = None,
        target: Optional[Tuple[float, float, float]] = None,
        direction: Optional[Tuple[float, float, float]] = None,
        up: Optional[Tuple[float, float, float]] = None,
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
    def set_pixel_viewport(self, x: int, y: int, width: int, height: int) -> None: ...
    def play(self, anim: "Animation") -> None: ...
    def wait(self, frames: int) -> None: ...
    def render(
        self,
        filename: str,
        fps: int = 60,
        backend: Optional[PyVideoBackend] = None,
        show_progress: bool = True,
        bitrate: Optional[int] = None,
        ssaa_factor: Optional[int] = None,
        msaa_samples: Optional[int] = None,
    ) -> None: ...
    def set_anti_aliasing(self, level: int) -> None: ...

class Timeline:
    def __init__(self, scene: Scene) -> None: ...
    def play(self, anim: "Animation") -> None: ...
    def wait(self, frames: int) -> None: ...
    def step_frame(self) -> bool: ...
    def render(
        self,
        filename: str,
        fps: int = 60,
        backend: Optional[PyVideoBackend] = None,
        show_progress: bool = True,
        bitrate: Optional[int] = None,
        ssaa_factor: Optional[int] = None,
        msaa_samples: Optional[int] = None,
    ) -> None: ...

class SceneRef:
    def add(
        self,
        obj: Union[
            "Line",
            "Rectangle",
            "PolyLine",
            "Arc",
            "Dot",
            "Text",
            "Sphere3D",
            "LineSegment3D",
            "Arrow3D",
            "Box3D",
            "TriangleMesh3D",
            "Cylinder3D",
            "Cone3D",
            "Mobject",
        ],
    ) -> "Mobject": ...
    def remove(self, arg: Union[int, "Mobject"]) -> None: ...
    def set_camera(
        self,
        position: Tuple[float, float, float] = (0.0, 0.0, 0.0),
        target: Tuple[float, float, float] = (0.0, 0.0, -1.0),
        up: Tuple[float, float, float] = (0.0, 1.0, 0.0),
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
    def set_anti_aliasing(self, level: int) -> None: ...

class Line(Mobject):
    def __init__(
        self,
        p0: Tuple[float, float, float],
        p1: Tuple[float, float, float],
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class Rectangle(Mobject):
    def __init__(
        self,
        p0: Tuple[float, float, float],
        p1: Tuple[float, float, float],
        p2: Tuple[float, float, float],
        p3: Tuple[float, float, float],
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class PolyLine(Mobject):
    def __init__(
        self,
        points: List[Tuple[float, float, float]],
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class Arc(Mobject):
    def __init__(
        self,
        center: Tuple[float, float, float],
        start_angle: float,
        end_angle: float,
        radius: float,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class Dot(Mobject):
    def __init__(
        self,
        position: Tuple[float, float, float] = (0.0, 0.0, 0.0),
        radius: float = 0.05,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class Text(Mobject):
    def __init__(
        self,
        text: str,
        position: Tuple[float, float, float] = (0.0, 0.0, 0.0),
        font_size: float = 32.0,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class Sphere3D(Mobject):
    def __init__(
        self,
        center: Tuple[float, float, float],
        radius: float,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class LineSegment3D(Mobject):
    def __init__(
        self,
        a: Tuple[float, float, float],
        b: Tuple[float, float, float],
        radius: float,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class Arrow3D(Mobject):
    def __init__(
        self,
        start: Tuple[float, float, float],
        end: Tuple[float, float, float],
        shaft_radius: float,
        head_radius: float,
        head_length: float,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class Box3D(Mobject):
    def __init__(
        self,
        center: Tuple[float, float, float],
        size: Tuple[float, float, float],
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class TriangleMesh3D(Mobject):
    def __init__(
        self,
        vertices: List[Tuple[float, float, float]],
        normals: List[Tuple[float, float, float]],
        colors: List[Tuple[float, float, float, float]],
        indices: List[int],
        color: Tuple[float, float, float, float] = (1.0, 1.0, 1.0, 1.0),
        model_matrix: Optional[List[List[float]]] = None,
    ) -> None: ...

class Mesh2DIn3D(Mobject):
    def __new__(cls, mobj: Mobject) -> "Mesh2DIn3D": ...

class Cylinder3D(Mobject):
    def __init__(
        self,
        start: Optional[Tuple[float, float, float]] = None,
        end: Optional[Tuple[float, float, float]] = None,
        radius: float = 1.0,
        segments: int = 32,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class Cone3D(Mobject):
    def __init__(
        self,
        base_center: Optional[Tuple[float, float, float]] = None,
        tip: Optional[Tuple[float, float, float]] = None,
        radius: float = 1.0,
        segments: int = 32,
        color: Optional[Tuple[int, int, int, int]] = None,
    ) -> None: ...

class Group(Mobject):
    def __init__(self, *args: Mobject) -> None: ...
    def add(self, obj: Mobject) -> None: ...

class Axes3DRound(Group):
    def __init__(
        self,
        length: float = 3.0,
        radius: float = 0.05,
        head_radius: float = 0.15,
        head_length: float = 0.5,
        x_color: Optional[Tuple[int, int, int, int]] = (255, 0, 0, 255),
        y_color: Optional[Tuple[int, int, int, int]] = (0, 255, 0, 255),
        z_color: Optional[Tuple[int, int, int, int]] = (0, 0, 255, 255),
    ) -> None: ...

class Axes3D(Group):
    def __init__(
        self,
        length: float = 3.0,
        head_size: float = 0.2,
        stroke_width: float = 2.0,
        x_color: Optional[Tuple[int, int, int, int]] = (255, 0, 0, 255),
        y_color: Optional[Tuple[int, int, int, int]] = (0, 255, 0, 255),
        z_color: Optional[Tuple[int, int, int, int]] = (0, 0, 255, 255),
    ) -> None: ...

class Animation: ...

class Move(Animation):
    def __init__(
        self,
        target: Mobject,
        displacement: Tuple[float, float, float],
        frames: int = 60,
    ) -> None: ...

class Rotate(Animation):
    def __init__(
        self,
        target: Mobject,
        axis: Tuple[float, float, float],
        center: Tuple[float, float, float],
        frames: int = 60,
    ) -> None: ...

class Wait(Animation):
    def __init__(self, frames: int = 60) -> None: ...

class SceneRef:
    def add(self, mobj: Mobject) -> Mobject: ...
    def remove(self, arg: Union[int, Mobject]) -> None: ...
    def set_camera(
        self,
        position: Optional[Tuple[float, float, float]] = None,
        target: Optional[Tuple[float, float, float]] = None,
        direction: Optional[Tuple[float, float, float]] = None,
        up: Optional[Tuple[float, float, float]] = None,
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
    def set_pixel_viewport(self, x: int, y: int, width: int, height: int) -> None: ...

class UpdateFromFunc(Animation):
    def __init__(
        self,
        func: Callable[[SceneRef, float], None],
        frames: int = 60,
        is_pure: Optional[bool] = None,
    ) -> None: ...

import typing

def scene(name: str) -> typing.Any: ...
def incremental(func: Callable) -> Callable: ...

registry: dict[str, typing.Any]
__all__: list[str]
__doc__: str | None

class Line2DMesh3D(Mobject):
    def __init__(
        self,
        p0: tuple[float, float, float] = (0.0, 0.0, 0.0),
        p1: tuple[float, float, float] = (1.0, 0.0, 0.0),
        stroke_width: float = 2.0,
        color: tuple[int, int, int, int] = (255, 255, 255, 255),
    ) -> None: ...

class Arc2DMesh3D(Mobject):
    def __init__(
        self,
        center: tuple[float, float, float] = (0.0, 0.0, 0.0),
        start_angle: float = 0.0,
        end_angle: float = 6.28,
        radius: float = 1.0,
        stroke_width: float = 2.0,
        fill: bool = False,
        color: tuple[int, int, int, int] = (255, 255, 255, 255),
    ) -> None: ...

class Rectangle2DMesh3D(Mobject):
    def __init__(
        self,
        p0: tuple[float, float, float],
        p1: tuple[float, float, float],
        p2: tuple[float, float, float],
        p3: tuple[float, float, float],
        stroke_width: float = 2.0,
        fill: bool = False,
        color: tuple[int, int, int, int] = (255, 255, 255, 255),
    ) -> None: ...

class PolyLine2DMesh3D(Mobject):
    def __init__(
        self,
        points: list[tuple[float, float, float]],
        stroke_width: float = 2.0,
        fill: bool = False,
        color: tuple[int, int, int, int] = (255, 255, 255, 255),
    ) -> None: ...

class Text2DMesh3D(Mobject):
    def __init__(
        self,
        text: str,
        position: tuple[float, float, float] = (0.0, 0.0, 0.0),
        font_size: float = 32.0,
        stroke_width: float | None = None,
        fill: bool | None = None,
        color: tuple[int, int, int, int] | None = None,
    ) -> None: ...
