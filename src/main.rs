fn main() {
    // Прочитать аргументы командной строки
    let args = vbukv::args::argsparse();
    //dbg!(&args);

    // Найти предположения
    let assumptions = vbukv::assumptions(&args);

    // Вывести предположения
    println!("Предположения:");
    println!("---------------------------------");

    for assumption in &assumptions {
        println!("{}", assumption);
    }

    println!("---------------------------------");
    println!("Найдено предположений: {}", assumptions.len());
}
