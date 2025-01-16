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

    Args::build(&args).unwrap();
}

#[test]
fn build_test_ok() -> Result<(), String> {
    let args = vec!["cmd".to_string(), "slovar.txt".to_string()];

    let args = Args::build(&args)?;

    assert_eq!(args.file, "slovar.txt");

    Ok(())
}

#[test]
fn build_test_err() -> Result<(), String> {
    let args = vec!["cmd".to_string()];

    assert!(Args::build(&args).is_err());

    Ok(())
}
