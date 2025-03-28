use super::*;

#[test]
fn build_test_success() -> Result<(), String> {
    let rule_cli = "a-";

    let rule = Rule::from_str(&rule_cli)?;

    assert_eq!(rule.letter, 'a');
    assert!(match rule.condition {
        Cond::Minus => true,
        _ => false,
    });
    assert_eq!(rule.position, None);

    Ok(())
}

#[test]
fn build_test_default_condition() -> Result<(), String> {
    let rule_cli = "a";

    let rule = Rule::from_str(&rule_cli)?;

    assert!(match rule.condition {
        Cond::Plus => true,
        _ => false,
    });

    Ok(())
}

#[test]
fn build_test_condition_plus() -> Result<(), String> {
    let rule_cli = "a+";

    let rule = Rule::from_str(&rule_cli)?;

    assert!(match rule.condition {
        Cond::Plus => true,
        _ => false,
    });

    Ok(())
}

#[test]
fn build_test_condition_equals() -> Result<(), String> {
    let rule_cli = "a=4";

    let rule = Rule::from_str(&rule_cli)?;

    assert!(match rule.condition {
        Cond::Equals => true,
        _ => false,
    });

    Ok(())
}

#[test]
fn build_test_condition_asterisk() -> Result<(), String> {
    let rule_cli = "a*4";

    let rule = Rule::from_str(&rule_cli)?;

    assert!(match rule.condition {
        Cond::Asterisk => true,
        _ => false,
    });

    Ok(())
}

#[test]
fn build_test_failed() -> Result<(), String> {
    let rule_cli = "a^";

    let rule = Rule::from_str(&rule_cli);

    assert!(rule.is_err());

    Ok(())
}

#[test]
fn struct_cond_from_str_test_failed() -> Result<(), String> {
    let cond = Cond::from_str("^");

    assert!(cond.is_err());

    Ok(())
}

#[test]
fn test_is_present_true() {
    let word = "паста".to_string();

    assert!(Rule::from_str("т").unwrap().is_present(&word));
    assert!(Rule::from_str("т+4").unwrap().is_present(&word));
}

#[test]
fn test_is_present_false() {
    let word = "паста".to_string();

    assert!(!Rule::from_str("г").unwrap().is_present(&word));
    assert!(!Rule::from_str("т+3").unwrap().is_present(&word));
}

#[test]
fn test_is_absent_true() {
    let word = "паста".to_string();

    assert!(Rule::from_str("г").unwrap().is_absent(&word));
    assert!(Rule::from_str("т-5").unwrap().is_absent(&word));
}

#[test]
fn test_is_absent_false() {
    let word = "паста".to_string();

    assert!(!Rule::from_str("т").unwrap().is_absent(&word));
    assert!(!Rule::from_str("т-4").unwrap().is_absent(&word));
}

#[test]
fn test_is_inner_true() {
    let word = "паста".to_string();

    assert!(Rule::from_str("т=4").unwrap().is_inner(&word));
}

#[test]
fn test_is_inner_false() {
    let word = "паста".to_string();

    assert!(!Rule::from_str("а=2").unwrap().is_inner(&word));
}

#[test]
fn test_is_inner_false_2() {
    let word = "паста".to_string();

    assert!(!Rule::from_str("г=1").unwrap().is_inner(&word));
}

#[test]
fn test_is_outer_true() {
    let word = "паста".to_string();

    assert!(Rule::from_str("т*5").unwrap().is_outer(&word));
}

#[test]
fn test_is_outer_false() {
    let word = "паста".to_string();

    assert!(!Rule::from_str("а*5").unwrap().is_outer(&word));
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
