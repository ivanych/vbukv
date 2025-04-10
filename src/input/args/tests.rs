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
<<<<<<< HEAD
#[test]
fn test_validate_rule_position() {
    let args = Args::parse_from(["vbukv,", "-l", "5", "-f", "test_slovar.txt", "c+6"]);

    args.validate_rule_position();
}
=======
>>>>>>> b675c884be70ec65ea4a30d2c05911d078e9911f
