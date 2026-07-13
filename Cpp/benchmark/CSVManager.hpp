#pragma once

#include <fstream>
#include <iostream>
#include <filesystem>
#include <vector>
#include <string>

class CSVManager
{

public:
  static void save(

      const std::string &algorithm,

      const std::string &language,

      const std::vector<std::vector<std::string>> &results

  )
  {

    std::filesystem::create_directories(
        "results/csv");

    std::string filename =
        "results/csv/" + algorithm + "_" + language + ".csv";

    std::ofstream file(filename);

    file
        << "Test,Input Size,Average(ms),Minimum(ms),Maximum(ms),Std,Memory(MB),Status\n";

    for (auto &row : results)
    {

      for (size_t i = 0; i < row.size(); i++)
      {

        file << row[i];

        if (i < row.size() - 1)
          file << ",";
      }

      file << "\n";
    }

    file.close();

    std::cout
        << "CSV Saved -> "
        << filename
        << std::endl;
  }
};