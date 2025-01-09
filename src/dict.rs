pub fn filter(words: Vec<String>) -> Vec<String> {
    // Правила
    let rules = [
        (String::from("п"), String::from("+"), String::from("1")),
        (String::from("а"), String::from("!"), String::from("*")),
        (String::from("с"), String::from("+"), String::from("*")),
        (String::from("с"), String::from("!"), String::from("3")),
        (String::from("т"), String::from("!"), String::from("*")),
        (String::from("е"), String::from("!"), String::from("*")),
        (String::from("н"), String::from("!"), String::from("*")),
        (String::from("я"), String::from("!"), String::from("*")),
        (String::from("о"), String::from("+"), String::from("2")),
        (String::from("и"), String::from("+"), String::from("+")),
        (String::from("и"), String::from("!"), String::from("3")),
        (String::from("с"), String::from("!"), String::from("4")),
    ];
    dbg!(&rules);

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
                let position = position.parse::<usize>().ok();
                dbg!(letter, condition, position);

                let letter_lc = letter.to_string().as_str().to_lowercase();

                let letter_pattern = letter_lc.as_str();

                // любое место
                if position.is_none() {
                    let is_contain = word.contains(letter_pattern);

                    if condition.to_string() == "+".to_string() {
                        is_contain
                    } else {
                        !is_contain
                    }
                }
                //  указанное место
                else if position.is_some() {
                    let symbol = symbol(word, position);
                    //dbg!(symbol);

                    let is_match = symbol == letter_pattern;

                    if condition.to_string() == "+".to_string() {
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

fn symbol(word: &String, position: Option<usize>) -> &str {
    let end = position.unwrap() * 2;
    let start = end - 2;
    //dbg!(start, end);

    let symbol = word.get(start..end).unwrap();

    symbol
}
