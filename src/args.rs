use crate::rule::Rule;
use clap::Parser;
// используется fancy_regex, потому что regex не умеет делать заглядывание вперёд (look-ahead),
// а оно нужно для проверки правила
use fancy_regex::Regex;

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
    pub length: usize,

    // TODO надо сделать не string, а спецтип для путей
    /// Файл словаря.
    ///
    /// Относительный путь к файлу словаря.
    /// Словарь должен быть в формате plain text, одно слово в строке, кодировка utf-8.
    #[arg(short, long, default_value = "slovar.txt")]
    pub file: String,

    /// Правило поиска (можно задать любое количество правил).
    ///
    /// Правила — это фильтры, которые говорят программе, какие слова следует выбрать из словаря.
    /// Выбираются только те слова, которые соответствуют ВСЕМ заданным правилам.
    ///
    /// Каждое правило должно соответствовать шаблону:
    ///
    /// L[C][P]
    ///
    /// где:
    ///
    /// L — буква
    ///
    /// C — условие; по умолчанию `+`
    ///
    /// Возможны 4 условия:
    ///
    /// `+` буква есть в слове (либо, если позиция указана, то есть на указанной позиции)
    ///
    /// `-` буквы нет в слове (либо, если позиция указана, то нет на указанной позиции)
    ///
    /// `=` буква есть только на указанной позиции и нигде больше
    ///
    /// `*` буква есть на любой позиции, кроме указанной
    ///
    /// N - позиция буквы в слове
    ///
    /// Если позиция не указана, то поиск происходит по всему слову.
    ///
    /// Позиция обязательно должна быть указана для условий `=` и `*`.
    ///
    /// Примеры:
    ///
    /// `а` в слове есть буква `а`
    ///
    /// `а+` то же самое, что предыдущий вариант, но условие задано явно
    ///
    /// `а1` первая буква в слове это буква `а`. При этом в слове могуть быть ещё буквы `а`
    /// на других местах.
    ///
    /// `а+1` то же самое, что предыдущий вариант, но условие задано явно
    ///
    /// `а=1` первая буква в слове это буква `а`. При этом в слове нет других букв `а`.
    ///
    /// `б-` в слове нет буквы `б`
    ///
    /// `б-1` первая буква в слове это не буква `б`. При этом в слове могут быть буквы `б`
    ///  на других местах
    ///
    /// `б*1` первая буква в слове это не буква `б`. При этом в слове есть буква `б`
    /// на каком-то другом месте.
    #[arg(value_name= "RULE", value_parser = rule_parse)]
    pub rules: Vec<Rule>,
}

fn rule_parse(r: &str) -> Result<Rule, String> {
    // Эта регулярка разбирает и одновременно проверяет синтаксис правила
    // Для проверки того, что условия `=` и `*` используются с позицией,
    // применяется заглядывание вперёд (?=), а для него нужен крейт fancy_regex,
    // обычный regex не подходит.
    let re = Regex::new(r"^(\w)([+\-]|[=*](?=\d))?(\d*)$").unwrap();

    // TODO не уверен в этой конструкции, надо бы разобраться точнее
    // map_err тут нужен из-за того, что captures возвращает Err<непонятный_тип>,
    // но rule_parse ожидает возврата Err<String>.
    // map_err нужен для приведения непонятного_типа к понятному String.
    let caps = re.captures(r).map_err(|e| e.to_string())?;

    let caps = if caps.is_some() {
        caps.unwrap()
    } else {
        return Err("правило не соответствует шаблону".to_string());
    };

    let letter = caps.get(1).unwrap().as_str().chars().next().unwrap();
    // Тут сложность в том, что если в правиле не указано условие,
    // то вторая группа в захвате будет пустой (None).
    // Пустой групе нельзя сделать unwrap, но и unwrap_or('+') тоже нельзя сделать,
    // потому что unwrap_or будет возвращать разные типы, Match и char, а так нельзя.
    // Для возврата одного типа — char — нужен map_or.
    let condition = caps
        .get(2)
        .map_or('+', |m| m.as_str().chars().next().unwrap());
    let position = caps[3].parse::<usize>().ok();

    Ok(Rule {
        letter,
        condition,
        position,
    })
}

#[cfg(test)]
mod tests;
