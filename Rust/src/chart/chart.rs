use plotters::prelude::*;
use csv::Reader;
use std::fs;


pub struct ChartGenerator;


impl ChartGenerator {


    pub fn draw(filename: &str) {


        fs::create_dir_all(
            "results/charts"
        )
        .unwrap();



        let csv_path = format!(
            "results/csv/{}",
            filename
        );



        let output = format!(
            "results/charts/{}.png",
            filename.replace(".csv", "")
        );



        let mut reader = Reader::from_path(
            csv_path
        )
        .unwrap();



        let mut data: Vec<(i32,f64)> = Vec::new();



        for result in reader.records() {


            let record = result.unwrap();


            let input_size: i32 = record[1]

                .parse()

                .unwrap();



            let time: f64 = record[2]

                .parse()

                .unwrap();



            data.push(

                (input_size,time)

            );

        }



        let root = BitMapBackend::new(

            &output,

            (1000,600)

        )
        .into_drawing_area();



        root.fill(&WHITE)
            .unwrap();



        let max_x = data

            .iter()

            .map(|x| x.0)

            .max()

            .unwrap();



        let max_y = data

            .iter()

            .map(|x| x.1)

            .fold(0.0, f64::max);



        let mut chart = ChartBuilder::on(
            &root
        )

        .caption(

            filename,

            ("sans-serif",30)

        )

        .margin(20)

        .x_label_area_size(40)

        .y_label_area_size(50)

        .build_cartesian_2d(

            0..max_x,

            0.0..max_y

        )

        .unwrap();



        chart.configure_mesh()

            .draw()

            .unwrap();



        chart.draw_series(

            LineSeries::new(

                data,

                &RED

            )

        )

        .unwrap();



        root.present()

            .unwrap();



        println!(

            "Chart saved -> {}",

            output

        );


    }


}