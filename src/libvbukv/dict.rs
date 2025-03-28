#[cfg(test)]
mod tests;

use crate::libvbukv::rule::Rule;

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
                    .all(|rule| rule.check_word(word))
            })
            // разыменовываем ссылки на слова
            .cloned()
            .collect()
    }
}
