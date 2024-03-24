# Nestify

Nestify is a Rust library offering a powerful macro to streamline the definition of nested structs and enums.
Designed to improve code readability and maintainability

[<img alt="crates.io" src="https://img.shields.io/crates/v/nestify.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/nestify)
[<img alt="github" src="https://img.shields.io/badge/snowfoxsh/nestify-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/snowfoxsh/nestify)
[<img alt="licence" src="https://img.shields.io/crates/l/nestify?style=for-the-badge&labelColor=555555&logo" height="20">](LICENCE)



## Abstract

Nestify reimagines Rust struct and enum definitions with its "Type is Definition" approach,
streamlining the way you handle nested structures.
Gone are the days of flipping back and forth between type definitions—Nestify
Unifies your codebase, making your code cleaner and far more readable.

Its syntax is designed to feel comfortable to rust developers.

This is especially handy when you're wrestling with the labyrinthine depths of JSON REST API responses.
With Nestify, those once-confounding schema definitions become as clear as day,
letting you and your code breathe easier.

**Give it a try!** Nestify has alot of features but don't be intimidated, it was designed to be as natural as possable,
nestify was designed to be readable by someone who has never seen it after just a few minuites.
Most of this guide is examples, since i beleive its easiest way to learn and i have poor writing writing skills haha.
if you have better writing skills please contribute to this guide.

## Features

- Simplify nested struct and enum definitions in Rust.
- Make your codebase more readable and less verbose.
- Ideal for modeling complex API responses.
- Advanced attribute modifiers.
- Works well with Serde.
- Intuitive syntax

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nestify = "0.3.1"
```

Then, in your project, add the crate and use the macro:

```rust
use nestify::nest;
```

## Quick Examples

### Simple Nested Structures

Here's a quick example to show how Nestify simplifies nested struct definitions:

```rust
// Define a user profile with nested address and preferences structures
nest! {
    struct UserProfile {
        name: String,
        address: struct Address {
            street: String,
            city: String,
        },
        preferences: struct Preferences {
            newsletter: bool,
        },
    }
}
```


<details class="expand">
    <summary>
    Expand
    </summary>

```rust
struct UserProfile {
    name: String,
    address: Address,
    preferences: Preferences,
}
struct Address {
    street: String,
    city: String,
}
struct Preferences {
    newsletter: bool,
}
```

</details>

### Simple Nested Enums
```rust
// Define a task with a nested status enum
nest! {
    struct Task {
        id: i32,
        description: String,
        status: enum Status {
            Pending,
            InProgress,
            Completed,
        },
    }
}
```

<details class="expand">
    <summary>
    Expand
    </summary>

```rust
struct Task {
    id: i32,
    description: String,
    status: Status,
}
enum Status {
    Pending,
    InProgress,
    Completed,
}

```

</details>


## Basic

```rust
use nestify::nest;

nest!{
    struct Outer {
        field1: struct Inner {
            subfield1: i32,
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
}
```

Enums can also be nested in structures like this!

```rust
nest! {
    struct Outer (
        enum Inner {
            Variant1,
            Variant2(i32),
            Variant3 {
                f: i32
            }
        }
    )
}
```

## Supported definitions

Nestify supports both [structs](https://doc.rust-lang.org/reference/expressions/struct-expr.html) and [enums](https://doc.rust-lang.org/reference/items/enumerations.html).

```rust
// field structs (named)
nest! {
    struct Named {
        f: struct Nested {}
    }
}

// tuple structs (unnamed)
nest! {
    struct Unnamed(struct Nested())
}

// unit structs
nest! {
    struct Unit {
        unit: struct UnitStruct
    }
}


// enums
nest! {
    enum EnumVariants {
        Unit,
        Tuple(i32, struct TupleNested),
        Struct {
            f1: i32,

        }
        DiscriminantVariant = 1,
    }
}
// note: any variant can have a discriminant
// just as in normal rust
```

### A note about types
*However,* defining items in type genrics is not supported (*yet*).
This: `struct One(Vec<struct Two>)` is invalid right now.

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

When defining nested generics, you need to add generics to types. Enter "FishHook" syntax.
To define generics on the field use `||<...>`. This will let you specify the nested generic types.
It also works with lifetimes if needed

```rust
nest! {
    struct Parent {
        child : struct Child<C> {
            s: &'c str,
            f: C
        } ||<i32>
    }
}
```

this will expand like this to the field. It allows you to type

```rust
// ~~snip~~
child: Child<i32>,
// ~~snip~~
```

## Attributes

You can apply attributes just like you would with a normal struct.

```rust
nest! {
    #[derive(Clone)]
    struct CloneMe {}
}
let x = CloneMe {};
let cl = x.clone();
```

#### `#[meta]*`

Using `*` syntax you can inherit attributes to child structures easily.

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

#### `#[meta]/`

You can end the recursion with a `/` attribute modifier.
It will remove a recursive attribute from the current structure and all nested structures
for example

```rust
nest! {
    #[nest]*
    struct One {
        two: struct Two {
            three: #[nest]/ 
            struct Three {
                four: struct Four { }
            }
        }
    }
}
```

will expand too

```rust
struct Four {}
struct Three {
    four: Four,
}
#[nest]
struct Two {
    three: Three,
}
#[nest]
struct One {
    two: Two,
}
```

#### `#[meta]-`

Using the `-` modifier will remove a recursive attribute from a single structure
To use the previous example using `-` instead of `/` will expand to this

```rs
#[nest]
struct Four {}
struct Three {
    four: Four,
}
#[nest]
struct Two {
    three: Three,
}
#[nest]
struct One {
    two: Two,
}
```

### Field Attributes `#>[meta]`

If you structure has many defined attributes it can become awkward to define attributes before the nested structure.
To combat this you can define attributes that apply to nested objects before fields and enum variants.
This can be accomplished by using `#>[meta]` notice the `>`.

For example:

```rs
nest! {
    struct MyStruct {
        #>[derive(Debug)]
        f: struct DebugableStruct { } 
    }
}

// becomes: 
#[derive(Debug)]
struct DebugableStruct {}

struct MyStruct {
    f: DebugableStruct,
}
```

## SemiColons

Rust mandates semicolons to mark the end of tuple struct and unit struct declarations. Nestify, however, introduces flexibility by making this semicolon optional.

### Rust Standard

- Tuple struct: `struct MyTuple(i32, String);`
- Unit struct: `struct MyUnit;`

### Nestify Flexibility

With Nestify, you can omit the semicolon without any impact:

```rust
// Unit struct without a semicolon
nest! {
    struct MyUnit
}

// Tuple struct without a semicolon
nest! {
    struct MyTuple(i32, String)
}
```
<details class="expand">
    <summary>
    Expand
    </summary>

```rust
struct MyUnit;
//           ^ automaticly added

struct MyTuple(i32, String);
//                         ^ automaticly added
```

</details>
<br>

This adjustment simplifies syntax, particularly in the context of defining nested structures, aligning with Nestify's goal of enhancing code readability and maintenance. Whether you include the semicolon or not, Nestify processes the definitions correctly, thanks to its domain-specific optimizations.

--- 

## Limitations

n Nestify, while you can work with a wide range of complex types to structure your data effectively, there's a specific limitation regarding the definition of new types directly within the generics of other types. This limitation affects scenarios where you might want to dynamically define a struct or enum inside a generic container like `Vec<T>`, `Option<T>`, or `Result<T, E>` as part of the type declaration.

The limitation is specifically around embedding a new type definition within the generic parameters of another type. For instance:

```rust
// This pattern is not supported:
struct One(Vec<struct Two { field: i32 }>);
```

Here, `struct Two` is being defined directly within the generic parameter of `Vec<T>`, which is not currently possible with Nestify.

### Another Example

To further illustrate, consider a scenario where you want to include an optional configuration struct within another struct:

```rust
// This pattern is also not supported:
struct AppConfig(Option<struct DatabaseConfig { url: String }>);
```

In this example, `struct DatabaseConfig` is defined directly within the `Option<T>` generic type in the declaration of `AppConfig`. This specific way of defining `DatabaseConfig` inline as part of the `AppConfig` declaration is not supported by Nestify at the moment.


Notice how `#[derive(Debug)]` is applied not to the field as an attribute
but to the nested structure. Applied field attributes are compatable with attribute modifiers.
Regular field attribtues are not compatable with attributes.


---

## Contributing

I love contributers. Im an bad writer so I would love comunity support to improve this guide!

To make code changes:

1. Fork the repository.
2. Create a new branch for your features or bug fixes.
3. Write tests for your changes.
4. Make sure all tests pass.
5. Submit a pull request.

Standard stuff!

## License

This project is licensed under the MIT License. If you need it under a diffrent licence *Contact Me*.
MIT licence support will always be maintained. Dont fear!

## Contact me

Check github for information @snowfoxsh
