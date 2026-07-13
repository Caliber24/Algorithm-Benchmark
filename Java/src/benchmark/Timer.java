package benchmark;


public class Timer {


    private long start;


    private long end;



    public void start(){

        start = System.nanoTime();

    }



    public void stop(){

        end = System.nanoTime();

    }



    public double elapsedMs(){

        return (end - start) / 1_000_000.0;

    }

}