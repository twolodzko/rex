mod io;
mod serializers;
mod utils;

use clap::Parser;
use io::{new_reader, new_writer, ok_or_warning};
use regex::Regex;
use serializers::{ColumnsSerializer, JsonSerializer, Serializer};
use std::io::BufRead;
use utils::unescape;

/// Command-line arguments
#[derive(Parser, Debug)]
struct Args {
    /// Regular expression for a pattern to extract
    #[arg(value_parser = clap::builder::NonEmptyStringValueParser::new())]
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

    /// Separator for the columns when using columnar format (it recognizes whitespace characters: \t, \n, \r)
    #[arg(short, long, default_value = "\t", name = "STRING")]
    separator: String,

    /// Print additional information to stderr
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Input data file, if not given, the input is read from stdin
    file: Option<String>,
}

fn main() {
    // Parse the CLI arguments
    let args = Args::parse();
    let regex = unwrap!(Regex::new(&args.regex));

    // Open the input and output
    let reader = unwrap!(new_reader(&args.file));
    let mut writer = unwrap!(new_writer(&args.output));

    // Pick and initialize a serializer
    let serializer: Box<dyn Serializer> = if args.json {
        // if the name "line" is already used for a name of a capturing group, don't use it
        let line_numbers =
            args.line_numbers && !regex.capture_names().any(|name| name == Some("line"));

        Box::new(JsonSerializer::new(&regex, line_numbers))
    } else {
        Box::new(ColumnsSerializer::new(
            &regex,
            args.line_numbers,
            unescape(args.separator),
        ))
    };

    // Do the work
    reader
        .lines()
        .enumerate()
        .map_while(ok_or_warning) // skip lines on read errors
        .for_each(|(idx, ref line)| {
            let idx = idx + 1;
            // process the line
            if let Some(ref caps) = regex.captures(line) {
                let matched = serializer.to_string(caps, idx);
                unwrap!(writeln!(writer, "{}", matched));
            } else if args.verbose {
                warning!(idx, "no match");
            }
        });
}
