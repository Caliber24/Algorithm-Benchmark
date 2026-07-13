use crate::benchmark::benchmark::Benchmark;

use rand::Rng;


fn merge_sort(arr: &mut [i32]) {

    let len = arr.len();


    if len <= 1 {

        return;

    }


    let mid = len / 2;


    merge_sort(&mut arr[..mid]);

    merge_sort(&mut arr[mid..]);


    let mut merged = arr.to_vec();


    merge(

        &arr[..mid],

        &arr[mid..],

        &mut merged[..],

    );


    arr.copy_from_slice(&merged);

}



fn merge(

    left: &[i32],

    right: &[i32],

    result: &mut [i32],

) {


    let mut i = 0;

    let mut j = 0;

    let mut k = 0;



    while i < left.len() && j < right.len() {


        if left[i] <= right[j] {

            result[k] = left[i];

            i += 1;

        }

        else {

            result[k] = right[j];

            j += 1;

        }


        k += 1;

    }



    while i < left.len() {


        result[k] = left[i];

        i += 1;

        k += 1;

    }



    while j < right.len() {


        result[k] = right[j];

        j += 1;

        k += 1;

    }

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

        "Merge Sort",

        "Rust",

        sizes,

        5,

    );



    benchmark.run(

        |mut data: Vec<i32>| {


            merge_sort(&mut data);


        },

        |size| generate_data(size),

    );

}