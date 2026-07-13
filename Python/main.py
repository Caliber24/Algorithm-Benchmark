
from algorithms.prime import run as run_prime
from algorithms.fibonacci import run as run_fibonacci
from algorithms.quicksort import run as run_quicksort
from algorithms.mergesort import run as run_mergesort
from algorithms.matrix import run as run_matrix
from algorithms.subset import run as run_subset

import sys
from pathlib import Path


ROOT_DIR = Path(__file__).resolve().parents[1]

sys.path.append(
    str(ROOT_DIR)
)
from generate_chart import ChartGenerator


def create_chart(filename):

    chart = ChartGenerator()

    csv_file = (

        Path("python/results")

        /

        "csv"

        /

        filename

    )

    if csv_file.exists():

        chart.draw(csv_file)

    else:

        print(

            f"CSV not found -> {csv_file}"

        )


def menu():

    print("\n" + "=" * 60)
    print("ALGORITHM BENCHMARK".center(60))
    print("=" * 60)

    print("1. Prime Numbers")
    print("2. Fibonacci")
    print("3. Quick Sort")
    print("4. Merge Sort")
    print("5. Matrix Multiplication")
    print("6. Subset Generation")
    print("7. Run All")
    print("0. Exit")

    print("=" * 60)


def main():

    while True:

        menu()

        choice = input("Select Option : ")

        if choice == "1":

            run_prime()

            create_chart(
                "prime_numbers_python.csv"
            )

        elif choice == "2":

            run_fibonacci()

            create_chart(
                "fibonacci_python.csv"
            )

        elif choice == "3":

            run_quicksort()

            create_chart(
                "quick_sort_python.csv"
            )

        elif choice == "4":

            run_mergesort()

            create_chart(
                "merge_sort_python.csv"
            )

        elif choice == "5":

            run_matrix()

            create_chart(
                "matrix_multiplication_python.csv"
            )

        elif choice == "6":

            run_subset()

            create_chart(
                "subset_generation_python.csv"
            )

        elif choice == "7":

            run_prime()

            run_fibonacci()

            run_quicksort()

            run_mergesort()

            run_matrix()

            run_subset()

            chart = ChartGenerator()

            chart.draw_all()

        elif choice == "0":

            print("\nGood Bye!")

            break

        else:

            print("\nInvalid Option\n")


if __name__ == "__main__":

    main()
