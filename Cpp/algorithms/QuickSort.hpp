#pragma once

#include <vector>
#include <algorithm>

class QuickSort
{

private:
  static int partition(

      std::vector<int> &arr,

      int low,

      int high

  )

  {

    int pivot = arr[high];

    int i = low - 1;

    for (

        int j = low;

        j < high;

        j++

    )

    {

      if (arr[j] <= pivot)

      {

        i++;

        std::swap(

            arr[i],

            arr[j]

        );
      }
    }

    std::swap(

        arr[i + 1],

        arr[high]

    );

    return i + 1;
  }

  static void sort(

      std::vector<int> &arr,

      int low,

      int high

  )

  {

    if (low < high)

    {

      int pi = partition(

          arr,

          low,

          high

      );

      sort(

          arr,

          low,

          pi - 1

      );

      sort(

          arr,

          pi + 1,

          high

      );
    }
  }

public:
  static std::vector<int> run(

      std::vector<int> data

  )

  {

    if (!data.empty())

    {

      sort(

          data,

          0,

          data.size() - 1

      );
    }

    return data;
  }
};