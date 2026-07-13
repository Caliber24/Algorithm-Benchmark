use csv::Writer;

use std::error::Error;

use std::fs;

use std::path::Path;


pub struct CSVManager;


impl CSVManager {

    pub fn save(

        algorithm: &str,

        language: &str,

        results: &Vec<Vec<String>>

    ) -> Result<(), Box<dyn Error>> {

        let directory = Path::new("results/csv");

        if !directory.exists() {

            fs::create_dir_all(directory)?;

        }

        let filename = format!(
            "{}_{}.csv",
            algorithm
                .to_lowercase()
                .replace(" ", "_"),
            language
                .to_lowercase()
        );

        let filepath = directory.join(filename);

        let mut writer = Writer::from_path(filepath)?;

        writer.write_record(&[
            "Test",
            "Input Size",
            "Average(ms)",
            "Minimum(ms)",
            "Maximum(ms)",
            "Std(ms)",
            "Memory(MB)",
            "Status"
        ])?;

        for row in results {

            writer.write_record(row)?;

        }

        writer.flush()?;

        Ok(())

    }

}