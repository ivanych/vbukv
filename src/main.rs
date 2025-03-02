use vbukv::args;
use vbukv::libvbukv;
use vbukv::output;

fn main() {
    // Прочитать аргументы командной строки
    let args = args::argsparse();

    //dbg!(&args);

    // Найти предположения
    let assumptions = libvbukv::assumptions(&args);

    // Вывести предположения
    output::term::output(&assumptions).expect("Failed to print output");
}
