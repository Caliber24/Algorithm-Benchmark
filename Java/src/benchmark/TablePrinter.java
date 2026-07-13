package benchmark;


public class TablePrinter {



    public static void title(

            String algorithm,

            String language

    ){


        System.out.println();

        System.out.println(
                "=".repeat(70)
        );


        System.out.println(

                "Algorithm: "
                + algorithm
                +
                " | Language: "
                + language

        );


        System.out.println(

                "=".repeat(70)

        );


    }




    public static void header(){


        System.out.printf(

                "%-8s %-12s %-15s %-15s %-15s %-12s %-12s%n",

                "Test",

                "Input",

                "Average(ms)",

                "Minimum",

                "Maximum",

                "Std",

                "Memory"

        );


        System.out.println(

                "-".repeat(90)

        );


    }




    public static void row(

            int test,

            int input,

            double avg,

            double min,

            double max,

            double std,

            double memory,

            String status

    ){


        System.out.printf(

                "%-8d %-12d %-15.3f %-15.3f %-15.3f %-12.3f %-12.3f %-8s%n",

                test,

                input,

                avg,

                min,

                max,

                std,

                memory,

                status

        );


    }




    public static void footer(){


        System.out.println(

                "=".repeat(90)

        );


    }


}