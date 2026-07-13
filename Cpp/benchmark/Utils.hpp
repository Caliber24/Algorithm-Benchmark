#pragma once

#include <vector>
#include <numeric>
#include <cmath>
#include <algorithm>

class Utils
{

public:
  static double average(
      const std::vector<double> &values)
  {

    double sum = std::accumulate(
        values.begin(),
        values.end(),
        0.0);

    return sum / values.size();
  }

  static double minimum(
      const std::vector<double> &values)
  {

    return *std::min_element(
        values.begin(),
        values.end());
  }

  static double maximum(
      const std::vector<double> &values)
  {

    return *std::max_element(
        values.begin(),
        values.end());
  }

  static double stdDeviation(
      const std::vector<double> &values)
  {

    double avg = average(values);

    double sum = 0;

    for (double value : values)
    {

      sum +=
          pow(value - avg, 2);
    }

    return sqrt(
        sum / values.size());
  }
};