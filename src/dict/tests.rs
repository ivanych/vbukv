use super::*;
use crate::dict;

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
            condition: '+',
            position: None,
        },
        Rule {
            letter: 'о',
            condition: '+',
            position: Some(3),
        },
    ];

    let assumptions = dict::filter(words, length, rules);

    println!("{:?}", assumptions);

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
            condition: '-',
            position: None,
        },
        Rule {
            letter: 'н',
            condition: '-',
            position: Some(4),
        },
    ];

    let assumptions = dict::filter(words, length, rules);

    println!("{:?}", assumptions);

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
        condition: '=',
        position: Some(5),
    }];

    let assumptions = dict::filter(words, length, rules);

    println!("{:?}", assumptions);

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
        condition: '*',
        position: Some(8),
    }];

    let assumptions = dict::filter(words, length, rules);

    println!("{:?}", assumptions);

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

    let rule = Rule {
        letter: 'т',
        condition: '+',
        position: None,
    };
    let is_true = is_present(&word, &rule);
    assert!(is_true);

    let rule = Rule {
        letter: 'т',
        condition: '+',
        position: Some(4),
    };
    let is_true = is_present(&word, &rule);
    assert!(is_true);
}

#[test]
fn test_is_present_false() {
    let word = "паста".to_string();

    let rule = Rule {
        letter: 'г',
        condition: '+',
        position: None,
    };
    let is_true = is_present(&word, &rule);
    assert!(!is_true);

    let rule = Rule {
        letter: 'т',
        condition: '+',
        position: Some(3),
    };
    let is_true = is_present(&word, &rule);
    assert!(!is_true);
}

#[test]
fn test_is_absent_true() {
    let word = "паста".to_string();

    let rule = Rule {
        letter: 'г',
        condition: '-',
        position: None,
    };
    let is_true = is_absent(&word, &rule);
    assert!(is_true);

    let rule = Rule {
        letter: 'т',
        condition: '-',
        position: Some(5),
    };
    let is_true = is_absent(&word, &rule);
    assert!(is_true);
}

#[test]
fn test_is_absent_false() {
    let word = "паста".to_string();

    let rule = Rule {
        letter: 'т',
        condition: '-',
        position: None,
    };
    let is_true = is_absent(&word, &rule);
    assert!(!is_true);

    let rule = Rule {
        letter: 'т',
        condition: '-',
        position: Some(4),
    };
    let is_true = is_absent(&word, &rule);
    assert!(!is_true);
}

#[test]
fn test_is_inner_true() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'т',
        condition: '=',
        position: Some(4),
    };

    let is_inner = is_inner(&word, &rule);

    assert!(is_inner);
}

#[test]
fn test_is_inner_false() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'а',
        condition: '=',
        position: Some(5),
    };

    let is_inner = is_inner(&word, &rule);

    assert!(!is_inner);
}

#[test]
fn test_is_inner_false_2() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'г',
        condition: '=',
        position: Some(1),
    };

    let is_inner = is_inner(&word, &rule);

    assert!(!is_inner);
}

#[test]
fn test_is_outer_true() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'т',
        condition: '*',
        position: Some(5),
    };

    let is_outer = is_outer(&word, &rule);

    assert!(is_outer);
}

#[test]
fn test_is_outer_false() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'а',
        condition: '*',
        position: Some(5),
    };

    let is_outer = crate::dict::is_outer(&word, &rule);

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
