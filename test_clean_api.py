from math import pi
from gmanim import Scene, Timeline, Line, Move, Rotate, Sphere3D, Arrow3D

def main():
    scene = Scene()
    # Now we just call set_orthographic_camera() with no args to get perfect 16:9 9.0 height aspect.
    # It also automatically pulls the camera position to z=10.0 so we don't slice the arrow.
    scene.set_orthographic_camera()

    line = Line(p0=(0, 0, 0), p1=(1, 1, 0))
    scene.add(line)
    
    arrow = Arrow3D(start=(0, 0, 0), end=(1, 1, 0), shaft_radius=0.02, head_radius=0.05, head_length=0.1)
    scene.add(arrow)

    rotate_arrow = Rotate(arrow, axis=(0, 0, pi), center=(0, 0, 0), frames=60)
    timeline = Timeline(scene)
    timeline.play(rotate_arrow)
    timeline.render("out_clean_api.mp4")

if __name__ == "__main__":
    main()
