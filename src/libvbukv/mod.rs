pub mod dict;
pub mod file;
pub mod rule;
#[cfg(test)]
mod tests;

use dict::Dict;
use rule::Rule;
use std::path::PathBuf;

pub fn assumptions(file: &PathBuf, length: usize, rules: &Vec<Rule>) -> Vec<String> {
    // Прочитать слова из файла
    let words = file::words_from_file(&file);
    println!("Прочитан файл {} ({} слов)", file.display(), words.len());

    // Создать словарь
    let dict = Dict::new(words);
    //dbg!(&dict);

    // Найти предположения
    let assumptions = dict.filter(length, rules);

    assumptions
}
