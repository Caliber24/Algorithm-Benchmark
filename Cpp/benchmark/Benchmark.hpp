#pragma once

#include <iostream>
#include <vector>
#include <string>
#include <functional>

#include "MemoryMonitor.hpp"
#include "Timer.hpp"
#include "CSVManager.hpp"
#include "TablePrinter.hpp"
#include "Utils.hpp"

class Benchmark
{

private:
    std::string algorithmName;

    std::string language;

    std::vector<int> sizes;

    int repeat;

    Timer timer;

    MemoryMonitor memory;

    std::vector<
        std::vector<std::string>>
        results;

public:
    Benchmark(

        std::string algorithm,

        std::string lang,

        std::vector<int> inputSizes,

        int repeatCount

    )

    {

        algorithmName = algorithm;

        language = lang;

        sizes = inputSizes;

        repeat = repeatCount;
    }

    void start()

    {

        TablePrinter::title(

            algorithmName,

            language

        );

        TablePrinter::header();
    }

    void addResult(

        int test,

        int inputSize,

        double average,

        double minimum,

        double maximum,

        double deviation,

        double memoryUsed

    )

    {

        results.push_back(

            {

                std::to_string(test),

                std::to_string(inputSize),

                std::to_string(average),

                std::to_string(minimum),

                std::to_string(maximum),

                std::to_string(deviation),

                std::to_string(memoryUsed),

                "PASS"

            }

        );
    }

    template <typename Algorithm, typename Generator>

    void run(

        Algorithm algorithm,

        Generator generator

    )

    {

        start();

        for (

            int i = 0;

            i < sizes.size();

            i++

        )

        {

            int size = sizes[i];

            std::vector<double> times;

            double memoryUsed = 0;

            for (

                int j = 0;

                j < repeat;

                j++

            )

            {

                auto data = generator(size);

                memory.start();

                timer.start();

                algorithm(data);

                timer.stop();

                memoryUsed = memory.stop();

                times.push_back(

                    timer.elapsedMs()

                );
            }

            double avg =

                Utils::average(times);

            double min =

                Utils::minimum(times);

            double max =

                Utils::maximum(times);

            double deviation =

                Utils::stdDeviation(times);

            TablePrinter::row(

                i + 1,

                size,

                avg,

                min,

                max,

                deviation,

                memoryUsed

            );

            addResult(

                i + 1,

                size,

                avg,

                min,

                max,

                deviation,

                memoryUsed

            );
        }

        TablePrinter::footer();

        CSVManager::save(

            algorithmName,

            language,

            results

        );
    }
};