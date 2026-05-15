use regex::Regex;
use std::collections::HashMap;
use std::fs;
use toml::{Table, Value};
use walkdir::DirEntry;

const DISABLED: &str = "disabled";
const EXT_MASK: &str = "ext_mask";
const FILE_MASK: &str = "file_mask";
const FILE_EXISTS: &str = "file_exists";
const DIR_EXISTS: &str = "dir_exists";
const IF: &str = "if";

pub fn load_file_as_sections() -> HashMap<String, Value> {
    match fs::read_to_string("./clean.toml") {
        Ok(res) => {
            let table = toml::from_str::<Table>(&res).unwrap();
            let mut res = HashMap::new();

            for key in table.keys() {
                res.insert(key.to_string(), table[key].clone());
            }

            res
        }

        Err(_) => panic!(
            "No clean.toml file found in the current directory [{}]",
            std::env::current_dir().unwrap().to_str().unwrap()
        ),
    }
}

pub fn get_filter_from_raw_section(s: &Value) -> Box<dyn Fn(&DirEntry) -> bool> {
    if let Some(v) = s.as_table() {
        get_filter_from_table(v)
    } else {
        panic!("Got section: {} and couldn't parse it", s);
    }
}

fn get_filter_from_table(t: &Table) -> Box<dyn Fn(&DirEntry) -> bool> {
    if let Some(disabled) = t.get(DISABLED) {
        if disabled
            .as_bool()
            .expect("Boolean value for disabled is invalid")
        {
            return Box::new(|_: &DirEntry| false);
        }
    }

    let mut acc: Vec<Box<dyn Fn(&DirEntry) -> bool>> = vec![];

    for key in t.keys() {
        let value = t.get(key).unwrap();
        let value_keys: Vec<String> = value.as_table().unwrap().keys().cloned().collect();

        if let Some(disabled) = value.get(DISABLED) {
            if disabled
                .as_bool()
                .expect("Boolean value for disabled is invalid")
            {
                continue;
            }
        }

        for value_key in &value_keys {
            match value_key.as_str() {
                FILE_EXISTS => acc.push(Box::new(|file| file.path().is_file())),
                DIR_EXISTS => acc.push(Box::new(|dir| dir.path().is_dir())),
                EXT_MASK => {
                    let pattern = get_regex(
                        value
                            .get(value_key)
                            .unwrap()
                            .as_str()
                            .expect("Expected string value for mask")
                            .to_string(),
                    );
                    acc.push(Box::new(move |file| {
                        file.path()
                            .extension()
                            .and_then(|e| e.to_str())
                            .map(|e| pattern.is_match(e))
                            .unwrap_or(false)
                    }))
                }
                FILE_MASK => {
                    let pattern = get_regex(
                        value
                            .get(value_key)
                            .unwrap()
                            .as_str()
                            .expect("Expected string value for mask")
                            .to_string(),
                    );
                    acc.push(Box::new(move |file| {
                        file.path()
                            .file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| pattern.is_match(n))
                            .unwrap_or(false)
                    }))
                }
                IF => {
                    let if_filter = build_if_filter(
                        value
                            .get(IF)
                            .unwrap()
                            .as_table()
                            .expect("Expected table value for 'if'"),
                    );
                    acc.push(if_filter);
                }
                DISABLED => {}
                _ => {}
            }
        }
    }
    Box::new(move |file: &DirEntry| acc.iter().map(|filter| filter(file)).all(|b| b))
}

fn build_if_filter(if_table: &Table) -> Box<dyn Fn(&DirEntry) -> bool> {
    if let Some(pattern_val) = if_table.get(FILE_EXISTS) {
        let pattern = pattern_val
            .as_str()
            .expect("Expected string value for 'if.file_exists'")
            .to_string();

        Box::new(move |entry: &DirEntry| {
            let resolved = resolve_pattern(&pattern, entry);
            entry
                .path()
                .parent()
                .map(|parent| parent.join(&resolved).is_file())
                .unwrap_or(false)
        })
    } else if let Some(pattern_val) = if_table.get(DIR_EXISTS) {
        let pattern = pattern_val
            .as_str()
            .expect("Expected string value for 'if.dir_exists'")
            .to_string();

        Box::new(move |entry: &DirEntry| {
            let resolved = resolve_pattern(&pattern, entry);
            entry
                .path()
                .parent()
                .map(|parent| parent.join(&resolved).is_dir())
                .unwrap_or(false)
        })
    } else {
        panic!("'if' block must contain either 'file_exists' or 'dir_exists'");
    }
}

fn resolve_pattern(pattern: &str, entry: &DirEntry) -> String {
    if !pattern.contains("{}") {
        return pattern.to_string();
    }

    let stem = entry
        .path()
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    pattern.replace("{}", stem)
}

fn get_regex(val: String) -> Regex {
    Regex::new(format!("^{}$", val).as_str()).expect("Regex is malformed")
}
