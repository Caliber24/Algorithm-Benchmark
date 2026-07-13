use crate::benchmark::benchmark::Benchmark;

use rand::Rng;



type Matrix = Vec<Vec<i32>>;



fn multiply(

    data: (Matrix, Matrix)

) -> Matrix {


    let (a, b) = data;


    let n = a.len();


    let mut c = vec![

        vec![0; n];

        n

    ];



    for i in 0..n {


        for j in 0..n {


            for k in 0..n {


                c[i][j] += a[i][k] * b[k][j];


            }

        }

    }



    c

}



fn generate_matrix(size: usize) -> Matrix {


    let mut rng = rand::rng();



    let mut matrix = vec![

        vec![0; size];

        size

    ];



    for i in 0..size {


        for j in 0..size {


            matrix[i][j] = rng.random_range(1..10);


        }

    }



    matrix

}



fn generate_data(size: usize) -> (Matrix, Matrix) {


    let a = generate_matrix(size);


    let b = generate_matrix(size);


    (a, b)

}



pub fn run() {


    let sizes = vec![

        50,

        100,

        200,

        300,


        500,

    ];



    let mut benchmark = Benchmark::new(

        "Matrix Multiplication",

        "Rust",

        sizes,

        3,

    );



    benchmark.run(

        |data: (Matrix, Matrix)| {


            multiply(data);


        },

        |size| generate_data(size),

    );



}