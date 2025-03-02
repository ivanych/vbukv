use super::*;

use crate::libvbukv::assumptions;
use clap::Parser;

#[test]
pub fn test_assumptions() {
    let args = Args::parse_from(["vbukv", "--file", "test_slovar.txt", "с+", "о+3"]);
    dbg!(&args);

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
