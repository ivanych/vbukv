#[cfg(test)]
mod tests;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn words_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
