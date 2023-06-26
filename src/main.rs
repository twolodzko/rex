use clap::Parser;
use regex::{Captures, Regex};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

/// Command-line arguments `rex <REGEX> [-l | -j | -o <FILE>] [FILE]`
#[derive(Parser, Debug)]
struct Args {
    /// Regular expression for a pattern to extract
    regex: String,

    /// Show line numbers
    #[arg(short, long, default_value_t = false)]
    line_numbers: bool,

    /// Use JSON format instead of columns
    #[arg(short, long, default_value_t = false)]
    json: bool,

    /// If given, write the result to a file
    #[arg(short, long, default_value = None, name = "FILE")]
    output: Option<String>,

    /// Input data file, if not given, the input is read from stdin
    file: Option<String>,
}

/// Initialize reader from a file or stdin
fn new_reader(file: &Option<String>) -> Result<BufReader<Box<dyn Read>>, io::Error> {
    let input: Box<dyn Read> = match file {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(io::stdin()),
    };
    Ok(BufReader::new(input))
}

/// Initialize writer from a file or stdin
fn new_writer(file: &Option<String>) -> Result<Box<dyn Write>, io::Error> {
    Ok(match file {
        Some(path) => Box::new(File::create(path)?),
        None => Box::new(io::stdout()),
    })
}

/// Unwrap the result or fail with an error
macro_rules! unwrap_or_fail {
    ( $result:expr ) => {
        match $result {
            Ok(val) => val,
            Err(err) => {
                // print error message to stderr and exit
                eprintln!("{}", err);
                std::process::exit(1);
            }
        }
    };
}

/// Extract capture names and name the unnamed captures
fn capture_names(regex: &Regex) -> Vec<String> {
    regex
        .capture_names()
        .enumerate()
        .map(|(i, name)| match name {
            Some(name) => name.to_string(),
            None => format!("{}", i), // name the unnamed capture
        })
        .collect()
}

/// Convert the `Captures` to a map (name => captured string)
fn captures_to_map<'a, 'b>(caps: &Captures<'a>, names: &'b [String]) -> HashMap<&'b str, &'a str> {
    caps.iter()
        .enumerate()
        .skip(1) // the whole regex
        .map_while(|(i, m)| Some((names[i].as_str(), m?.as_str()))) // skip empty matches
        .collect()
}

/// Convert the `Captures` to a vec of the extracted strings
fn captures_to_vec<'a>(caps: &Captures<'a>) -> Vec<&'a str> {
    caps.iter()
        .skip(1) // the whole regex
        .map(|m| match m {
            Some(m) => m.as_str(),
            None => "", // empty columns for no match
        })
        .collect()
}

/// Create iterator over lines that enumerates them and prints the read errors to stderr
fn lines(reader: BufReader<impl Read>) -> impl Iterator<Item = (usize, String)> {
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

fn main() {
    // Parse the CLI arguments
    let args = Args::parse();

    // Open the input and output
    let reader = unwrap_or_fail!(new_reader(&args.file));
    let mut writer = unwrap_or_fail!(new_writer(&args.output));

    // Parse the regex
    let regex = unwrap_or_fail!(Regex::new(&args.regex));

    if args.json {
        // Output in JSON format

        // if the name "line" is already used for a name of a capturing group, don't use it
        let line_numbers =
            args.line_numbers && !regex.capture_names().any(|name| name == Some("line"));
        let names = capture_names(&regex);

        for (idx, line) in lines(reader) {
            if let Some(ref caps) = regex.captures(&line) {
                let mut fields = captures_to_map(caps, &names);
                let idx_str;
                if line_numbers {
                    idx_str = idx.to_string();
                    fields.insert("line", &idx_str);
                }
                unwrap_or_fail!(writeln!(writer, "{}", json!(fields)));
            }
        }
    } else {
        // Output in columnar format
        let sep = "\t";

        for (idx, line) in lines(reader) {
            if let Some(ref caps) = regex.captures(&line) {
                let fields = captures_to_vec(caps);
                if args.line_numbers {
                    unwrap_or_fail!(write!(writer, "{}{}", idx, sep));
                }
                unwrap_or_fail!(writeln!(writer, "{}", fields.join(sep)));
            }
        }
    }
}
