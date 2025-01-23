use super::*;

#[test]
fn build_test_success() -> Result<(), String> {
    let rule_cli = "a-";

    let rule = Rule::build(&rule_cli)?;

    assert_eq!(rule.letter, 'a');
    assert_eq!(rule.condition, '-');
    assert_eq!(rule.position, None);

    Ok(())
}

#[test]
fn build_test_condition() -> Result<(), String> {
    let rule_cli = "a";

    let rule = Rule::build(&rule_cli)?;

    assert_eq!(rule.condition, '+');

    Ok(())
}

#[test]
fn build_test_fail() -> Result<(), String> {
    let rule_cli = "ab";

    assert!(Rule::build(&rule_cli).is_err());

    Ok(())
}
