using System;
using System.IO;
using System.Collections.Generic;


namespace Benchmark;


public class CSVManager

{


    public static void Save(

        string algorithm,

        string language,

        List<string[]> results

    )

    {


        string directory =

            "Results/csv";



        if(!Directory.Exists(directory))

        {

            Directory.CreateDirectory(directory);

        }




        string filename =

            algorithm.ToLower()

            .Replace(" ", "_")

            +

            "_"

            +

            language.ToLower()

            +

            ".csv";





        string path =

            Path.Combine(

                directory,

                filename

            );





        using(StreamWriter writer = new StreamWriter(path))

        {


            writer.WriteLine(

                "Test,Input Size,Average(ms),Minimum(ms),Maximum(ms),Std,Memory(MB),Status"

            );




            foreach(string[] row in results)

            {


                writer.WriteLine(

                    string.Join(",", row)

                );


            }


        }




        Console.WriteLine(

            $"CSV Saved -> {path}"

        );


    }


}