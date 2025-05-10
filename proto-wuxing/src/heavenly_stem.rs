use crate::element::Element;
use crate::yin_yang::YinYang;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeavenlyStem {
    Jia,  // 甲 (Wood, Yang)
    Yi,   // 乙 (Wood, Yin)
    Bing, // 丙 (Fire, Yang)
    Ding, // 丁 (Fire, Yin)
    Wu,   // 戊 (Earth, Yang)
    Ji,   // 己 (Earth, Yin)
    Geng, // 庚 (Metal, Yang)
    Xin,  // 辛 (Metal, Yin)
    Ren,  // 壬 (Water, Yang)
    Gui,  // 癸 (Water, Yin)
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
