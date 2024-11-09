import time
import threading
import subprocess
import re


def _count_ansi_escape_sequences(s: str):
    """Returns the total number of characters consumed by ANSI escape sequences in the string"""
    ansi_escape_pattern = r"\033\[[^m]*m"
    sequences = re.findall(ansi_escape_pattern, s)
    total_length = sum(len(seq) for seq in sequences)

    return total_length


class Task:
    def __init__(self, message: str):
        """
        Initialize the Task object with a message.
        :param message: The message to display with the spinner.
        """
        self.message = message
        self.spinner_chars = ["⡿", "⣟", "⣯", "⣷", "⣾", "⣽", "⣻", "⢿"]
        self.delay = 0.1
        self.spinner_index = 0
        self.start_time = None
        self.escape_n = self._count_ansi_escape_sequences(message)
        self.message = message.ljust(50 + self.escape_n, " ")
        self.spinner_thread = None
        self._running = False

    def _count_ansi_escape_sequences(self, s: str):
        """Returns the total number of characters consumed by ANSI escape sequences in the string"""
        ansi_escape_pattern = r"\033\[[^m]*m"
        sequences = re.findall(ansi_escape_pattern, s)
        total_length = sum(len(seq) for seq in sequences)
        return total_length

    def _spinner(self):
        """Runs the spinner in a separate thread."""
        while self._running:
            print(
                f"\r🔁 {self.message} {" "*7}{self.spinner_chars[self.spinner_index]}",
                end="",
                flush=True,
            )
            self.spinner_index = (self.spinner_index + 1) % len(self.spinner_chars)
            time.sleep(self.delay)

    def start(self):
        """Starts the task"""
        self.start_time = time.time()
        self._running = True
        self.spinner_thread = threading.Thread(target=self._spinner)
        self.spinner_thread.daemon = (
            True  # This allows the thread to exit when the program ends
        )
        self.spinner_thread.start()

    def finish(self):
        """Finishes the task, showing the success message and elapsed time."""
        self._running = False
        if self.spinner_thread is not None:
            self.spinner_thread.join()  # Ensure the spinner thread has finished before proceeding

        elapsed_time = time.time() - self.start_time
        print(f"\r✅ {self.message}", end="")
        print(f"∴ {elapsed_time:5.1f} s", end="\n", flush=True)

    def error(self):
        """Handles an error by displaying an error message."""
        self._running = False
        if self.spinner_thread is not None:
            self.spinner_thread.join()

        print(f"\r❌ {self.message} error.", end="\n", flush=True)
