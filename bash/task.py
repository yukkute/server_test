import time
import threading
import subprocess
import re


class Task:
    def __init__(self, message: str):
        self.message = message
        self.spinner_chars = ["⡿", "⣟", "⣯", "⣷", "⣾", "⣽", "⣻", "⢿"]
        self.delay = 0.1
        self.spinner_index = 0
        self.start_time = None
        self.spinner_thread = None
        self._running = False

    def _measure_time(self):
        t = elapsed_time = time.time() - self.start_time
        return f"{t:5.1f}"

    def _spinner(self):
        c = "\033[1;34m"
        r = "\033[0m"
        while self._running:
            sc = self.spinner_chars[self.spinner_index]
            print(
                f"\r{c}[{sc} {self._measure_time()} s]{r} {self.message}",
                end="",
                flush=True,
            )
            self.spinner_index = (self.spinner_index + 1) % len(self.spinner_chars)
            time.sleep(self.delay)

    def start(self):
        self.start_time = time.time()
        self._running = True
        self.spinner_thread = threading.Thread(target=self._spinner)
        self.spinner_thread.daemon = True
        self.spinner_thread.start()

    def finish(self):
        self._running = False
        if self.spinner_thread is not None:
            self.spinner_thread.join()

        c = "\033[1;32m"
        r = "\033[0m"

        elapsed_time = time.time() - self.start_time
        print(
            f"\r{c}[✓ {self._measure_time()} s]{r} {self.message}",
            end="\n",
            flush=True,
        )

    def error(self):
        """Handles an error by displaying an error message."""
        self._running = False
        if self.spinner_thread is not None:
            self.spinner_thread.join()

        print(f"\r❌ {self.message} error.", end="\n", flush=True)
