import gmanim
from gmanim import Scene, Timeline, Line, Rectangle, PolyLine, Arc, Dot, Text, Rotate, Move, Wait
import math
import os

print("Starting test_all.py...")

scene = Scene()

# Add various mobjects
line = scene.add(Line(p0=(-3.0, 2.0, 0.0), p1=(3.0, 2.0, 0.0), stroke_width=0.1, color=(255, 0, 0, 255)))
rect = scene.add(Rectangle(p0=(-2.0, -1.0, 0.0), p1=(2.0, -1.0, 0.0), p2=(2.0, 1.0, 0.0), p3=(-2.0, 1.0, 0.0), stroke_width=0.08, color=(0, 255, 0, 255)))
polyline = scene.add(PolyLine(points=[(-4.0, -2.0, 0.0), (-2.0, -3.0, 0.0), (0.0, -2.0, 0.0), (2.0, -3.0, 0.0), (4.0, -2.0, 0.0)], stroke_width=0.15, color=(0, 0, 255, 255)))
arc = scene.add(Arc(center=(0.0, 0.0, 0.0), start_angle=0.0, end_angle=math.pi, radius=2.0, stroke_width=0.1, color=(255, 255, 0, 255)))
dot = scene.add(Dot(position=(0.0, 0.0, 0.0), radius=0.15, color=(255, 0, 255, 255)))
text = scene.add(Text(text="Hello gmanim!", position=(-2.0, 3.0, 0.0), font_size=48.0, color=(255, 255, 255, 255)))

print("All types of mobjects created and added successfully!")

# Orchestrate timeline
timeline = Timeline(scene, output_width=1920, output_height=1080)

# Play rotate and move animations
print("Playing Rotate and Move animations...")
timeline.play(Rotate(line, axis=(0.0, 0.0, 1.57), center=(0.0, 2.0, 0.0), frames=60))
timeline.play(Move(dot, displacement=(1.0, 1.0, 0.0), frames=45))
timeline.play(Move(text, displacement=(0.0, -1.0, 0.0), frames=45))

# Run scene script in callback
print("Running callback to modify scene...")
def modify_scene(s):
    print("Inside callback! Modifying scene...")
    # Add a new dot
    new_dot = s.add(Dot(position=(3.0, 3.0, 0.0), radius=0.2, color=(0, 255, 255, 255)))
    # Remove the polyline
    s.remove(2)
    # Remove the newly added dot by passing the mobject directly
    s.remove(new_dot)

timeline.run(modify_scene)
timeline.play(Wait(30))

print("Rendering video output_all.mp4...")
timeline.render("output_all.mp4")
print("Finished! output_all.mp4 has been successfully rendered.")

# Clean up residual files
if os.path.exists("output_all.mp4"):
    os.remove("output_all.mp4")
    print("Cleaned up residual file: output_all.mp4")
