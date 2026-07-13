#pragma once

#include <windows.h>
#include <psapi.h>

class MemoryMonitor
{

private:
  SIZE_T startMemory;

public:
  void start()

  {

    PROCESS_MEMORY_COUNTERS pmc;

    GetProcessMemoryInfo(

        GetCurrentProcess(),

        &pmc,

        sizeof(pmc)

    );

    startMemory =

        pmc.WorkingSetSize;
  }

  double stop()

  {

    PROCESS_MEMORY_COUNTERS pmc;

    GetProcessMemoryInfo(

        GetCurrentProcess(),

        &pmc,

        sizeof(pmc)

    );

    SIZE_T endMemory =

        pmc.WorkingSetSize;

    double diff =

        static_cast<double>(

            endMemory - startMemory

        );

    return diff / (1024 * 1024);
  }
};