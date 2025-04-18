use super::*;

#[test]
fn words_from_file_test_success() {
    let filename = "test_slovar.txt".to_string();

    let words = words_from_file(filename);

    assert_eq!(
        words.len(),
        11,
        "Из словаря прочитано не столько слов, сколько ожидалось"
    );

    let word = "паста".to_string();
    assert!(
        words.contains(&word),
        "Среди прочитанных слов нет слова '{}'",
        &word
    );
}
