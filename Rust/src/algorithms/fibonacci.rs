use crate::benchmark::benchmark::Benchmark;

fn fibonacci(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    let mut a: u64 = 0;

    let mut b: u64 = 1;

    for _ in 2..=n {
        let temp = a + b;

        a = b;

        b = temp;
    }

    b
}

fn generate_data(size: usize) -> usize {
    size
}

pub fn run() {
    let sizes = vec![10, 20, 50, 60, 90];

    let mut benchmark = Benchmark::new("Fibonacci", "Rust", sizes, 5);

    benchmark.run(
        |n: usize| {
            fibonacci(n);
        },
        |size| generate_data(size),
    );
}
