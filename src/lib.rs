pub mod args;
pub mod dict;
pub mod file;
pub mod rule;

use args::Args;

pub fn assumptions(args: &Args) -> Vec<String> {
    // Прочитать слова из файла
    let words = file::words_from_file(&args.file);
    println!(
        "Прочитан словарь: {:?} ({} слов)",
        args.file.file_name().unwrap(),
        words.len()
    );

    // Найти предположения
    let assumptions = dict::filter(words, args.length, &args.rules);

    assumptions
}

#[cfg(test)]
mod tests;
