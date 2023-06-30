use regex::{Captures, Regex};
use serde_json::json;
use std::collections::HashMap;

pub trait Serializer {
    /// Serialize `Regex::Captures` as a string
    fn to_string(&self, caps: &Captures, idx: usize) -> String;
}

/// Serialize `Regex::Captures` as columns separated by `separator`
pub struct ColumnsSerializer {
    has_groups: bool,
    line_numbers: bool,
    separator: String,
}

impl ColumnsSerializer {
    pub fn new(regex: &Regex, line_numbers: bool, separator: String) -> Self {
        let has_groups = regex.captures_len() > 1;

        ColumnsSerializer {
            has_groups,
            line_numbers,
            separator,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    fn from_groups(&self, caps: &Captures) -> String {
        let fields: Vec<&str> = caps
            .iter()
            .skip(1) // the whole regex
            .map(|m| match m {
                Some(m) => m.as_str(),
                None => "", // empty columns for no match
            })
            .collect();
        fields.join(&self.separator)
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    fn from_whole(&self, caps: &Captures) -> String {
        caps.get(0).map_or("", |m| m.as_str()).to_string()
    }
}

impl Serializer for ColumnsSerializer {
    #[inline]
    fn to_string(&self, caps: &Captures, idx: usize) -> String {
        let columns = if self.has_groups {
            self.from_groups(caps)
        } else {
            self.from_whole(caps)
        };

        if self.line_numbers {
            format!("{}{}{}", idx, self.separator, columns)
        } else {
            columns
        }
    }
}

/// Serialize `Regex::Captures` as JSON
pub struct JsonSerializer {
    has_groups: bool,
    line_numbers: bool,
    group_names: Vec<String>,
}

impl JsonSerializer {
    /// Initialize new JSON serializer
    pub fn new(regex: &Regex, line_numbers: bool) -> Self {
        let group_names = capture_groups_names(regex);
        let has_groups = !group_names.is_empty();

        JsonSerializer {
            has_groups,
            line_numbers,
            group_names,
        }
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    fn from_groups<'a>(&'a self, caps: &Captures<'a>) -> HashMap<&'a str, &'a str> {
        caps.iter()
            .skip(1) // the whole regex
            .enumerate()
            .map_while(|(i, m)| Some((self.group_names[i].as_str(), m?.as_str()))) // skip empty matches
            .collect()
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    fn from_whole<'a>(&self, caps: &Captures<'a>) -> HashMap<&'a str, &'a str> {
        let matched = caps.get(0).map_or("", |m| m.as_str());
        let mut fields = HashMap::new();
        fields.insert("match", matched);
        fields
    }
}

impl Serializer for JsonSerializer {
    #[inline]
    fn to_string(&self, caps: &Captures, idx: usize) -> String {
        let mut fields = if self.has_groups {
            self.from_groups(caps)
        } else {
            self.from_whole(caps)
        };

        let idx_str;
        if self.line_numbers {
            idx_str = idx.to_string();
            fields.insert("line", &idx_str);
        }
        json!(fields).to_string()
    }
}

/// Extract capture names and name the unnamed captures
fn capture_groups_names(regex: &Regex) -> Vec<String> {
    regex
        .capture_names()
        .skip(1) // the whole regex
        .enumerate()
        .map(|(i, name)| match name {
            Some(name) => name.to_string(),
            None => format!("{}", i + 1), // name the unnamed capture
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{ColumnsSerializer, JsonSerializer, Serializer};
    use regex::Regex;
    use test_case::test_case;

    #[test_case(r"", vec![]; "empty regex")]
    #[test_case(r"[a-z0-9]* [a-z]{1,3}", vec![]; "no matching groups")]
    #[test_case(r"()", vec![String::from("1")]; "empty group")]
    #[test_case(r"([^ ]+) xyz", vec![String::from("1")]; "unnamed group")]
    #[test_case(r"([^ ]+) xyz ([0-9])([a-z])", vec![String::from("1"), String::from("2"), String::from("3")]; "many unnamed groups")]
    #[test_case(r"(?<AAA>[^ ]+) xyz", vec![String::from("AAA")]; "named group")]
    #[test_case(r"(?<aaa>[^ ]+) xyz (?<bbb>[0-9])(?<ccc>[a-z])", vec![String::from("aaa"), String::from("bbb"), String::from("ccc")]; "many named groups")]
    #[test_case(r"([^ ]+) xyz (?<BBB>[0-9])([a-z])", vec![String::from("1"), String::from("BBB"), String::from("3")]; "mixed named and unnamed groups")]
    fn capture_groups_names(regex: &str, expected: Vec<String>) {
        let regex = Regex::new(regex).unwrap();
        assert_eq!(super::capture_groups_names(&regex), expected);
    }

    #[test_case(
        false, r"", r"", r#"{"match":""}"#;
        "empty"
    )]
    #[test_case(
        true, r"", r"", r#"{"line":"42","match":""}"#;
        "empty with line number"
    )]
    #[test_case(
        false, r".*", r"abc123", r#"{"match":"abc123"}"#;
        "everything"
    )]
    #[test_case(
        true, r".*", r"abc123", r#"{"line":"42","match":"abc123"}"#;
        "everything with line number"
    )]
    #[test_case(
        false, r"([^ ]+) xyz", r"abc123 xyz", r#"{"1":"abc123"}"#;
        "single unnamed group"
    )]
    #[test_case(
        true, r"([^ ]+) xyz", r"abc123 xyz", r#"{"1":"abc123","line":"42"}"#;
        "single unnamed group with line number"
    )]
    #[test_case(
        false, r"(?<ABc>[^ ]+) xyz", r"abc123 xyz", r#"{"ABc":"abc123"}"#;
        "single named group"
    )]
    #[test_case(
        true, r"(?<ABc>[^ ]+) xyz", r"abc123 xyz", r#"{"ABc":"abc123","line":"42"}"#;
        "single named group with line number"
    )]
    #[test_case(
        false, r"([0-9]{3})-([0-9]{2})-([0-9]{3})", r"prefix 111-22-333 suffix abc 123", r#"{"1":"111","2":"22","3":"333"}"#;
        "many unnamed groups"
    )]
    #[test_case(
        true, r"([0-9]{3})-([0-9]{2})-([0-9]{3})", r"prefix 111-22-333 suffix abc 123", r#"{"1":"111","2":"22","3":"333","line":"42"}"#;
        "many unnamed groups with line number"
    )]
    #[test_case(
        false, r"(?<A>[0-9]{3})-(?<B>[0-9]{2})-(?<C>[0-9]{3})", r"prefix 111-22-333 suffix abc 123", r#"{"A":"111","B":"22","C":"333"}"#;
        "many named groups"
    )]
    #[test_case(
        true, r"(?<A>[0-9]{3})-(?<B>[0-9]{2})-(?<C>[0-9]{3})", r"prefix 111-22-333 suffix abc 123", r#"{"A":"111","B":"22","C":"333","line":"42"}"#;
        "many named groups with line number"
    )]
    fn json_serializer(line_numbers: bool, regex: &str, example: &str, expected: &str) {
        let regex = Regex::new(regex).unwrap();
        let serializer = JsonSerializer::new(&regex, line_numbers);
        let captures = regex.captures(example).unwrap();
        assert_eq!(serializer.to_string(&captures, 42), expected);
    }

    #[test_case(
        false, "\t", r"", r"", "";
        "empty"
    )]
    #[test_case(
        true, "\t", r"", r"", "42\t";
        "empty with line number"
    )]
    #[test_case(
        false, "\t", r".*", r"abc123", "abc123";
        "everything"
    )]
    #[test_case(
        true, "\t", r".*", r"abc123", "42\tabc123";
        "everything with line number"
    )]
    #[test_case(
        false, "\t", r"([^ ]+) xyz", r"abc123 xyz", "abc123";
        "single unnamed group"
    )]
    #[test_case(
        true, "\t", r"([^ ]+) xyz", r"abc123 xyz", "42\tabc123";
        "single unnamed group with line number"
    )]
    #[test_case(
        false, "\t", r"(?<XXX>[^ ]+) xyz", r"abc123 xyz", "abc123";
        "single named group"
    )]
    #[test_case(
        true, "\t", r"(?<XXX>[^ ]+) xyz", r"abc123 xyz", "42\tabc123";
        "single named group with line number"
    )]
    #[test_case(
        false, "\t", r"([0-9]{3})-([0-9]{2})-([0-9]{3})", r"prefix 111-22-333 suffix abc 123", "111\t22\t333";
        "many unnamed groups"
    )]
    #[test_case(
        true, "\t", r"([0-9]{3})-([0-9]{2})-([0-9]{3})", r"prefix 111-22-333 suffix abc 123", "42\t111\t22\t333";
        "many unnamed groups with line number"
    )]
    #[test_case(
        false, "\t", r"(?<A>[0-9]{3})-(?<B>[0-9]{2})-(?<C>[0-9]{3})", r"prefix 111-22-333 suffix abc 123", "111\t22\t333";
        "many named groups"
    )]
    #[test_case(
        true, "\t", r"(?<A>[0-9]{3})-(?<B>[0-9]{2})-(?<C>[0-9]{3})", r"prefix 111-22-333 suffix abc 123", "42\t111\t22\t333";
        "many named groups with line number"
    )]
    fn columns_serializer(
        line_numbers: bool,
        separator: &str,
        regex: &str,
        example: &str,
        expected: &str,
    ) {
        let regex = Regex::new(regex).unwrap();
        let serializer = ColumnsSerializer::new(&regex, line_numbers, separator.to_string());
        let captures = regex.captures(example).unwrap();
        assert_eq!(serializer.to_string(&captures, 42), expected);
    }
}
