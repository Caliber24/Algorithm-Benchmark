import random

from benchmark.benchmark import Benchmark


def merge_sort(arr):

    if len(arr) <= 1:
        return arr

    mid = len(arr) // 2

    left = merge_sort(arr[:mid])

    right = merge_sort(arr[mid:])

    return merge(left, right)


def merge(left, right):

    result = []

    i = 0

    j = 0

    while i < len(left) and j < len(right):

        if left[i] <= right[j]:

            result.append(left[i])

            i += 1

        else:

            result.append(right[j])

            j += 1

    result.extend(left[i:])

    result.extend(right[j:])

    return result


def generate_data(size):

    return [
        random.randint(1, size)
        for _ in range(size)
    ]


def run():

    benchmark = Benchmark(

        algorithm="Merge Sort",

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

        algorithm_function=merge_sort,

        data_generator=generate_data

    )


if __name__ == "__main__":

    run()