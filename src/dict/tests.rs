use super::*;
use crate::rule::Cond;

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
        Rule {
            letter: 'с',
            condition: Cond::Plus,
            position: None,
        },
        Rule {
            letter: 'о',
            condition: Cond::Plus,
            position: Some(3),
        },
    ];

    let assumptions = filter(words, length, &rules);

    //dbg!(&assumptions);

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
        Rule {
            letter: 'с',
            condition: Cond::Minus,
            position: None,
        },
        Rule {
            letter: 'н',
            condition: Cond::Minus,
            position: Some(4),
        },
    ];

    let assumptions = filter(words, length, &rules);

    //dbg!(&assumptions);

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

    let rules = vec![Rule {
        letter: 'т',
        condition: Cond::Equals,
        position: Some(5),
    }];

    let assumptions = filter(words, length, &rules);

    //dbg!(&assumptions);

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

    let rules = vec![Rule {
        letter: 'к',
        condition: Cond::Asterisk,
        position: Some(8),
    }];

    let assumptions = filter(words, length, &rules);

    //dbg!(&assumptions);

    let word1 = "концепция".to_string();

    assert_eq!(
        assumptions.len(),
        1,
        "найдено неправильное количество предположений"
    );
    assert!(assumptions.contains(&word1));
}

#[test]
fn test_is_present_true() {
    let word = "паста".to_string();

    let is_true = is_present(&word, 'т', &None);
    assert!(is_true);

    let is_true = is_present(&word, 'т', &Some(4));
    assert!(is_true);
}

#[test]
fn test_is_present_false() {
    let word = "паста".to_string();

    let is_true = is_present(&word, 'г', &None);
    assert!(!is_true);

    let is_true = is_present(&word, 'т', &Some(3));
    assert!(!is_true);
}

#[test]
fn test_is_absent_true() {
    let word = "паста".to_string();

    let is_true = is_absent(&word, 'г', &None);
    assert!(is_true);

    let is_true = is_absent(&word, 'т', &Some(5));
    assert!(is_true);
}

#[test]
fn test_is_absent_false() {
    let word = "паста".to_string();

    let is_true = is_absent(&word, 'т', &None);
    assert!(!is_true);

    let is_true = is_absent(&word, 'т', &Some(4));
    assert!(!is_true);
}

#[test]
fn test_is_inner_true() {
    let word = "паста".to_string();

    let is_inner = is_inner(&word, 'т', &Some(4));
    assert!(is_inner);
}

#[test]
fn test_is_inner_false() {
    let word = "паста".to_string();

    let is_inner = is_inner(&word, 'а', &Some(5));
    assert!(!is_inner);
}

#[test]
fn test_is_inner_false_2() {
    let word = "паста".to_string();

    let is_inner = is_inner(&word, 'г', &Some(1));
    assert!(!is_inner);
}

#[test]
fn test_is_outer_true() {
    let word = "паста".to_string();

    let is_outer = is_outer(&word, 'т', &Some(5));
    assert!(is_outer);
}

#[test]
fn test_is_outer_false() {
    let word = "паста".to_string();

    let is_outer = is_outer(&word, 'а', &Some(5));
    assert!(!is_outer);
}

#[test]
fn test_position_symbol() {
    let word = "паста".to_string();

    let symbol = position_symbol(&word, &Some(4));

    assert_eq!(symbol, 'т');
}

#[test]
fn test_word_without_position() {
    let word = "паста".to_string();

    let word = word_without_position(&word, &Some(4));

    assert_eq!(word, "паса".to_string());
}
