use std::env;
use std::process::exit;
use vbukv::{args::Args, dict, file, rule::Rule};

fn main() {
    // Прочитать аргументы командной строки в вектор
    let args: Vec<String> = env::args().collect();

    let args = Args::build(&args).unwrap_or_else(|err| {
        println!("Возникла ошибка при разборе аргументов: {err}");
        exit(1)
    });
    //dbg!(&args);

    // Слова
    let words = file::words_from_file(&args.file);
    println!("Словарь: {} ({} слов)", args.file, words.len());

    // Длина
    let length: usize = 5;
    println!("Длина предположений: {length}");

    // Правила
    let rules = vec![
        Rule {
            letter: String::from("п"),
            condition: String::from("!"),
            position: None,
        },
        Rule {
            letter: String::from("А"),
            condition: String::from("+"),
            position: Some(1),
        },
        Rule {
            letter: String::from("к"),
            condition: String::from("+"),
            position: Some(3),
        },
    ];
    println!("Правила:\n{rules:#?}");

    // Предположения
    let assumptions = dict::filter(words, length, rules);

    println!("Предположения:");
    println!("---------------------------------");

    for assumption in &assumptions {
        println!("{}", assumption);
    }

    println!("---------------------------------");
    println!("Найдено предположений: {}", assumptions.len());
}
