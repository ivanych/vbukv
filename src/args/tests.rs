use super::*;

#[test]
fn test_args() {
    let args = Args::parse_from(["vbukv,", "-l", "5", "-f", "test_slovar.txt", "c+"]);

    assert_eq!(args.length, 5);
    assert_eq!(args.file.into_os_string(), "test_slovar.txt");
    assert_eq!(args.rules.len(), 1);
    assert_eq!(args.rules.iter().nth(0).unwrap().letter, 'c');
}
