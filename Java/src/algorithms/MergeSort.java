package algorithms;

import benchmark.Benchmark;

import java.util.Random;
import java.util.function.Function;

public class MergeSort {

  public static void mergeSort(

      int[] array,

      int left,

      int right

  ) {

    if (left < right) {

      int middle =

          (left + right) / 2;

      mergeSort(

          array,

          left,

          middle

      );

      mergeSort(

          array,

          middle + 1,

          right

      );

      merge(

          array,

          left,

          middle,

          right

      );

    }

  }

  private static void merge(

      int[] array,

      int left,

      int middle,

      int right

  ) {

    int n1 = middle - left + 1;

    int n2 = right - middle;

    int[] L = new int[n1];

    int[] R = new int[n2];

    for (int i = 0; i < n1; i++) {

      L[i] = array[left + i];

    }

    for (int j = 0; j < n2; j++) {

      R[j] = array[middle + 1 + j];

    }

    int i = 0;

    int j = 0;

    int k = left;

    while (i < n1 && j < n2) {

      if (L[i] <= R[j]) {

        array[k] = L[i];

        i++;

      }

      else {

        array[k] = R[j];

        j++;

      }

      k++;

    }

    while (i < n1) {

      array[k] = L[i];

      i++;

      k++;

    }

    while (j < n2) {

      array[k] = R[j];

      j++;

      k++;

    }

  }

  public static int[] sort(

      int[] data

  ) {

    mergeSort(

        data,

        0,

        data.length - 1

    );

    return data;

  }

  public static void run() {

    int[] sizes = {

        1000,

        10000,

        100000,

        500000,

        1000000

    };

    Benchmark benchmark = new Benchmark(

        "merge_sort",

        "java",

        sizes,

        10

    );

    Function<int[], int[]> algorithm = data -> {

      return sort(data);

    };

    Function<Integer, int[]> generator = size -> {

      Random random = new Random();

      int[] data = new int[size];

      for (int i = 0; i < size; i++) {

        data[i] = random.nextInt(100000);

      }

      return data;

    };

    benchmark.run(

        algorithm,

        generator

    );

  }

}