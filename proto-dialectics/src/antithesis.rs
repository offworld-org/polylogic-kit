#[derive(Debug, Clone, PartialEq)]
pub struct Antithesis {
    pub content: String,
    pub strength: f64,
    pub thesis_content: String,
}

impl Antithesis {
    pub fn new(content: String, strength: f64, thesis_content: String) -> Self {
        Self {
            content,
            strength: strength.clamp(0.0, 1.0),
            thesis_content,
        }
    }

    pub fn weaken(&mut self, amount: f64) {
        self.strength = (self.strength - amount).clamp(0.0, 1.0);
    }

    pub fn strengthen(&mut self, amount: f64) {
        self.strength = (self.strength + amount).clamp(0.0, 1.0);
    }

    pub fn is_valid_opposition(&self, thesis_content: &str) -> bool {
        self.thesis_content == thesis_content
    }
}
