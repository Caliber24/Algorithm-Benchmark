package algorithms;


import benchmark.Benchmark;

import java.util.Random;
import java.util.function.Function;



public class QuickSort {



    public static void quickSort(

            int[] array,

            int low,

            int high

    ){


        if(low < high){


            int pivot = partition(

                    array,

                    low,

                    high

            );



            quickSort(

                    array,

                    low,

                    pivot - 1

            );



            quickSort(

                    array,

                    pivot + 1,

                    high

            );


        }


    }






    private static int partition(

            int[] array,

            int low,

            int high

    ){


        int pivot = array[high];


        int i = low - 1;



        for(int j = low; j < high; j++){



            if(array[j] <= pivot){


                i++;


                int temp = array[i];


                array[i] = array[j];


                array[j] = temp;


            }


        }




        int temp = array[i + 1];


        array[i + 1] = array[high];


        array[high] = temp;



        return i + 1;


    }







    public static int[] sort(int[] data){



        quickSort(

                data,

                0,

                data.length - 1

        );


        return data;


    }








    public static void run(){



        int[] sizes = {


                1000,

                10000,

                100000,

                500000,

                1000000

        };






        Benchmark benchmark = new Benchmark(


                "quick_sort",

                "java",

                sizes,

                10


        );







        Function<int[], int[]> algorithm = data -> {


            return sort(data);


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