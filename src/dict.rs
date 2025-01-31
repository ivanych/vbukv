use crate::rule::{Cond, Rule};

pub fn filter(words: Vec<String>, length: usize, rules: &Vec<Rule>) -> Vec<String> {
    words
        .into_iter() // почему тут into_iter? надо разобраться...
        // Берём только нужные слова
        .filter(|word| {
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
                    Cond::Minus => is_absent(word, rule),
                    Cond::Equals => is_inner(word, rule),
                    Cond::Asterisk => is_outer(word, rule),
                })
        })
        .collect()
}

fn is_present(word: &String, letter: char, position: &Option<usize>) -> bool {
    match position {
        None => word.contains(letter),
        Some(_) => position_symbol(word, position) == letter,
    }
}

fn is_absent(word: &String, rule: &Rule) -> bool {
    !match rule.position {
        None => word.contains(rule.letter),
        Some(_) => position_symbol(word, &rule.position) == rule.letter,
    }
}

// TODO тут не нужно правило целиком, надо принимать только букву и позицию
fn is_inner(word: &String, rule: &Rule) -> bool {
    // буква должно быть на указанном месте
    position_symbol(word, &rule.position) == rule.letter
        // и буквы не должно быть где-то в другом месте
        && !word_without_position(word, &rule.position).contains(rule.letter)
}

fn is_outer(word: &String, rule: &Rule) -> bool {
    // буквы не должно быть на указанном месте
    position_symbol(word, &rule.position) != rule.letter
        // и буква должна быть где-то в другом месте
        && word_without_position(word, &rule.position).contains(rule.letter)
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

#[cfg(test)]
mod tests;
