#[cfg(test)]
mod tests;

use crate::rule::Rule;

pub fn filter(words: Vec<String>, length: usize, rules: Vec<Rule>) -> Vec<String> {
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
                .all(|rule| {
                    let is_find = find_letter(word, rule);

                    if rule.condition {
                        is_find
                    } else {
                        !is_find
                    }
                })
        })
        .collect()
}

fn find_letter(word: &String, rule: &Rule) -> bool {
    match rule.position {
        None => word.contains(rule.letter),
        Some(_) => position_symbol(word, &rule.position) == rule.letter,
    }
}

fn position_symbol(word: &String, position: &Option<usize>) -> char {
    let index = position.unwrap() - 1;

    word.chars().nth(index).unwrap()
}
