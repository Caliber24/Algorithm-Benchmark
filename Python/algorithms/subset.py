from benchmark.benchmark import Benchmark


def generate_subsets(items):

    n = len(items)

    subsets = []

    for mask in range(1 << n):

        subset = []

        for i in range(n):

            if mask & (1 << i):

                subset.append(items[i])

        subsets.append(subset)

    return subsets


def generate_data(size):

    return list(range(size))


def run():

    benchmark = Benchmark(

        algorithm="Subset Generation",

        language="Python",

        sizes=[
            5,
            10,
            15,
            20,
            22

        ],

        repeat=3

    )

    benchmark.run(

        algorithm_function=generate_subsets,

        data_generator=generate_data

    )


if __name__ == "__main__":

    run()