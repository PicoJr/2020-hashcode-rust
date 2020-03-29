use crate::solver::{InputDataSet, Library, OutputDataSet};
use std::io::{BufRead, Write};
use std::str::FromStr;

pub fn parse_input_file(input_file: &mut dyn BufRead) -> anyhow::Result<InputDataSet> {
    let mut buff = String::new();
    let _ = input_file.read_line(&mut buff)?;
    let sp = buff.split_whitespace();
    let sp: Vec<&str> = sp.collect();
    let (total_books, total_libraries, total_days) = (match sp.as_slice() {
        [a, b, c] => Ok((
            usize::from_str(a)?,
            usize::from_str(b)?,
            usize::from_str(c)?,
        )),
        _ => Err(anyhow::anyhow!("could not parse {}", buff)),
    })?;
    buff.clear();
    let _ = input_file.read_line(&mut buff)?;
    let sp = buff.split_whitespace();
    let mut book_scores: Vec<usize> = vec![];
    for s in sp {
        book_scores.push(usize::from_str(s)?);
    }
    let mut libraries: Vec<Library> = vec![];
    for id in 0..total_libraries {
        buff.clear();
        let _ = input_file.read_line(&mut buff)?;
        let sp = buff.split_whitespace();
        let sp: Vec<&str> = sp.collect();
        let (n_books, signup_delay, books_per_day) = (match sp.as_slice() {
            [a, b, c] => Ok((
                usize::from_str(a)?,
                usize::from_str(b)?,
                usize::from_str(c)?,
            )),
            _ => Err(anyhow::anyhow!("could not parse {}", buff)),
        })?;
        buff.clear();
        let _ = input_file.read_line(&mut buff)?;
        let sp = buff.split_whitespace();
        let mut books: Vec<usize> = vec![];
        for s in sp {
            books.push(usize::from_str(s)?);
        }
        libraries.push(Library {
            id,
            n_books,
            signup_delay,
            books_per_day,
            books,
        });
    }
    Ok(InputDataSet {
        n_books: total_books,
        n_libraries: total_libraries,
        n_days: total_days,
        book_scores,
        libraries,
    })
}

pub fn write_output_file(
    output_data: &OutputDataSet,
    output_file: &mut dyn Write,
) -> anyhow::Result<()> {
    writeln!(output_file, "{}", output_data.n_libraries)?;
    for lib in &output_data.library_orders {
        writeln!(output_file, "{} {}", lib.id, lib.n_books)?;
        let books: Vec<String> = lib.books.iter().map(|&b| b.to_string()).collect();
        writeln!(output_file, "{}", books.join(" "))?;
    }
    Ok(())
}
