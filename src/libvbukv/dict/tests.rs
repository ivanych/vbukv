use super::*;
use std::str::FromStr;

#[test]
fn test_filter_plus() {
    let words = vec![
        String::from("паста"),
        String::from("стопа"),
        String::from("кнопка"),
        String::from("шпонка"),
        String::from("трактор"),
        String::from("масштаб"),
        String::from("концепция"),
        String::from("постройка"),
        String::from("министерство"),
    ];

    let length: usize = 5;

    let rules = vec![
        Rule::from_str("с+").unwrap(),
        Rule::from_str("о+3").unwrap(),
    ];

    let assumptions = Dict::new(words).filter(length, &rules);

    let word1 = "стопа".to_string();

    assert_eq!(
        assumptions.len(),
        1,
        "найдено неправильное количество предположений"
    );
    assert!(assumptions.contains(&word1));
}

#[test]
fn test_filter_minus() {
    let words = vec![
        String::from("паста"),
        String::from("стопа"),
        String::from("кнопка"),
        String::from("шпонка"),
        String::from("трактор"),
        String::from("масштаб"),
        String::from("концепция"),
        String::from("постройка"),
        String::from("министерство"),
    ];

    let length: usize = 6;

    let rules = vec![
        Rule::from_str("с-").unwrap(),
        Rule::from_str("н-4").unwrap(),
    ];

    let assumptions = Dict::new(words).filter(length, &rules);

    let word1 = "кнопка".to_string();

    assert_eq!(
        assumptions.len(),
        1,
        "найдено неправильное количество предположений"
    );
    assert!(assumptions.contains(&word1));
}

#[test]
fn test_filter_equals() {
    let words = vec![
        String::from("паста"),
        String::from("стопа"),
        String::from("кнопка"),
        String::from("шпонка"),
        String::from("трактор"),
        String::from("масштаб"),
        String::from("концепция"),
        String::from("постройка"),
        String::from("министерство"),
    ];

    let length: usize = 7;

    let rules = vec![Rule::from_str("т=5").unwrap()];

    let assumptions = Dict::new(words).filter(length, &rules);

    let word1 = "масштаб".to_string();

    assert_eq!(
        assumptions.len(),
        1,
        "найдено неправильное количество предположений"
    );
    assert!(assumptions.contains(&word1));
}

#[test]
fn test_filter_asterisk() {
    let words = vec![
        String::from("паста"),
        String::from("стопа"),
        String::from("кнопка"),
        String::from("шпонка"),
        String::from("трактор"),
        String::from("масштаб"),
        String::from("концепция"),
        String::from("постройка"),
        String::from("министерство"),
    ];

    let length: usize = 9;

    let rules = vec![Rule::from_str("к*8").unwrap()];

    let assumptions = Dict::new(words).filter(length, &rules);

    let word1 = "концепция".to_string();

    assert_eq!(
        assumptions.len(),
        1,
        "найдено неправильное количество предположений"
    );
    assert!(assumptions.contains(&word1));
}
