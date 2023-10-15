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
nestify = "0.2.1"
```

Then, in your Rust code, import the crate and macro:

```rust
extern crate nestify; // optional
use nestify::nest;
```

## Usage

### Basic example:

```rust
use nestify::nest;

nest!{
    struct Outer {
        field1: struct Inner {
            subfield1: i32,
            subfield2: String,
        },
        field2: f64,
    }
}
```

This will produce:

```rust
struct Outer {
    field1: Inner,
    field2: f64,
}

struct Inner {
    subfield1: i32,
    subfield2: String,
}
```

### Generics

```rust
use nestify::nest;

nest! {
    struct Example<'a, T> {
        s: &'a str,
        t: T
    }
}
```

This will produce: 

```rust
struct Struct<'a, T> { 
    s: &'a str, 
    t: T, 
}
```

#### Nested Generics
When defining nested generics you need to add generics to types. Enter "fish" syntax.
To define generics on the field write `::<...>`. This will let you specify the nested generic types.

```rust
nest! {
    struct Parent<'p, P> {
        child::<'p, P> : struct Child<'c, C> {
            s: &'c str,
            f: C
        }
    }
}
```

This will produce: 
```rust
struct Parent<'p, P> { 
    child: Child<'p, P>,
}

struct Child<'c, C> {
    s: &'c str,
    f: C,
}
```

### Attributes
You can apply attributes just like you would with a normal struct.

```rust
nest! {
    #[derive(Clone)]
    struct CloneMe {}
}
let x = CloneMe {};
let cl = x.clone();
```

Using `*` syntax you can inherit attributes to child structures.

```rust
nest! {
    #[apply_all]*
    #[apply_this]
    struct GrandParent {
        parent: struct Parent {
            child: struct Child {
                payload: ()
            }
        }
    }
}
```
This will produce: 

```rust
#[apply_all]
#[apply_this]
struct GrandParent {
    parent: Parent,
}

#[apply_all]
struct Parent {
    child: Child,
}

#[apply_all]
struct Child {
    payload: (),
}
```

## Contributing

1. Fork the repository.
2. Create a new branch for your features or bug fixes.
3. Write tests for your changes.
4. Make sure all tests pass.
5. Submit a pull request.

## License

This project is licensed under the MIT License
