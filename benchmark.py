import time
from math import pi
from gmanim import Scene, Timeline, Arrow3D, Rotate

def main():
    scene = Scene()
    scene.set_orthographic_camera()
    
    arrow = Arrow3D(start=(0, 0, 0), end=(1, 1, 0), shaft_radius=0.02, head_radius=0.05, head_length=0.1)
    scene.add(arrow)

    rotate_arrow = Rotate(arrow, axis=(0, 0, pi), center=(0, 0, 0), frames=240)
    
    # Pre-build timeline to avoid initialization overhead in timing
    timeline = Timeline(scene)
    timeline.play(rotate_arrow)

    start_time = time.time()
    timeline.render("benchmark.mp4")
    end_time = time.time()
    
    elapsed = end_time - start_time
    fps = 240 / elapsed
    print(f"Rendered 240 frames in {elapsed:.4f} seconds ({fps:.2f} FPS)")

if __name__ == "__main__":
    main()
