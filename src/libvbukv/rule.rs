#[cfg(test)]
mod tests;

use std::str::FromStr;

use fancy_regex::Regex;

#[derive(Debug, Clone)]
pub enum Cond {
    Plus,
    Minus,
    Equals,
    Asterisk,
}

impl FromStr for Cond {
    type Err = ();

    fn from_str(s: &str) -> Result<Cond, Self::Err> {
        match s {
            "+" => Ok(Cond::Plus),
            "-" => Ok(Cond::Minus),
            "=" => Ok(Cond::Equals),
            "*" => Ok(Cond::Asterisk),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub letter: char,
    pub condition: Cond,
    pub position: Option<usize>,
}

impl Rule {
    pub fn check_word(&self, word: &String) -> bool {
        match self.condition {
            Cond::Plus => self.is_present(word),
            Cond::Minus => self.is_absent(word),
            Cond::Equals => self.is_inner(word),
            Cond::Asterisk => self.is_outer(word),
        }
    }

    fn is_present(&self, word: &String) -> bool {
        match &self.position {
            None => word.contains(self.letter),
            Some(_) => position_symbol(word, &self.position) == self.letter,
        }
    }

    fn is_absent(&self, word: &String) -> bool {
        !match &self.position {
            None => word.contains(self.letter),
            Some(_) => position_symbol(word, &self.position) == self.letter,
        }
    }

    fn is_inner(&self, word: &String) -> bool {
        // буква должно быть на указанном месте
        position_symbol(word, &self.position) == self.letter
            // и буквы не должно быть где-то в другом месте
            && !word_without_position(word, &self.position).contains(self.letter)
    }

    fn is_outer(&self, word: &String) -> bool {
        // буквы не должно быть на указанном месте
        position_symbol(word, &self.position) != self.letter
            // и буква должна быть где-то в другом месте
            && word_without_position(word, &self.position).contains(self.letter)
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Rule, Self::Err> {
        // Эта регулярка разбирает и одновременно проверяет синтаксис правила
        // Для проверки того, что условия `=` и `*` используются с позицией,
        // применяется заглядывание вперёд (?=), а для него нужен крейт fancy_regex,
        // обычный regex не подходит.
        let re = Regex::new(r"^(\w)([+\-]|[=*](?=\d))?(\d*)$").unwrap();

        // TODO Непонятно, как воспроизвести ошибку, которая может тут возникнуть.
        // Вроде сравнение с регуляркой должно работать всегда, успешного захвата может и не быть,
        // но ошибка всё-равно вроде не должна возникать...
        // Из-за этого строчка остаётся не покрытой тестами.
        let caps = re.captures(s).map_err(|e| e.to_string())?;

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
            .map_or(Cond::Plus, |m| Cond::from_str(m.as_str()).unwrap());

        let position = caps[3].parse::<usize>().ok();

        Ok(Rule {
            letter,
            condition,
            position,
        })
    }
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
