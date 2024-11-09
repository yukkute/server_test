#!/usr/bin/env python3

import os
import subprocess
from pathlib import Path
import helpers as h
from task import Task


def compile_mobx():
    """Compile MobX bindings"""
    project_root = h.project_root()

    client_dir = project_root / "client"
    os.chdir(client_dir)

    c = "\033[1;33m"
    r = "\033[0m"

    message = f"Compiling {c}MobX{r} ⚡ bindings"
    task = Task(message)

    try:
        task.start()
        subprocess.run(
            ["dart", "run", "build_runner", "build"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        task.finish()

    except Exception as e:
        task.error()
        h.abort_with(f"Error occurred while compiling MobX: {e}")


def main():
    compile_mobx()


if __name__ == "__main__":
    main()
