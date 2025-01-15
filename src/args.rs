
#[derive(Debug)]
pub struct Args {
    pub file: String,
}

impl Args {
    pub fn build(args: &Vec<String>) -> Result<Args, &'static str> {

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
