import gc

from benchmark.csv_manager import CSVManager
from benchmark.memory import MemoryMonitor
from benchmark.system_info import SystemInfo
from benchmark.table import TablePrinter
from benchmark.timer import Timer
from benchmark.utils import average, maximum, minimum, std


class Benchmark:

    def __init__(
            self,
            algorithm,
            language,
            sizes,
            repeat=10
    ):

        self.algorithm = algorithm

        self.language = language

        self.sizes = sizes

        self.repeat = repeat

        self.timer = Timer()

        self.memory = MemoryMonitor()

        self.csv = CSVManager()

        self.results = []

    # --------------------------------------------------------

    def clean(self):

        gc.collect()

    # --------------------------------------------------------

    def start(self):

        print()

        SystemInfo.print_info()

        print()

        TablePrinter.title(

            self.algorithm,

            self.language

        )

        TablePrinter.header()

    # --------------------------------------------------------

    def add_result(

            self,

            size,

            avg,

            minimum_time,

            maximum_time,

            std_time,

            memory

    ):

        self.results.append({

            "test": len(self.results)+1,

            "input_size": size,

            "average": avg,

            "minimum": minimum_time,

            "maximum": maximum_time,

            "std": std_time,

            "memory": memory,

            "status": "PASS"

        })

    def run(self, algorithm_function, data_generator):

        self.start()

        for size in self.sizes:

            result = self.measure(
                algorithm_function=algorithm_function,
                data_generator=data_generator,
                size=size
            )

            self.add_result(
                size=size,
                avg=result["average"],
                minimum_time=result["minimum"],
                maximum_time=result["maximum"],
                std_time=result["std"],
                memory=result["memory"]
            )

        self.finish()

    def measure(
        self,
        algorithm_function,
        data_generator,
        size
    ):

        times = []

        peak_memory = 0

        for _ in range(self.repeat):

            self.clean()

            data = data_generator(size)

            self.memory.start()

            self.timer.start()

            if isinstance(data, tuple):
                algorithm_function(*data)
            else:
                algorithm_function(data)

            self.timer.stop()

            info = self.memory.memory_info()

            self.memory.stop()

            times.append(
                self.timer.elapsed_ms()
            )

            peak_memory = max(
                peak_memory,
                info["Peak(MB)"]
            )

        return {

            "average": average(times),

            "minimum": minimum(times),

            "maximum": maximum(times),

            "std": std(times),

            "memory": peak_memory

        }

    def finish(self):

        for row in self.results:

            TablePrinter.row(
                test=row["test"],
                size=row["input_size"],
                repeat=self.repeat,
                avg=row["average"],
                minimum=row["minimum"],
                maximum=row["maximum"],
                std=row["std"],
                memory=row["memory"],
                status=row["status"]
            )

        TablePrinter.summary(self.results)

        self.csv.save(
            algorithm=self.algorithm,
            language=self.language,
            benchmark_results=self.results
        )

    def get_results(self):

        return self.results
