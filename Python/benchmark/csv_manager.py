import csv
from pathlib import Path


class CSVManager:

    def __init__(self):

        self.output_dir = Path("python/results/csv")

        self.output_dir.mkdir(parents=True, exist_ok=True)


    def save(self,
             algorithm,
             language,
             benchmark_results):
        """
        Save benchmark results.

        Parameters
        ----------

        algorithm : str

        language : str

        benchmark_results : list
        """

        filename = f"{algorithm.lower().replace(' ','_')}_{language.lower()}.csv"

        filepath = self.output_dir / filename

        with open(filepath,
                  "w",
                  newline="",
                  encoding="utf-8") as file:

            writer = csv.writer(file)

            writer.writerow([
                "Test",
                "Input Size",
                "Average(ms)",
                "Minimum(ms)",
                "Maximum(ms)",
                "Std(ms)",
                "Memory(MB)",
                "Status"
            ])

            for row in benchmark_results:

                writer.writerow([
                    row["test"],
                    row["input_size"],
                    row["average"],
                    row["minimum"],
                    row["maximum"],
                    row["std"],
                    row["memory"],
                    row["status"]
                ])

        return filepath