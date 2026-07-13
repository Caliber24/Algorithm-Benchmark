import statistics
from datetime import datetime


def current_datetime():
    """
    Return current date and time.
    """
    return datetime.now().strftime("%Y-%m-%d %H:%M:%S")


def line(length=90):

    return "=" * length


def separator(length=90):

    return "-" * length


def average(values):

    if not values:
        return 0

    return statistics.mean(values)


def minimum(values):

    if not values:
        return 0

    return min(values)


def maximum(values):

    if not values:
        return 0

    return max(values)


def std(values):

    if len(values) <= 1:
        return 0

    return statistics.stdev(values)


def format_number(value):

    return f"{value:,}"


def format_time(ms):

    return f"{ms:.3f}"


def format_memory(memory):

    return f"{memory:.2f}"


def print_title(title):

    print()
    print(line())
    print(title.center(90))
    print(line())


def print_header(algorithm, language):

    print_title("PERFORMANCE BENCHMARK REPORT")

    print(f"Algorithm      : {algorithm}")
    print(f"Language       : {language}")
    print(f"Execution Date : {current_datetime()}")

    print(line())


def print_summary(times):

    print()

    print(separator())

    print("Summary")

    print(separator())

    print(f"Total Tests : {len(times)}")

    print(f"Average     : {average(times):.3f} ms")

    print(f"Minimum     : {minimum(times):.3f} ms")

    print(f"Maximum     : {maximum(times):.3f} ms")

    print(f"Std Dev     : {std(times):.3f} ms")

    print(line())
