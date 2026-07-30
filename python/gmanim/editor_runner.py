import argparse
import runpy
import sys
import os
import json
import socket
import gmanim
from gmanim.config import config

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("script")
    parser.add_argument("--ctrl-socket", required=True)
    args = parser.parse_args()

    config.editor_mode = True
    config.ctrl_socket = args.ctrl_socket

    sys.path.insert(0, os.path.dirname(os.path.abspath(args.script)))

    rendered_scenes = {}

    # In editor mode render publishes the completed scene instead of encoding video.
    def publish_scene(scene, *args, **kwargs):
        existing = rendered_scenes.get(scene.name)
        if existing is not None and existing is not scene:
            raise ValueError(f"duplicate rendered scene name: {scene.name!r}")
        rendered_scenes[scene.name] = scene

    gmanim.Scene.render = publish_scene

    # Execute the user's normal entry point so Scene construction stays authoritative.
    runpy.run_path(args.script, run_name="__main__")

    # Connect to Editor
    if os.name == "nt":
        ctrl_stream = open(args.ctrl_socket, "r+b")
    else:
        ctrl_socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        ctrl_socket.connect(args.ctrl_socket)
        ctrl_stream = ctrl_socket.makefile("rwb")

    def send_event(event):
        data = (json.dumps(event) + "\n").encode("utf-8")
        if os.name == "nt":
            ctrl_stream.write(data)
            ctrl_stream.flush()
        else:
            ctrl_stream.write(data)
            ctrl_stream.flush()

    scenes = list(rendered_scenes)
    send_event({"event": "scenes_info", "scenes": scenes})
    pending_scene = None
    preview = None

    while True:
        line = ctrl_stream.readline()
        if not line:
            break

        cmd = json.loads(line.decode("utf-8"))

        if cmd["cmd"] == "load_scene":
            scene_name = cmd.get("name", "")
            scene = rendered_scenes.get(scene_name)
            if scene is None and scenes:
                scene = rendered_scenes[scenes[0]]

            if scene is not None:
                total_frames, width, height, framerate = scene._get_render_info()
                pending_scene = scene
                preview = None
                send_event(
                    {
                        "event": "scene_ready",
                        "total_frames": total_frames,
                        "width": width,
                        "height": height,
                        "framerate": framerate,
                    }
                )

        elif cmd["cmd"] == "open_preview":
            if pending_scene is None:
                send_event({"event": "error", "message": "no scene is ready for preview"})
                continue
            try:
                preview = pending_scene._open_preview(cmd["shm_id"])
                pending_scene = None
                send_event({"event": "preview_opened"})
            except Exception as error:
                send_event({"event": "error", "message": str(error)})

        elif cmd["cmd"] == "render_frame":
            if preview is None:
                send_event({"event": "error", "message": "preview is not open"})
                continue
            request_id = cmd["request_id"]
            frame = cmd["frame"]
            slot = cmd["slot"]
            try:
                preview.render_frame(request_id, frame, slot)
                send_event(
                    {
                        "event": "frame_ready",
                        "request_id": request_id,
                        "frame": frame,
                        "slot": slot,
                    }
                )
            except Exception as error:
                send_event({"event": "error", "message": str(error)})

        elif cmd["cmd"] == "quit":
            break
