use super::*;
use crate::rule::{Cond, Rule};

#[test]
fn test_args() {
    let args = Args {
        length: 5,
        file: "test_slovar.txt".to_string(),
        rules: vec![Rule {
            letter: 'с',
            condition: Cond::Plus,
            position: None,
        }],
        markdown_help: false,
    };

    assert_eq!(args.length, 5);
}
