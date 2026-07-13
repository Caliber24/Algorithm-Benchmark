from benchmark.benchmark import Benchmark


def fibonacci(n):

    if n <= 0:
        return []

    if n == 1:
        return [0]

    sequence = [0, 1]

    while len(sequence) < n:

        sequence.append(
            sequence[-1] + sequence[-2]
        )

    return sequence

def generate_data(size):

    return size
  
def run():

    benchmark = Benchmark(

        algorithm="Fibonacci",

        language="Python",

        sizes=[
            10000,
            20000,
            50000,
            60000,
            100000
        ],

        repeat=10

    )

    benchmark.run(

        algorithm_function=fibonacci,

        data_generator=generate_data

    )


if __name__ == "__main__":

    run()