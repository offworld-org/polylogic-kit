use crate::element::Element;
use crate::yin_yang::YinYang;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Zodiac {
    Rat,
    Ox,
    Tiger,
    Rabbit,
    Dragon,
    Snake,
    Horse,
    Goat,
    Monkey,
    Rooster,
    Dog,
    Pig,
}

impl Zodiac {
    pub fn associated_element(&self) -> Element {
        match self {
            Zodiac::Rat | Zodiac::Pig => Element::Water,
            Zodiac::Ox | Zodiac::Dragon | Zodiac::Goat | Zodiac::Dog => Element::Earth,
            Zodiac::Tiger | Zodiac::Rabbit => Element::Wood,
            Zodiac::Snake | Zodiac::Horse => Element::Fire,
            Zodiac::Monkey | Zodiac::Rooster => Element::Metal,
        }
    }

    pub fn default_yin_yang(&self) -> YinYang {
        match self {
            Zodiac::Rat
            | Zodiac::Tiger
            | Zodiac::Dragon
            | Zodiac::Horse
            | Zodiac::Monkey
            | Zodiac::Dog => YinYang::Yang,
            Zodiac::Ox
            | Zodiac::Rabbit
            | Zodiac::Snake
            | Zodiac::Goat
            | Zodiac::Rooster
            | Zodiac::Pig => YinYang::Yin,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Zodiac::Rat => Zodiac::Ox,
            Zodiac::Ox => Zodiac::Tiger,
            Zodiac::Tiger => Zodiac::Rabbit,
            Zodiac::Rabbit => Zodiac::Dragon,
            Zodiac::Dragon => Zodiac::Snake,
            Zodiac::Snake => Zodiac::Horse,
            Zodiac::Horse => Zodiac::Goat,
            Zodiac::Goat => Zodiac::Monkey,
            Zodiac::Monkey => Zodiac::Rooster,
            Zodiac::Rooster => Zodiac::Dog,
            Zodiac::Dog => Zodiac::Pig,
            Zodiac::Pig => Zodiac::Rat,
        }
    }
}
