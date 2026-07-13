#pragma once

#include <vector>

class Prime
{

public:
  static std::vector<int> run(int limit)

  {

    std::vector<int> primes;

    for (int number = 2; number <= limit; number++)

    {

      bool isPrime = true;

      for (
          int i = 2;
          i * i <= number;
          i++)

      {

        if (number % i == 0)

        {

          isPrime = false;

          break;
        }
      }

      if (isPrime)

      {

        primes.push_back(number);
      }
    }

    return primes;
  }
};