package benchmark;


public class SystemInfo {



    public static void printInfo(){


        System.out.println();


        System.out.println(
                "System Information"
        );


        System.out.println(
                "-".repeat(40)
        );



        System.out.println(

                "OS: "
                +
                System.getProperty("os.name")

        );



        System.out.println(

                "OS Version: "
                +
                System.getProperty("os.version")

        );



        System.out.println(

                "Architecture: "
                +
                System.getProperty("os.arch")

        );



        System.out.println(

                "Java Version: "
                +
                System.getProperty("java.version")

        );



        System.out.println(

                "CPU Cores: "
                +
                Runtime.getRuntime()
                .availableProcessors()

        );



        System.out.println(
                "-".repeat(40)
        );


    }


}