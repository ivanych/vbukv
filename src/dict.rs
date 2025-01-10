pub fn filter(words: Vec<String>) -> Vec<String> {
    // Правила
    let rules = [
        (String::from("п"), String::from("+"), Some(1)),
        (String::from("а"), String::from("!"), None),
        (String::from("с"), String::from("!"), Some(3)),
        (String::from("т"), String::from("!"), None),
        (String::from("о"), String::from("+"), Some(2)),
        (String::from("и"), String::from("+"), None),
        (String::from("и"), String::from("!"), Some(3)),
    ];
    //dbg!(&rules);

    // Условия
    let presence = String::from("+");

    let dict_iter = words.into_iter();

    let ass = dict_iter
        .clone()
        .filter(|word| {
            //dbg!(&w);

            // Длина
            if word.len() != 10 {
                return false;
            }

            rules.iter().all(|rule| {
                //dbg!(rule);

                let (letter, condition, position) = rule;
                //dbg!(letter, condition, position);

                let letter_lc = letter.to_string().as_str().to_lowercase();

                // любое место
                if position.is_none() {
                    let is_contain = word.contains(letter_lc.as_str());

                    if condition.to_string() == presence {
                        is_contain
                    } else {
                        !is_contain
                    }
                }
                //  указанное место
                else if position.is_some() {
                    let is_match = position_symbol(word, position) == letter_lc;

                    if condition.to_string() == presence {
                        is_match
                    } else {
                        !is_match
                    }
                }
                // все прочие случаи
                else {
                    false
                }
            })
        })
        .collect::<Vec<_>>();

    ass
}

fn position_symbol(word: &String, position: &Option<i32>) -> String {
    let end = position.unwrap() as usize * 2;
    let start = end - 2;
    //dbg!(start, end);

    let symbol = word.get(start..end).unwrap().to_string();

    symbol
}
