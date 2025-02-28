use vbukv::args;
use vbukv::output;

fn main() {
    // Прочитать аргументы командной строки
    let args = args::argsparse();
    //dbg!(&args);

    // Найти предположения
    let assumptions = vbukv::assumptions(&args);

    // Вывести предположения
    output::term::output(&assumptions).expect("Failed to print output");
}
