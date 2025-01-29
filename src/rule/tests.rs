use super::*;

#[test]
fn build_test_success() -> Result<(), String> {
    let rule_cli = "a-";

    let rule = Rule::build(&rule_cli)?;

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

    let rule = Rule::build(&rule_cli)?;

    assert!(match rule.condition {
        Cond::Plus => true,
        _ => false,
    });

    Ok(())
}

#[test]
fn build_test_condition_plus() -> Result<(), String> {
    let rule_cli = "a+";

    let rule = Rule::build(&rule_cli)?;

    assert!(match rule.condition {
        Cond::Plus => true,
        _ => false,
    });

    Ok(())
}

#[test]
fn build_test_condition_equals() -> Result<(), String> {
    let rule_cli = "a=4";

    let rule = Rule::build(&rule_cli)?;

    assert!(match rule.condition {
        Cond::Equals => true,
        _ => false,
    });

    Ok(())
}

#[test]
fn build_test_condition_asterisk() -> Result<(), String> {
    let rule_cli = "a*4";

    let rule = Rule::build(&rule_cli)?;

    assert!(match rule.condition {
        Cond::Asterisk => true,
        _ => false,
    });

    Ok(())
}

#[test]
fn build_test_failed() -> Result<(), String> {
    let rule_cli = "a^";

    let rule = Rule::build(&rule_cli);

    assert!(rule.is_err());

    Ok(())
}

#[test]
fn struct_cond_from_str_test_failed() -> Result<(), String> {
    let cond = Cond::from_str("^");

    assert!(cond.is_err());

    Ok(())
}
