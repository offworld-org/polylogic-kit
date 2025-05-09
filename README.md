# Polylogic Kit

A Rust library collection for implementing various logical systems and dialectical reasoning patterns.

## Current Features

- **Wuxing (五行)**: Implementation of the Five Elements theory
  - Generation cycle (相生): Wood → Fire → Earth → Metal → Water → Wood
  - Control cycle (相克): Wood → Earth → Water → Fire → Metal → Wood

## Future Plans

The library will be expanded to include other dialectical logic systems beyond the Five Elements theory. Planned additions include:

- Hegelian dialectics (thesis, antithesis, synthesis)
- Yin-Yang dynamics
- Other traditional philosophical frameworks

## Usage

```rust
use wuxing::{Element, WuxingCycle};

// Generation cycle example
let next = WuxingCycle::generate(Element::Wood); // Returns Element::Fire

// Control cycle example
let controls = WuxingCycle::control(Element::Wood); // Returns Element::Earth
```

## License

MIT
