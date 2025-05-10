# Proto Wuxing

A Rust-based implementation of the Yin-Yang Five Elements (Wuxing) system.

## Description

Proto Wuxing is a comprehensive implementation of the traditional Yin-Yang Five Elements (Wuxing) system. It provides a framework for modeling and manipulating the interactions between these fundamental forces of nature.

The system implements:

- Five Elements (Wood, Fire, Earth, Metal, Water)
- Yin-Yang polarity for each element
- Generation and Control cycles
- Balance checking and adjustment
- Twelve Earthly Branches (Zodiac) integration

## Usage

### Prerequisites

- Rust 1.70 or higher
- Cargo (Rust's package manager)

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
proto-wuxing = "0.0.1"
```

### Basic Usage

```rust
use proto_wuxing::{Element, WuxingElement, YinYang, WuxingCycle};

// Create a Wood element with Yang polarity
let wood_yang = WuxingElement::new(Element::Wood, YinYang::Yang);

// Apply the Generation cycle (creates Fire with Yin polarity)
let fire_yin = WuxingCycle::generate(wood_yang);

// Apply the Control cycle
let controlled = WuxingCycle::control(fire_yin);

// Check and adjust balance
let balanced = WuxingCycle::balance(controlled);

// Create an element with default Yin-Yang polarity
let earth = WuxingElement::with_default_yin_yang(Element::Earth);
```

### Advanced Usage

```rust
use proto_wuxing::{Element, WuxingElement, YinYang, WuxingCycle};

// Create a cycle of elements
fn create_element_cycle(start: WuxingElement) -> Vec<WuxingElement> {
    let mut cycle = Vec::new();
    let mut current = start;

    for _ in 0..5 {
        cycle.push(current);
        current = WuxingCycle::generate(current);
    }

    cycle
}

// Example: Create a complete cycle starting with Wood (Yang)
let wood_yang = WuxingElement::new(Element::Wood, YinYang::Yang);
let cycle = create_element_cycle(wood_yang);

// Check balance of all elements
let balanced_cycle: Vec<WuxingElement> = cycle
    .into_iter()
    .map(WuxingCycle::balance)
    .collect();
```

## Features

- **Five Elements**: Implementation of Wood, Fire, Earth, Metal, and Water elements
- **Yin-Yang Polarity**: Each element can have either Yin or Yang polarity
- **Generation Cycle**: Implements the creative cycle of elements
- **Control Cycle**: Implements the controlling relationships between elements
- **Balance System**: Tools for checking and maintaining elemental balance
- **Twelve Earthly Branches**: Integration of the 12 Earthly Branches with the Five Elements

## Twelve Earthly Branches Integration

The system includes the 12 Earthly Branches which adds another layer of complexity and meaning to the Yin-Yang Five Elements system. Each branch is associated with an element and has its own Yin-Yang polarity.

### Branch-Element Relationships

```rust
use proto_wuxing::{Element, WuxingElement, YinYang, Zodiac, WuxingCycle};

// Create an element with a branch
let wood_tiger = WuxingElement::with_zodiac(
    Element::Wood,
    YinYang::Yang,
    Zodiac::Tiger
);

// Check branch-element compatibility
assert!(wood_tiger.is_zodiac_compatible());

// Generate next element with branch
let fire_snake = WuxingCycle::generate(wood_tiger);
```

### Branch Cycle

The 12 Earthly Branches follow a specific cycle:

1. **Rat** - Water, Yang
2. **Ox** - Earth, Yin
3. **Tiger** - Wood, Yang
4. **Rabbit** - Wood, Yin
5. **Dragon** - Earth, Yang
6. **Snake** - Fire, Yin
7. **Horse** - Fire, Yang
8. **Goat** - Earth, Yin
9. **Monkey** - Metal, Yang
10. **Rooster** - Metal, Yin
11. **Dog** - Earth, Yang
12. **Pig** - Water, Yin

### Element-Branch Distribution

- **Wood**: Tiger, Rabbit
- **Fire**: Snake, Horse
- **Earth**: Ox, Dragon, Goat, Dog
- **Metal**: Monkey, Rooster
- **Water**: Rat, Pig

### Practical Applications

#### 1. Complete Branch Cycle with Elements

```rust
fn demonstrate_branch_element_cycle() {
    let mut current = WuxingElement::with_zodiac(
        Element::Wood,
        YinYang::Yang,
        Zodiac::Tiger
    );

    println!("Branch-Element Cycle:");
    for _ in 0..12 {
        println!("Current: {:?} with {:?} and {:?}",
            current.element,
            current.yin_yang,
            current.zodiac.unwrap()
        );
        current = WuxingCycle::generate(current);
    }
}
```

#### 2. Balance Check with Branch

```rust
// Create an imbalanced element-branch combination
let imbalanced = WuxingElement::with_zodiac(
    Element::Wood,
    YinYang::Yin,
    Zodiac::Rat  // Rat is associated with Water, not Wood
);

assert!(!imbalanced.is_zodiac_compatible());

// Restore balance
let balanced = WuxingCycle::balance(imbalanced);
assert!(balanced.is_zodiac_compatible());
```

### Interpreting Branch-Element Relationships

1. **Elemental Harmony**

   - Each branch has a natural element
   - Elements can be strengthened or weakened by their branch associations
   - Balance considers both elemental and branch compatibility

2. **Temporal Aspects**

   - The branch cycle represents a 12-year cycle
   - Each year has its own elemental and yin-yang characteristics
   - Can be used to model cyclical patterns and changes

3. **Spatial Relationships**
   - Branches can be mapped to directions
   - Elements and branches together create a more complete spatial model
   - Useful for understanding environmental and directional influences

These relationships can be used to create more complex and nuanced models of natural phenomena, time cycles, and spatial relationships.

## Development

To build the project:

```bash
cargo build
```

To run tests:

```bash
cargo test
```

## License

Unless otherwise stated, all code in this repository is dual-licensed under:

- MIT License (LICENSE-MIT)
- Apache License, Version 2.0 (LICENSE-APACHE)

You may choose either license according to your needs. All contributions are accepted under these dual-license terms.
