use crate::benchmark::benchmark::Benchmark;


fn is_prime(number: usize) -> bool {

    if number < 2 {

        return false;

    }


    for i in 2..=((number as f64).sqrt() as usize) {

        if number % i == 0 {

            return false;

        }

    }


    true

}



fn generate_primes(n: usize) -> Vec<usize> {

    let mut primes: Vec<usize> = Vec::new();

    let mut number = 2;


    while primes.len() < n {

        if is_prime(number) {

            primes.push(number);

        }


        number += 1;

    }


    primes

}



pub fn run() {


    let sizes = vec![

        1000,

        5000,

        10000,

        50000,

        100000,

    ];



    let mut benchmark = Benchmark::new(

        "Prime Numbers",

        "Rust",

        sizes,

        5,

    );



    benchmark.run(

        |_data: Vec<usize>| {},

        |size| generate_primes(size),

    );

}