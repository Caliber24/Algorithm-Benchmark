package benchmark;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Function;

public class Benchmark {

  private String algorithm;

  private String language;

  private int[] sizes;

  private int repeat;

  private Timer timer;

  private MemoryMonitor memory;

  private List<String[]> results;

  public Benchmark(

      String algorithm,

      String language,

      int[] sizes,

      int repeat

  ) {

    this.algorithm = algorithm;

    this.language = language;

    this.sizes = sizes;

    this.repeat = repeat;

    this.timer = new Timer();

    this.memory = new MemoryMonitor();

    this.results = new ArrayList<>();

  }

  public void start() {

    SystemInfo.printInfo();

    TablePrinter.title(

        algorithm,

        language

    );

    TablePrinter.header();

  }

  public void addResult(

      int test,

      int inputSize,

      double average,

      double minimum,

      double maximum,

      double std,

      double memoryUsed

  ) {

    results.add(

        new String[] {

            String.valueOf(test),

            String.valueOf(inputSize),

            String.format("%.3f", average),

            String.format("%.3f", minimum),

            String.format("%.3f", maximum),

            String.format("%.3f", std),

            String.format("%.3f", memoryUsed),

            "PASS"

        }

    );

  }

  public <I, O> void run(

      Function<I, O> algorithm,

      Function<Integer, I> generator

  ) {

    start();

    for (int i = 0; i < sizes.length; i++) {

      int size = sizes[i];

      List<Double> times = new ArrayList<>();

      double memoryUsed = 0;

      for (int j = 0; j < repeat; j++) {

        I data = generator.apply(size);

        memory.start();

        timer.start();

        algorithm.apply(data);

        timer.stop();

        memoryUsed = memory.stop();

        times.add(

            timer.elapsedMs()

        );

      }

      double avg = Utils.average(times);

      double min = Utils.minimum(times);

      double max = Utils.maximum(times);

      double deviation = Utils.std(times);

      TablePrinter.row(

          i + 1,

          size,

          avg,

          min,

          max,

          deviation,

          memoryUsed,

          "PASS"

      );

      addResult(

          i + 1,

          size,

          avg,

          min,

          max,

          deviation,

          memoryUsed

      );

    }

    TablePrinter.footer();

    CSVManager.save(

        this.algorithm,

        this.language,

        this.results

    );

  }
}