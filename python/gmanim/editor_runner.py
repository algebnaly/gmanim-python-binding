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
    parser.add_argument("--shm-id", required=True)
    parser.add_argument("--ctrl-socket", required=True)
    args = parser.parse_args()

    config.editor_mode = True
    config.shm_id = args.shm_id
    config.ctrl_socket = args.ctrl_socket

    sys.path.insert(0, os.path.dirname(os.path.abspath(args.script)))

    # Hijack Scene.render so the user's script doesn't render mp4
    gmanim.Scene.render = lambda self, *a, **kw: None

    # Execute user script to populate gmanim.registry
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

    scenes = list(gmanim.registry.keys())
    send_event({"event": "scenes_info", "scenes": scenes})

    while True:
        line = ctrl_stream.readline()
        if not line:
            break

        cmd = json.loads(line.decode("utf-8"))

        if cmd["cmd"] == "render_scene":
            scene_name = cmd.get("name", "")
            func = gmanim.registry.get(scene_name)
            if not func and scenes:
                func = gmanim.registry[scenes[0]]

            if func:
                scene = gmanim.Scene()
                func(scene)

                total_frames, width, height = scene._get_render_info()
                send_event(
                    {
                        "event": "start_render",
                        "total_frames": total_frames,
                        "width": width,
                        "height": height,
                    }
                )

                # C-Extension renders and pushes to SHM
                scene._render_to_shm(args.shm_id)

                send_event({"event": "finish_render"})

        elif cmd["cmd"] == "quit":
            break
