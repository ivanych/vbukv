//! Разбор аргументов командной строки
//!
//! # SYNOPSIS
//! ```
//! use vbukv::input::args::Args;
//!
//! let args = Args::argparse();
//! # let args = Args::argparse_from(["vbukv,", "-l", "5", "-f", "test_slovar.txt", "c+"]);
//!
//! // В этом примере подразумевается, что программа была запущена
//! // в командной строке с такими аргументами:
//! // vbukv -l 5 -f test_slovar.txt c+
//! assert_eq!(args.length, 5, "Длина должна соответствовать опции -l");
//! assert_eq!(
//!     args.file.into_os_string(),
//!     "test_slovar.txt",
//!     "Файл должен соответствовать опции -f"
//! );
//! assert_eq!(args.rules.len(), 1, "Должно быть создано 1 правило");
//! assert_eq!(
//!     args.rules.iter().nth(0).unwrap().to_string(),
//!     "c+",
//!     "Текстовая запись правила должна соответствовать аргументу"
//! );
//! assert_eq!(
//!     args.rules.iter().nth(0).unwrap().letter,
//!     'c',
//!     "Буква должна соответствовать букве из текстовой записи правила"
//! );
//! ```
//!
//! # DESCRIPTION
//!
//! Модуль args используется для разбора аргументов командной строки.
//!
//! Основная функция модуля — [argparse].
//! Приложение [vbukv](../../../vbukv_cli/index.html) использует
//! именно эту функцию.
//!
//! Также в модуле есть вспомогательная функция [argparse_from]. Она не разбирает
//! аргументы командной строки напрямую, но разбирает список текстовых значений,
//! идентичный значениям, задаваемым в командной строке.
//! Эта функция нужна для отладочных целей или для построения приложения,
//! берущего параметры запуска не из аргументов командной строки, а, например,
//! из конфига.
//!
//! Обе функции возвращают структуру [Args], поля которой содержат
//! значения соответствующих аргументов командной строки.

#[cfg(test)]
mod tests;

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use clap_markdown;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::exit;

use crate::libvbukv::Rule;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
/// Угадывалка слов по буквам
///
/// Программа ищет слово (или слова) в словаре по заданным правилам.
/// Например: "слово из пяти букв, третья буква Б, четвёртая не Ю
/// и где-то в слове есть буква Ш".
pub struct Args {
    /// Длина искомого слова.
    ///
    /// Будут найдены только те слова, которые состоят из указанного количества букв.
    #[arg(short, long, default_value_t = 5)]
    // TODO Почему-то эта строчка не попадает в покрытие тестами. Надо разобраться.
    pub length: usize,

    /// Файл словаря.
    ///
    /// Относительный путь к файлу словаря.
    /// Словарь должен быть в формате plain text, одно слово в строке, кодировка utf-8.
    #[arg(short, long, default_value = "slovar.txt")]
    // TODO Почему-то эта строчка не попадает в покрытие тестами. Надо разобраться.
    pub file: PathBuf,

    /// Правило поиска (можно задать любое количество правил).
    ///
    /// Правила — это фильтры, которые говорят программе, какие слова следует выбрать из словаря.
    /// Выбираются только те слова, которые соответствуют ВСЕМ заданным правилам.
    ///
    /// Текстовая запись правила должна соответствовать шаблону
    ///
    /// `L[C][P]`
    ///
    /// где:
    ///
    /// `L` — буква.
    ///
    /// `C` — требование к букве; по умолчанию `+`.
    ///
    /// Возможны 4 требования:
    ///
    /// `+` — буква есть в слове (либо, если позиция указана, то есть
    /// на указанной позиции)
    ///
    /// `-` — буквы нет в слове (либо, если позиция указана, то нет
    /// на указанной позиции)
    ///
    /// `=` — буква есть только на указанной позиции и нигде больше
    ///
    /// `*` — буква есть на любой позиции, кроме указанной
    ///
    /// `P` — позиция буквы в слове.
    ///
    /// Если позиция не указана, то поиск происходит по всему слову.
    ///
    /// Позиция обязательно должна быть указана для требований `=` и `*`.
    ///
    /// Если позиция указана, то она должна иметь значение в диапазоне от 1
    /// до длины искомого слова, заданной опцией `--length`.
    ///
    /// Примеры правил:
    ///
    /// `а` — в слове есть буква `а`.
    ///
    /// `а+` — то же самое, что предыдущий вариант, но условие задано явно.
    ///
    /// `а1` — первая буква в слове это буква `а`. При этом в слове
    /// могуть быть ещё буквы `а` на других местах.
    ///
    /// `а+1` — то же самое, что предыдущий вариант, но условие задано явно.
    ///
    /// `а=1` — первая буква в слове это буква `а`. При этом в слове
    /// нет других букв `а`.
    ///
    /// `б-` — в слове нет буквы `б`.
    ///
    /// `б-1` — первая буква в слове это не буква `б`. При этом в слове
    /// могут быть буквы `б` на других местах.
    ///
    /// `б*1` — первая буква в слове это не буква `б`. При этом в слове
    /// есть буква `б` на каком-то другом месте.
    #[arg(value_name = "RULE")]
    // TODO Надо Vec<Rule> переделать на структуру Rules
    pub rules: Vec<Rule>,

    #[arg(long, hide = true)]
    // TODO Почему-то эта строчка не попадает в покрытие тестами. Надо разобраться.
    /// Аналогично `--help`, но результат будет выдан в формате Markdown.
    pub markdown_help: bool,
}

impl Args {
    // TODO Надо разобраться: эту функцию пока не получается покрыть тестами,
    // потому что непонятно, как мокнуть Args::parse().
    /// Разобрать аргументы командной строки
    ///
    /// ```
    /// use vbukv::input::args::Args;
    ///
    /// let args = Args::argparse();
    /// # let args = Args::argparse_from(["vbukv,", "-l", "5", "-f", "test_slovar.txt", "c+"]);
    ///
    /// // В этом примере подразумевается, что программа была запущена
    /// // в командной строке с такими аргументами:
    /// // vbukv -l 5 -f test_slovar.txt c+
    /// assert_eq!(args.length, 5, "Длина должна соответствовать опции -l");
    /// assert_eq!(
    ///     args.file.into_os_string(),
    ///     "test_slovar.txt",
    ///     "Файл должен соответствовать опции -f"
    /// );
    /// assert_eq!(args.rules.len(), 1, "Должно быть создано 1 правило");
    /// assert_eq!(
    ///     args.rules.iter().nth(0).unwrap().to_string(),
    ///     "c+",
    ///     "Текстовая запись правила должна соответствовать аргументу"
    /// );
    /// assert_eq!(
    ///     args.rules.iter().nth(0).unwrap().letter,
    ///     'c',
    ///     "Буква должна соответствовать букве из текстовой записи правила"
    /// );
    /// ```
    ///
    /// Это основная функция для получения параметров запуска программы
    /// (см. также вспомогательную функцию [argparse_from]).
    ///
    /// Функция не имеет собственных параметров, но она читает
    /// аргументы командной строки, заданные при запуске программы.
    /// Описание всех возможных аргументов находится в документации
    /// приложения [vbukv](../../../vbukv_cli/index.html).
    ///
    /// Дополнительно можно посмотреть низкоуровневое описание структуры [Args] —
    /// после разбора аргументы командной строки сохраняются в полях этой структуры.
    pub fn argparse() -> Args {
        let args = Args::parse();

        args.validate();

        args.markdown_help();

        args
    }

    // TODO Надо разобраться: сигнатура списана один-в-один с функции parse_from,
    // в которую дальше передаём itr. Как это работает?
    /// Разобрать список текстовых значений, синтаксически аналогичный
    /// аргументам командной строки.
    ///
    /// ```
    /// use vbukv::input::args::Args;
    ///
    /// let args = Args::argparse_from(["vbukv,", "-l", "5", "-f", "test_slovar.txt", "c+"]);
    ///
    /// assert_eq!(args.length, 5, "Длина должна соответствовать опции -l");
    /// assert_eq!(
    ///     args.file.into_os_string(),
    ///     "test_slovar.txt",
    ///     "Файл должен соответствовать опции -f"
    /// );
    /// assert_eq!(args.rules.len(), 1, "Должно быть создано 1 правило");
    /// assert_eq!(
    ///     args.rules.iter().nth(0).unwrap().to_string(),
    ///     "c+",
    ///     "Текстовая запись правила должна соответствовать аргументу"
    /// );
    /// assert_eq!(
    ///     args.rules.iter().nth(0).unwrap().letter,
    ///     'c',
    ///     "Буква должна соответствовать букве из текстовой записи правила"
    /// );
    /// ```
    ///
    /// Это вспомогательная функция (основная — [argparse]).
    /// Эта функция используется в тех случаях, когда параметры программы задаются
    /// не аргументами командной строки (основной способ), а как-либо иначе,
    /// например — берутся из конфига.
    ///
    /// Функция принимает список значений, идентичный тем,
    /// которые передаются в программу при запуске её в командной строке.
    ///
    /// Первым значением должно быть указано название программы
    /// (в примере выше — `vbukv`).
    /// Это значение требуется здесь по формальным причинам,
    /// для точного соответствия аргументам командной строки.
    /// Можно задать любое значение, в дальнейшей работе это значение
    /// никак не используется.
    ///
    /// Все остальные значения идентичны аргументам командной строки приложения
    /// [vbukv](../../../vbukv_cli/index.html).
    ///
    /// Дополнительно можно посмотреть низкоуровневое описание структуры [Args] —
    /// после разбора аргументы командной строки сохраняются в полях этой структуры.
    pub fn argparse_from<I, T>(itr: I) -> Args
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let args = Args::parse_from(itr);

        args.validate();

        args.markdown_help();

        args
    }

    // Если в правиле задана позиция, то она должна быть
    // - больше нуля
    // - меньше длины искомого слова (length)
    fn validate_rule_position(&self) -> Result<(), String> {
        for rule in self.rules.iter() {
            if rule.position.is_some_and(|x| (x <= 0) || (x > self.length)) {
                return Err(format!(
                    "position in rule '{}' ({}) выходит за пределы допустимого \
                    диапазона (1-{})",
                    rule,
                    rule.position.unwrap(),
                    self.length,
                ));
            }
        }

        Ok(())
    }

    fn validate(&self) {
        let result = self.validate_rule_position();

        if result.is_err() {
            Args::command()
                .error(ErrorKind::ValueValidation, result.unwrap_err())
                .exit()
        }
    }

    // TODO Надо разобраться: эту функцию пока не получается покрыть тестами,
    // потому что непонятно, как мокнуть exit().
    fn markdown_help(&self) {
        if self.markdown_help {
            clap_markdown::print_help_markdown::<Args>();
            exit(0);
        }
    }
}
