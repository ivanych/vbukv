use super::*;

#[test]
fn test_parse_from_normal() {
    let args = parse_from(["vbukv,", "-l", "5", "-f", "test_slovar.txt", "c+"]);

    assert_eq!(args.length, 5);
    assert_eq!(args.file.into_os_string(), "test_slovar.txt");
    assert_eq!(args.rules.len(), 1);
    //assert_eq!(args.rules.iter().nth(0).unwrap().letter, 'c');
}

// TODO тут нужен тест parse_from с опцией --markdown-help,
// но для этого нужен мок функции exit в parse_from

#[test]
fn test_validate_rule_position() -> Result<(), String> {
    let args = parse_from(["vbukv,", "-l", "5", "-f", "test_slovar.txt", "c+5"]);

    Ok(args.validate_rule_position()?)
}

#[test]
fn test_fail_validate_rule_position() -> Result<(), String> {
    let args = parse_from(["vbukv,", "-l", "5", "-f", "test_slovar.txt", "c+6"]);

    assert!(args.validate_rule_position().is_err());

    Ok(())
}
