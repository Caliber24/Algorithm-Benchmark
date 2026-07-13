using System;
using System.Collections.Generic;


namespace Benchmark;


public class BenchmarkRunner

{


  private string algorithmName;

  private string language;

  private int[] sizes;

  private int repeat;


  private Timer timer;

  private MemoryMonitor memory;


  private List<string[]> results;




  public BenchmarkRunner(

      string algorithm,

      string lang,

      int[] inputSizes,

      int repeatCount

  )

  {

    algorithmName = algorithm;

    language = lang;

    sizes = inputSizes;

    repeat = repeatCount;


    timer = new Timer();

    memory = new MemoryMonitor();


    results = new List<string[]>();

  }







  private void AddResult(

      int test,

      int size,

      double average,

      double minimum,

      double maximum,

      double std,

      double memoryUsed

  )

  {


    results.Add(

        new string[]

        {

                test.ToString(),

                size.ToString(),

                average.ToString("F3"),

                minimum.ToString("F3"),

                maximum.ToString("F3"),

                std.ToString("F3"),

                memoryUsed.ToString("F3"),

                "PASS"

        }

    );


  }







  public void Run<T>(

      Func<int, T> generator,

      Action<T> algorithm

  )

  {


    TablePrinter.Title(

        algorithmName,

        language

    );


    TablePrinter.Header();





    for (int i = 0; i < sizes.Length; i++)

    {


      int size = sizes[i];


      List<double> times =

          new List<double>();



      double memoryUsed = 0;





      for (int j = 0; j < repeat; j++)

      {


        T data =

            generator(size);





        memory.Start();


        timer.Start();



        algorithm(data);



        timer.Stop();



        memoryUsed =

            memory.Stop();





        times.Add(

            timer.ElapsedMs()

        );



      }





      double avg = Average(times);


      double min = Minimum(times);


      double max = Maximum(times);


      double std = Std(times);






      TablePrinter.Row(

          i + 1,

          size,

          avg,

          min,

          max,

          std,

          memoryUsed,

          "PASS"

      );




      AddResult(

          i + 1,

          size,

          avg,

          min,

          max,

          std,

          memoryUsed

      );


    }




    TablePrinter.Footer();




    CSVManager.Save(

        algorithmName,

        language,

        results

    );



  }







  private double Average(

      List<double> values

  )

  {

    double sum = 0;


    foreach (double v in values)

      sum += v;



    return sum / values.Count;

  }







  private double Minimum(

      List<double> values

  )

  {

    double min = values[0];


    foreach (double v in values)

    {

      if (v < min)

        min = v;

    }


    return min;

  }







  private double Maximum(

      List<double> values

  )

  {

    double max = values[0];


    foreach (double v in values)

    {

      if (v > max)

        max = v;

    }


    return max;

  }







  private double Std(

      List<double> values

  )

  {

    double avg = Average(values);


    double sum = 0;


    foreach (double v in values)

    {

      sum += Math.Pow(

          v - avg,

          2

      );

    }



    return Math.Sqrt(

        sum / values.Count

    );

  }


}