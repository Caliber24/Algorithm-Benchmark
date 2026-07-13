using System.Diagnostics;


namespace Benchmark;


public class Timer

{

  private Stopwatch stopwatch;



  public Timer()

  {

    stopwatch = new Stopwatch();

  }



  public void Start()

  {

    stopwatch.Restart();

  }



  public void Stop()

  {

    stopwatch.Stop();

  }



  public double ElapsedMs()

  {

    return stopwatch.Elapsed.TotalMilliseconds;

  }

}