#pragma once

#include <vector>

class Subset
{

private:
  static void generate(

      const std::vector<int> &data,

      int index,

      std::vector<int> &current,

      std::vector<std::vector<int>> &result

  )

  {

    if (index == data.size())

    {

      result.push_back(current);

      return;
    }

    // بدون انتخاب عنصر فعلی

    generate(

        data,

        index + 1,

        current,

        result

    );

    // انتخاب عنصر فعلی

    current.push_back(

        data[index]

    );

    generate(

        data,

        index + 1,

        current,

        result

    );

    current.pop_back();
  }

public:
  static std::vector<std::vector<int>> run(

      std::vector<int> data

  )

  {

    std::vector<std::vector<int>> result;

    std::vector<int> current;

    generate(

        data,

        0,

        current,

        result

    );

    return result;
  }
};