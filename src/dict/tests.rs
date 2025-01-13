use super::*;

#[test]
fn position_symbol_test() {
    let word = "паста".to_string();
    let rule = Rule {
        letter: String::from("а"),
        condition: String::from("!"),
        position: Some(2),
    };

    let ps = position_symbol(&word, &rule.position);

    assert_eq!(
        ps,
        rule.letter_lc(),
        "В слове 'слово' неправильно определён символ на позиции {}",
        rule.position.unwrap()
    )
}
