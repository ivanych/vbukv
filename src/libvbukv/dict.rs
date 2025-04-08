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

use crate::libvbukv::rule::Rule;

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
                    .all(|rule| rule.check_word(word))
            })
            // разыменовываем ссылки на слова
            .cloned()
            .collect()
    }
}
