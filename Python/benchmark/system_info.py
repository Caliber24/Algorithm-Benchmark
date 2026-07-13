import os
import platform
import psutil
import socket


class SystemInfo:

    @staticmethod
    def get_info():

        memory = psutil.virtual_memory()

        return {
            "Computer Name": socket.gethostname(),
            "Operating System": f"{platform.system()} {platform.release()}",
            "Architecture": platform.machine(),
            "Processor": platform.processor(),
            "CPU Cores (Physical)": psutil.cpu_count(logical=False),
            "CPU Threads (Logical)": psutil.cpu_count(logical=True),
            "Total RAM (GB)": round(memory.total / (1024 ** 3), 2),
            "Python Version": platform.python_version()
        }

    @staticmethod
    def print_info():

        info = SystemInfo.get_info()

        print("=" * 90)
        print("SYSTEM INFORMATION".center(90))
        print("=" * 90)

        for key, value in info.items():
            print(f"{key:<25}: {value}")

        print("=" * 90)