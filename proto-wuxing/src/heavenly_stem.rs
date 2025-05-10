use crate::element::Element;
use crate::yin_yang::YinYang;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeavenlyStem {
    Jia,
    Yi,
    Bing,
    Ding,
    Wu,
    Ji,
    Geng,
    Xin,
    Ren,
    Gui,
}

impl HeavenlyStem {
    pub fn associated_element(&self) -> Element {
        match self {
            HeavenlyStem::Jia | HeavenlyStem::Yi => Element::Wood,
            HeavenlyStem::Bing | HeavenlyStem::Ding => Element::Fire,
            HeavenlyStem::Wu | HeavenlyStem::Ji => Element::Earth,
            HeavenlyStem::Geng | HeavenlyStem::Xin => Element::Metal,
            HeavenlyStem::Ren | HeavenlyStem::Gui => Element::Water,
        }
    }

    pub fn default_yin_yang(&self) -> YinYang {
        match self {
            HeavenlyStem::Jia
            | HeavenlyStem::Bing
            | HeavenlyStem::Wu
            | HeavenlyStem::Geng
            | HeavenlyStem::Ren => YinYang::Yang,
            HeavenlyStem::Yi
            | HeavenlyStem::Ding
            | HeavenlyStem::Ji
            | HeavenlyStem::Xin
            | HeavenlyStem::Gui => YinYang::Yin,
        }
    }

    pub fn next(&self) -> Self {
        match self {
            HeavenlyStem::Jia => HeavenlyStem::Yi,
            HeavenlyStem::Yi => HeavenlyStem::Bing,
            HeavenlyStem::Bing => HeavenlyStem::Ding,
            HeavenlyStem::Ding => HeavenlyStem::Wu,
            HeavenlyStem::Wu => HeavenlyStem::Ji,
            HeavenlyStem::Ji => HeavenlyStem::Geng,
            HeavenlyStem::Geng => HeavenlyStem::Xin,
            HeavenlyStem::Xin => HeavenlyStem::Ren,
            HeavenlyStem::Ren => HeavenlyStem::Gui,
            HeavenlyStem::Gui => HeavenlyStem::Jia,
        }
    }
}
