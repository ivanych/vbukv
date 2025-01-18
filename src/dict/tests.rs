use super::*;

#[test]
fn find_letter_test_true1() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'т',
        condition: true,
        position: None,
    };

    let is_find = find_letter(&word, &rule);

    assert!(
        is_find,
        "В слове '{}' не найдена буква {} на позиции {:?}",
        word, rule.letter, rule.position,
    )
}

#[test]
fn find_letter_test_true2() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'п',
        condition: true,
        position: Some(1),
    };

    let is_find = find_letter(&word, &rule);

    assert!(
        is_find,
        "В слове '{}' не найдена буква {} на позиции {:?}",
        word, rule.letter, rule.position,
    )
}

#[test]
#[should_panic]
fn find_letter_test_false1() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'б',
        condition: true,
        position: None,
    };

    let is_find = find_letter(&word, &rule);

    assert!(
        is_find,
        "В слове '{}' не найдена буква {} на позиции {:?}",
        word, rule.letter, rule.position,
    )
}

#[test]
#[should_panic]
fn find_letter_test_false2() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'б',
        condition: true,
        position: Some(1),
    };

    let is_find = find_letter(&word, &rule);

    assert!(
        is_find,
        "В слове '{}' не найдена буква {} на позиции {:?}",
        word, rule.letter, rule.position,
    )
}

#[test]
fn position_symbol_test_true() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'а',
        condition: false,
        position: Some(2),
    };

    let ps = position_symbol(&word, &rule.position);

    assert_eq!(
        ps,
        rule.letter,
        "В слове '{}' неправильно определён символ на позиции {}",
        word,
        rule.position.unwrap(),
    )
}

#[test]
#[should_panic]
fn position_symbol_test_false() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: 'а',
        condition: false,
        position: Some(3),
    };

    let ps = position_symbol(&word, &rule.position);

    assert_eq!(
        ps,
        rule.letter,
        "В слове '{}' неправильно определён символ на позиции {}",
        word,
        rule.position.unwrap(),
    )
}
