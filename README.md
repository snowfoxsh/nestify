# Nestify

Nestify provides a Rust macro for streamlined nested struct definitions, enhancing code readability. It's especially valuable for handling API responses.

[![Crates.io](https://img.shields.io/crates/v/nestify.svg)](https://crates.io/crates/nestify)
[![Documentation](https://docs.rs/nestify/badge.svg)](https://docs.rs/nestify)

## Features

- Simplify nested struct definitions in Rust.
- Make your codebase more readable and less verbose.
- Ideal for modeling complex API responses.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nestify = "0.1.1"
```

Then, in your Rust code, import the crate and macro:

```rust
extern crate nestify; // optional
use nestify::nest;
```

## Usage

Here's a basic example:

```rust
use nestify::nest;

nest!{
    struct MyOuterStruct {
        field1: struct MyInnerStruct {
            subfield1: i32,
            subfield2: String,
        },
        field2: f64,
    }
}
```

This will produce:

```rust
struct MyInnerStruct {
    subfield1: i32,
    subfield2: String,
}

struct MyOuterStruct {
    field1: MyInnerStruct,
    field2: f64,
}
```
## Todo
This project is being activley worked on. Check proc branch for the proc macro version.
The proc version will soon become the main version!

- [x] Trailing Commas
- [x] Suport for no parens around types
- [ ] Derive macros
- [ ] Serde Integration
- [ ] Enums
- [ ] Tuple Structs
- [ ] Attribute Compatabilty
- [ ] Autonamed fields
- [ ] Better Documnation
- [ ] Better Testing
- [ ] Visibility Modifiers
- [ ] Generic Parameters
- [ ] Lifetimes


## Contributing

1. Fork the repository.
2. Create a new branch for your features or bug fixes.
3. Write tests for your changes.
4. Make sure all tests pass.
5. Submit a pull request.

## License

This project is licensed under the MIT License
