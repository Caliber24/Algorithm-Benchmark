#include <iostream>
#include <vector>
#include <random>
#include <cstdlib>
#include <string>

#include "benchmark/Benchmark.hpp"

#include "algorithms/Prime.hpp"
#include "algorithms/Fibonacci.hpp"
#include "algorithms/QuickSort.hpp"
#include "algorithms/MergeSort.hpp"
#include "algorithms/Matrix.hpp"
#include "algorithms/Subset.hpp"

void createChart(
    std::string filename)

{

  std::string command =

      "python ../generate_chart.py "

      + filename;

  system(

      command.c_str()

  );
}

std::vector<int> generateArray(
    int size)

{

  std::vector<int> data(size);

  std::random_device rd;

  std::mt19937 gen(rd());

  std::uniform_int_distribution<> dist(
      1,
      10000);

  for (int i = 0; i < size; i++)

  {

    data[i] = dist(gen);
  }

  return data;
}

std::vector<std::vector<int>> generateMatrix(
    int size)

{

  std::vector<std::vector<int>> matrix(

      size,

      std::vector<int>(size)

  );

  for (int i = 0; i < size; i++)

  {

    for (int j = 0; j < size; j++)

    {

      matrix[i][j] = i + j;
    }
  }

  return matrix;
}

void runPrime()

{

  Benchmark benchmark(

      "prime_numbers",

      "cpp",

      {1000, 10000, 100000, 500000, 10000000},

      10

  );

  benchmark.run(

      [](int n)

      {
        Prime::run(n);
      },

      [](int size)

      {
        return size;
      }

  );

  createChart(
      "prime_numbers_cpp.csv");
}

void runFibonacci()

{

  Benchmark benchmark(

      "fibonacci",

      "cpp",

      {1000000, 2000000, 5000000, 6000000, 10000000},

      10

  );

  benchmark.run(

      [](int n)

      {
        Fibonacci::run(n);
      },

      [](int size)

      {
        return size;
      }

  );

  createChart(
      "fibonacci_cpp.csv");
}

void runQuickSort()

{

  Benchmark benchmark(

      "quick_sort",

      "cpp",

      {1000, 10000, 100000,500000,1000000},

      10

  );

  benchmark.run(

      [](std::vector<int> data)

      {
        QuickSort::run(data);
      },

      [](int size)

      {
        return generateArray(size);
      }

  );

  createChart(
      "quick_sort_cpp.csv");
}

void runMergeSort()

{

  Benchmark benchmark(

      "merge_sort",

      "cpp",

      {1000, 10000, 100000, 500000, 1000000},

      10

  );

  benchmark.run(

      [](std::vector<int> data)

      {
        MergeSort::run(data);
      },

      [](int size)

      {
        return generateArray(size);
      }

  );

  createChart(
      "merge_sort_cpp.csv");
}

void runMatrix()

{

  Benchmark benchmark(

      "matrix_multiplication",

      "cpp",

      {50, 100, 200, 300, 500},

      10

  );

  benchmark.run(

      [](std::vector<std::vector<int>> A)

      {
        Matrix::run(
            A,
            A);
      },

      [](int size)

      {
        return generateMatrix(size);
      }

  );

  createChart(
      "matrix_multiplication_cpp.csv");
}

void runSubset()

{

  Benchmark benchmark(

      "subset_generation",

      "cpp",

      {5, 10, 15, 20, 22},

      10

  );

  benchmark.run(

      [](std::vector<int> data)

      {
        Subset::run(data);
      },

      [](int size)

      {
        return generateArray(size);
      }

  );

  createChart(
      "subset_generation_cpp.csv");
}

int main()

{

  while (true)

  {

    std::cout << "\n";

    std::cout << "==============================\n";

    std::cout << "ALGORITHM BENCHMARK C++\n";

    std::cout << "==============================\n";

    std::cout << "1. Prime Numbers\n";

    std::cout << "2. Fibonacci\n";

    std::cout << "3. Quick Sort\n";

    std::cout << "4. Merge Sort\n";

    std::cout << "5. Matrix Multiplication\n";

    std::cout << "6. Subset Generation\n";

    std::cout << "7. Run All\n";

    std::cout << "0. Exit\n";

    std::cout << "==============================\n";

    std::cout << "Select Option: ";

    int choice;

    std::cin >> choice;

    switch (choice)

    {

    case 1:

      runPrime();

      break;

    case 2:

      runFibonacci();

      break;

    case 3:

      runQuickSort();

      break;

    case 4:

      runMergeSort();

      break;

    case 5:

      runMatrix();

      break;

    case 6:

      runSubset();

      break;

    case 7:

      runPrime();

      runFibonacci();

      runQuickSort();

      runMergeSort();

      runMatrix();

      runSubset();

      break;

    case 0:

      std::cout << "Good Bye\n";

      return 0;

    default:

      std::cout << "Invalid Option\n";
    }
  }
}


