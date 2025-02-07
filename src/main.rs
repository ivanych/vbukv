use std::process::exit;
// TODO непонятно, зачем здесь импорт clap-а, но без него не работает. Надо разобраться.
use clap::Parser;
use clap_markdown;
use vbukv;
use vbukv::args::Args;
use vbukv::term;

fn main() {
    // TODO Эту конструкцию надо бы засунуть в функцию vbukv::args,
    // чтобы все вызовы здесь были из библиотеки vbukv, для единообразия.
    // Но в библиотеке эту функцию нужно тестировать, а я пока не знаю,
    // как протестировать функцию, которая читает аргументы командной строки.
    let args = Args::parse();
    //dbg!(&args);

    if args.markdown_help {
        clap_markdown::print_help_markdown::<Args>();
        exit(0);
    }

    let assumptions = vbukv::assumptions(&args);

    // Вывести предположения
    term::output(&assumptions).expect("Failed to print output");
}
