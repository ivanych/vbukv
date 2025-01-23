use fancy_regex::Regex;

#[derive(Debug, Clone)]
pub struct Rule {
    pub letter: char,
    pub condition: char,
    pub position: Option<usize>,
}

impl Rule {
    pub fn build(r: &str) -> Result<Rule, String> {
        // Эта регулярка разбирает и одновременно проверяет синтаксис правила
        // Для проверки того, что условия `=` и `*` используются с позицией,
        // применяется заглядывание вперёд (?=), а для него нужен крейт fancy_regex,
        // обычный regex не подходит.
        let re = Regex::new(r"^(\w)([+\-]|[=*](?=\d))?(\d*)$").unwrap();

        // TODO не уверен в этой конструкции, надо бы разобраться точнее
        // map_err тут нужен из-за того, что captures возвращает Err<непонятный_тип>,
        // но rule_parse ожидает возврата Err<String>.
        // map_err нужен для приведения непонятного_типа к понятному String.
        // TODO Непонятно, как воспроизвести ошибку, которая может тут возникнуть.
        // Вроде сравнение с регуляркой должно работать всегда, успешного захвата может и не быть,
        // но ошибка всё-равно вроде не должна возникать...
        // Из-за этого строчка остаётся не покрытой тестами.
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
}

#[cfg(test)]
mod tests;
