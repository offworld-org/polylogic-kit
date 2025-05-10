#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YinYang {
    Yin,
    Yang,
}

impl YinYang {
    pub fn opposite(&self) -> Self {
        match self {
            YinYang::Yin => YinYang::Yang,
            YinYang::Yang => YinYang::Yin,
        }
    }
}
