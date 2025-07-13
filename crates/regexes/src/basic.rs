use regex::Regex;

pub fn is_number(n: &str) -> bool {
    let re = Regex::new(r"^\d").unwrap();

    re.is_match(n)
}

#[cfg(test)]
#[path = "./basic_test.rs"]
mod basic_test;
