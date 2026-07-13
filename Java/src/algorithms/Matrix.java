package algorithms;

import java.util.Random;
import java.util.function.Function;

import benchmark.Benchmark;

public class Matrix {

  public static int[][] multiply(

      int[][] A,

      int[][] B

  ) {

    int size = A.length;

    int[][] result = new int[size][size];

    for (int i = 0; i < size; i++) {

      for (int j = 0; j < size; j++) {

        for (int k = 0; k < size; k++) {

          result[i][j] +=

              A[i][k] * B[k][j];

        }

      }

    }

    return result;

  }

  public static int[][][] generate(

      int size

  ) {

    Random random = new Random();

    int[][] A = new int[size][size];

    int[][] B = new int[size][size];

    for (int i = 0; i < size; i++) {

      for (int j = 0; j < size; j++) {

        A[i][j] = random.nextInt(100);

        B[i][j] = random.nextInt(100);

      }

    }

    return new int[][][] {

        A,

        B

    };

  }

  public static void run() {

    int[] sizes = {

        50,

        100,

        200,

        300,

        500

    };

    Benchmark benchmark = new Benchmark(

        "matrix_multiplication",

        "java",

        sizes,

        5

    );

    Function<int[][], int[][]> algorithm = data -> {

      return data;

    };

    Function<Integer, int[][]> generator = size -> {

      int[][][] matrices = generate(size);

      int[][] A = matrices[0];

      int[][] B = matrices[1];

      int[][] result = multiply(

          A,

          B

      );

      return result;

    };

    benchmark.run(

        algorithm,

        generator

    );

  }

}