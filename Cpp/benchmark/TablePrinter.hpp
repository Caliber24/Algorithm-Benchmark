#pragma once

#include <iostream>
#include <iomanip>

class TablePrinter
{

public:
  static void title(
      std::string algorithm,
      std::string language)
  {

    std::cout
        << "\n====================================\n";

    std::cout
        << "Algorithm: "
        << algorithm
        << " | Language: "
        << language
        << "\n";

    std::cout
        << "====================================\n";
  }

  static void header()
  {

    std::cout
        << std::left

        << std::setw(10)
        << "Test"

        << std::setw(15)
        << "Input"

        << std::setw(15)
        << "Average"

        << std::setw(15)
        << "Min"

        << std::setw(15)
        << "Max"

        << std::setw(15)
        << "Std"

        << std::setw(15)
        << "Memory"

        << "Status\n";
  }

  static void row(
      int test,
      int input,
      double avg,
      double min,
      double max,
      double std,
      double memory)
  {

    std::cout

        << std::setw(10)
        << test

        << std::setw(15)
        << input

        << std::setw(15)
        << avg

        << std::setw(15)
        << min

        << std::setw(15)
        << max

        << std::setw(15)
        << std

        << std::setw(15)
        << memory

        << "PASS\n";
  }

  static void footer()
  {

    std::cout
        << "====================================\n";
  }
};