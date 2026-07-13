import algorithms.Prime;
import algorithms.Fibonacci;
import algorithms.QuickSort;
import algorithms.MergeSort;
import algorithms.Matrix;
import algorithms.Subset;

import java.util.Scanner;


public class Main {


    private static void createChart(String filename) {


        try {


            ProcessBuilder builder = new ProcessBuilder(

                    "python",

                    "generate_chart.py",

                    filename

            );


            builder.inheritIO();


            Process process = builder.start();


            process.waitFor();



        } catch (Exception e) {


            System.out.println(

                    "Chart Error: " + e.getMessage()

            );


        }


    }




    public static void menu(){


        System.out.println();

        System.out.println("==============================");

        System.out.println("ALGORITHM BENCHMARK JAVA");

        System.out.println("==============================");


        System.out.println("1. Prime Numbers");

        System.out.println("2. Fibonacci");

        System.out.println("3. Quick Sort");

        System.out.println("4. Merge Sort");

        System.out.println("5. Matrix Multiplication");

        System.out.println("6. Subset Generation");

        System.out.println("7. Run All");

        System.out.println("0. Exit");


        System.out.println("==============================");


    }







    public static void main(String[] args){



        Scanner scanner = new Scanner(System.in);




        while(true){



            menu();



            System.out.print(

                    "Select Option : "

            );



            int choice = scanner.nextInt();




            switch(choice){



                case 1:


                    Prime.run();


                    createChart(

                            "prime_numbers_java.csv"

                    );


                    break;





                case 2:


                    Fibonacci.run();


                    createChart(

                            "fibonacci_java.csv"

                    );


                    break;





                case 3:


                    QuickSort.run();


                    createChart(

                            "quick_sort_java.csv"

                    );


                    break;





                case 4:


                    MergeSort.run();


                    createChart(

                            "merge_sort_java.csv"

                    );


                    break;





                case 5:


                    Matrix.run();


                    createChart(

                            "matrix_multiplication_java.csv"

                    );


                    break;





                case 6:


                    Subset.run();


                    createChart(

                            "subset_generation_java.csv"

                    );


                    break;





                case 7:



                    Prime.run();


                    Fibonacci.run();


                    QuickSort.run();


                    MergeSort.run();


                    Matrix.run();


                    Subset.run();




                    createChart(

                            "prime_numbers_java.csv"

                    );


                    createChart(

                            "fibonacci_java.csv"

                    );


                    createChart(

                            "quick_sort_java.csv"

                    );


                    createChart(

                            "merge_sort_java.csv"

                    );


                    createChart(

                            "matrix_multiplication_java.csv"

                    );


                    createChart(

                            "subset_generation_java.csv"

                    );



                    break;





                case 0:



                    System.out.println(

                            "Good Bye"

                    );


                    scanner.close();


                    return;





                default:



                    System.out.println(

                            "Invalid Option"

                    );


            }



        }


    }


}