import gmanim.gmanim as _gmanim


class Line2DMesh3D(_gmanim.Mesh2DIn3D):
    def __new__(
        cls,
        p0=(0.0, 0.0, 0.0),
        p1=(1.0, 0.0, 0.0),
        stroke_width=2.0,
        color=(255, 255, 255, 255),
    ):
        inner = _gmanim.Line(p0=p0, p1=p1, stroke_width=stroke_width, color=color)
        return super().__new__(cls, inner)


class Arc2DMesh3D(_gmanim.Mesh2DIn3D):
    def __new__(
        cls,
        center=(0.0, 0.0, 0.0),
        start_angle=0.0,
        end_angle=6.28,
        radius=1.0,
        stroke_width=2.0,
        fill=False,
        color=(255, 255, 255, 255),
    ):
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
        cls, p0, p1, p2, p3, stroke_width=2.0, fill=False, color=(255, 255, 255, 255)
    ):
        inner = _gmanim.Rectangle(
            p0=p0,
            p1=p1,
            p2=p2,
            p3=p3,
            stroke_width=stroke_width,
            fill=fill,
            color=color,
        )
        return super().__new__(cls, inner)


class PolyLine2DMesh3D(_gmanim.Mesh2DIn3D):
    def __new__(cls, points, stroke_width=2.0, fill=False, color=(255, 255, 255, 255)):
        inner = _gmanim.PolyLine(
            points=points, stroke_width=stroke_width, fill=fill, color=color
        )
        return super().__new__(cls, inner)


class Text2DMesh3D(_gmanim.Mesh2DIn3D):
    def __new__(
        cls,
        text,
        position=(0.0, 0.0, 0.0),
        font_size=32.0,
        stroke_width=None,
        fill=None,
        color=None,
    ):
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
