# Nestify

Nestify provides a Rust macro for streamlined nested struct definitions, enhancing code readability. It's especially valuable for handling API responses.

[![Crates.io](https://img.shields.io/crates/v/nestify.svg)](https://crates.io/crates/nestify)
[![Documentation](https://docs.rs/nestify/badge.svg)](https://docs.rs/nestify)


## Abstract
Nestify allows "Definition is Type" support. What does that mean? 
Basicly instead of defining a type then later defining the type,
You can do it all in one step. 
This makes certen code significantly more readable since you dont have to hunting for structure definitions
and instead you can just see it all in one place. Consider a JSON REST API response that you want to Desearlize. 
Most JSON responses are very "deep/nested" and thus when defining a schemea to encompas it things become unclear.
The nest macro makes the schema object much easier to read and understaand. 

**Give it a try!** Nestify has alot of features but don't be intimidated, it was designed to be as natural as possable,
nestify was designed to be readable by someone who has never seen it after just a few minuites. 
Most of this guide is examples, since i beleive its the easiest way to learn and i have poor writing writing skills haha. 
if you have better writing skills please contribute to this guide. 



## Features

- Simplify nested struct and enum definitions in Rust.
- Make your codebase more readable and less verbose.
- Ideal for modeling complex API responses.
- Advanced attribute modifiers.
- Workds well with Serde.
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

## Supported definitions
Nestify supports both [structs](https://doc.rust-lang.org/reference/expressions/struct-expr.html) and [enums](https://doc.rust-lang.org/reference/items/enumerations.html).
See the links for more information on structures and enumerations in rust. All items can be embeded in each other!

```rs
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
All "complex" types are supported.
By "complex" I mean something like `Result<Option<Value>, Error>` in comparison to a "simple" type
such as `usize` that is just a single word.

*However* defining items in type genrics is not supported (*yet*).
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
When defining nested generics you need to add generics to types. Enter "FishHook" syntax.
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

Using the `-` modifier will will remove a recursive attribute from a single structure
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

Notice how `#[derive(Debug)]` is applied not to the field as an attribute
but to the nested structure. Applied field attributes are compatable with attribute modifiers.
Regular field attribtues are not compatable with attributes.


## Semi colons
In rust tuple structs and unit structs must be followed by a semi colon. 
Nestify is domain limited enough to make this optional!

You can write `struct Unit;,` or `struct Unit,` there is no diffrence


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