//! Чтение списка слов из файла
//!
//! # SYNOPSIS
//!
//! ```
//! use vbukv::libvbukv::file;
//!
//! // словарь (этот файл есть в репозитории проекта)
//! let filename = "test_slovar.txt".to_string();
//!
//! // Прочитать слова из файла
//! let words = file::words_from_file(filename);
//!
//! // Проверка: должно быть прочитано 11 слов
//! assert_eq!(words.len(), 11);
//! ```
//!
//! # DESCRIPTION
//!
//! Модуль file предоставляет функции для чтения списка слов из файла.
//!
//! Файл должен быть простым текстовым файлом в формате
//! "одна строка — одно слово". Если в строке будет больше одного слова,
//! то это не вызовет технических проблем, но логика работы будет нарушена —
//! вся строка будет считаться одним словом.
#[cfg(test)]
mod tests;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Прочитать список слов из файла
///
/// ```
/// use vbukv::libvbukv::file;
///
/// // словарь (этот файл есть в репозитории проекта)
/// let filename = "test_slovar.txt".to_string();
///
/// // Прочитать слова из файла
/// let words = file::words_from_file(filename);
///
/// // Проверка: должно быть прочитано 11 слов
/// assert_eq!(words.len(), 11);
/// ```
///
/// Аргументы:
///
/// * `filename` — путь к файлу.
pub fn words_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
