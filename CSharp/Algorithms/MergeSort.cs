using System.Collections.Generic;


namespace Algorithms;


public class MergeSort

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

      int left,

      int right

  )

  {

    if (left < right)

    {

      int mid =

          (left + right) / 2;



      Sort(

          arr,

          left,

          mid

      );


      Sort(

          arr,

          mid + 1,

          right

      );


      Merge(

          arr,

          left,

          mid,

          right

      );

    }

  }







  private static void Merge(

      List<int> arr,

      int left,

      int mid,

      int right

  )

  {


    List<int> temp = new List<int>();


    int i = left;

    int j = mid + 1;



    while (i <= mid && j <= right)

    {

      if (arr[i] <= arr[j])

      {

        temp.Add(arr[i++]);

      }

      else

      {

        temp.Add(arr[j++]);

      }

    }



    while (i <= mid)

      temp.Add(arr[i++]);



    while (j <= right)

      temp.Add(arr[j++]);





    for (int k = 0; k < temp.Count; k++)

      arr[left + k] = temp[k];


  }

}