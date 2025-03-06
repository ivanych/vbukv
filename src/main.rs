#![warn(missing_docs)]
#![doc = include_str!("../README.md")]


use vbukv::input;
use vbukv::libvbukv;
use vbukv::output;

#[doc(hidden)]
fn main() {
    // Прочитать аргументы командной строки
    let args = input::args::argsparse();

    // Найти предположения
    let assumptions = libvbukv::assumptions(&args.file, args.length, &args.rules);

    // Вывести предположения
    output::term::output(&assumptions).expect("Failed to print output");
}
