using System;
using System.Collections.Generic;
using System.Diagnostics;

using Algorithms;
using Benchmark;



class Program

{


  static void CreateChart(string filename)

  {

    try

    {

      Process process = new Process();


      process.StartInfo.FileName = "python";


      process.StartInfo.Arguments =

          $"../generate_chart.py {filename}";


      process.StartInfo.UseShellExecute = false;


      process.Start();


      process.WaitForExit();


    }

    catch (Exception e)

    {

      Console.WriteLine(

          "Chart Error: " + e.Message

      );

    }

  }








  static List<int> GenerateArray(int size)

  {

    Random random = new Random();


    List<int> data = new List<int>();


    for (int i = 0; i < size; i++)

    {

      data.Add(

          random.Next(1, 10000)

      );

    }


    return data;

  }








  static int[,] GenerateMatrix(int size)

  {

    int[,] matrix =

        new int[size, size];



    for (int i = 0; i < size; i++)

    {

      for (int j = 0; j < size; j++)

      {

        matrix[i, j] = i + j;

      }

    }



    return matrix;

  }









  static void RunPrime()

  {


    BenchmarkRunner benchmark =

        new BenchmarkRunner(

            "prime_numbers",

            "csharp",

            new int[]

            {

                    1000,

                    10000,

                    100000,

                    500000,

                    10000000

            },

            10

        );




    benchmark.Run<int>(

        size => size,


        n => Prime.Run(n)

    );



    CreateChart(

        "prime_numbers_csharp.csv"

    );

  }









  static void RunFibonacci()

  {


    BenchmarkRunner benchmark =

        new BenchmarkRunner(

            "fibonacci",

            "csharp",

            new int[]

            {

                    1000000,

                    2000000,

                    50000000,

                    6000000,
                    100000000

            },

            10

        );



    benchmark.Run<int>(

        size => size,


        n => Fibonacci.Run(n)

    );



    CreateChart(

        "fibonacci_csharp.csv"

    );

  }









  static void RunQuickSort()

  {


    BenchmarkRunner benchmark =

        new BenchmarkRunner(

            "quick_sort",

            "csharp",

            new int[]

            {

                    1000,

                    10000,

                    100000,
                    500000,
                    1000000

            },

            10

        );




    benchmark.Run<List<int>>(

        size => GenerateArray(size),


        data => QuickSort.Run(data)

    );




    CreateChart(

        "quick_sort_csharp.csv"

    );


  }









  static void RunMergeSort()

  {


    BenchmarkRunner benchmark =

        new BenchmarkRunner(

            "merge_sort",

            "csharp",

            new int[]

            {

                    1000,

                    10000,

                    100000,
                    500000,
                    1000000

            },

            10

        );



    benchmark.Run<List<int>>(

        size => GenerateArray(size),


        data => MergeSort.Run(data)

    );



    CreateChart(

        "merge_sort_csharp.csv"

    );


  }









  static void RunMatrix()

  {


    BenchmarkRunner benchmark =

        new BenchmarkRunner(

            "matrix_multiplication",

            "csharp",

            new int[]

            {

                    50,

                    100,

                    200,

                    300,
                    500

            },

            10

        );




    benchmark.Run<int[,]>(

        size => GenerateMatrix(size),


        matrix => Matrix.Run(

            matrix,

            matrix

        )

    );




    CreateChart(

        "matrix_multiplication_csharp.csv"

    );


  }









  static void RunSubset()

  {


    BenchmarkRunner benchmark =

        new BenchmarkRunner(

            "subset_generation",

            "csharp",

            new int[]

            {
                    5,
                    10,
                    15,
                    20,
                    22

            },

            10

        );




    benchmark.Run<List<int>>(

        size => GenerateArray(size),


        data => Subset.Run(data)

    );



    CreateChart(

        "subset_generation_csharp.csv"

    );


  }









  static void Main()

  {


    while (true)

    {


      Console.WriteLine();


      Console.WriteLine("==============================");

      Console.WriteLine("ALGORITHM BENCHMARK C#");

      Console.WriteLine("==============================");


      Console.WriteLine("1. Prime Numbers");

      Console.WriteLine("2. Fibonacci");

      Console.WriteLine("3. Quick Sort");

      Console.WriteLine("4. Merge Sort");

      Console.WriteLine("5. Matrix Multiplication");

      Console.WriteLine("6. Subset Generation");

      Console.WriteLine("7. Run All");

      Console.WriteLine("0. Exit");


      Console.WriteLine("==============================");


      Console.Write("Select Option : ");



      string? input = Console.ReadLine();


      if (!int.TryParse(input, out int choice))
      {
        choice = -1;
      }





      switch (choice)

      {


        case 1:

          RunPrime();

          break;



        case 2:

          RunFibonacci();

          break;



        case 3:

          RunQuickSort();

          break;



        case 4:

          RunMergeSort();

          break;



        case 5:

          RunMatrix();

          break;



        case 6:

          RunSubset();

          break;



        case 7:

          RunPrime();

          RunFibonacci();

          RunQuickSort();

          RunMergeSort();

          RunMatrix();

          RunSubset();

          break;



        case 0:

          Console.WriteLine(

              "Good Bye"

          );

          return;



        default:

          Console.WriteLine(

              "Invalid Option"

          );

          break;

      }

    }


  }


}


