use crate::antithesis::Antithesis;
use crate::thesis::Thesis;

#[derive(Debug, Clone, PartialEq)]
pub struct Synthesis {
    pub content: String,
    pub strength: f64,
    pub thesis_content: String,
    pub antithesis_content: String,
}

impl Synthesis {
    pub fn new(
        content: String,
        strength: f64,
        thesis_content: String,
        antithesis_content: String,
    ) -> Self {
        Self {
            content,
            strength: strength.clamp(0.0, 1.0),
            thesis_content,
            antithesis_content,
        }
    }

    pub fn from_thesis_antithesis(
        thesis: &Thesis,
        antithesis: &Antithesis,
        content: String,
        strength: f64,
    ) -> Option<Self> {
        if !antithesis.is_valid_opposition(&thesis.content) {
            return None;
        }

        Some(Self {
            content,
            strength: strength.clamp(0.0, 1.0),
            thesis_content: thesis.content.clone(),
            antithesis_content: antithesis.content.clone(),
        })
    }

    pub fn weaken(&mut self, amount: f64) {
        self.strength = (self.strength - amount).clamp(0.0, 1.0);
    }

    pub fn strengthen(&mut self, amount: f64) {
        self.strength = (self.strength + amount).clamp(0.0, 1.0);
    }
}
