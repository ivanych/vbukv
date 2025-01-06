pub fn filter(words: Vec<String>) {
    let dict_iter = words.iter();

    for word in dict_iter
        // длина слова
        .filter(|x| x.len() == 10)
        //.filter(|x| x.contains("с"))
        //.filter(|x| x.contains("т"))
        .filter(|x| x.get(0..2) != Some(&String::from("с")))
        .filter(|x| x.get(0..2) != Some(&String::from("в")))
        .filter(|x| x.get(0..2) != Some(&String::from("ч")))
        .filter(|x| x.get(0..2) != Some(&String::from("ж")))
        .filter(|x| x.get(0..2) != Some(&String::from("ш")))
        .filter(|x| x.get(2..4) == Some(&String::from("е")))
        .filter(|x| x.get(4..6) == Some(&String::from("с")))
        .filter(|x| x.get(6..8) == Some(&String::from("т")))
        .filter(|x| !x.contains("а"))
        .filter(|x| !x.contains("л"))
        .filter(|x| !x.contains("ю"))
    //.filter(|x| !x.contains("ь"))
    //.filter(|x| !x.contains("п"))
    //.filter(|x| !x.contains("р"))
    //.filter(|x| !x.contains("и"))
    //.filter(|x| !x.contains("ц"))
    {
        println!("{}", word);
    }
}
