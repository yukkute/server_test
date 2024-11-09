#!/usr/bin/env python3

import os
import subprocess
from pathlib import Path
import helpers as h
from task import Task


def build_flutter_client():
    """Compile MobX bindings"""
    project_root = h.project_root()

    client_dir = project_root / "client"
    os.chdir(client_dir)

    c = "\033[1;36m"
    r = "\033[0m"

    message = f"Building  {c}Flutter{r} 🐦 client"
    task = Task(message)

    try:
        task.start()
        subprocess.run(
            ["flutter", "build", "linux", "--release"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        task.finish()

    except Exception as e:
        task.error()
        h.abort_with(f"Error occurred while compiling MobX: {e}")


def main():
    build_flutter_client()


if __name__ == "__main__":
    main()
