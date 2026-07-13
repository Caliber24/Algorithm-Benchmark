use crate::benchmark::benchmark::Benchmark;

use rand::Rng;


fn quick_sort(arr: &mut [i32]) {

    if arr.len() <= 1 {

        return;

    }


    let pivot_index = partition(arr);


    let (left, right) = arr.split_at_mut(pivot_index);


    quick_sort(left);

    quick_sort(&mut right[1..]);

}



fn partition(arr: &mut [i32]) -> usize {

    let len = arr.len();

    let pivot = arr[len - 1];


    let mut i = 0;


    for j in 0..len - 1 {

        if arr[j] <= pivot {

            arr.swap(i, j);

            i += 1;

        }

    }


    arr.swap(i, len - 1);


    i

}



fn generate_data(size: usize) -> Vec<i32> {


    let mut rng = rand::rng();


    let mut data = Vec::with_capacity(size);


    for _ in 0..size {

        data.push(

            rng.random_range(1..100000)

        );

    }


    data

}



pub fn run() {


    let sizes = vec![

        1000,

        10000,

        100000,

        500000,

        1000000,

    ];



    let mut benchmark = Benchmark::new(

        "Quick Sort",

        "Rust",

        sizes,

        5,

    );



    benchmark.run(

        |mut data: Vec<i32>| {

            quick_sort(&mut data);

        },

        |size| generate_data(size),

    );

}