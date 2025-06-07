use crate::heavenly_stem::HeavenlyStem;
use crate::yin_yang::YinYang;
use crate::zodiac::Zodiac;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WuxingElement {
    pub element: Element,
    pub yin_yang: Option<YinYang>,
    pub earthly_branch: Option<Zodiac>,
    pub heavenly_stem: Option<HeavenlyStem>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Wood,
    Fire,
    Earth,
    Metal,
    Water,
}

impl Element {
    pub fn default_yin_yang(&self) -> YinYang {
        match self {
            Element::Wood => YinYang::Yang,
            Element::Fire => YinYang::Yang,
            Element::Earth => YinYang::Yin,
            Element::Metal => YinYang::Yin,
            Element::Water => YinYang::Yin,
        }
    }

    pub fn associated_earthly_branches(&self) -> Vec<Zodiac> {
        match self {
            Element::Wood => vec![Zodiac::Tiger, Zodiac::Rabbit],
            Element::Fire => vec![Zodiac::Snake, Zodiac::Horse],
            Element::Earth => vec![Zodiac::Ox, Zodiac::Dragon, Zodiac::Goat, Zodiac::Dog],
            Element::Metal => vec![Zodiac::Monkey, Zodiac::Rooster],
            Element::Water => vec![Zodiac::Rat, Zodiac::Pig],
        }
    }

    pub fn associated_heavenly_stems(&self) -> Vec<HeavenlyStem> {
        match self {
            Element::Wood => vec![HeavenlyStem::Jia, HeavenlyStem::Yi],
            Element::Fire => vec![HeavenlyStem::Bing, HeavenlyStem::Ding],
            Element::Earth => vec![HeavenlyStem::Wu, HeavenlyStem::Ji],
            Element::Metal => vec![HeavenlyStem::Geng, HeavenlyStem::Xin],
            Element::Water => vec![HeavenlyStem::Ren, HeavenlyStem::Gui],
        }
    }
}

impl WuxingElement {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            yin_yang: Some(element.default_yin_yang()),
            earthly_branch: None,
            heavenly_stem: None,
        }
    }

    pub fn with_yin_yang(element: Element, yin_yang: YinYang) -> Self {
        Self {
            element,
            yin_yang: Some(yin_yang),
            earthly_branch: None,
            heavenly_stem: None,
        }
    }

    pub fn with_branches(
        element: Element,
        yin_yang: YinYang,
        earthly_branch: Zodiac,
        heavenly_stem: HeavenlyStem,
    ) -> Self {
        Self {
            element,
            yin_yang: Some(yin_yang),
            earthly_branch: Some(earthly_branch),
            heavenly_stem: Some(heavenly_stem),
        }
    }

    pub fn is_balanced(&self) -> bool {
        if let Some(yin_yang) = self.yin_yang {
            match self.element {
                Element::Wood => yin_yang == YinYang::Yang,
                Element::Fire => yin_yang == YinYang::Yang,
                Element::Earth => yin_yang == YinYang::Yin,
                Element::Metal => yin_yang == YinYang::Yin,
                Element::Water => yin_yang == YinYang::Yin,
            }
        } else {
            true
        }
    }

    pub fn is_branch_compatible(&self) -> bool {
        if let Some(branch) = self.earthly_branch {
            branch.associated_element() == self.element
        } else {
            true
        }
    }

    pub fn is_stem_compatible(&self) -> bool {
        if let Some(stem) = self.heavenly_stem {
            stem.associated_element() == self.element
        } else {
            true
        }
    }
}
