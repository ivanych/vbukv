// TODO Надо разобраться — почему для вызова Rule::from_str() нужно импортировать FromStr?
// Вот для вызова PathBuf::from() почему-то не нужно импортировать From, как-то без него работает...

//! Библиотека и консольное приложение для поиска слов в словаре по буквам
//!
//! # SYNOPSIS
//!
//! ```
//! use std::path::PathBuf;
//! use std::str::FromStr;
//! use vbukv::libvbukv;
//!
//! // словарь
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
//! Пакет [crate::vbukv] содержит библиотеку для поиска слов в словаре по буквам.
//! Также в пакете есть готовое консольное приложение **vbukv**,
//! сделанное с использованием этой библиотеки.
//!
//! Для поиска слов могут быть заданы следующие параметры:
//! * **file** — словарь (путь к файлу словаря)
//! * **length** — длина искомых слов (число)
//! * **rules** — список правил, по которым нужно искать слова в словаре
//!   (строки)
//!
//! ## Ввод параметров
//!
//! Консольное приложение **vbukv** получает параметры из аргументов
//! командной строки с помощью модуля [vbukv::input::args]:
//!
//! ```
//! use vbukv::input::args;
//!
//! // Получить параметры поиска из аргументов командной строки
//! // (на самом деле приложение vbukv использует не `parse_from`, а `parse`,
//! // но для примера удобнее использовать `parse_from`)
//! let args = args::parse_from(["vbukv", "-l", "5", "с+"]);
//!
//! // Проверка: параметр "длина искомых слов" получен из аргументов правильно
//! assert!(args.length == 5);
//!
//! // Найти слова, подходящие под заданные параметры
//! // let assumptions = libvbukv::assumptions(&args.file, args.length, &args.rules)
//! ```
//!
//! Библиотека [vbukv] не накладывает ограничений на источник параметров.
//! Параметры могут быть получены откуда угодно или вообще захардкожены
//! (см. синопсис).
//!
//! ## Вывод результата
//!
//! Результат работы консольного приложения **vbukv** выводится в терминал
//! с помощью модуля [vbukv::output::term]:
//!
//! ```
//! use vbukv::output::term;
//!
//! // Найти слова, подходящие под заданные параметры
//! // let assumptions = libvbukv::assumptions(&args.file, args.length, &args.rules)
//! # let assumptions = vec![
//! #    String::from("раз"),
//! #    String::from("два"),
//! #    String::from("три"),
//! # ];
//!
//! // Вывести найденные слова
//! term::output(&assumptions).expect("Failed to print output");
//! ```
//!
//! Библиотека [vbukv] не имеет средств вывода,
//! релизация вывода остаётся решением программиста.
//!

#![warn(missing_docs)]

pub mod input;
pub mod libvbukv;
pub mod output;
