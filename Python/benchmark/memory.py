import os
import psutil
import tracemalloc


class MemoryMonitor:

    def __init__(self):

        self.process = psutil.Process(os.getpid())

    # ------------------------------------------

    def start(self):
        """
        Start tracing memory allocation.
        """

        tracemalloc.start()

    # ------------------------------------------

    def stop(self):
        """
        Stop tracing memory allocation.
        """

        tracemalloc.stop()

    # ------------------------------------------

    def current_memory(self):
        """
        Current process memory (RSS) in MB.
        """

        memory = self.process.memory_info().rss

        return memory / 1024 / 1024

    # ------------------------------------------

    def peak_memory(self):
        """
        Peak allocated memory (Python allocation)
        in MB.
        """

        current, peak = tracemalloc.get_traced_memory()

        return peak / 1024 / 1024

    # ------------------------------------------

    def memory_info(self):

        return {
            "Current(MB)": round(self.current_memory(), 2),
            "Peak(MB)": round(self.peak_memory(), 2)
        }