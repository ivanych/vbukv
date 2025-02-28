use super::*;

#[test]
fn output_test_success() -> Result<(), String> {
    let assumptions = vec![
        String::from("раз"),
        String::from("два"),
        String::from("три"),
    ];

    // Вывод сработал успешно
    output(&assumptions)?;

    // TODO По хорошему, тут нужно добавить тест с перехватом stdout
    // и проверить, что вывелись не просто предположения,
    // а весь связанный текст. В том числе правильно посчитано и показано
    // количество предположений.

    Ok(())
}
