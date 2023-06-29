mod io;
mod serializers;
mod utils;

use clap::Parser;
use io::{lines, new_reader, new_writer};
use regex::Regex;
use serializers::{ColumnsSerializer, JsonSerializer, Serializer};
use utils::{capture_groups_names, unescape};

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

    /// Separator for the columns when using columnar format
    #[arg(short, long, default_value = "\t", name = "STRING")]
    separator: String,

    /// Input data file, if not given, the input is read from stdin
    file: Option<String>,
}

fn main() {
    // Parse the CLI arguments
    let args = Args::parse();
    let regex = unwrap!(Regex::new(&args.regex));
    let has_groups = regex.captures_len() > 1;

    // Open the input and output
    let reader = unwrap!(new_reader(&args.file));
    let mut writer = unwrap!(new_writer(&args.output));

    let serializer: Box<dyn Serializer> = if args.json {
        // if the name "line" is already used for a name of a capturing group, don't use it
        let line_numbers =
            args.line_numbers && !regex.capture_names().any(|name| name == Some("line"));
        let names = capture_groups_names(&regex);

        Box::new(JsonSerializer::new(has_groups, line_numbers, names))
    } else {
        Box::new(ColumnsSerializer::new(
            has_groups,
            args.line_numbers,
            unescape(args.separator),
        ))
    };

    for (idx, line) in lines(reader) {
        if let Some(ref caps) = regex.captures(&line) {
            let matched = serializer.to_string(caps, idx);
            unwrap!(writeln!(writer, "{}", matched));
        }
    }
}
