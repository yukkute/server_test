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


def installed(command: str) -> bool:
    """
    Checks whether a command is installed by running 'command --version'.

    Args:
        command (str): The command to check (e.g., 'protoc', 'python', 'git').

    Returns:
        bool: True if the command is installed
    """
    try:
        # Run the command with the --version flag
        subprocess.run(
            [command, "--version"],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        return True
    except subprocess.CalledProcessError:
        return False
    except FileNotFoundError:
        return False
