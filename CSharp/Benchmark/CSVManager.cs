using System;


namespace Benchmark;


public class MemoryMonitor

{

    private long startMemory;



    public void Start()

    {

        GC.Collect();

        GC.WaitForPendingFinalizers();


        startMemory =

            GC.GetTotalMemory(true);

    }





    public double Stop()

    {

        long endMemory =

            GC.GetTotalMemory(true);



        long difference =

            endMemory - startMemory;



        return difference /

            (1024.0 * 1024.0);

    }


}