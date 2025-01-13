use crate::rule::Rule;

const PRESENCE: &str = "+";

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

                    if rule.condition.as_str() == PRESENCE {
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
        None => word.contains(rule.letter_lc().as_str()),
        Some(_) => position_symbol(word, &rule.position) == rule.letter_lc(),
    }
}

fn position_symbol(word: &String, position: &Option<usize>) -> String {
    let index = position.unwrap() - 1;

    word.chars().nth(index).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_symbol_test() {
        let word = "паста".to_string();
        let rule = Rule {
            letter: String::from("а"),
            condition: String::from("!"),
            position: Some(2),
        };

        let ps = position_symbol(&word, &rule.position);

        assert_eq!(
            ps,
            rule.letter_lc(),
            "В слове 'слово' неправильно определён символ на позиции {}",
            rule.position.unwrap()
        )
    }

}
