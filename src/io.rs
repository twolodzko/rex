use std::fs::File;
use std::io::{self, BufReader, Read, Write};

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
