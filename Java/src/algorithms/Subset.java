package algorithms;

import benchmark.Benchmark;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Function;

public class Subset {

  public static void generate(

      int[] data,

      int index,

      List<List<Integer>> result,

      List<Integer> current

  ) {

    if (index == data.length) {

      result.add(

          new ArrayList<>(current)

      );

      return;

    }

    generate(

        data,

        index + 1,

        result,

        current

    );

    current.add(

        data[index]

    );

    generate(

        data,

        index + 1,

        result,

        current

    );

    current.remove(

        current.size() - 1

    );

  }

  public static List<List<Integer>> createSubsets(

      int[] data

  ) {

    List<List<Integer>> result = new ArrayList<>();

    generate(

        data,

        0,

        result,

        new ArrayList<>()

    );

    return result;

  }

  public static void run() {

    int[] sizes = {
        5,

        10,

        15,

        20,

        22,

    };

    Benchmark benchmark = new Benchmark(

        "subset_generation",

        "java",

        sizes,

        5

    );

    Function<int[], List<List<Integer>>> algorithm = data -> {

      return createSubsets(data);

    };

    Function<Integer, int[]> generator = size -> {

      int[] data = new int[size];

      for (int i = 0; i < size; i++) {

        data[i] = i;

      }

      return data;

    };

    benchmark.run(

        algorithm,

        generator

    );

  }

}