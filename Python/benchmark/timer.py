import time


class Timer:

    def __init__(self):
        self.start_time = 0.0
        self.end_time = 0.0

    def start(self):
        """
        Start timer
        """
        self.start_time = time.perf_counter()

    def stop(self):
        """
        Stop timer
        """
        self.end_time = time.perf_counter()

    def elapsed_ms(self):
        """
        Return elapsed time in milliseconds.
        """
        return (self.end_time - self.start_time) * 1000

    def elapsed_seconds(self):
        """
        Return elapsed time in seconds.
        """
        return self.end_time - self.start_time