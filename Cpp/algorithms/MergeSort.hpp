#pragma once

#include <vector>

class MergeSort
{

private:
  static void merge(

      std::vector<int> &arr,

      int left,

      int mid,

      int right

  )

  {

    int n1 = mid - left + 1;

    int n2 = right - mid;

    std::vector<int> L(n1);

    std::vector<int> R(n2);

    for (int i = 0; i < n1; i++)

    {

      L[i] = arr[left + i];
    }

    for (int i = 0; i < n2; i++)

    {

      R[i] = arr[mid + 1 + i];
    }

    int i = 0;

    int j = 0;

    int k = left;

    while (

        i < n1 && j < n2

    )

    {

      if (L[i] <= R[j])

      {

        arr[k] = L[i];

        i++;
      }

      else

      {

        arr[k] = R[j];

        j++;
      }

      k++;
    }

    while (i < n1)

    {

      arr[k] = L[i];

      i++;

      k++;
    }

    while (j < n2)

    {

      arr[k] = R[j];

      j++;

      k++;
    }
  }

  static void sort(

      std::vector<int> &arr,

      int left,

      int right

  )

  {

    if (left < right)

    {

      int mid =

          left + (right - left) / 2;

      sort(

          arr,

          left,

          mid

      );

      sort(

          arr,

          mid + 1,

          right

      );

      merge(

          arr,

          left,

          mid,

          right

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