use std::env;

#[derive(Debug)]
pub struct Args {
    pub file: String,
}

impl Args {
    pub fn build() -> Result<Args, &'static str> {
        // Прочитать все аргументы в вектор
        let args: Vec<String> = env::args().collect();
        //dbg!(&args);

        // Проверки
        // Нужен как минимум один аргумент (помимо названия самой программы) — файл словаря
        if args.len() < 2 {
            return Err("не указан файл словаря");
        }

        Ok(Args {
            file: args[1].clone(),
        })
    }
}
