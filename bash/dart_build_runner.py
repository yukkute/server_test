#!/usr/bin/env python3

import os
import subprocess
from pathlib import Path
import helpers as h
from task import Task


def run_build_runner():
    """Compile Protobuf and MobX bindings"""
    project_root = h.project_root()
    os.chdir(project_root)

    client_dir = project_root / "client"
    proto_source = project_root / "proto"
    proto_symlink_destination = client_dir / "lib" / "generated" / "proto_symlink"

    c_mobx = "\033[1;33m"
    c_proto = "\033[1;97m"
    r = "\033[0m"

    message = f"Compiling {c_mobx}MobX{r} ⚡ and {c_proto}Protobuf{r} 📊"
    task = Task(message)

    try:
        task.start()

        if not proto_symlink_destination.exists():
            os.symlink(proto_source, proto_symlink_destination)

        subprocess.run(
            ["dart", "run", "build_runner", "build"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=project_root / "client",
        )

        task.finish()

    except Exception as e:
        task.error()
        h.abort_with(f"Error occurred while compiling MobX: {e}")


def main():
    run_build_runner()


if __name__ == "__main__":
    main()
