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

/// Convert from `Result<String, Error>` to `Option<String>` and print the `Error` to stderr
#[inline]
pub fn ok_or_warn(line: Result<String, std::io::Error>) -> Option<String> {
    match line {
        Ok(line) => Some(line),
        Err(err) => {
            // print errors to stderr as warnings and carry on
            eprintln!("{}", err);
            None
        }
    }
}
