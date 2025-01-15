use std::process::exit;
use crate::args::Args;

#[test]
fn build_test_succes() {
    let args = vec!["cmd".to_string(), "slovar.txt".to_string()];

    let args = Args::build(&args).unwrap_or_else(|err| {
        println!("Возникла ошибка при разборе аргументов: {err}");
        exit(1)
    });

    assert_eq!(args.file, "slovar.txt")
}

#[test]
#[should_panic]
fn build_test_fail() {
    let args = vec!["cmd".to_string()];

    let args = Args::build(&args).unwrap_or_else(|err| {
        println!("Возникла ошибка при разборе аргументов: {err}");
        exit(1)
    });

    assert_eq!(args.file, "slovar.txt")
}

