use super::*;
use crate::term;

#[test]
fn output_test_success() -> Result<(), String> {
    let assumptions = vec![
        String::from("раз"),
        String::from("два"),
        String::from("три"),
    ];

    output(&assumptions).expect("Failed to print output");

    assert_eq!(rule.letter, 'a');
    assert!(match rule.condition {
        Cond::Minus => true,
        _ => false,
    });
    assert_eq!(rule.position, None);

    Ok(())
}
