#[derive(Debug)]
pub struct Rule {
    pub letter: String,
    pub condition: String,
    pub position: Option<usize>,
}

impl Rule {
    pub fn letter_lc(&self) -> String {
        self.letter.to_lowercase()
    }
}
