use crate::rule::Rule;

#[derive(Debug)]
pub struct Args {
    pub length: usize,
    pub rules: Vec<Rule>,
    pub file: String,
}

impl Args {
    pub fn build(args: &Vec<String>) -> Result<Args, &'static str> {
        // Проверки
        // Нужен как минимум один аргумент (помимо названия самой программы) — файл словаря
        if args.len() < 2 {
            return Err("не указан файл словаря");
        }

        // Длина искомого слова
        let length: usize = 5;

        // Правила
        let rules = vec![
            Rule {
                letter: 'п',
                condition: false,
                position: None,
            },
            Rule {
                letter: 'а',
                condition: true,
                position: Some(1),
            },
            Rule {
                letter: 'к',
                condition: true,
                position: Some(3),
            },
        ];

        Ok(Args {
            length,
            rules,
            file: args[1].clone(),
        })
    }
}

#[cfg(test)]
mod tests;
