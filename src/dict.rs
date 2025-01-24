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
                .all(|rule| match rule.condition.to_string().as_str() {
                    "+" => is_present(word, rule),
                    "-" => is_absent(word, rule),
                    "=" => is_inner(word, rule),
                    "*" => is_outer(word, rule),
                    _ => false,
                })
        })
        .collect()
}

// TODO тут не нужно правило целиком, надо принимать только букву и позицию
fn is_present(word: &String, rule: &Rule) -> bool {
    match rule.position {
        None => word.contains(rule.letter),
        Some(_) => position_symbol(word, &rule.position) == rule.letter,
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
