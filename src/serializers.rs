use regex::Captures;
use serde_json::json;
use std::collections::HashMap;

pub trait Serializer {
    #[inline]
    fn to_string(&self, caps: &Captures, idx: usize) -> String {
        if self.has_groups() {
            self.capture_groups_to_string(caps, idx)
        } else {
            self.whole_match_to_string(caps, idx)
        }
    }

    fn has_groups(&self) -> bool;
    fn capture_groups_to_string(&self, caps: &Captures, idx: usize) -> String;
    fn whole_match_to_string(&self, caps: &Captures, idx: usize) -> String;
}

pub struct ColumnsSerializer {
    has_groups: bool,
    line_numbers: bool,
    separator: String,
}

impl ColumnsSerializer {
    pub fn new(has_groups: bool, line_numbers: bool, separator: String) -> Self {
        ColumnsSerializer {
            has_groups,
            line_numbers,
            separator,
        }
    }
}

impl Serializer for ColumnsSerializer {
    #[inline]
    fn has_groups(&self) -> bool {
        self.has_groups
    }

    #[inline]
    fn capture_groups_to_string(&self, caps: &Captures, idx: usize) -> String {
        let fields: Vec<&str> = caps
            .iter()
            .skip(1) // the whole regex
            .map(|m| match m {
                Some(m) => m.as_str(),
                None => "", // empty columns for no match
            })
            .collect();
        let columns = fields.join(&self.separator);
        if self.line_numbers {
            format!("{}{}{}", idx, self.separator, columns)
        } else {
            columns
        }
    }

    #[inline]
    fn whole_match_to_string(&self, caps: &Captures, idx: usize) -> String {
        let matched = caps.get(0).map_or("", |m| m.as_str());
        if self.line_numbers {
            format!("{}{}{}", idx, self.separator, matched)
        } else {
            matched.to_string()
        }
    }
}

pub struct JsonSerializer {
    has_groups: bool,
    line_numbers: bool,
    group_names: Vec<String>,
}

impl JsonSerializer {
    /// Initialize new JSON serializer
    pub fn new(has_groups: bool, line_numbers: bool, group_names: Vec<String>) -> Self {
        JsonSerializer {
            has_groups,
            line_numbers,
            group_names,
        }
    }
}

impl Serializer for JsonSerializer {
    #[inline]
    fn has_groups(&self) -> bool {
        self.has_groups
    }

    #[inline]
    fn capture_groups_to_string(&self, caps: &Captures, idx: usize) -> String {
        let mut fields: HashMap<&str, &str> = caps
            .iter()
            .enumerate()
            .skip(1) // the whole regex
            .map_while(|(i, m)| Some((self.group_names[i].as_str(), m?.as_str()))) // skip empty matches
            .collect();
        let idx_str;
        if self.line_numbers {
            idx_str = idx.to_string();
            fields.insert("line", &idx_str);
        }
        json!(fields).to_string()
    }

    #[inline]
    fn whole_match_to_string(&self, caps: &Captures, idx: usize) -> String {
        let mut fields: HashMap<&str, &str> = HashMap::new();
        let matched = caps.get(0).map_or("", |m| m.as_str());
        fields.insert("match", matched);

        let idx_str;
        if self.line_numbers {
            idx_str = idx.to_string();
            fields.insert("line", &idx_str);
        }

        json!(fields).to_string()
    }
}
