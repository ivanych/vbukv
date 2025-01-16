use crate::args::Args;

#[test]
fn build_test_success() {
    let args = vec!["cmd".to_string(), "slovar.txt".to_string()];

    let args = Args::build(&args).unwrap();

    assert_eq!(args.file, "slovar.txt")
}

#[test]
#[should_panic]
fn build_test_fail() {
    let args = vec!["cmd".to_string()];

    let args = Args::build(&args).unwrap();

    assert_eq!(args.file, "slovar.txt")
}
