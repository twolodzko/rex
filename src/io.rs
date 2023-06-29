use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

/// Initialize reader from a file or stdin
pub fn new_reader(file: &Option<String>) -> Result<BufReader<Box<dyn Read>>, io::Error> {
    let input: Box<dyn Read> = match file {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(io::stdin()),
    };
    Ok(BufReader::new(input))
}

/// Initialize writer from a file or stdin
pub fn new_writer(file: &Option<String>) -> Result<Box<dyn Write>, io::Error> {
    Ok(match file {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    })
}

/// Create iterator over lines that enumerates them and prints the read errors to stderr
pub fn lines(reader: BufReader<impl Read>) -> impl Iterator<Item = (usize, String)> {
    reader
        .lines()
        .map_while(|line| match line {
            Ok(line) => Some(line),
            Err(err) => {
                // print errors to stderr as warnings and carry on
                eprintln!("{}", err);
                None
            }
        })
        .enumerate()
        .map(|(i, x)| (i + 1, x)) // line numbering starts at 1
}
