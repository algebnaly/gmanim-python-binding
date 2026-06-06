from math import pi
from gmanim import Scene, Timeline, Line, Move, Rotate, Sphere3D, Arrow3D

def main():
    scene = Scene()
    scene.set_camera(position=(0.0, 0.0, 1.0), look_at=(0.0, 0.0, -1.0))

    line = Line(p0=(0, 0, 0), p1=(1, 1, 0))
    scene.add(line)
    move_animation = Move(line, displacement=(1, 0, 0), frames=120)
    rotate_animation = Rotate(line, axis=(0, 0, pi), center=(0, 0, 0), frames=600)
    sphere = Sphere3D(center=(0, 0, 0), radius=0.05)
    scene.add(sphere)
    arrow = Arrow3D(start=(0, 0, 0), end=(1, 1, 0), shaft_radius=0.02, head_radius=0.05, head_length=0.1)
    scene.add(arrow)
    timeline = Timeline(scene)
    timeline.wait(1)
    timeline.render("test_arrow.mp4")

if __name__ == "__main__":
    main()
