use super::*;

// TODO Надо разобраться — почему для вызова Rule::from_str() нужно импортировать FromStr?
// Вот для вызова PathBuf::from() почему-то не нужно импортировать From, как-то без него работает...
use std::str::FromStr;

#[test]
pub fn test_assumptions() {
    let file = PathBuf::from("test_slovar.txt");
    dbg!(&file);

    let length = 5;
    dbg!(&length);

    let rules = vec![
        Rule::from_str("с+").unwrap(),
        Rule::from_str("о+3").unwrap(),
    ];
    dbg!(&rules);

    // Предположения
    let assumptions = assumptions(&file, length, &rules);
    dbg!(&assumptions);

    let word1 = "стопа".to_string();

    assert_eq!(
        assumptions.len(),
        1,
        "найдено неправильное количество предположений"
    );
    assert!(assumptions.contains(&word1));
}
