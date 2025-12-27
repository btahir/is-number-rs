# is-number-rs

> Because JavaScript's type coercion wasn't confusing enough, we decided to solve it with WebAssembly.

A blazingly fast, memory-safe, fearlessly concurrent way to check if a string is a number. Written in Rust, compiled to WASM, delivered to npm. The triforce of modern web development.

## Why?

The JavaScript ecosystem has mass downloaded variations of `is-number` over 60 million times per week. Clearly, determining whether something is a number is one of humanity's greatest unsolved challenges.

We're here to mass download solutions.

This package brings the raw, unbridled power of Rust's `parse::<f64>()` to your node_modules. Finally.

## Installation

```bash
npm install is-number-rs
```

## Usage

```javascript
import { is_number } from 'is-number-rs';

is_number("42");        // true - that's a number alright
is_number("3.14159");   // true - pi-ish
is_number("-273.15");   // true - absolute zero, absolutely a number
is_number("1e10");      // true - science!
is_number("Infinity");  // false - we have standards
is_number("NaN");       // false - we REALLY have standards
is_number("");          // false - the void is not a number
is_number("   ");       // false - neither is whitespace
is_number("abc");       // false - letters aren't numbers (controversial take)
is_number("12px");      // false - CSS units don't count
```

## Features

- Written in Rust (mass production ready)
- Compiles to WebAssembly (buzzword compliant)
- Handles edge cases (we trim whitespace!)
- Rejects `Infinity` and `NaN` (because `isFinite` said so)
- Zero runtime dependencies*

*^(in Rust)

## Performance

Is it faster than a simple regex or `parseFloat` check in JavaScript?

Look, we didn't write this in Rust and compile it to WASM for it to be *slower*. That would defeat the entire purpose of...

Actually, we haven't benchmarked it. But it *feels* fast.

## How It Works

```rust
pub fn is_number(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }
    match trimmed.parse::<f64>() {
        Ok(n) => n.is_finite(),
        Err(_) => false,
    }
}
```

That's it. That's the whole thing. 13 lines of Rust doing what JavaScript has needed 60 million weekly downloads to figure out.

## Contributing

Found a number we're not recognizing? Open an issue. Found something that isn't a number that we think is? That's probably more concerning. Also open an issue.

## License

MIT - Use it however you want. We're not responsible for any existential crises caused by questioning what a number really is.

---

*Made with mass downloads and mass production in mind.*
