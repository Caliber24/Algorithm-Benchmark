package benchmark;


import java.util.List;


public class Utils {



    public static double average(
            List<Double> values
    ){

        double sum = 0;


        for(double value : values){

            sum += value;

        }


        return sum / values.size();

    }




    public static double minimum(
            List<Double> values
    ){

        double min = values.get(0);


        for(double value : values){

            if(value < min){

                min = value;

            }

        }


        return min;

    }




    public static double maximum(
            List<Double> values
    ){

        double max = values.get(0);


        for(double value : values){

            if(value > max){

                max = value;

            }

        }


        return max;

    }




    public static double std(
            List<Double> values
    ){


        double avg = average(values);


        double sum = 0;



        for(double value : values){


            sum += Math.pow(

                    value - avg,

                    2

            );


        }



        return Math.sqrt(

                sum / values.size()

        );


    }


}