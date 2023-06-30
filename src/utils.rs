/// Unwrap the result or fail with an error and exit with a status code "1"
#[macro_export]
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

/// Parse the escape sequences (`\t`, `\n`, `\r`, `\\`) in a string
pub fn unescape(string: String) -> String {
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

#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(r"", ""; "empty string")]
    #[test_case(r"abcd", "abcd"; "nothing to unescape")]
    #[test_case(r"\t", "\t"; "tab")]
    #[test_case(r"\n", "\n"; "newline")]
    #[test_case(r"\r", "\r"; "carriage return")]
    #[test_case(r"\r\n", "\r\n"; "carriage return with line feed")]
    #[test_case(r"\\", "\\"; "escaped backslash")]
    #[test_case(r"\", "\\"; "single backslash")]
    #[test_case(r"\a", "\\a"; "ignore other special characters")]
    #[test_case(r"\\n", "\\n"; "escaped newline character")]
    #[test_case(r"\\\nx\t\T", "\\\nx\t\\T"; "multiple characters")]
    fn unescape(input: &str, expected: &str) {
        assert_eq!(super::unescape(input.to_string()), expected.to_string());
    }
}
