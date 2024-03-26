# Nestify

Nestify is a Rust library offering a powerful macro to streamline the definition of nested structs and enums.
Designed to improve code readability and maintainability

[<img alt="crates.io" src="https://img.shields.io/crates/v/nestify.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/nestify)
[<img alt="github" src="https://img.shields.io/badge/snowfoxsh/nestify-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/snowfoxsh/nestify)
[<img alt="License" src="https://img.shields.io/crates/l/nestify?style=for-the-badge&labelColor=555555&logo" height="20">](LICENSE)


## Abstract

Nestify re-imagines Rust struct and enum definitions with its "Type is Definition" approach,
streamlining the way you handle nested structures.
Gone are the days of flipping back and forth between type definitions—Nestify
Unifies your codebase, making your code cleaner and far more readable.

Nestify is crafted for ease of learning, with its syntax tailored to be comfortable for Rust developers. The aim is for anyone, even those unfamiliar with the Nest macro, to quickly grasp its concept upon first glance.

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

Then use the macro:

```rust
use nestify::nest;
```

> [!NOTE]
> A nightly toolchain might provide better error diagnostics

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
    <br>

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
    <br>

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

<details class="expand">
    <summary>
    Expand
    </summary>
    <br>


```rust
// field structs (named)
struct Named {
    f: Nested,
}
struct Nested {}

// tuple structs (unnamed)
struct Unnamed(Nested,);
struct Nested();

// unit structs
struct Unit {
    unit: UnitStruct,
}
struct UnitStruct;


// enums
enum EnumVariants {
    Unit,
    Tuple(i32, TupleNested),
    Struct { 
        f1: i32 
    },
    DiscriminantVariant = 1,
}
struct TupleNested;
```

</details>

## Generics
Nestify fully supports Rust's generic parameters. This compatibility ensures that you can incorporate both lifetime and type parameters within your nested struct definitions, just as you would in standard Rust code.

```rust
nest! {
    struct Example<'a, T> {
        s: &'a str,
        t: T
    }
}
```
<details class="expand">
    <summary>
    Expand
    </summary>
    <br>


```rust
struct Example<'a, T> { 
    s: &'a str, 
    t: T, 
}
```
</details>

### Nested Generics

When defining nested generics, you need to add generics to types. Enter "FishHook" syntax.
To define generics on the field use `||<...>`. This will let you specify the nested generic types.
It also works with lifetimes if needed. 

```rust
nest! {
    struct Parent<'a> {
        child : struct Child<'c, C> {
            s: &'c str,
            f: C
        } ||<'a, i32>
    }
}
```

<details class="expand">
    <summary>
    Expand
    </summary>
    <br>

```rust
struct Parent<'a> {
    child: Child<'a, i32>,
    //           ^^^^^^^^ FishHook expands to this part
}

struct Child<'c, C> {
    s: &'c str,
    f: C,
}
```

</details>

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


### Recursive Attributes **`#[meta]*`**

Using `*` syntax you can inherit attributes to child structures easily. The attribute
will propagate to each nested structure or enum. 

```rust
nest! {
    #[apply_all]*
    struct One {
        two: struct Two {
            three: struct Three {
                payload: ()
            }
        }
    }
}
```

<details class="expand">
    <summary>
    Expand
    </summary>
    <br>


```rust
#[apply_all]
struct One {
    two: Tow,
}

#[apply_all]
struct Two {
    three: Three,
}

#[apply_all]
struct Three {
    payload: (),
}
```

</details>

### Removal Syntax

#### Disable Propagation **`#[meta]/`**

You can end the recursion of an attribute with a `/` attribute modifier.
It will remove a recursive attribute from the current structure and all nested structures

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

<details class="expand">
    <summary>
    Expand
    </summary>
    <br>


```rust
#[nest]
struct One {
    two: Two,
}

#[nest]
struct Two {
    three: Three,
}

struct Three {
    four: Four,
}

struct Four {}
```

</details>

#### Disable Single **`#[meta]-`**

Using the `-` modifier will remove a recursive attribute from a single structure
To use the previous example using `-` instead of `/`:

```rust
nest! {
    #[nest]*
    struct One {
        two: struct Two {
            three: #[nest]- 
            struct Three {
                four: struct Four { }
            }
        }
    }
}
```

<details class="expand">
    <summary>
    Expand
    </summary>
    <br>


```rust
#[nest]
struct One {
    two: Two,
}

#[nest]
struct Two {
    three: Three,
}

struct Three {
    four: Four,
}

#[nest]
struct Four {}
```

</details>

### Field Attributes **`#>[meta]`**

If you structure has many defined attributes, it can become awkward to define attributes before the nested structure. To combat this, you can define attributes that apply to nested objects before fields and enum variants. This can be accomplished by using `#>[meta]` syntax. `#>` will apply the attribute to the next struct.

```rust
nest! {
    struct MyStruct {
        #>[derive(Debug)]
        f: struct DebugableStruct { } 
        // equivlent to: 
        // f: #[derive(Debug)]
        // struct DebugableStruct { }
    }
}
```


<details class="expand">
    <summary>
    Expand
    </summary>
    <br>

```rust
struct MyStruct {
    f: DebugableStruct,
}

#[derive(Debug)]
//       ^^^^^ applied to structure and not field `f`
struct DebugableStruct {}
```

</details>

#### Enum Variant Attributes
Field attributes can also be applied to an enum variant. If there are multiple items defined in a single variant then the attribute will be applied to each.

```rust
nest! {
    enum MyEnum {
        #>[derive(Debug)]
        Variant {
            // #[derive(Debug)
            one: struct One,
            // #[derive(Debug)
            two: struct Two
        }
    }
}
```

<details class="expand">
    <summary>
    Expand
    </summary>
    <br>

```rust
enum MyEnum {
    Variant {
        one: One,
        two: Two,
    }
}

#[derive(Debug)]
struct One;

#[derive(Debug)]
struct Two;
```

</details>

## Semicolons

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
    <br>

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

In Nestify, while you can work with a wide range of complex types to structure your data effectively, there's a specific limitation regarding the definition of new types directly within the generics of other types. This limitation affects scenarios where you might want to dynamically define a struct or enum inside a generic container like `Vec<T>`, `Option<T>`, or `Result<T, E>` as part of the type declaration.

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

---

## Contributing

I love contributors. I'm a bad writer, so I would love community support to improve this guide!

To make code changes:

1. Fork the repository.
2. Create a new branch for your features or bug fixes.
3. Write tests for your changes.
4. Make sure all tests pass.
5. Submit a pull request.

Standard stuff!

## License

This project is licensed under the MIT License. If you need it under a different license *Contact Me*.
MIT license support will always be maintained. Don't fear!

## Contact me

Check github for information @snowfoxsh