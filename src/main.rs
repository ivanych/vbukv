use vbukv::input;
use vbukv::output;

fn main() {
    // Прочитать аргументы командной строки
    let args = input::args::argsparse();

    // Найти предположения
    let assumptions = vbukv::assumptions(&args);

    // Вывести предположения
    output::term::output(&assumptions).expect("Failed to print output");
}
