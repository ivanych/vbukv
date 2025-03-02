#[cfg(test)]
mod tests;

use crate::libvbukv::rule::{Cond, Rule};

#[derive(Debug)]
pub struct Dict {
    words: Vec<String>,
}

impl Dict {
    pub fn new(words: Vec<String>) -> Dict {
        Dict { words }
    }

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
