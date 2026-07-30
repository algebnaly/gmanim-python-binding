import math

import gmanim.gmanim as _gmanim
from gmanim.gmanim import Cone3D, Cylinder3D, Group
from gmanim.types import Color

from .mesh2d import Line2DMesh3D, PolyLine2DMesh3D


class Axes3DRound(_gmanim.Group):
    """
    A 3D coordinate system (Axes) composed of 3 Cylinders and 3 Cones.
    """

    x_cylinder: Cylinder3D
    x_cone: Cone3D
    y_cylinder: Cylinder3D
    y_cone: Cone3D
    z_cylinder: Cylinder3D
    z_cone: Cone3D

    def __init__(
        self,
        length: float = 3.0,
        radius: float = 0.05,
        head_radius: float = 0.15,
        head_length: float = 0.5,
        backward_length: float = 1.0,
        axis_color: Color = (150, 150, 150, 255),
        x_color: Color = (235, 83, 83, 255),
        y_color: Color = (75, 181, 67, 255),
        z_color: Color = (54, 162, 235, 255),
    ) -> None:
        super().__init__()

        # X-axis
        self.x_cylinder = _gmanim.Cylinder3D(
            start=(-backward_length, 0.0, 0.0),
            end=(length, 0.0, 0.0),
            radius=radius,
            color=axis_color,
        )
        self.x_cone = _gmanim.Cone3D(
            base_center=(length, 0.0, 0.0),
            tip=(length + head_length, 0.0, 0.0),
            radius=head_radius,
            color=x_color,
        )

        # Y-axis
        self.y_cylinder = _gmanim.Cylinder3D(
            start=(0.0, -backward_length, 0.0),
            end=(0.0, length, 0.0),
            radius=radius,
            color=axis_color,
        )
        self.y_cone = _gmanim.Cone3D(
            base_center=(0.0, length, 0.0),
            tip=(0.0, length + head_length, 0.0),
            radius=head_radius,
            color=y_color,
        )

        # Z-axis
        self.z_cylinder = _gmanim.Cylinder3D(
            start=(0.0, 0.0, -backward_length),
            end=(0.0, 0.0, length),
            radius=radius,
            color=axis_color,
        )
        self.z_cone = _gmanim.Cone3D(
            base_center=(0.0, 0.0, length),
            tip=(0.0, 0.0, length + head_length),
            radius=head_radius,
            color=z_color,
        )

        self.add(self.x_cylinder)
        self.add(self.x_cone)
        self.add(self.y_cylinder)
        self.add(self.y_cone)
        self.add(self.z_cylinder)
        self.add(self.z_cone)


class Axes3D(_gmanim.Group):
    """
    A 3D coordinate system (Axes) composed of thickness-less 2D Lines with perspective.
    """

    x_axis: Group
    y_axis: Group
    z_axis: Group

    def __init__(
        self,
        length: float = 3.0,
        head_size: float = 0.15,
        stroke_width: float = 0.05,
        backward_length: float = 1.0,
        axis_color: Color = (150, 150, 150, 255),
        x_color: Color = (235, 83, 83, 255),
        y_color: Color = (75, 181, 67, 255),
        z_color: Color = (54, 162, 235, 255),
    ) -> None:
        super().__init__()

        def make_arrow(color: Color) -> Group:
            group = _gmanim.Group()
            line = Line2DMesh3D(
                p0=(-backward_length, 0.0, 0.0),
                p1=(length - head_size, 0.0, 0.0),
                stroke_width=stroke_width,
                color=axis_color,
            )

            # The tip is at `length`, and the two corners are at `length - head_size`
            p1 = (length, 0.0, 0.0)
            p2 = (length - head_size, head_size * 0.5, 0.0)
            p3 = (length - head_size, -head_size * 0.5, 0.0)

            head = PolyLine2DMesh3D(
                points=[p1, p2, p3], stroke_width=0.0, fill=True, color=color
            )

            group.add(line)
            group.add(head)
            return group

        self.x_axis = make_arrow(x_color)

        self.y_axis = make_arrow(y_color)
        self.y_axis.rotate_z(math.pi / 2.0)

        self.z_axis = make_arrow(z_color)
        self.z_axis.rotate_y(-math.pi / 2.0)

        self.add(self.x_axis)
        self.add(self.y_axis)
        self.add(self.z_axis)


__all__ = ["Axes3D", "Axes3DRound"]
