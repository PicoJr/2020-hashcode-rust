use crate::solver::{solve, InputDataSet, OutputDataSet};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use crate::parser::{parse_input_file, write_output_file};
use std::time::Instant;

mod cli;
mod solver;
mod parser;

fn main() -> anyhow::Result<()> {
    let app = cli::get_app();
    let matches = app.get_matches();
    let input_files = matches.values_of("input").expect("clap should ensure it is provided");
    for input_file_path in input_files {
        let input_file = File::open(input_file_path)?;
        let input_data_set: InputDataSet = parse_input_file(&mut BufReader::new(input_file))?;
        let now = Instant::now();
        let output_data_set: OutputDataSet = solve(&input_data_set);
        let output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("{}.out", input_file_path))?;
        write_output_file(&output_data_set, &mut BufWriter::new(output_file))?;
        let ms = now.elapsed().as_millis();
        println!("{} done in {}s {}ms", input_file_path, ms / 1000, ms % 1000);
    }
    Ok(())
}
