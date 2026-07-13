from math import isqrt

from benchmark.benchmark import Benchmark


def prime_numbers(limit):

    if limit < 2:
        return []

    prime = [True] * (limit + 1)

    prime[0] = False
    prime[1] = False

    for i in range(2, isqrt(limit) + 1):

        if prime[i]:

            for j in range(i * i, limit + 1, i):

                prime[j] = False

    return [i for i, value in enumerate(prime) if value]


def generate_data(size):

    return size


def run():

    benchmark = Benchmark(
        algorithm="Prime Numbers",
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
        algorithm_function=prime_numbers,
        data_generator=generate_data
    )


if __name__ == "__main__":
    run()