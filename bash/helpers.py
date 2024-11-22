from pathlib import Path
import subprocess
import sys


def project_root():
    """Get the project root directory"""
    return Path(__file__).resolve().parent.parent


def abort_with(message: str):
    """Print error message and terminate"""
    print(f"Error: {message}", file=sys.stderr)
    exit(1)
