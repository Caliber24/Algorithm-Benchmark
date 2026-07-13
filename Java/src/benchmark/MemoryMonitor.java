package benchmark;


public class MemoryMonitor {


    private Runtime runtime;


    private long startMemory;



    public MemoryMonitor(){

        runtime = Runtime.getRuntime();

    }



    public void start(){


        System.gc();


        startMemory = runtime.totalMemory()

                - runtime.freeMemory();

    }




    public double stop(){


        long endMemory = runtime.totalMemory()

                - runtime.freeMemory();



        long used = endMemory - startMemory;



        return used / (1024.0 * 1024.0);

    }


}