#[derive(Debug)]
pub struct Rule {
    pub letter: char,
    pub condition: bool,
    pub position: Option<usize>,
}

//impl Rule {
//    pub fn letter_lc(&self) -> String {
//        self.letter.to_lowercase()
//    }
//}
