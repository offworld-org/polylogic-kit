#[derive(Debug, Clone, PartialEq)]
pub struct Thesis {
    pub content: String,
    pub strength: f64,
}

impl Thesis {
    pub fn new(content: String, strength: f64) -> Self {
        Self {
            content,
            strength: strength.clamp(0.0, 1.0),
        }
    }

    pub fn weaken(&mut self, amount: f64) {
        self.strength = (self.strength - amount).clamp(0.0, 1.0);
    }

    pub fn strengthen(&mut self, amount: f64) {
        self.strength = (self.strength + amount).clamp(0.0, 1.0);
    }
}
