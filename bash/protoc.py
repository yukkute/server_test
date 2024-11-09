#!/usr/bin/env python3

import os
import subprocess
import glob
import time
import helpers as h
from task import Task


def generate_protobuf_files():
    """Generate Pb and gRPC classes for Flutter client"""
    project_root = h.project_root()

    protobuf_gen_dir = project_root / "client" / "lib" / "generated" / "protobuf"
    protobuf_gen_dir.mkdir(parents=True, exist_ok=True)

    files = glob.glob(str(project_root / "proto" / "*.proto"))

    c = "\033[1;97m"
    r = "\033[0m"

    message = f"Compiling {c}Protobuf{r} 📊"
    task = Task(message)

    try:
        task.start()
        subprocess.run(
            [
                "protoc",
                f'--proto_path={project_root / "proto"}',
                f"--dart_out=grpc:{protobuf_gen_dir}",
                *files,
            ],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        task.finish()

    except Exception as e:
        task.error()
        h.abort_with(f"{e}")


if __name__ == "__main__":
    generate_protobuf_files()
