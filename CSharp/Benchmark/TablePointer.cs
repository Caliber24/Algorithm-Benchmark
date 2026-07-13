using System;


namespace Benchmark;


public class TablePrinter

{


  public static void Title(

      string algorithm,

      string language

  )

  {

    Console.WriteLine();


    Console.WriteLine(

        new string('=', 70)

    );


    Console.WriteLine(

        $"Algorithm: {algorithm} | Language: {language}"

    );


    Console.WriteLine(

        new string('=', 70)

    );


  }






  public static void Header()

  {

    Console.WriteLine(

        "{0,-10}{1,-15}{2,-15}{3,-15}{4,-15}{5,-12}{6,-12}{7,-10}",

        "Test",

        "Input",

        "Average(ms)",

        "Minimum",

        "Maximum",

        "Std",

        "Memory",

        "Status"

    );



    Console.WriteLine(

        new string('-', 100)

    );

  }








  public static void Row(

      int test,

      int input,

      double average,

      double minimum,

      double maximum,

      double std,

      double memory,

      string status

  )

  {


    Console.WriteLine(

        "{0,-10}{1,-15}{2,-15:F3}{3,-15:F3}{4,-15:F3}{5,-12:F3}{6,-12:F3}{7,-10}",

        test,

        input,

        average,

        minimum,

        maximum,

        std,

        memory,

        status

    );


  }






  public static void Footer()

  {

    Console.WriteLine();


    Console.WriteLine(

        new string('=', 100)

    );


  }


}