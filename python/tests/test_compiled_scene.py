import gmanim


def test_compiled_scene_and_recorded_callback() -> None:
    scene = gmanim.Scene("compiled", resolution=(64, 64))
    rectangle = gmanim.Rectangle(width=2.0, height=1.0, fill=True)
    group = gmanim.Group(rectangle)
    scene.add(group)
    scene.set_point_light((4.0, 5.0, 6.0), (255, 240, 220, 255), 120.0)
    scene.set_environment_light((100, 120, 140, 255), 0.2)
    scene.play(gmanim.Move(group, (2.0, 0.0, 0.0), frames=2))

    samples = []

    def update(frame, alpha):
        frame.move_by(group, (0.25, 0.0, 0.0))
        frame.set_rectangle_corners(
            rectangle,
            [(-1.0, 0.5, 0.0), (-1.0, -0.5, 0.0), (1.0, -0.5, 0.0), (1.0, 0.5, 0.0)],
        )
        frame.set_orthographic_camera(height=9.0)
        frame.set_viewport(0.0, 0.0, 16.0, 9.0)
        frame.set_point_light((4.0, 5.0, 6.0), (255, 240, 220, 255), 120.0)
        frame.set_environment_light((100, 120, 140, 255), 0.2)
        samples.append((frame.frame, alpha, frame.get_position(group)[0]))

    scene.play(gmanim.UpdateFromFunc(update, frames=3))

    assert [sample[0] for sample in samples] == [1, 2, 3]
    assert [sample[2] for sample in samples] == [2.25, 2.5, 2.75]
    assert scene._get_render_info() == (5, 64, 64, 60)


def test_scene_identity_and_fps_are_constructor_configuration() -> None:
    scene = gmanim.Scene("high_fps_test", resolution=(64, 64), fps=120)
    scene.wait(120)

    assert scene.name == "high_fps_test"
    assert scene.fps == 120
    assert scene._get_render_info() == (120, 64, 64, 120)


def test_foreign_scene_handles_are_rejected() -> None:
    rectangle = gmanim.Rectangle()
    first = gmanim.Scene("first", resolution=(64, 64))
    second = gmanim.Scene("second", resolution=(64, 64))
    first.add(rectangle)

    try:
        second.play(gmanim.Move(rectangle, (1.0, 0.0, 0.0), frames=1))
    except ValueError:
        return
    raise AssertionError("foreign scene handle was accepted")


def test_callback_errors_propagate_without_poisoning_builder() -> None:
    scene = gmanim.Scene("callback_error", resolution=(64, 64))
    scene.add(gmanim.Rectangle())

    def fail(_frame, _alpha):
        raise LookupError("intentional callback failure")

    try:
        scene.play(gmanim.UpdateFromFunc(fail, frames=1))
    except LookupError as error:
        assert "intentional callback failure" in str(error)
    else:
        raise AssertionError("callback error did not propagate")

    scene.wait(1)
    assert scene._get_render_info()[0] == 1
