use crate::antithesis::Antithesis;
use crate::synthesis::Synthesis;
use crate::thesis::Thesis;

#[derive(Debug, Clone)]
pub struct DialecticalCycle {
    pub thesis: Thesis,
    pub antithesis: Option<Antithesis>,
    pub synthesis: Option<Synthesis>,
}

impl DialecticalCycle {
    pub fn new(thesis: Thesis) -> Self {
        Self {
            thesis,
            antithesis: None,
            synthesis: None,
        }
    }

    pub fn add_antithesis(&mut self, antithesis: Antithesis) -> bool {
        if !antithesis.is_valid_opposition(&self.thesis.content) {
            return false;
        }
        self.antithesis = Some(antithesis);
        true
    }

    pub fn synthesize(&mut self, content: String, strength: f64) -> bool {
        if let Some(antithesis) = &self.antithesis {
            if let Some(synthesis) =
                Synthesis::from_thesis_antithesis(&self.thesis, antithesis, content, strength)
            {
                self.synthesis = Some(synthesis);
                return true;
            }
        }
        false
    }

    pub fn is_complete(&self) -> bool {
        self.antithesis.is_some() && self.synthesis.is_some()
    }

    pub fn get_next_thesis(&self) -> Option<Thesis> {
        if let Some(synthesis) = &self.synthesis {
            Some(Thesis::new(synthesis.content.clone(), synthesis.strength))
        } else {
            None
        }
    }
}
