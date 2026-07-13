#pragma once

#include <vector>

class Fibonacci
{

public:
  static std::vector<long long> run(int n)

  {

    std::vector<long long> sequence;

    if (n <= 0)

    {

      return sequence;
    }

    sequence.push_back(0);

    if (n == 1)

    {

      return sequence;
    }

    sequence.push_back(1);

    for (

        int i = 2;

        i < n;

        i++

    )

    {

      sequence.push_back(

          sequence[i - 1] + sequence[i - 2]

      );
    }

    return sequence;
  }
};