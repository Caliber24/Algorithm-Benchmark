class TablePrinter:

    WIDTH = 118

    @staticmethod
    def line():

        print("=" * TablePrinter.WIDTH)

    @staticmethod
    def separator():

        print("-" * TablePrinter.WIDTH)

    @staticmethod
    def title(algorithm, language):

        TablePrinter.line()

        print("PERFORMANCE BENCHMARK REPORT".center(TablePrinter.WIDTH))

        TablePrinter.line()

        print(f"Algorithm : {algorithm}")

        print(f"Language  : {language}")

        TablePrinter.line()

    @staticmethod
    def header():

        print(
            f"{'Test':<6}"
            f"{'Input Size':>15}"
            f"{'Repeat':>12}"
            f"{'Avg(ms)':>14}"
            f"{'Min(ms)':>14}"
            f"{'Max(ms)':>14}"
            f"{'Std(ms)':>14}"
            f"{'Memory(MB)':>16}"
            f"{'Status':>13}"
        )

        TablePrinter.separator()

    @staticmethod
    def row(
            test,
            size,
            repeat,
            avg,
            minimum,
            maximum,
            std,
            memory,
            status
    ):

        print(
            f"{test:<6}"
            f"{size:>15,}"
            f"{repeat:>12}"
            f"{avg:>14.3f}"
            f"{minimum:>14.3f}"
            f"{maximum:>14.3f}"
            f"{std:>14.3f}"
            f"{memory:>16.2f}"
            f"{status:>13}"
        )

    @staticmethod
    def summary(results):

        TablePrinter.separator()

        avg = sum(r["average"] for r in results) / len(results)

        best = min(r["minimum"] for r in results)

        worst = max(r["maximum"] for r in results)

        print()

        print(f"Total Tests : {len(results)}")

        print(f"Average Time: {avg:.3f} ms")

        print(f"Fastest Run : {best:.3f} ms")

        print(f"Slowest Run : {worst:.3f} ms")

        TablePrinter.line()