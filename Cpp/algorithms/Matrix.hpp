#pragma once

#include <vector>

class Matrix
{

public:
  static std::vector<std::vector<int>> run(

      std::vector<std::vector<int>> A,

      std::vector<std::vector<int>> B

  )

  {

    int n = A.size();

    int m = B[0].size();

    int p = B.size();

    std::vector<std::vector<int>> result(

        n,

        std::vector<int>(m, 0)

    );

    for (int i = 0; i < n; i++)

    {

      for (int j = 0; j < m; j++)

      {

        for (int k = 0; k < p; k++)

        {

          result[i][j] +=

              A[i][k] * B[k][j];
        }
      }
    }

    return result;
  }
};