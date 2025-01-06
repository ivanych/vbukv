pub fn filter(words: Vec<String>) -> Vec<String> {
    let dict_iter = words.into_iter();

    dict_iter
        // длина слова
        .filter(|x| x.len() == 10)
        //
        .filter(|x| x.contains("а"))
        //
        .filter(|x| x.get(0..2) == Some(&String::from("з")))
        .filter(|x| x.get(6..8) == Some(&String::from("о")))
        .filter(|x| x.get(8..10) == Some(&String::from("р")))
        //
        .filter(|x| !x.contains("с"))
        .filter(|x| !x.contains("в"))
        .filter(|x| !x.contains("и"))
        .filter(|x| !x.contains("т"))
        .filter(|x| !x.contains("я"))
        .filter(|x| !x.contains("г"))
        .filter(|x| !x.contains("у"))
        .filter(|x| !x.contains("н"))
        .filter(|x| !x.contains("к"))
        .filter(|x| !x.contains("е"))
        .filter(|x| !x.contains("б"))
        .filter(|x| !x.contains("д"))
        //
        .collect()
}
