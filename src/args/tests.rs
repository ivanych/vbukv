use super::*;
use crate::rule::Rule;

#[test]
fn test_args() {
    let args = Args {
        length: 5,
        file: "test_slovar.txt".to_string(),
        rules: vec![Rule {
            letter: 'с',
            condition: '+',
            position: None,
        }],
    };

    assert_eq!(args.length, 5);
}
