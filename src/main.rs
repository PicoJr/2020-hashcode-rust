use crate::solver::{solve, InputDataSet, OutputDataSet};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use crate::parser::{parse_input_file, write_output_file};

mod cli;
mod solver;
mod parser;

fn main() -> anyhow::Result<()> {
    let app = cli::get_app();
    let matches = app.get_matches();
    let input_file = matches
        .value_of("input")
        .expect("clap should ensure it's provided");
    let input_file = File::open(input_file)?;
    let input_data_set: InputDataSet = parse_input_file(&mut BufReader::new(input_file))?;
    let output_data_set: OutputDataSet = solve(&input_data_set);
    let output_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("out.txt")?;
    write_output_file(&output_data_set, &mut BufWriter::new(output_file))?;
    println!("libraries: {}", output_data_set.n_libraries);
    Ok(())
}
