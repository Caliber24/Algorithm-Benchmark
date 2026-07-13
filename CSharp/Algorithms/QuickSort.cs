using System.Collections.Generic;


namespace Algorithms;


public class QuickSort

{


  public static void Run(

      List<int> array

  )

  {

    Sort(

        array,

        0,

        array.Count - 1

    );

  }






  private static void Sort(

      List<int> arr,

      int low,

      int high

  )

  {


    if (low < high)

    {

      int p = Partition(

          arr,

          low,

          high

      );


      Sort(

          arr,

          low,

          p - 1

      );


      Sort(

          arr,

          p + 1,

          high

      );

    }

  }






  private static int Partition(

      List<int> arr,

      int low,

      int high

  )

  {


    int pivot = arr[high];


    int i = low - 1;



    for (int j = low; j < high; j++)

    {

      if (arr[j] <= pivot)

      {

        i++;


        int temp = arr[i];

        arr[i] = arr[j];

        arr[j] = temp;

      }

    }



    int t = arr[i + 1];

    arr[i + 1] = arr[high];

    arr[high] = t;



    return i + 1;

  }

}