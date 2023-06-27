use clap::Parser;
use regex::{Captures, Regex};
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

/// Command-line arguments
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

    /// Separator for the columns when using columnar format
    #[arg(short, long, default_value = "\t", name = "STRING")]
    separator: String,

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
macro_rules! unwrap {
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
fn capture_groups_names(regex: &Regex) -> Vec<String> {
    regex
        .capture_names()
        .enumerate()
        .map(|(i, name)| match name {
            Some(name) => name.to_string(),
            None => format!("{}", i), // name the unnamed capture
        })
        .collect()
}

/// Transform the `Regex::Captures` to a string in JSON format
fn captures_to_json(caps: &Captures, idx: usize, names: &[String], line_numbers: bool) -> String {
    let mut fields: HashMap<&str, &str> = caps
        .iter()
        .enumerate()
        .skip(1) // the whole regex
        .map_while(|(i, m)| Some((names[i].as_str(), m?.as_str()))) // skip empty matches
        .collect();
    let idx_str;
    if line_numbers {
        idx_str = idx.to_string();
        fields.insert("line", &idx_str);
    }
    json!(fields).to_string()
}

/// Transform the `Regex::Captures` to a string in columns format
fn captures_to_columns(caps: &Captures, idx: usize, sep: &str, line_numbers: bool) -> String {
    let fields: Vec<&str> = caps
        .iter()
        .skip(1) // the whole regex
        .map(|m| match m {
            Some(m) => m.as_str(),
            None => "", // empty columns for no match
        })
        .collect();
    let columns = fields.join(sep);
    if line_numbers {
        format!("{}{}{}", idx, sep, columns)
    } else {
        columns
    }
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

/// Parse the escape sequences (`\t`, `\n`, `\r`, `\\`) in a string
fn unescape(string: String) -> String {
    let mut chars = string.chars();
    let mut res = String::with_capacity(string.len());
    while let Some(lhs) = chars.next() {
        let ch = match lhs {
            '\\' => match chars.next() {
                Some('t') => '\t',
                Some('n') => '\n',
                Some('r') => '\r',
                Some('\\') => '\\',
                Some(rhs) => {
                    // ignore other escape-like sequences, take them as-is
                    res.push(lhs);
                    rhs
                }
                None => lhs, // '\' as the last character in a string, take it
            },
            _ => lhs,
        };
        res.push(ch);
    }
    res
}

fn main() {
    // Parse the CLI arguments
    let args = Args::parse();
    let sep = unescape(args.separator);
    let regex = unwrap!(Regex::new(&args.regex));

    // Open the input and output
    let reader = unwrap!(new_reader(&args.file));
    let mut writer = unwrap!(new_writer(&args.output));

    if args.json {
        // Output in JSON format

        // if the name "line" is already used for a name of a capturing group, don't use it
        let line_numbers =
            args.line_numbers && !regex.capture_names().any(|name| name == Some("line"));
        let names = capture_groups_names(&regex);

        for (idx, line) in lines(reader) {
            if let Some(ref caps) = regex.captures(&line) {
                let json = captures_to_json(caps, idx, &names, line_numbers);
                unwrap!(writeln!(writer, "{}", json));
            }
        }
    } else {
        // Output in columnar format

        for (idx, line) in lines(reader) {
            if let Some(ref caps) = regex.captures(&line) {
                let columns = captures_to_columns(caps, idx, &sep, args.line_numbers);
                unwrap!(writeln!(writer, "{}", columns));
            }
        }
    }
}
