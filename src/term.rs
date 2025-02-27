
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

#[cfg(test)]
mod tests;
