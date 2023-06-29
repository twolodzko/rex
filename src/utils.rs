use regex::Regex;

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

/// Extract capture names and name the unnamed captures
pub fn capture_groups_names(regex: &Regex) -> Vec<String> {
    regex
        .capture_names()
        .enumerate()
        .map(|(i, name)| match name {
            Some(name) => name.to_string(),
            None => format!("{}", i), // name the unnamed capture
        })
        .collect()
}
