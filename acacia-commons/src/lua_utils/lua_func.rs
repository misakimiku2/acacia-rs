use regex::Regex;
use std::sync::OnceLock;

pub fn parse_function_call<'a>(code: &'a str) -> Option<(&'a str, &'a str)> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"^([A-Za-z0-9_.]+)\((.*)\)$").unwrap());

    match re.captures(code) {
        Some(caps) => {
            let func_name = caps.get(1).unwrap().as_str().trim();
            let raw_args = caps.get(2).unwrap().as_str().trim();
            Some((func_name, raw_args))
        }
        None => {
            println!("Unknown Code: {}", code);
            None
        }
    }
}

pub fn parse_function_args(args: &str) -> Vec<String> {
    if args.is_empty() {
        return Vec::new();
    }
    let mut result = Vec::new();
    let mut current = String::new();
    let mut brace_level = 0;
    for c in args.chars() {
        match c {
            '{' => {
                brace_level += 1;
                current.push(c);
            }
            '}' => {
                brace_level -= 1;
                current.push(c);
            }
            ',' if brace_level == 0 => {
                result.push(current.trim().to_string());
                current.clear();
            }
            _ => {
                current.push(c);
            }
        }
    }
    if !current.is_empty() || args.ends_with(',') {
        result.push(current.trim().to_string());
    }
    result
}
