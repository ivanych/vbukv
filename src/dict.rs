pub fn filter(words: Vec<String>) -> Vec<String> {
    // Правила
    let rules = [
        (String::from("п"), String::from("1")),
        (String::from("а"), String::from("*")),
        (String::from("А"), String::from("2")),
        (String::from("С"), String::from("*")),
        (String::from("Т"), String::from("*")),
        (String::from("А"), String::from("5")),
        (String::from("о"), String::from("2")),
        (String::from("В"), String::from("*")),
        (String::from("а"), String::from("4")),
        (String::from("Р"), String::from("*")),

    ];
    dbg!(&rules);

    let dict_iter = words.into_iter();

    let ass = dict_iter
        .clone()
        .filter(|w| {
            //dbg!(&w);

            // Длина
            if w.len() != 10 {
                return false;
            }

            rules.iter().all(|rule| {
                //dbg!(rule);

                let (letter, position) = rule;
                let position = position.parse::<usize>().ok();
                dbg!(letter, position);

                // любое место
                if position.is_none() {
                    let is_contain =
                        w.contains(letter.to_string().as_str().to_lowercase().as_str());

                    if letter.chars().all(char::is_lowercase) {
                        is_contain
                    } else {
                        !is_contain
                    }
                }
                //  указанное место
                else if position.is_some() {
                    let end = position.unwrap() * 2;
                    let start = end - 2;
                    //dbg!(start, end);

                    let symbol = w.get(start..end);
                    //dbg!(symbol);

                    let is_match =
                        symbol == Some(letter.to_string().as_str().to_lowercase().as_str());

                    if letter.chars().all(char::is_lowercase) {
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
