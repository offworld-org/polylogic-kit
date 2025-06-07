use proto_wuxing::{Element, HeavenlyStem, WuxingElement, YinYang, Zodiac};

#[test]
fn branch_compatibility_matches_element() {
    let elem = WuxingElement::with_branches(
        Element::Wood,
        YinYang::Yang,
        Zodiac::Tiger,
        HeavenlyStem::Jia,
    );
    assert!(elem.is_branch_compatible());
}

#[test]
fn branch_compatibility_mismatch_element() {
    let elem = WuxingElement::with_branches(
        Element::Wood,
        YinYang::Yang,
        Zodiac::Dragon,
        HeavenlyStem::Jia,
    );
    assert!(!elem.is_branch_compatible());
}

#[test]
fn stem_compatibility_matches_element() {
    let elem = WuxingElement::with_branches(
        Element::Metal,
        YinYang::Yin,
        Zodiac::Monkey,
        HeavenlyStem::Geng,
    );
    assert!(elem.is_stem_compatible());
}

#[test]
fn stem_compatibility_mismatch_element() {
    let elem = WuxingElement::with_branches(
        Element::Metal,
        YinYang::Yin,
        Zodiac::Monkey,
        HeavenlyStem::Jia,
    );
    assert!(!elem.is_stem_compatible());
}
