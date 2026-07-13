namespace Algorithms;


public class Fibonacci

{

  public static long Run(int n)

  {

    if (n <= 1)

      return n;



    long a = 0;

    long b = 1;



    for (int i = 2; i <= n; i++)

    {

      long temp = a + b;

      a = b;

      b = temp;

    }



    return b;

  }

}