use crate::element::Element;

pub struct WuxingCycle;

impl WuxingCycle {
    pub fn generate(e: Element) -> Element {
        match e {
            Element::Wood => Element::Fire,
            Element::Fire => Element::Earth,
            Element::Earth => Element::Metal,
            Element::Metal => Element::Water,
            Element::Water => Element::Wood,
        }
    }

    pub fn control(e: Element) -> Element {
        match e {
            Element::Wood => Element::Earth,
            Element::Fire => Element::Metal,
            Element::Earth => Element::Water,
            Element::Metal => Element::Wood,
            Element::Water => Element::Fire,
        }
    }
}
