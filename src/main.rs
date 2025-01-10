use std::process::exit;
use vbukv::{args::Args, dict, file};

fn main() {
    let args = Args::build().unwrap_or_else(|err| {
        println!("Возникла ошибка при разборе аргументов: {err}");
        exit(1)
    });
    //dbg!(&args);

    let words = file::words_from_file(&args.file);
    println!("Словарь: {} ({} слов)", args.file, words.len());

    let assumptions = dict::filter(words);

    println!("---------------------------------");

    for assumption in &assumptions {
        println!("{}", assumption);
    }

    println!("---------------------------------");
    println!("Найдено предположений: {}", assumptions.len());
}
