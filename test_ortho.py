from math import pi
from gmanim import Scene, Timeline, Line, Move, Rotate, Sphere3D, Arrow3D

def main():
    scene = Scene()
    scene.set_camera(position=(0.0, 0.0, 3.0), look_at=(0.0, 0.0, -1.0))
    # Test orthographic camera
    # A standard perspective view would show perspective distortion
    # Ortho should keep parallel lines parallel and object sizes constant with depth.
    scene.set_orthographic_camera(left=-2.0, right=2.0, bottom=-1.5, top=1.5, near=0.1, far=10.0)

    sphere1 = Sphere3D(center=(-1.0, 0, 0), radius=0.2, color=(255, 100, 100, 255))
    sphere2 = Sphere3D(center=(1.0, 0, -2.0), radius=0.2, color=(100, 255, 100, 255)) # Further away, but should look same size
    scene.add(sphere1)
    scene.add(sphere2)
    
    arrow = Arrow3D(start=(0, -1, 0), end=(0, 1, 0), shaft_radius=0.02, head_radius=0.05, head_length=0.1)
    scene.add(arrow)
    
    timeline = Timeline(scene)
    timeline.wait(1)
    timeline.render("test_ortho.mp4")

if __name__ == "__main__":
    main()
