# Proto Dialectics

A Rust library for implementing dialectical reasoning and synthesis.

## Description

This library provides a structured way to model and process dialectical relationships between ideas, following the Hegelian dialectic pattern of thesis, antithesis, and synthesis.

## Features

- **Thesis**: Represents the initial proposition or idea
- **Antithesis**: Represents the opposing proposition or idea
- **Synthesis**: Represents the resolution that combines elements of both thesis and antithesis
- **DialecticalCycle**: Manages the complete dialectical process

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
proto-dialectics = "0.0.1"
```

## Usage

```rust
use proto_dialectics::{Thesis, Antithesis, Synthesis, DialecticalCycle};

// Create a new thesis
let thesis = Thesis::new("Democracy is the best form of government".to_string(), 0.8);

// Create a dialectical cycle
let mut cycle = DialecticalCycle::new(thesis);

// Add an antithesis
let antithesis = Antithesis::new(
    "Democracy is inefficient and slow in decision-making".to_string(),
    0.7,
    "Democracy is the best form of government".to_string(),
);
cycle.add_antithesis(antithesis);

// Create a synthesis
cycle.synthesize(
    "Democracy needs to balance efficiency with participation".to_string(),
    0.9,
);

// Check if the cycle is complete
if cycle.is_complete() {
    // Get the next thesis for a new cycle
    if let Some(next_thesis) = cycle.get_next_thesis() {
        // Start a new dialectical cycle
    }
}
```

## Components

### Thesis

- Represents the initial proposition
- Has a content and strength (0.0 to 1.0)
- Can be weakened or strengthened

### Antithesis

- Represents the opposing proposition
- Must be valid opposition to the thesis
- Has its own strength and content

### Synthesis

- Combines elements of both thesis and antithesis
- Creates a new, higher-level understanding
- Can be used as the starting point for a new cycle

### DialecticalCycle

- Manages the complete dialectical process
- Ensures valid relationships between components
- Provides methods for cycle completion and progression

## References

To better understand the philosophical foundations of each logic module, consider the following references:

- [Dialectic – Method of argument and reasoning](https://en.wikipedia.org/wiki/Dialectic)
- [Hegelian Dialectic – Thesis, antithesis, synthesis](https://en.wikipedia.org/wiki/Dialectic#Hegelian_dialectic)

## License

Unless otherwise stated, all code in this repository is dual-licensed under:

- MIT License (LICENSE-MIT)
- Apache License, Version 2.0 (LICENSE-APACHE)

You may choose either license according to your needs. All contributions are accepted under these dual-license terms.
