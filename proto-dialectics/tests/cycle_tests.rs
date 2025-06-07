use proto_dialectics::{Antithesis, DialecticalCycle, Thesis};

#[test]
fn add_valid_antithesis() {
    let thesis = Thesis::new("A".to_string(), 0.5);
    let mut cycle = DialecticalCycle::new(thesis.clone());
    let anti = Antithesis::new("not A".to_string(), 0.5, thesis.content.clone());
    assert!(cycle.add_antithesis(anti));
    assert!(cycle.antithesis.is_some());
}

#[test]
fn add_invalid_antithesis() {
    let thesis = Thesis::new("A".to_string(), 0.5);
    let mut cycle = DialecticalCycle::new(thesis);
    let anti = Antithesis::new("not A".to_string(), 0.5, "B".to_string());
    assert!(!cycle.add_antithesis(anti));
    assert!(cycle.antithesis.is_none());
}

#[test]
fn synthesize_with_valid_antithesis() {
    let thesis = Thesis::new("A".to_string(), 0.5);
    let mut cycle = DialecticalCycle::new(thesis.clone());
    let anti = Antithesis::new("not A".to_string(), 0.5, thesis.content.clone());
    assert!(cycle.add_antithesis(anti));
    assert!(cycle.synthesize("A and not A".to_string(), 0.8));
    assert!(cycle.is_complete());
}

#[test]
fn synthesize_without_antithesis() {
    let thesis = Thesis::new("A".to_string(), 0.5);
    let mut cycle = DialecticalCycle::new(thesis);
    assert!(!cycle.synthesize("something".to_string(), 0.8));
    assert!(cycle.synthesis.is_none());
}
