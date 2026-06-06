from typing import List, Tuple, Union, Optional, Callable

class Mobject:
    def set_position(self, position: Tuple[float, float, float]) -> None: ...
    def get_position(self) -> Tuple[float, float, float]: ...

class Scene:
    def __init__(self) -> None: ...
    def add(self, obj: Union['Line', 'Rectangle', 'PolyLine', 'Arc', 'Dot', 'Text', 'Sphere3D', 'LineSegment3D', 'Arrow3D', 'Mobject']) -> Mobject: ...
    def remove(self, arg: Union[int, Mobject]) -> None: ...
    def set_camera(
        self,
        position: Optional[Tuple[float, float, float]] = None,
        target: Optional[Tuple[float, float, float]] = None,
        direction: Optional[Tuple[float, float, float]] = None,
        up: Optional[Tuple[float, float, float]] = None
    ) -> None: ...
    def set_orthographic_camera(self, height: float = 9.0, width: Optional[float] = None, near: float = 0.1, far: float = 50.0) -> None: ...
    def set_perspective_camera(self, fovy: float = 1.5707964, aspect: Optional[float] = None, near: float = 0.1, far: float = 50.0) -> None: ...
    def set_viewport(self, center_x: float, center_y: float, width: float, height: float) -> None: ...
    def set_pixel_viewport(self, x: int, y: int, width: int, height: int) -> None: ...

class SceneRef:
    def add(self, obj: Union['Line', 'Rectangle', 'PolyLine', 'Arc', 'Dot', 'Text', 'Sphere3D', 'LineSegment3D', 'Arrow3D', 'Mobject']) -> Mobject: ...
    def remove(self, arg: Union[int, Mobject]) -> None: ...
    def set_camera(
        self,
        position: Tuple[float, float, float] = (0.0, 0.0, 0.0),
        look_at: Tuple[float, float, float] = (0.0, 0.0, -1.0),
        up: Tuple[float, float, float] = (0.0, 1.0, 0.0)
    ) -> None: ...
    def set_orthographic_camera(self, height: float = 9.0, width: Optional[float] = None, near: float = 0.1, far: float = 50.0) -> None: ...
    def set_perspective_camera(self, fovy: float = 1.5707964, aspect: Optional[float] = None, near: float = 0.1, far: float = 50.0) -> None: ...

class Timeline:
    def __init__(
        self,
        scene: Scene,
        width: Optional[float] = None,
        height: Optional[float] = None,
        resolution: Tuple[int, int] = (1920, 1080),
        scale_factor: Optional[float] = None
    ) -> None: ...
    def play(self, anim: Animation) -> None: ...
    def wait(self, frames: int) -> None: ...
    def run(self, callback: Callable[[SceneRef], None]) -> None: ...
    def render(self, filename: str, fps: int = 60, backend: str = "ffmpeg", show_progress: bool = True) -> None: ...

class Line(Mobject):
    def __init__(
        self,
        p0: Tuple[float, float, float],
        p1: Tuple[float, float, float],
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Tuple[int, int, int, int]] = None
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
        color: Optional[Tuple[int, int, int, int]] = None
    ) -> None: ...

class PolyLine(Mobject):
    def __init__(
        self,
        points: List[Tuple[float, float, float]],
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Tuple[int, int, int, int]] = None
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
        color: Optional[Tuple[int, int, int, int]] = None
    ) -> None: ...

class Dot(Mobject):
    def __init__(
        self,
        position: Tuple[float, float, float] = (0.0, 0.0, 0.0),
        radius: float = 0.05,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Tuple[int, int, int, int]] = None
    ) -> None: ...

class Text(Mobject):
    def __init__(
        self,
        text: str,
        position: Tuple[float, float, float] = (0.0, 0.0, 0.0),
        font_size: float = 32.0,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Tuple[int, int, int, int]] = None
    ) -> None: ...

class Sphere3D(Mobject):
    def __init__(
        self,
        center: Tuple[float, float, float],
        radius: float,
        color: Optional[Tuple[int, int, int, int]] = None
    ) -> None: ...

class LineSegment3D(Mobject):
    def __init__(
        self,
        a: Tuple[float, float, float],
        b: Tuple[float, float, float],
        radius: float,
        color: Optional[Tuple[int, int, int, int]] = None
    ) -> None: ...

class Arrow3D(Mobject):
    def __init__(
        self,
        start: Tuple[float, float, float],
        end: Tuple[float, float, float],
        shaft_radius: float,
        head_radius: float,
        head_length: float,
        color: Optional[Tuple[int, int, int, int]] = None
    ) -> None: ...

class Box3D(Mobject):
    def __init__(
        self,
        center: Tuple[float, float, float],
        size: Tuple[float, float, float],
        color: Optional[Tuple[int, int, int, int]] = None
    ) -> None: ...

class Animation: ...

class Move(Animation):
    def __init__(
        self,
        target: Mobject,
        displacement: Tuple[float, float, float],
        frames: int = 60
    ) -> None: ...

class Rotate(Animation):
    def __init__(
        self,
        target: Mobject,
        axis: Tuple[float, float, float],
        center: Tuple[float, float, float],
        frames: int = 60
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
        up: Optional[Tuple[float, float, float]] = None
    ) -> None: ...
    def set_orthographic_camera(
        self, height: float = 9.0, width: Optional[float] = None, near: float = 0.1, far: float = 50.0
    ) -> None: ...
    def set_perspective_camera(
        self, fovy: float = 1.5707964, aspect: Optional[float] = None, near: float = 0.1, far: float = 50.0
    ) -> None: ...
    def set_viewport(self, center_x: float, center_y: float, width: float, height: float) -> None: ...
    def set_pixel_viewport(self, x: int, y: int, width: int, height: int) -> None: ...

class UpdateFromFunc(Animation):
    def __init__(
        self,
        callback: Callable[[SceneRef, float], None],
        frames: int = 60
    ) -> None: ...

