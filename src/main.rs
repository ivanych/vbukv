//! тест maint
#![warn(missing_docs)]

use vbukv::input;
use vbukv::libvbukv;
use vbukv::output;

/// документация к main()
fn main() {
    // Прочитать аргументы командной строки
    let args = input::args::argsparse();

    // Найти предположения
    let assumptions = libvbukv::assumptions(&args.file, args.length, &args.rules);

    // Вывести предположения
    output::term::output(&assumptions).expect("Failed to print output");
}
