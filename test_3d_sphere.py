from math import pi
from gmanim import Scene, Timeline, Line, Move, Rotate, Sphere3D

def main():
    scene = Scene()
    line = Line(p0=(0, 0, 0), p1=(1, 1, 0))
    scene.add(line)
    
    # Place sphere in front of the camera (which is at 0,0,0 looking at -Z)
    sphere = Sphere3D(center=(0, 0, -2.0), radius=0.5, color=(255, 100, 100, 255))
    scene.add(sphere)
    
    timeline = Timeline(scene, resolution=(640, 360))
    # Just render one frame to see what it looks like
    timeline.wait(1)
    timeline.render("test_3d_sphere.mp4")

if __name__ == "__main__":
    main()
