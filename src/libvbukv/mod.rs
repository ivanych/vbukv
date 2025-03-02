pub mod dict;
pub mod file;
pub mod rule;
#[cfg(test)]
mod tests;

use crate::args::Args;
use dict::Dict;

pub fn assumptions(args: &Args) -> Vec<String> {
    // Прочитать слова из файла
    let words = file::words_from_file(&args.file);
    println!(
        "Прочитан файл {} ({} слов)",
        args.file.display(),
        words.len()
    );

    // Создать словарь
    let dict = Dict::new(words);
    //dbg!(&dict);

    // Найти предположения
    let assumptions = dict.filter(args.length, &args.rules);

    assumptions
}
