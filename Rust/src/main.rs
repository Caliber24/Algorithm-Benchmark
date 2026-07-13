mod algorithms;
mod benchmark;
mod chart;

use chart::chart::ChartGenerator;
use std::io;

use algorithms::{fibonacci, matrix, mergesort, prime, quicksort, subset};

fn menu() {
    println!();

    println!("{}", "=".repeat(60));

    println!("{:^60}", "ALGORITHM BENCHMARK RUST");

    println!("{}", "=".repeat(60));

    println!("1. Prime Numbers");

    println!("2. Fibonacci");

    println!("3. Quick Sort");

    println!("4. Merge Sort");

    println!("5. Matrix Multiplication");

    println!("6. Subset Generation");

    println!("7. Run All");

    println!("0. Exit");

    println!("{}", "=".repeat(60));
}

fn main() {
    loop {
        menu();

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                prime::run();

                ChartGenerator::draw("prime_numbers_rust.csv");
            }

            "2" => {
                fibonacci::run();
                ChartGenerator::draw("fibonacci_rust.csv");
            }

            "3" => {
                quicksort::run();
                ChartGenerator::draw("quick_sort_rust.csv");
            }

            "4" => {
                mergesort::run();
                ChartGenerator::draw("merge_sort_rust.csv");
            }

            "5" => {
                matrix::run();
                ChartGenerator::draw("matrix_multiplication_rust.csv");
            }

            "6" => {
                subset::run();
                ChartGenerator::draw("subset_generation_rust.csv");
            }

            "7" => {
                prime::run();

                ChartGenerator::draw("prime_numbers_rust.csv");

                fibonacci::run();

                ChartGenerator::draw("fibonacci_rust.csv");

                quicksort::run();

                ChartGenerator::draw("quick_sort_rust.csv");

                mergesort::run();

                ChartGenerator::draw("merge_sort_rust.csv");

                matrix::run();

                ChartGenerator::draw("matrix_multiplication_rust.csv");

                subset::run();

                ChartGenerator::draw("subset_generation_rust.csv");
            }

            "0" => {
                println!("Good Bye!");

                break;
            }

            _ => {
                println!("Invalid Option");
            }
        }
    }
}
