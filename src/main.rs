use std::env;
use std::process::exit;
use vbukv::{args::Args, dict, file};

fn main() {
    // Прочитать аргументы командной строки в вектор
    let args: Vec<String> = env::args().collect();

    // Раcпарсить аргументы
    let args = Args::build(&args).unwrap_or_else(|err| {
        println!("Возникла ошибка при разборе аргументов: {err}");
        exit(1)
    });

    // Прочитать слова из файла
    let words = file::words_from_file(&args.file);
    println!("Словарь: {} ({} слов)", args.file, words.len());

    // Найти предположения
    let assumptions = dict::filter(words, args.length, args.rules);

    println!("Предположения:");
    println!("---------------------------------");

    for assumption in &assumptions {
        println!("{}", assumption);
    }

    println!("---------------------------------");
    println!("Найдено предположений: {}", assumptions.len());
}
