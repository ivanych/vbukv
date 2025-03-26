//! Низкоуровневые методы для работы со словарём
//!
//! # SYNOPSIS
//! ```
//! use std::str::FromStr;
//! use vbukv::libvbukv;
//! use vbukv::libvbukv::dict::Dict;
//!
//! // Слова, среди которых будет происходить поиск
//! let words = vec![
//!    String::from("раз"),
//!    String::from("два"),
//!    String::from("три"),
//! ];
//!
//! // Создать словарь
//! let dict = Dict::new(words);
//!
//! // длина искомых слов
//! let length: usize = 3;
//!
//! // список правил поиска
//! let rules = vec![
//!     libvbukv::Rule::from_str("р+").unwrap(),
//! ];
//!
//! // Найти слова, подходящие под заданные параметры
//! let assumptions = dict.filter(length, &rules);
//!
//! assert_eq!(assumptions.len(), 2);
//! ```
//!
//! # DESCRIPTION
//!
//! Модуль dict содержит низкоуровненые методы для создания словаря и поиска
//! слов в созданном словаре.
//!
//! Обратите внимание — в большинстве случаев проще и удобнее будет использовать
//! более высокоуровневый модуль [libvbukv](../index.html).

#[cfg(test)]
mod tests;

use crate::libvbukv::rule::{Cond, Rule};

/// Словарь
///
/// Структура Dict хранит словарь целиком в оперативной памяти.
#[derive(Debug)]
pub struct Dict {
    words: Vec<String>,
}

impl Dict {
    /// Конструктор словаря
    ///
    /// ```
    /// use vbukv::libvbukv::dict::Dict;
    ///
    /// // Слова, среди которых будет происходить поиск
    /// let words = vec![
    ///    String::from("раз"),
    ///    String::from("два"),
    ///    String::from("три"),
    /// ];
    ///
    /// // Создать словарь
    /// let dict = Dict::new(words);
    /// ```
    ///
    /// Аргументы:
    ///
    /// * `words` — список слов для создания словаря
    ///
    pub fn new(words: Vec<String>) -> Dict {
        Dict { words }
    }

    /// Найти слова, подходящие под заданные параметры
    ///
    /// ```
    /// use std::str::FromStr;
    /// use vbukv::libvbukv;
    /// use vbukv::libvbukv::dict::Dict;
    ///
    /// // Слова, среди которых будет происходить поиск
    /// let words = vec![
    ///    String::from("раз"),
    ///    String::from("два"),
    ///    String::from("три"),
    /// ];
    ///
    /// // Создать словарь
    /// let dict = Dict::new(words);
    ///
    /// // длина искомых слов
    /// let length = 3;
    ///
    /// // список правил поиска
    /// let rules = vec![
    ///     libvbukv::Rule::from_str("р+").unwrap(),
    /// ];
    ///
    /// // Найти слова, подходящие под заданные параметры
    /// let assumptions = dict.filter(length, &rules);
    ///
    /// assert_eq!(assumptions.len(), 2);
    /// ```
    pub fn filter(&self, length: usize, rules: &Vec<Rule>) -> Vec<String> {
        self.words
            // идём по ссылкам на слова
            .iter()
            // Берём только нужные слова
            .filter(|&word| {
                //dbg!(&word);

                // Длина
                if word.chars().count() != length {
                    return false;
                }

                rules
                    .iter()
                    // Все правила должны выполниться
                    .all(|rule| match rule.condition {
                        Cond::Plus => is_present(word, rule.letter, &rule.position),
                        Cond::Minus => is_absent(word, rule.letter, &rule.position),
                        Cond::Equals => is_inner(word, rule.letter, &rule.position),
                        Cond::Asterisk => is_outer(word, rule.letter, &rule.position),
                    })
            })
            // разыменовываем ссылки на слова
            .cloned()
            .collect()
    }
}

fn is_present(word: &String, letter: char, position: &Option<usize>) -> bool {
    match position {
        None => word.contains(letter),
        Some(_) => position_symbol(word, position) == letter,
    }
}

fn is_absent(word: &String, letter: char, position: &Option<usize>) -> bool {
    !match position {
        None => word.contains(letter),
        Some(_) => position_symbol(word, position) == letter,
    }
}

fn is_inner(word: &String, letter: char, position: &Option<usize>) -> bool {
    // буква должно быть на указанном месте
    position_symbol(word, position) == letter
        // и буквы не должно быть где-то в другом месте
        && !word_without_position(word, position).contains(letter)
}

fn is_outer(word: &String, letter: char, position: &Option<usize>) -> bool {
    // буквы не должно быть на указанном месте
    position_symbol(word, position) != letter
        // и буква должна быть где-то в другом месте
        && word_without_position(word, position).contains(letter)
}

fn position_symbol(word: &String, position: &Option<usize>) -> char {
    let index = position.unwrap() - 1;

    word.chars().nth(index).unwrap()
}

fn word_without_position(word: &String, position: &Option<usize>) -> String {
    let index = position.unwrap() - 1;

    word.chars()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, c)| c)
        .collect()
}
