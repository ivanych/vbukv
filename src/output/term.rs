//! Вывод результата в терминал.

#[cfg(test)]
mod tests;

/// Выводит результат в терминал в виде простого текста.
/// ```
/// use vbukv::output::term;
///
/// // Найденные слова
/// let assumptions = vec![
///     String::from("раз"),
///     String::from("два"),
///     String::from("три"),
/// ];
///
/// // Вывести найденные слова
/// term::output(&assumptions).expect("Failed to print output");
/// ```
///
/// Результат имеет следующий вид:
///
/// ```text
/// Предположения:
/// ---------------------------------
/// <слово1>
/// ..
/// <словоN>
/// ---------------------------------
/// Найдено предположений: <N>
/// ```
pub fn output(assumptions: &Vec<String>) -> Result<bool, String> {
    println!("Предположения:");
    println!("---------------------------------");

    for assumption in assumptions {
        println!("{}", assumption);
    }

    println!("---------------------------------");
    println!("Найдено предположений: {}", assumptions.len());

    Ok(true)
}
