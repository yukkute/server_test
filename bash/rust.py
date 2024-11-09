#!/usr/bin/env python3

import os
import subprocess
from pathlib import Path
import helpers as h
from task import Task


def build_rust_server():
    """Build the Rust server"""
    project_root = h.project_root()

    rust_project_dir = project_root / "server"
    os.chdir(rust_project_dir)

    c = "\033[1;38;5;214m"
    r = "\033[0m"

    message = f"Building  {c}Rust{r} 🦀 server"
    task = Task(message)

    try:
        task.start()
        subprocess.run(
            [
                "cargo",
                "build",
                "--quiet",
                "--release",
                "--target",
                "x86_64-unknown-linux-gnu",
            ],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        task.finish()

    except Exception as e:
        task.error()
        h.abort_with(f"Error occurred while building Rust server: {e}")


def copy_shared_lib():
    """Copy the shared library to Flutter client's bundle directory"""
    project_root = h.project_root()

    lib_path = (
        project_root
        / "server"
        / "target"
        / "x86_64-unknown-linux-gnu"
        / "release"
        / "libmoreonigiri_server.so"
    )

    linux_target = project_root / "client" / "build" / "linux" / "x64"

    dest_dirs = [
        linux_target / "debug" / "bundle" / "lib",
        linux_target / "release" / "bundle" / "lib",
    ]

    message = f"Adding shared libraries 🔗 to bundle"
    task = Task(message)
    task.start()

    for dest_dir in dest_dirs:
        dest_dir.mkdir(parents=True, exist_ok=True)
        try:
            subprocess.run(["cp", str(lib_path), str(dest_dir)], check=True)

        except Exception as e:
            task.error()
            h.abort_with(f"Error copying shared library: {e}")

    task.finish()


def main():
    build_rust_server()
    copy_shared_lib()


if __name__ == "__main__":
    main()
