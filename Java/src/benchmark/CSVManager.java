package benchmark;


import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.util.List;



public class CSVManager {



    public static void save(

            String algorithm,

            String language,

            List<String[]> results

    ){



        try {


            File directory = new File(
                    "results/csv"
            );


            if(!directory.exists()){

                directory.mkdirs();

            }



            String filename =

                    algorithm
                    .toLowerCase()
                    .replace(" ","_")
                    +
                    "_"
                    +
                    language
                    .toLowerCase()
                    +
                    ".csv";



            FileWriter writer = new FileWriter(

                    "Java/results/csv/" + filename

            );



            writer.append(

                    "Test,Input Size,Average(ms),Minimum(ms),Maximum(ms),Std,Memory(MB),Status\n"

            );



            for(String[] row : results){


                writer.append(

                        String.join(",",row)

                );


                writer.append("\n");


            }



            writer.close();



            System.out.println(

                    "CSV Saved -> results/csv/" + filename

            );



        }

        catch(IOException e){


            System.out.println(

                    "CSV Error: " + e.getMessage()

            );


        }


    }


}