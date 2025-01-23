#[derive(Debug, Clone)]
pub struct Rule {
    pub letter: char,
    pub condition: char,
    pub position: Option<usize>,
}
