import random

from benchmark.benchmark import Benchmark


def quick_sort(arr):

    if len(arr) <= 1:
        return arr

    pivot = arr[len(arr) // 2]

    left = [x for x in arr if x < pivot]

    middle = [x for x in arr if x == pivot]

    right = [x for x in arr if x > pivot]

    return quick_sort(left) + middle + quick_sort(right)

def generate_data(size):

    return [
        random.randint(1, size)
        for _ in range(size)
    ]
  
def run():

    benchmark = Benchmark(

        algorithm="Quick Sort",

        language="Python",

        sizes=[
            1000,
            10000,
            100000,
            500000,
            1000000
        ],

        repeat=10

    )

    benchmark.run(

        algorithm_function=quick_sort,

        data_generator=generate_data

    )


if __name__ == "__main__":

    run()