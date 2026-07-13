package algorithms;

import benchmark.Benchmark;

import java.util.function.Function;

public class Fibonacci {

  public static int fibonacci(int n) {

    if (n <= 1) {

      return n;

    }

    int a = 0;

    int b = 1;

    for (int i = 2; i <= n; i++) {

      int temp = a + b;

      a = b;

      b = temp;

    }

    return b;

  }

  public static void run() {

    int[] sizes = {

        1000000,

        2000000,

        5000000,

        6000000,

        10000000

    };

    Benchmark benchmark = new Benchmark(

        "fibonacci",

        "java",

        sizes,

        10

    );

    Function<int[], int[]> algorithm = data -> {

      int result = 0;

      for (int value : data) {

        result = fibonacci(value);

      }

      return new int[] { result };

    };

    Function<Integer, int[]> generator = size -> {

      int[] data = new int[size];

      for (int i = 0; i < size; i++) {

        data[i] = i % 40;

      }

      return data;

    };

    benchmark.run(

        algorithm,

        generator

    );

  }

}