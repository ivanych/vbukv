use super::*;

#[test]
fn words_from_file_test_success() {
    // TODO Вроде нет стандартного места для размещения такиех файлов...
    // Размещу рядом с тестом.
    let filename = "test_slovar.txt".to_string();

    let words = words_from_file(filename);

    assert_eq!(
        words.len(),
        9,
        "Из словаря прочитано не столько слов, сколько ожидалось"
    );

    let word = "паста".to_string();
    assert!(
        words.contains(&word),
        "Среди прочитанных слов нет слова '{}'",
        &word
    );
}
