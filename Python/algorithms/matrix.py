import random

from benchmark.benchmark import Benchmark


def matrix_multiply(A, B):

    n = len(A)

    C = [[0] * n for _ in range(n)]

    for i in range(n):
        for j in range(n):

            total = 0

            for k in range(n):

                total += A[i][k] * B[k][j]

            C[i][j] = total

    return C
  
def generate_data(size):

    A = [

        [

            random.randint(1, 10)

            for _ in range(size)

        ]

        for _ in range(size)

    ]

    B = [

        [

            random.randint(1, 10)

            for _ in range(size)

        ]

        for _ in range(size)

    ]

    return (A, B)


def run():

    benchmark = Benchmark(

        algorithm="Matrix Multiplication",

        language="Python",

        sizes=[
            50,
            100,
            200,
            300,
            500
            ],

        repeat=3

    )

    benchmark.run(

        algorithm_function=matrix_multiply,

        data_generator=generate_data

    )


if __name__ == "__main__":

    run()