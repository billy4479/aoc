use std::{fmt::Debug, fs, str::FromStr};

pub fn parse_input_file_as_vec<T>(file_name: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let raw = fs::read_to_string(file_name).unwrap();
    raw.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
