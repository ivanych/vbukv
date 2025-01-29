// TODO непонятно, зачем здесь импорт clap-а, но без него не работает. Надо разобраться.
use clap::Parser;

use vbukv;
use vbukv::args::Args;

fn main() {
    // TODO Эту конструкцию надо бы засунуть в функцию vbukv::args,
    // чтобы все вызовы здесь были из библиотеки vbukv, для единообразия.
    // Но в библиотеке эту функцию нужно тестировать, а я пока не знаю,
    // как протестировать функцию, которая читает аргументы командной строки.
    let args = Args::parse();
    //dbg!(&args);

    let assumptions = vbukv::assumptions(&args);

    println!("Предположения:");
    println!("---------------------------------");

    for assumption in &assumptions {
        println!("{}", assumption);
    }

    println!("---------------------------------");
    println!("Найдено предположений: {}", assumptions.len());
}
