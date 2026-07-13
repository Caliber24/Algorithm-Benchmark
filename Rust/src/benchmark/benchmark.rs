use crate::benchmark::csv_manager::CSVManager;
use crate::benchmark::memory::MemoryMonitor;
use crate::benchmark::system_info::SystemInfo;
use crate::benchmark::table::TablePrinter;
use crate::benchmark::timer::Timer;
use crate::benchmark::utils::*;


pub struct Benchmark {

    algorithm: String,

    language: String,

    sizes: Vec<usize>,

    repeat: usize,

    timer: Timer,

    memory: MemoryMonitor,

    results: Vec<Vec<String>>,

}


impl Benchmark {


    pub fn new(

        algorithm: &str,

        language: &str,

        sizes: Vec<usize>,

        repeat: usize,

    ) -> Self {

        Self {

            algorithm: algorithm.to_string(),

            language: language.to_string(),

            sizes,

            repeat,

            timer: Timer::new(),

            memory: MemoryMonitor::new(),

            results: Vec::new(),

        }

    }


    pub fn start(&self) {

        SystemInfo::print_info();

        TablePrinter::title(

            &self.algorithm,

            &self.language,

        );

        TablePrinter::header();

    }


    pub fn add_result(

        &mut self,

        test: usize,

        input_size: usize,

        average: f64,

        minimum: f64,

        maximum: f64,

        std: f64,

        memory: f64,

    ) {


        self.results.push(vec![

            test.to_string(),

            input_size.to_string(),

            format!("{:.3}", average),

            format!("{:.3}", minimum),

            format!("{:.3}", maximum),

            format!("{:.3}", std),

            format!("{:.3}", memory),

            "PASS".to_string(),

        ]);

    }


    pub fn run<F, G, D>(

        &mut self,

        algorithm: F,

        generator: G,

    )

    where

      F: Fn(D),

      G: Fn(usize) -> D,

    {


        self.start();


        for (index, size) in self.sizes.clone().iter().enumerate() {


            let mut times: Vec<f64> = Vec::new();


            let mut memory_used = 0.0;



            for _ in 0..self.repeat {


                let data = generator(*size);



                self.memory.start();



                self.timer.start();



                algorithm(data);



                self.timer.stop();



                memory_used = self.memory.stop();



                times.push(

                    self.timer.elapsed_ms()

                );

            }



            let avg = average(&times);


            let min = minimum(&times);


            let max = maximum(&times);


            let deviation = std(&times);



            TablePrinter::row(

                index + 1,

                *size,

                avg,

                min,

                max,

                deviation,

                memory_used,

                "PASS",

            );



            self.add_result(

                index + 1,

                *size,

                avg,

                min,

                max,

                deviation,

                memory_used,

            );


        }


        TablePrinter::footer();



        if let Err(error) = CSVManager::save(

            &self.algorithm,

            &self.language,

            &self.results,

        ) {


            println!(

                "CSV Error: {}",

                error

            );

        }


    }


}