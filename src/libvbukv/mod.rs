//! Поиск слов в словаре по буквам
//!
//! # SYNOPSIS
//!
//! ```
//! use std::path::PathBuf;
//! use std::str::FromStr;
//! use vbukv::libvbukv;
//!
//! // словарь (этот файл есть в репозитории проекта)
//! let file = PathBuf::from("test_slovar.txt");
//!
//! // длина искомых слов
//! let length = 5;
//!
//! // список правил поиска
//! let rules = vec![
//!     libvbukv::Rule::from_str("с+").unwrap(),
//!     libvbukv::Rule::from_str("о+3").unwrap(),
//! ];
//!
//! // Найти слова, подходящие под заданные параметры
//! let assumptions = libvbukv::assumptions(&file, length, &rules);
//!
//! // Проверка: это слово должно быть среди найденных
//! let word = "стопа".to_string();
//! assert!(assumptions.contains(&word));
//! ```
//!
//! # DESCRIPTION
//!
//! Модуль libvbukv предлагает механизм для поиска слов в словаре по заданным
//! правилам.
//!
//! В качестве словаря используется простой текстовый файл в формате
//! "одна строка — одно слово".
//!
//! Ограничений на длину словаря нет. На практике автор использует словарь
//! примерно из 150 тысяч слов (это практически все слова русского языка),
//! в таком словаре модуль находит заданные слова примерно за 0.1 секунды.

pub mod dict;
pub mod file;
pub mod rule;
#[cfg(test)]
mod tests;

use dict::Dict;
pub use rule::Rule;
use std::path::PathBuf;

/// Прочитать файл словаря и найти слова, подходящие под заданные параметры
///
/// Пример:
///
/// ```
/// use std::path::PathBuf;
/// use std::str::FromStr;
/// use vbukv::libvbukv;
///
/// // словарь (этот файл есть в репозитории проекта)
/// let file = PathBuf::from("test_slovar.txt");
///
/// // длина искомых слов
/// let length = 5;
///
/// // список правил поиска
/// let rules = vec![
///     libvbukv::Rule::from_str("с+").unwrap(),
///     libvbukv::Rule::from_str("о+3").unwrap(),
/// ];
///
/// // Найти слова, подходящие под заданные параметры
/// let assumptions = libvbukv::assumptions(&file, length, &rules);
///
/// // Проверка: это слово должно быть среди найденных
/// let word = "стопа".to_string();
/// assert!(assumptions.contains(&word));
/// ```
///
/// Аргументы:
///
/// * `file` — относительный путь к файлу словаря.
/// * `length` — длина искомых слов в буквах.
/// * `rules` — список правил для поиска слов.
///
/// Синтаксис правил описан в модуле [rule].
///
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
