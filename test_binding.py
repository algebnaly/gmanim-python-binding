import gmanim
from gmanim import Scene, Timeline, Line, Rectangle, Rotate, Move, Wait

print("Imports succeeded!")

scene = Scene()
print("Scene created!")

line = scene.add(Line(p0=(0.0, 0.0, 0.0), p1=(1.0, 1.0, 0.0)))
rect = scene.add(Rectangle(p0=(0.0, 0.0, 0.0), p1=(2.0, 0.0, 0.0), p2=(2.0, 1.0, 0.0), p3=(0.0, 1.0, 0.0)))
print("Mobjects added to Scene!")

timeline = Timeline(scene, output_width=1920, output_height=1080)
print("Timeline created!")

timeline.play(Rotate(line, axis=(0.0, 0.0, 3.14), center=(0.0, 0.0, 0.0), frames=120))
timeline.play(Move(rect, displacement=(2.0, 0.0, 0.0), frames=60))
print("Animations queued!")

# Modify scene between animations
timeline.run(lambda s: s.remove(1))  # remove rect (index 1)
timeline.play(Wait(30))
print("Wait animation queued!")

# Render
timeline.render("output.mp4")
print("Rendering finished and output.mp4 generated!")
