use crate::element::{Element, WuxingElement};
use crate::heavenly_stem::HeavenlyStem;
use crate::zodiac::Zodiac;

pub struct WuxingCycle {
    elements: Vec<WuxingElement>,
}

impl WuxingCycle {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: WuxingElement) {
        self.elements.push(element);
    }

    pub fn generate(&self) -> Option<WuxingElement> {
        if let Some(last) = self.elements.last() {
            let next_element = match last.element {
                Element::Wood => Element::Fire,
                Element::Fire => Element::Earth,
                Element::Earth => Element::Metal,
                Element::Metal => Element::Water,
                Element::Water => Element::Wood,
            };
            Some(WuxingElement::new(next_element))
        } else {
            None
        }
    }

    pub fn control(&self) -> Option<WuxingElement> {
        if let Some(last) = self.elements.last() {
            let next_element = match last.element {
                Element::Wood => Element::Earth,
                Element::Earth => Element::Water,
                Element::Water => Element::Fire,
                Element::Fire => Element::Metal,
                Element::Metal => Element::Wood,
            };
            Some(WuxingElement::new(next_element))
        } else {
            None
        }
    }

    pub fn balance(&self) -> Option<WuxingElement> {
        if let Some(last) = self.elements.last() {
            let next_element = match last.element {
                Element::Wood => Element::Metal,
                Element::Metal => Element::Wood,
                Element::Fire => Element::Water,
                Element::Water => Element::Fire,
                Element::Earth => Element::Earth,
            };
            Some(WuxingElement::new(next_element))
        } else {
            None
        }
    }

    pub fn get_earthly_branch_cycle(&self) -> Vec<Zodiac> {
        vec![
            Zodiac::Rat,
            Zodiac::Ox,
            Zodiac::Tiger,
            Zodiac::Rabbit,
            Zodiac::Dragon,
            Zodiac::Snake,
            Zodiac::Horse,
            Zodiac::Goat,
            Zodiac::Monkey,
            Zodiac::Rooster,
            Zodiac::Dog,
            Zodiac::Pig,
        ]
    }

    pub fn get_heavenly_stem_cycle(&self) -> Vec<HeavenlyStem> {
        vec![
            HeavenlyStem::Jia,
            HeavenlyStem::Yi,
            HeavenlyStem::Bing,
            HeavenlyStem::Ding,
            HeavenlyStem::Wu,
            HeavenlyStem::Ji,
            HeavenlyStem::Geng,
            HeavenlyStem::Xin,
            HeavenlyStem::Ren,
            HeavenlyStem::Gui,
        ]
    }
}
