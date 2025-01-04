use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let dict = lines_from_file("slovar.txt");

    let dict_iter = dict.iter();

    for slovo in dict_iter
        .filter(|x| x.len() == 10)
        .filter(|x| x.get(6..8) == Some(&String::from("а")))
        .filter(|x| x.contains("в"))
        .filter(|x| x.contains("п"))
    {
        println!("{}", slovo);
    }
}
