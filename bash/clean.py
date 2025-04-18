#!/usr/bin/env python3

import os
import shutil
import subprocess
import helpers as h
from task import Task


def clean_build_directories(ask=False):
    """Clean the build directories for client and server"""
    project_root = h.project_root()

    if ask:
        while True:
            i = input("❔ Do you want a clean build? (Y/n): ").strip().lower()
            if i in ["", "y"]:
                break
            elif i in ["n"]:
                return
            else:
                continue

    client_generated_dir = project_root / "client" / "lib" / "generated"
    protobuf_dir = client_generated_dir / "protobuf"
    mobx_dir = client_generated_dir / "mobx"

    proto_symlink = client_generated_dir / "proto_symlink"

    dirs_to_clean = [protobuf_dir, mobx_dir]

    message = f"Cleaning 🧹 build directories"
    task = Task(message)

    try:
        task.start()

        if proto_symlink.exists() and proto_symlink.is_symlink():
            proto_symlink.unlink()

        for dir_path in dirs_to_clean:
            if dir_path.exists():
                shutil.rmtree(dir_path)

        subprocess.run(
            ["flutter", "clean"],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=project_root / "client",
        )

        subprocess.run(
            ["cargo", "clean"],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd=project_root / "server",
        )
        task.finish()

    except Exception as e:
        task.error()
        h.abort_with(f"Unexpected error: {e}")


def main():
    clean_build_directories(ask=False)


if __name__ == "__main__":
    main()
