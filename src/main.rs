//use std::env;
// TODO непонятно, зачем здесь импорт clap-а, но без него не работает. Надо разобраться.
use clap::Parser;
//use std::process::exit;
use vbukv::{args::Args, dict, file};

fn main() {
    // Прочитать аргументы командной строки в вектор
    //let args: Vec<String> = env::args().collect();

    let args = Args::parse();
    dbg!(&args);

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
