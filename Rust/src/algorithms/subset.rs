use crate::benchmark::benchmark::Benchmark;



fn generate_subsets(

    data: &[i32],

) -> Vec<Vec<i32>> {


    let mut result: Vec<Vec<i32>> = Vec::new();


    let mut current: Vec<i32> = Vec::new();


    backtrack(

        data,

        0,

        &mut current,

        &mut result,

    );


    result

}



fn backtrack(

    data: &[i32],

    index: usize,

    current: &mut Vec<i32>,

    result: &mut Vec<Vec<i32>>,

) {


    if index == data.len() {


        result.push(

            current.clone()

        );


        return;

    }



    backtrack(

        data,

        index + 1,

        current,

        result,

    );



    current.push(

        data[index]

    );



    backtrack(

        data,

        index + 1,

        current,

        result,

    );



    current.pop();

}



fn generate_data(size: usize) -> Vec<i32> {


    (0..size)

        .map(|x| x as i32)

        .collect()

}



pub fn run() {


    let sizes = vec![

        5,

        10,

        15,

        20,

        22,

    ];



    let mut benchmark = Benchmark::new(

        "Subset Generation",

        "Rust",

        sizes,

        3,

    );



    benchmark.run(

        |data: Vec<i32>| {


            generate_subsets(&data);


        },

        |size| generate_data(size),

    );



}