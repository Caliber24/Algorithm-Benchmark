package algorithms;


import benchmark.Benchmark;

import java.util.Random;
import java.util.function.Function;



public class Prime {



    public static boolean isPrime(int number){


        if(number <= 1){

            return false;

        }


        for(int i = 2; i * i <= number; i++){


            if(number % i == 0){

                return false;

            }

        }


        return true;

    }




    public static int[] generatePrimes(int size){


        int[] result = new int[size];


        int count = 0;

        int number = 2;



        while(count < size){


            if(isPrime(number)){


                result[count] = number;

                count++;

            }


            number++;

        }


        return result;

    }






    public static void run(){



        int[] sizes = {

                1000,

                10000,

                100000,

                500000,

                10000000

        };




        Benchmark benchmark = new Benchmark(

                "prime_numbers",

                "java",

                sizes,

                10

        );





        Function<int[], int[]> algorithm = data -> {


            int count = 0;


            for(int value : data){


                if(isPrime(value)){


                    count++;

                }


            }


            return new int[]{count};


        };






        Function<Integer,int[]> generator = size -> {


            Random random = new Random();


            int[] data = new int[size];



            for(int i = 0; i < size; i++){


                data[i] = random.nextInt(100000);


            }



            return data;

        };





        benchmark.run(

                algorithm,

                generator

        );



    }



}