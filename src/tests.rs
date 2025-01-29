use super::*;

use crate::rule::Rule;

#[test]
pub fn test_assumptions() {
    let args = Args {
        length: 5usize,
        file: "test_slovar.txt".to_string(),
        rules: vec![
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
        ],
    };

    let assumptions = assumptions(&args);
    dbg!(&assumptions);

    let word1 = "стопа".to_string();

    assert_eq!(
        assumptions.len(),
        1,
        "найдено неправильное количество предположений"
    );
    assert!(assumptions.contains(&word1));
}
