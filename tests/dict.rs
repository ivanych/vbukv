use vbukv::dict;
use vbukv::rule::Rule;

#[test]
fn filter_test_1() {
    let words = vec![
        String::from("паста"),
        String::from("стопа"),
        String::from("кнопка"),
        String::from("шпонка"),
        String::from("трактор"),
        String::from("масштаб"),
        String::from("концепция"),
        String::from("постройка"),
        String::from("министерство"),
    ];

    let length: usize = 5;

    let rules = vec![
        Rule {
            letter: 'с',
            condition: true,
            position: None,
        },
        Rule {
            letter: 'о',
            condition: false,
            position: None,
        },
    ];

    let assumptions = dict::filter(words, length, rules);

    println!("{:?}", assumptions);

    let word1 = "паста".to_string();

    assert!(assumptions.contains(&word1));
}

#[test]
fn filter_test_2() {
    let words = vec![
        String::from("паста"),
        String::from("стопа"),
        String::from("кнопка"),
        String::from("шпонка"),
        String::from("трактор"),
        String::from("масштаб"),
        String::from("концепция"),
        String::from("постройка"),
        String::from("министерство"),
    ];

    let length: usize = 9;

    let rules = vec![
        Rule {
            letter: 'о',
            condition: true,
            position: Some(2),
        },
        Rule {
            letter: 'ц',
            condition: false,
            position: Some(4),
        },
    ];

    let assumptions = dict::filter(words, length, rules);

    println!("{:?}", assumptions);

    let word1 = "постройка".to_string();

    assert!(assumptions.contains(&word1));
}
