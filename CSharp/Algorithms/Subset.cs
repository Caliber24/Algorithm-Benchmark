using System.Collections.Generic;


namespace Algorithms;


public class Subset

{


  public static void Run(

      List<int> data

  )

  {

    List<List<int>> result =

        new List<List<int>>();



    Generate(

        data,

        0,

        new List<int>(),

        result

    );


  }







  private static void Generate(

      List<int> data,

      int index,

      List<int> current,

      List<List<int>> result

  )

  {


    if (index == data.Count)

    {

      result.Add(

          new List<int>(current)

      );

      return;

    }



    Generate(

        data,

        index + 1,

        current,

        result

    );




    current.Add(

        data[index]

    );



    Generate(

        data,

        index + 1,

        current,

        result

    );



    current.RemoveAt(

        current.Count - 1

    );


  }

}