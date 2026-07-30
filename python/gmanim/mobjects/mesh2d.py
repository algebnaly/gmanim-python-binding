from collections.abc import Sequence
from typing import Optional

import gmanim.gmanim as _gmanim
from gmanim.types import Color, Point3


class Line2DMesh3D(_gmanim.Mesh2DIn3D):
    def __new__(
        cls,
        p0: Point3 = (0.0, 0.0, 0.0),
        p1: Point3 = (1.0, 0.0, 0.0),
        stroke_width: float = 2.0,
        color: Color = (255, 255, 255, 255),
    ) -> "Line2DMesh3D":
        inner = _gmanim.Line(p0=p0, p1=p1, stroke_width=stroke_width, color=color)
        return super().__new__(cls, inner)


class Arc2DMesh3D(_gmanim.Mesh2DIn3D):
    def __new__(
        cls,
        center: Point3 = (0.0, 0.0, 0.0),
        start_angle: float = 0.0,
        end_angle: float = 6.28,
        radius: float = 1.0,
        stroke_width: float = 2.0,
        fill: bool = False,
        color: Color = (255, 255, 255, 255),
    ) -> "Arc2DMesh3D":
        inner = _gmanim.Arc(
            center=center,
            start_angle=start_angle,
            end_angle=end_angle,
            radius=radius,
            stroke_width=stroke_width,
            fill=fill,
            color=color,
        )
        return super().__new__(cls, inner)


class Rectangle2DMesh3D(_gmanim.Mesh2DIn3D):
    def __new__(
        cls,
        p0: Point3,
        p1: Point3,
        p2: Point3,
        p3: Point3,
        stroke_width: float = 2.0,
        fill: bool = False,
        color: Color = (255, 255, 255, 255),
    ) -> "Rectangle2DMesh3D":
        inner = _gmanim.Rectangle(
            corners=[p0, p1, p2, p3],
            stroke_width=stroke_width,
            fill=fill,
            color=color,
        )
        return super().__new__(cls, inner)


class PolyLine2DMesh3D(_gmanim.Mesh2DIn3D):
    def __new__(
        cls,
        points: Sequence[Point3],
        stroke_width: float = 2.0,
        fill: bool = False,
        color: Color = (255, 255, 255, 255),
    ) -> "PolyLine2DMesh3D":
        inner = _gmanim.PolyLine(
            points=points, stroke_width=stroke_width, fill=fill, color=color
        )
        return super().__new__(cls, inner)


class Text2DMesh3D(_gmanim.Mesh2DIn3D):
    def __new__(
        cls,
        text: str,
        position: Point3 = (0.0, 0.0, 0.0),
        font_size: float = 32.0,
        stroke_width: Optional[float] = None,
        fill: Optional[bool] = None,
        color: Optional[Color] = None,
    ) -> "Text2DMesh3D":
        inner = _gmanim.Text(
            text=text,
            position=position,
            font_size=font_size,
            stroke_width=stroke_width,
            fill=fill,
            color=color,
        )
        return super().__new__(cls, inner)


__all__ = [
    "Line2DMesh3D",
    "Arc2DMesh3D",
    "Rectangle2DMesh3D",
    "PolyLine2DMesh3D",
    "Text2DMesh3D",
]
