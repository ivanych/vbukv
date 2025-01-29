use clap::Parser;

use crate::rule::Rule;

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
    // TODO Почему-то эта строчка не попадает в покрытие тестами. Надо разобраться.
    #[arg(short, long, default_value_t = 5)]
    pub length: usize,

    // TODO надо сделать не string, а спецтип для путей
    /// Файл словаря.
    ///
    /// Относительный путь к файлу словаря.
    /// Словарь должен быть в формате plain text, одно слово в строке, кодировка utf-8.
    #[arg(short, long, default_value = "slovar.txt")]
    // TODO Почему-то эта строчка не попадает в покрытие тестами. Надо разобраться.
    pub file: String,

    /// Правило поиска (можно задать любое количество правил).
    ///
    /// Правила — это фильтры, которые говорят программе, какие слова следует выбрать из словаря.
    /// Выбираются только те слова, которые соответствуют ВСЕМ заданным правилам.
    ///
    /// Каждое правило должно соответствовать шаблону:
    ///
    /// `L[C][P]`
    ///
    /// где:
    ///
    /// L — буква
    ///
    /// C — условие; по умолчанию `+`
    ///
    /// Возможны 4 условия:
    ///
    /// `+` — буква есть в слове (либо, если позиция указана, то есть на указанной позиции)
    ///
    /// `-` — буквы нет в слове (либо, если позиция указана, то нет на указанной позиции)
    ///
    /// `=` — буква есть только на указанной позиции и нигде больше
    ///
    /// `*` — буква есть на любой позиции, кроме указанной
    ///
    /// P — позиция буквы в слове
    ///
    /// Если позиция не указана, то поиск происходит по всему слову.
    ///
    /// Позиция обязательно должна быть указана для условий `=` и `*`.
    ///
    /// Примеры правил:
    ///
    /// `а` — в слове есть буква `а`
    ///
    /// `а+` — то же самое, что предыдущий вариант, но условие задано явно
    ///
    /// `а1` — первая буква в слове это буква `а`. При этом в слове могуть быть ещё буквы `а`
    /// на других местах.
    ///
    /// `а+1` — то же самое, что предыдущий вариант, но условие задано явно
    ///
    /// `а=1` — первая буква в слове это буква `а`. При этом в слове нет других букв `а`.
    ///
    /// `б-` — в слове нет буквы `б`
    ///
    /// `б-1` — первая буква в слове это не буква `б`. При этом в слове могут быть буквы `б`
    ///  на других местах
    ///
    /// `б*1` — первая буква в слове это не буква `б`. При этом в слове есть буква `б`
    /// на каком-то другом месте.
    #[arg(value_name= "RULE", value_parser = Rule::build)]
    // TODO Надо Vec<Rule> переделать на структуру Rules
    // TODO Почему-то эта строчка не попадает в покрытие тестами. Надо разобраться.
    pub rules: Vec<Rule>,

    #[arg(long, hide = true)]
    pub markdown_help: bool,
}

#[cfg(test)]
mod tests;
