# rustdoc_types API (0.39.0)

Types for rustdoc's json output

## 1: Manifest

- Repository: <https://github.com/rust-lang/rustdoc-types>
- License: MIT OR Apache-2.0
- edition: `2018`

### 1.1: Features

- `default`
- `rustc-hash`


## 2: README

### Rustdoc Types

[Docs](https://docs.rs/rustdoc-types/latest/rustdoc_types/)

This crate contains the type definitions for rustdoc's currently-unstable
`--output-format=json` flag. They can be deserialized with `serde-json` from
the output of `cargo +nightly rustdoc -- --output-format json -Z unstable-options`:

```rust
let json_string = std::fs::read_to_string("./target/doc/rustdoc_types.json")?;
let krate: rustdoc_types::Crate = serde_json::from_str(&json_string)?;

println!("the index has {} items", krate.index.len());
```

For performance sensitive crates, consider turning on the `rustc-hash`
feature. This switches all data structures from `std::collections::HashMap` to
`rustc-hash::FxHashMap` which improves performance when reading big JSON files
(like `aws_sdk_rs`'s).

`cargo-semver-checks` benchmarked this change with `aws_sdk_ec2`'s JSON and
[observed a -3% improvement to the runtime][csc benchmarks]. The performance
here depends on how much time you spend querying the `HashMap`s, so as always,
measure first.

[csc benchmarks]: https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc/topic/rustc-hash.20and.20performance.20of.20rustdoc-types/near/474855731

#### Contributing

This repo is a reexport of
[`rustdoc-json-types`](https://github.com/rust-lang/rust/blob/master/src/rustdoc-json-types/lib.rs)
from the rust repo. Any change to the contents of [`src/`](src/), should be sent
to [`rust-lang/rust`](https://github.com/rust-lang/rust/), via their [normal
contribution
procedures](https://rustc-dev-guide.rust-lang.org/contributing.html). Once
reviewed and merged there, the change will be pulled to this repo and published
to crates.io.

##### Release Procedure

1. Run `./update.sh` to pull code from upstream
2. Run `cargo test`
3. Run `./clgen.sh <old_version> <new_version>`
4. Follow printed instructions to commit and push.

#### License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

##### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.




## 3: Module: `rustdoc_types`

Rustdoc's JSON output interface

These types are the public API exposed through the `--output-format json` flag. The [`Crate`]
struct is the root of the JSON blob and all other items are contained within.

We expose a `rustc-hash` feature that is disabled by default. This feature switches the
[`std::collections::HashMap`] for [`rustc_hash::FxHashMap`] to improve the performance of said
`HashMap` in specific situations.

`cargo-semver-checks` for example, saw a [-3% improvement][1] when benchmarking using the
`aws_sdk_ec2` JSON output (~500MB of JSON). As always, we recommend measuring the impact before
turning this feature on, as [`FxHashMap`][2] only concerns itself with hash speed, and may
increase the number of collisions.

[1]: https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc/topic/rustc-hash.20and.20performance.20of.20rustdoc-types/near/474855731
[2]: https://crates.io/crates/rustc-hash


### 3.1: Structs

#### 3.1.1: `struct AssocItemConstraint`

```rust
pub struct AssocItemConstraint {
    pub name: String,
    pub args: rustdoc_types::GenericArgs,
    pub binding: rustdoc_types::AssocItemConstraintKind,
}
```

Describes a bound applied to an associated type/constant.

Example:
```text
IntoIterator<Item = u32, IntoIter: Clone>
             ^^^^^^^^^^  ^^^^^^^^^^^^^^^
```

##### 3.1.1.1: Fields

###### 3.1.1.1.1: `name`

The name of the associated type/constant.

###### 3.1.1.1.2: `args`

Arguments provided to the associated type/constant.

###### 3.1.1.1.3: `binding`

The kind of bound applied to the associated type/constant.

##### 3.1.1.2: Trait Implementations for `AssocItemConstraint`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::AssocItemConstraint {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.2: `struct Constant`

```rust
pub struct Constant {
    pub expr: String,
    pub value: option::Option<String>,
    pub is_literal: bool,
}
```

A constant.

##### 3.1.2.1: Fields

###### 3.1.2.1.1: `expr`

The stringified expression of this constant. Note that its mapping to the original
source code is unstable and it's not guaranteed that it'll match the source code.

###### 3.1.2.1.2: `value`

The value of the evaluated expression for this constant, which is only computed for numeric
types.

###### 3.1.2.1.3: `is_literal`

Whether this constant is a bool, numeric, string, or char literal.

##### 3.1.2.2: Trait Implementations for `Constant`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Constant {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.3: `struct Crate`

```rust
pub struct Crate {
    pub root: rustdoc_types::Id,
    pub crate_version: option::Option<String>,
    pub includes_private: bool,
    pub index: rustc_hash::FxHashMap<rustdoc_types::Id, rustdoc_types::Item>,
    pub paths: rustc_hash::FxHashMap<rustdoc_types::Id, rustdoc_types::ItemSummary>,
    pub external_crates: rustc_hash::FxHashMap<u32, rustdoc_types::ExternalCrate>,
    pub format_version: u32,
}
```

The root of the emitted JSON blob.

It contains all type/documentation information
about the language items in the local crate, as well as info about external items to allow
tools to find or link to them.

##### 3.1.3.1: Fields

###### 3.1.3.1.1: `root`

The id of the root [`Module`] item of the local crate.

###### 3.1.3.1.2: `crate_version`

The version string given to `--crate-version`, if any.

###### 3.1.3.1.3: `includes_private`

Whether or not the output includes private items.

###### 3.1.3.1.4: `index`

A collection of all items in the local crate as well as some external traits and their
items that are referenced locally.

###### 3.1.3.1.5: `paths`

Maps IDs to fully qualified paths and other info helpful for generating links.

###### 3.1.3.1.6: `external_crates`

Maps `crate_id` of items to a crate name and html_root_url if it exists.

###### 3.1.3.1.7: `format_version`

A single version number to be used in the future when making backwards incompatible changes
to the JSON output.

##### 3.1.3.2: Trait Implementations for `Crate`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Crate {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.4: `struct Deprecation`

```rust
pub struct Deprecation {
    pub since: option::Option<String>,
    pub note: option::Option<String>,
}
```

Information about the deprecation of an [`Item`].

##### 3.1.4.1: Fields

###### 3.1.4.1.1: `since`

Usually a version number when this [`Item`] first became deprecated.

###### 3.1.4.1.2: `note`

The reason for deprecation and/or what alternatives to use.

##### 3.1.4.2: Trait Implementations for `Deprecation`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Deprecation {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.5: `struct Discriminant`

```rust
pub struct Discriminant {
    pub expr: String,
    pub value: String,
}
```

The value that distinguishes a variant in an [`Enum`] from other variants.

##### 3.1.5.1: Fields

###### 3.1.5.1.1: `expr`

The expression that produced the discriminant.

Unlike `value`, this preserves the original formatting (eg suffixes,
hexadecimal, and underscores), making it unsuitable to be machine
interpreted.

In some cases, when the value is too complex, this may be `"{ _ }"`.
When this occurs is unstable, and may change without notice.

###### 3.1.5.1.2: `value`

The numerical value of the discriminant. Stored as a string due to
JSON's poor support for large integers, and the fact that it would need
to store from [`i128::MIN`] to [`u128::MAX`].

##### 3.1.5.2: Trait Implementations for `Discriminant`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Discriminant {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.6: `struct DynTrait`

```rust
pub struct DynTrait {
    pub traits: Vec<rustdoc_types::PolyTrait>,
    pub lifetime: option::Option<String>,
}
```

Dynamic trait object type (`dyn Trait`).

##### 3.1.6.1: Fields

###### 3.1.6.1.1: `traits`

All the traits implemented. One of them is the vtable, and the rest must be auto traits.

###### 3.1.6.1.2: `lifetime`

The lifetime of the whole dyn object
```text
dyn Debug + 'static
            ^^^^^^^
            |
            this part
```

##### 3.1.6.2: Trait Implementations for `DynTrait`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::DynTrait {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.7: `struct Enum`

```rust
pub struct Enum {
    pub generics: rustdoc_types::Generics,
    pub has_stripped_variants: bool,
    pub variants: Vec<rustdoc_types::Id>,
    pub impls: Vec<rustdoc_types::Id>,
}
```

An `enum`.

##### 3.1.7.1: Fields

###### 3.1.7.1.1: `generics`

Information about the type parameters and `where` clauses of the enum.

###### 3.1.7.1.2: `has_stripped_variants`

Whether any variants have been removed from the result, due to being private or hidden.

###### 3.1.7.1.3: `variants`

The list of variants in the enum.

All of the corresponding [`Item`]s are of kind [`ItemEnum::Variant`]

###### 3.1.7.1.4: `impls`

`impl`s for the enum.

##### 3.1.7.2: Trait Implementations for `Enum`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Enum {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.8: `struct ExternalCrate`

```rust
pub struct ExternalCrate {
    pub name: String,
    pub html_root_url: option::Option<String>,
}
```

Metadata of a crate, either the same crate on which `rustdoc` was invoked, or its dependency.

##### 3.1.8.1: Fields

###### 3.1.8.1.1: `name`

The name of the crate.

Note: This is the [*crate* name][crate-name], which may not be the same as the
[*package* name][package-name]. For example, for <https://crates.io/crates/regex-syntax>,
this field will be `regex_syntax` (which uses an `_`, not a `-`).

[crate-name]: https://doc.rust-lang.org/stable/cargo/reference/cargo-targets.html#the-name-field
[package-name]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-name-field

###### 3.1.8.1.2: `html_root_url`

The root URL at which the crate's documentation lives.

##### 3.1.8.2: Trait Implementations for `ExternalCrate`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::ExternalCrate {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.9: `struct Function`

```rust
pub struct Function {
    pub sig: rustdoc_types::FunctionSignature,
    pub generics: rustdoc_types::Generics,
    pub header: rustdoc_types::FunctionHeader,
    pub has_body: bool,
}
```

A function declaration (including methods and other associated functions).

##### 3.1.9.1: Fields

###### 3.1.9.1.1: `sig`

Information about the function signature, or declaration.

###### 3.1.9.1.2: `generics`

Information about the function’s type parameters and `where` clauses.

###### 3.1.9.1.3: `header`

Information about core properties of the function, e.g. whether it's `const`, its ABI, etc.

###### 3.1.9.1.4: `has_body`

Whether the function has a body, i.e. an implementation.

##### 3.1.9.2: Trait Implementations for `Function`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Function {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.10: `struct FunctionHeader`

```rust
pub struct FunctionHeader {
    pub is_const: bool,
    pub is_unsafe: bool,
    pub is_async: bool,
    pub abi: rustdoc_types::Abi,
}
```

A set of fundamental properties of a function.

##### 3.1.10.1: Fields

###### 3.1.10.1.1: `is_const`

Is this function marked as `const`?

###### 3.1.10.1.2: `is_unsafe`

Is this function unsafe?

###### 3.1.10.1.3: `is_async`

Is this function async?

###### 3.1.10.1.4: `abi`

The ABI used by the function.

##### 3.1.10.2: Trait Implementations for `FunctionHeader`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::FunctionHeader {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.11: `struct FunctionPointer`

```rust
pub struct FunctionPointer {
    pub sig: rustdoc_types::FunctionSignature,
    pub generic_params: Vec<rustdoc_types::GenericParamDef>,
    pub header: rustdoc_types::FunctionHeader,
}
```

A type that is a function pointer.

##### 3.1.11.1: Fields

###### 3.1.11.1.1: `sig`

The signature of the function.

###### 3.1.11.1.2: `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)

```ignore (incomplete expression)
   for<'c> fn(val: &'c i32) -> i32
// ^^^^^^^
```

###### 3.1.11.1.3: `header`

The core properties of the function, such as the ABI it conforms to, whether it's unsafe, etc.

##### 3.1.11.2: Trait Implementations for `FunctionPointer`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::FunctionPointer {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.12: `struct FunctionSignature`

```rust
pub struct FunctionSignature {
    pub inputs: Vec<(String, rustdoc_types::Type)>,
    pub output: option::Option<rustdoc_types::Type>,
    pub is_c_variadic: bool,
}
```

The signature of a function.

##### 3.1.12.1: Fields

###### 3.1.12.1.1: `inputs`

List of argument names and their type.

Note that not all names will be valid identifiers, as some of
them may be patterns.

###### 3.1.12.1.2: `output`

The output type, if specified.

###### 3.1.12.1.3: `is_c_variadic`

Whether the function accepts an arbitrary amount of trailing arguments the C way.

```ignore (incomplete code)
fn printf(fmt: &str, ...);
```

##### 3.1.12.2: Trait Implementations for `FunctionSignature`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::FunctionSignature {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.13: `struct GenericParamDef`

```rust
pub struct GenericParamDef {
    pub name: String,
    pub kind: rustdoc_types::GenericParamDefKind,
}
```

One generic parameter accepted by an item.

##### 3.1.13.1: Fields

###### 3.1.13.1.1: `name`

Name of the parameter.
```rust
fn f<'resource, Resource>(x: &'resource Resource) {}
//    ^^^^^^^^  ^^^^^^^^
```

###### 3.1.13.1.2: `kind`

The kind of the parameter and data specific to a particular parameter kind, e.g. type
bounds.

##### 3.1.13.2: Trait Implementations for `GenericParamDef`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::GenericParamDef {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.14: `struct Generics`

```rust
pub struct Generics {
    pub params: Vec<rustdoc_types::GenericParamDef>,
    pub where_predicates: Vec<rustdoc_types::WherePredicate>,
}
```

Generic parameters accepted by an item and `where` clauses imposed on it and the parameters.

##### 3.1.14.1: Fields

###### 3.1.14.1.1: `params`

A list of generic parameter definitions (e.g. `<T: Clone + Hash, U: Copy>`).

###### 3.1.14.1.2: `where_predicates`

A list of where predicates (e.g. `where T: Iterator, T::Item: Copy`).

##### 3.1.14.2: Trait Implementations for `Generics`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Generics {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.15: `struct Id`

```rust
pub struct Id(pub u32);
```

An opaque identifier for an item.

It can be used to lookup in [`Crate::index`] or [`Crate::paths`] to resolve it
to an [`Item`].

Id's are only valid within a single JSON blob. They cannot be used to
resolve references between the JSON output's for different crates.

Rustdoc makes no guarantees about the inner value of Id's. Applications
should treat them as opaque keys to lookup items, and avoid attempting
to parse them, or otherwise depend on any implementation details.

##### 3.1.15.1: Trait Implementations for `Id`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Copy`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Id {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.16: `struct Impl`

```rust
pub struct Impl {
    pub is_unsafe: bool,
    pub generics: rustdoc_types::Generics,
    pub provided_trait_methods: Vec<String>,
    pub trait_: option::Option<rustdoc_types::Path>,
    pub for_: rustdoc_types::Type,
    pub items: Vec<rustdoc_types::Id>,
    pub is_negative: bool,
    pub is_synthetic: bool,
    pub blanket_impl: option::Option<rustdoc_types::Type>,
}
```

An `impl` block.

##### 3.1.16.1: Fields

###### 3.1.16.1.1: `is_unsafe`

Whether this impl is for an unsafe trait.

###### 3.1.16.1.2: `generics`

Information about the impl’s type parameters and `where` clauses.

###### 3.1.16.1.3: `provided_trait_methods`

The list of the names of all the trait methods that weren't mentioned in this impl but
were provided by the trait itself.

For example, for this impl of the [`PartialEq`] trait:
```rust
struct Foo;

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool { todo!() }
}
```
This field will be `["ne"]`, as it has a default implementation defined for it.

###### 3.1.16.1.4: `trait_`

The trait being implemented or `None` if the impl is inherent, which means
`impl Struct {}` as opposed to `impl Trait for Struct {}`.

###### 3.1.16.1.5: `for_`

The type that the impl block is for.

###### 3.1.16.1.6: `items`

The list of associated items contained in this impl block.

###### 3.1.16.1.7: `is_negative`

Whether this is a negative impl (e.g. `!Sized` or `!Send`).

###### 3.1.16.1.8: `is_synthetic`

Whether this is an impl that’s implied by the compiler
(for autotraits, e.g. `Send` or `Sync`).

##### 3.1.16.2: Trait Implementations for `Impl`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Impl {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.17: `struct Item`

```rust
pub struct Item {
    pub id: rustdoc_types::Id,
    pub crate_id: u32,
    pub name: option::Option<String>,
    pub span: option::Option<rustdoc_types::Span>,
    pub visibility: rustdoc_types::Visibility,
    pub docs: option::Option<String>,
    pub links: rustc_hash::FxHashMap<String, rustdoc_types::Id>,
    pub attrs: Vec<String>,
    pub deprecation: option::Option<rustdoc_types::Deprecation>,
    pub inner: rustdoc_types::ItemEnum,
}
```

Anything that can hold documentation - modules, structs, enums, functions, traits, etc.

The `Item` data type holds fields that can apply to any of these,
and leaves kind-specific details (like function args or enum variants) to the `inner` field.

##### 3.1.17.1: Fields

###### 3.1.17.1.1: `id`

The unique identifier of this item. Can be used to find this item in various mappings.

###### 3.1.17.1.2: `crate_id`

This can be used as a key to the `external_crates` map of [`Crate`] to see which crate
this item came from.

###### 3.1.17.1.3: `name`

Some items such as impls don't have names.

###### 3.1.17.1.4: `span`

The source location of this item (absent if it came from a macro expansion or inline
assembly).

###### 3.1.17.1.5: `visibility`

By default all documented items are public, but you can tell rustdoc to output private items
so this field is needed to differentiate.

###### 3.1.17.1.6: `docs`

The full markdown docstring of this item. Absent if there is no documentation at all,
Some("") if there is some documentation but it is empty (EG `#[doc = ""]`).

###### 3.1.17.1.7: `links`

This mapping resolves [intra-doc links](https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md) from the docstring to their IDs

###### 3.1.17.1.8: `attrs`

Attributes on this item.

Does not include `#[deprecated]` attributes: see the [`Self::deprecation`] field instead.

Some attributes appear in pretty-printed Rust form, regardless of their formatting
in the original source code. For example:
- `#[non_exhaustive]` and `#[must_use]` are represented as themselves.
- `#[no_mangle]` and `#[export_name]` are also represented as themselves.
- `#[repr(C)]` and other reprs also appear as themselves,
  though potentially with a different order: e.g. `repr(i8, C)` may become `repr(C, i8)`.
  Multiple repr attributes on the same item may be combined into an equivalent single attr.

Other attributes may appear debug-printed. For example:
- `#[inline]` becomes something similar to `#[attr="Inline(Hint)"]`.

As an internal implementation detail subject to change, this debug-printing format
is currently equivalent to the HIR pretty-printing of parsed attributes.

###### 3.1.17.1.9: `deprecation`

Information about the item’s deprecation, if present.

###### 3.1.17.1.10: `inner`

The type-specific fields describing this item.

##### 3.1.17.2: Trait Implementations for `Item`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Item {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.18: `struct ItemSummary`

```rust
pub struct ItemSummary {
    pub crate_id: u32,
    pub path: Vec<String>,
    pub kind: rustdoc_types::ItemKind,
}
```

Information about an external (not defined in the local crate) [`Item`].

For external items, you don't get the same level of
information. This struct should contain enough to generate a link/reference to the item in
question, or can be used by a tool that takes the json output of multiple crates to find
the actual item definition with all the relevant info.

##### 3.1.18.1: Fields

###### 3.1.18.1.1: `crate_id`

Can be used to look up the name and html_root_url of the crate this item came from in the
`external_crates` map.

###### 3.1.18.1.2: `path`

The list of path components for the fully qualified path of this item (e.g.
`["std", "io", "lazy", "Lazy"]` for `std::io::lazy::Lazy`).

Note that items can appear in multiple paths, and the one chosen is implementation
defined. Currently, this is the full path to where the item was defined. Eg
[`String`] is currently `["alloc", "string", "String"]` and [`HashMap`][`std::collections::HashMap`]
is `["std", "collections", "hash", "map", "HashMap"]`, but this is subject to change.

###### 3.1.18.1.3: `kind`

Whether this item is a struct, trait, macro, etc.

##### 3.1.18.2: Trait Implementations for `ItemSummary`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::ItemSummary {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.19: `struct Module`

```rust
pub struct Module {
    pub is_crate: bool,
    pub items: Vec<rustdoc_types::Id>,
    pub is_stripped: bool,
}
```

A module declaration, e.g. `mod foo;` or `mod foo {}`.

##### 3.1.19.1: Fields

###### 3.1.19.1.1: `is_crate`

Whether this is the root item of a crate.

This item doesn't correspond to any construction in the source code and is generated by the
compiler.

###### 3.1.19.1.2: `items`

[`Item`]s declared inside this module.

###### 3.1.19.1.3: `is_stripped`

If `true`, this module is not part of the public API, but it contains
items that are re-exported as public API.

##### 3.1.19.2: Trait Implementations for `Module`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Module {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.20: `struct Path`

```rust
pub struct Path {
    pub path: String,
    pub id: rustdoc_types::Id,
    pub args: option::Option<Box<rustdoc_types::GenericArgs>>,
}
```

A type that has a simple path to it. This is the kind of type of structs, unions, enums, etc.

##### 3.1.20.1: Fields

###### 3.1.20.1.1: `path`

The path of the type.

This will be the path that is *used* (not where it is defined), so
multiple `Path`s may have different values for this field even if
they all refer to the same item. e.g.

```rust
pub type Vec1 = std::vec::Vec<i32>; // path: "std::vec::Vec"
pub type Vec2 = Vec<i32>; // path: "Vec"
pub type Vec3 = std::prelude::v1::Vec<i32>; // path: "std::prelude::v1::Vec"
```

###### 3.1.20.1.2: `id`

The ID of the type.

###### 3.1.20.1.3: `args`

Generic arguments to the type.

```ignore (incomplete expression)
std::borrow::Cow<'static, str>
//              ^^^^^^^^^^^^^^
```

##### 3.1.20.2: Trait Implementations for `Path`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Path {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.21: `struct PolyTrait`

```rust
pub struct PolyTrait {
    pub trait_: rustdoc_types::Path,
    pub generic_params: Vec<rustdoc_types::GenericParamDef>,
}
```

A trait and potential HRTBs

##### 3.1.21.1: Fields

###### 3.1.21.1.1: `trait_`

The path to the trait.

###### 3.1.21.1.2: `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)
```text
dyn for<'a> Fn() -> &'a i32"
    ^^^^^^^
```

##### 3.1.21.2: Trait Implementations for `PolyTrait`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::PolyTrait {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.22: `struct Primitive`

```rust
pub struct Primitive {
    pub name: String,
    pub impls: Vec<rustdoc_types::Id>,
}
```

A primitive type declaration. Declarations of this kind can only come from the core library.

##### 3.1.22.1: Fields

###### 3.1.22.1.1: `name`

The name of the type.

###### 3.1.22.1.2: `impls`

The implementations, inherent and of traits, on the primitive type.

##### 3.1.22.2: Trait Implementations for `Primitive`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Primitive {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.23: `struct ProcMacro`

```rust
pub struct ProcMacro {
    pub kind: rustdoc_types::MacroKind,
    pub helpers: Vec<String>,
}
```

A procedural macro.

##### 3.1.23.1: Fields

###### 3.1.23.1.1: `kind`

How this macro is supposed to be called: `foo!()`, `#[foo]` or `#[derive(foo)]`

###### 3.1.23.1.2: `helpers`

Helper attributes defined by a macro to be used inside it.

Defined only for derive macros.

E.g. the [`Default`] derive macro defines a `#[default]` helper attribute so that one can
do:

```rust
#[derive(Default)]
enum Option<T> {
    #[default]
    None,
    Some(T),
}
```

##### 3.1.23.2: Trait Implementations for `ProcMacro`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::ProcMacro {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.24: `struct Span`

```rust
pub struct Span {
    pub filename: path::PathBuf,
    pub begin: (usize, usize),
    pub end: (usize, usize),
}
```

A range of source code.

##### 3.1.24.1: Fields

###### 3.1.24.1.1: `filename`

The path to the source file for this span relative to the path `rustdoc` was invoked with.

###### 3.1.24.1.2: `begin`

Zero indexed Line and Column of the first character of the `Span`

###### 3.1.24.1.3: `end`

Zero indexed Line and Column of the last character of the `Span`

##### 3.1.24.2: Trait Implementations for `Span`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Span {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.25: `struct Static`

```rust
pub struct Static {
    pub type_: rustdoc_types::Type,
    pub is_mutable: bool,
    pub expr: String,
    pub is_unsafe: bool,
}
```

A `static` declaration.

##### 3.1.25.1: Fields

###### 3.1.25.1.1: `type_`

The type of the static.

###### 3.1.25.1.2: `is_mutable`

This is `true` for mutable statics, declared as `static mut X: T = f();`

###### 3.1.25.1.3: `expr`

The stringified expression for the initial value.

It's not guaranteed that it'll match the actual source code for the initial value.

###### 3.1.25.1.4: `is_unsafe`

Is the static `unsafe`?

This is only true if it's in an `extern` block, and not explicity marked
as `safe`.

```rust
unsafe extern {
    static A: i32;      // unsafe
    safe static B: i32; // safe
}

static C: i32 = 0;     // safe
static mut D: i32 = 0; // safe
```

##### 3.1.25.2: Trait Implementations for `Static`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Static {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.26: `struct Struct`

```rust
pub struct Struct {
    pub kind: rustdoc_types::StructKind,
    pub generics: rustdoc_types::Generics,
    pub impls: Vec<rustdoc_types::Id>,
}
```

A `struct`.

##### 3.1.26.1: Fields

###### 3.1.26.1.1: `kind`

The kind of the struct (e.g. unit, tuple-like or struct-like) and the data specific to it,
i.e. fields.

###### 3.1.26.1.2: `generics`

The generic parameters and where clauses on this struct.

###### 3.1.26.1.3: `impls`

All impls (both of traits and inherent) for this struct.
All of the corresponding [`Item`]s are of kind [`ItemEnum::Impl`].

##### 3.1.26.2: Trait Implementations for `Struct`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Struct {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.27: `struct Trait`

```rust
pub struct Trait {
    pub is_auto: bool,
    pub is_unsafe: bool,
    pub is_dyn_compatible: bool,
    pub items: Vec<rustdoc_types::Id>,
    pub generics: rustdoc_types::Generics,
    pub bounds: Vec<rustdoc_types::GenericBound>,
    pub implementations: Vec<rustdoc_types::Id>,
}
```

A `trait` declaration.

##### 3.1.27.1: Fields

###### 3.1.27.1.1: `is_auto`

Whether the trait is marked `auto` and is thus implemented automatically
for all applicable types.

###### 3.1.27.1.2: `is_unsafe`

Whether the trait is marked as `unsafe`.

###### 3.1.27.1.3: `is_dyn_compatible`

Whether the trait is [dyn compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)[^1].

[^1]: Formerly known as "object safe".

###### 3.1.27.1.4: `items`

Associated [`Item`]s that can/must be implemented by the `impl` blocks.

###### 3.1.27.1.5: `generics`

Information about the type parameters and `where` clauses of the trait.

###### 3.1.27.1.6: `bounds`

Constraints that must be met by the implementor of the trait.

###### 3.1.27.1.7: `implementations`

The implementations of the trait.

##### 3.1.27.2: Trait Implementations for `Trait`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Trait {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.28: `struct TraitAlias`

```rust
pub struct TraitAlias {
    pub generics: rustdoc_types::Generics,
    pub params: Vec<rustdoc_types::GenericBound>,
}
```

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

##### 3.1.28.1: Fields

###### 3.1.28.1.1: `generics`

Information about the type parameters and `where` clauses of the alias.

###### 3.1.28.1.2: `params`

The bounds that are associated with the alias.

##### 3.1.28.2: Trait Implementations for `TraitAlias`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::TraitAlias {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.29: `struct TypeAlias`

```rust
pub struct TypeAlias {
    pub type_: rustdoc_types::Type,
    pub generics: rustdoc_types::Generics,
}
```

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

##### 3.1.29.1: Fields

###### 3.1.29.1.1: `type_`

The type referred to by this alias.

###### 3.1.29.1.2: `generics`

Information about the type parameters and `where` clauses of the alias.

##### 3.1.29.2: Trait Implementations for `TypeAlias`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::TypeAlias {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.30: `struct Union`

```rust
pub struct Union {
    pub generics: rustdoc_types::Generics,
    pub has_stripped_fields: bool,
    pub fields: Vec<rustdoc_types::Id>,
    pub impls: Vec<rustdoc_types::Id>,
}
```

A `union`.

##### 3.1.30.1: Fields

###### 3.1.30.1.1: `generics`

The generic parameters and where clauses on this union.

###### 3.1.30.1.2: `has_stripped_fields`

Whether any fields have been removed from the result, due to being private or hidden.

###### 3.1.30.1.3: `fields`

The list of fields in the union.

All of the corresponding [`Item`]s are of kind [`ItemEnum::StructField`].

###### 3.1.30.1.4: `impls`

All impls (both of traits and inherent) for this union.

All of the corresponding [`Item`]s are of kind [`ItemEnum::Impl`].

##### 3.1.30.2: Trait Implementations for `Union`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Union {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.31: `struct Use`

```rust
pub struct Use {
    pub source: String,
    pub name: String,
    pub id: option::Option<rustdoc_types::Id>,
    pub is_glob: bool,
}
```

A `use` statement.

##### 3.1.31.1: Fields

###### 3.1.31.1.1: `source`

The full path being imported.

###### 3.1.31.1.2: `name`

May be different from the last segment of `source` when renaming imports:
`use source as name;`

###### 3.1.31.1.3: `id`

The ID of the item being imported. Will be `None` in case of re-exports of primitives:
```rust
pub use i32 as my_i32;
```

###### 3.1.31.1.4: `is_glob`

Whether this statement is a wildcard `use`, e.g. `use source::*;`

##### 3.1.31.2: Trait Implementations for `Use`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Use {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.1.32: `struct Variant`

```rust
pub struct Variant {
    pub kind: rustdoc_types::VariantKind,
    pub discriminant: option::Option<rustdoc_types::Discriminant>,
}
```

A variant of an enum.

##### 3.1.32.1: Fields

###### 3.1.32.1.1: `kind`

Whether the variant is plain, a tuple-like, or struct-like. Contains the fields.

###### 3.1.32.1.2: `discriminant`

The discriminant, if explicitly specified.

##### 3.1.32.2: Trait Implementations for `Variant`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Variant {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


### 3.2: Enums

#### 3.2.1: `enum Abi`

```rust
pub enum Abi {
    Rust,
    C { unwind: bool },
    Cdecl { unwind: bool },
    Stdcall { unwind: bool },
    Fastcall { unwind: bool },
    Aapcs { unwind: bool },
    Win64 { unwind: bool },
    SysV64 { unwind: bool },
    System { unwind: bool },
    Other(String),
}
```

The ABI (Application Binary Interface) used by a function.

If a variant has an `unwind` field, this means the ABI that it represents can be specified in 2
ways: `extern "_"` and `extern "_-unwind"`, and a value of `true` for that field signifies the
latter variant.

See the [Rustonomicon section](https://doc.rust-lang.org/nightly/nomicon/ffi.html#ffi-and-unwinding)
on unwinding for more info.

##### 3.2.1.2: Variants

###### 3.2.1.2.2: `Rust`

The default ABI, but that can also be written explicitly with `extern "Rust"`.

###### 3.2.1.2.3: `C { unwind: bool }`

Can be specified as `extern "C"` or, as a shorthand, just `extern`.

###### 3.2.1.2.4: `Cdecl { unwind: bool }`

Can be specified as `extern "cdecl"`.

###### 3.2.1.2.5: `Stdcall { unwind: bool }`

Can be specified as `extern "stdcall"`.

###### 3.2.1.2.6: `Fastcall { unwind: bool }`

Can be specified as `extern "fastcall"`.

###### 3.2.1.2.7: `Aapcs { unwind: bool }`

Can be specified as `extern "aapcs"`.

###### 3.2.1.2.8: `Win64 { unwind: bool }`

Can be specified as `extern "win64"`.

###### 3.2.1.2.9: `SysV64 { unwind: bool }`

Can be specified as `extern "sysv64"`.

###### 3.2.1.2.10: `System { unwind: bool }`

Can be specified as `extern "system"`.

###### 3.2.1.2.11: `Other(String)`

Any other ABI, including unstable ones.

##### 3.2.1.2: Trait Implementations for `Abi`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Abi {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.2: `enum AssocItemConstraintKind`

```rust
pub enum AssocItemConstraintKind {
    Equality(rustdoc_types::Term),
    Constraint(Vec<rustdoc_types::GenericBound>),
}
```

The way in which an associate type/constant is bound.

##### 3.2.2.2: Variants

###### 3.2.2.2.2: `Equality(rustdoc_types::Term)`

The required value/type is specified exactly. e.g.
```text
Iterator<Item = u32, IntoIter: DoubleEndedIterator>
         ^^^^^^^^^^
```

###### 3.2.2.2.3: `Constraint(Vec<rustdoc_types::GenericBound>)`

The type is required to satisfy a set of bounds.
```text
Iterator<Item = u32, IntoIter: DoubleEndedIterator>
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

##### 3.2.2.2: Trait Implementations for `AssocItemConstraintKind`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::AssocItemConstraintKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.3: `enum GenericArg`

```rust
pub enum GenericArg {
    Lifetime(String),
    Type(rustdoc_types::Type),
    Const(rustdoc_types::Constant),
    Infer,
}
```

One argument in a list of generic arguments to a path segment.

Part of [`GenericArgs`].

##### 3.2.3.2: Variants

###### 3.2.3.2.2: `Lifetime(String)`

A lifetime argument.
```text
std::borrow::Cow<'static, str>
                 ^^^^^^^
```

###### 3.2.3.2.3: `Type(rustdoc_types::Type)`

A type argument.
```text
std::borrow::Cow<'static, str>
                          ^^^
```

###### 3.2.3.2.4: `Const(rustdoc_types::Constant)`

A constant as a generic argument.
```text
core::array::IntoIter<u32, { 640 * 1024 }>
                           ^^^^^^^^^^^^^^
```

###### 3.2.3.2.5: `Infer`

A generic argument that's explicitly set to be inferred.
```text
std::vec::Vec::<_>::new()
                ^
```

##### 3.2.3.2: Trait Implementations for `GenericArg`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::GenericArg {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.4: `enum GenericArgs`

```rust
pub enum GenericArgs {
    AngleBracketed { args: Vec<rustdoc_types::GenericArg>, constraints: Vec<rustdoc_types::AssocItemConstraint> },
    Parenthesized { inputs: Vec<rustdoc_types::Type>, output: option::Option<rustdoc_types::Type> },
    ReturnTypeNotation,
}
```

A set of generic arguments provided to a path segment, e.g.

```text
std::option::Option::<u32>::None
                     ^^^^^
```

##### 3.2.4.2: Variants

###### 3.2.4.2.2: `AngleBracketed { args: Vec<rustdoc_types::GenericArg>, constraints: Vec<rustdoc_types::AssocItemConstraint> }`

`<'a, 32, B: Copy, C = u32>`

####### 3.2.4.2.2.2: Fields

######## 3.2.4.2.2.2.2: `args`

The list of each argument on this type.
```text
<'a, 32, B: Copy, C = u32>
 ^^^^^^
```

######## 3.2.4.2.2.2.3: `constraints`

Associated type or constant bindings (e.g. `Item=i32` or `Item: Clone`) for this type.

###### 3.2.4.2.3: `Parenthesized { inputs: Vec<rustdoc_types::Type>, output: option::Option<rustdoc_types::Type> }`

`Fn(A, B) -> C`

####### 3.2.4.2.3.2: Fields

######## 3.2.4.2.3.2.2: `inputs`

The input types, enclosed in parentheses.

######## 3.2.4.2.3.2.3: `output`

The output type provided after the `->`, if present.

###### 3.2.4.2.4: `ReturnTypeNotation`

`T::method(..)`

##### 3.2.4.2: Trait Implementations for `GenericArgs`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::GenericArgs {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.5: `enum GenericBound`

```rust
pub enum GenericBound {
    TraitBound { trait_: rustdoc_types::Path, generic_params: Vec<rustdoc_types::GenericParamDef>, modifier: rustdoc_types::TraitBoundModifier },
    Outlives(String),
    Use(Vec<rustdoc_types::PreciseCapturingArg>),
}
```

Either a trait bound or a lifetime bound.

##### 3.2.5.2: Variants

###### 3.2.5.2.2: `TraitBound { trait_: rustdoc_types::Path, generic_params: Vec<rustdoc_types::GenericParamDef>, modifier: rustdoc_types::TraitBoundModifier }`

A trait bound.

####### 3.2.5.2.2.2: Fields

######## 3.2.5.2.2.2.2: `trait_`

The full path to the trait.

######## 3.2.5.2.2.2.3: `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)
```text
where F: for<'a, 'b> Fn(&'a u8, &'b u8)
         ^^^^^^^^^^^
         |
         this part
```

######## 3.2.5.2.2.2.4: `modifier`

The context for which a trait is supposed to be used, e.g. `const

###### 3.2.5.2.3: `Outlives(String)`

A lifetime bound, e.g.
```rust
fn f<'a, T>(x: &'a str, y: &T) where T: 'a {}
//                                     ^^^
```

###### 3.2.5.2.4: `Use(Vec<rustdoc_types::PreciseCapturingArg>)`

`use<'a, T>` precise-capturing bound syntax

##### 3.2.5.2: Trait Implementations for `GenericBound`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::GenericBound {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.6: `enum GenericParamDefKind`

```rust
pub enum GenericParamDefKind {
    Lifetime { outlives: Vec<String> },
    Type { bounds: Vec<rustdoc_types::GenericBound>, default: option::Option<rustdoc_types::Type>, is_synthetic: bool },
    Const { type_: rustdoc_types::Type, default: option::Option<String> },
}
```

The kind of a [`GenericParamDef`].

##### 3.2.6.2: Variants

###### 3.2.6.2.2: `Lifetime { outlives: Vec<String> }`

Denotes a lifetime parameter.

####### 3.2.6.2.2.2: Fields

######## 3.2.6.2.2.2.2: `outlives`

Lifetimes that this lifetime parameter is required to outlive.

```rust
fn f<'a, 'b, 'resource: 'a + 'b>(a: &'a str, b: &'b str, res: &'resource str) {}
//                      ^^^^^^^
```

###### 3.2.6.2.3: `Type { bounds: Vec<rustdoc_types::GenericBound>, default: option::Option<rustdoc_types::Type>, is_synthetic: bool }`

Denotes a type parameter.

####### 3.2.6.2.3.2: Fields

######## 3.2.6.2.3.2.2: `bounds`

Bounds applied directly to the type. Note that the bounds from `where` clauses
that constrain this parameter won't appear here.

```rust
fn default2<T: Default>() -> [T; 2] where T: Clone { todo!() }
//             ^^^^^^^
```

######## 3.2.6.2.3.2.3: `default`

The default type for this parameter, if provided, e.g.

```rust
trait PartialEq<Rhs = Self> {}
//                    ^^^^
```

######## 3.2.6.2.3.2.4: `is_synthetic`

This is normally `false`, which means that this generic parameter is
declared in the Rust source text.

If it is `true`, this generic parameter has been introduced by the
compiler behind the scenes.

###### Example

Consider

```ignore (pseudo-rust)
pub fn f(_: impl Trait) {}
```

The compiler will transform this behind the scenes to

```ignore (pseudo-rust)
pub fn f<impl Trait: Trait>(_: impl Trait) {}
```

In this example, the generic parameter named `impl Trait` (and which
is bound by `Trait`) is synthetic, because it was not originally in
the Rust source text.

###### 3.2.6.2.4: `Const { type_: rustdoc_types::Type, default: option::Option<String> }`

Denotes a constant parameter.

####### 3.2.6.2.4.2: Fields

######## 3.2.6.2.4.2.2: `type_`

The type of the constant as declared.

######## 3.2.6.2.4.2.3: `default`

The stringified expression for the default value, if provided. It's not guaranteed that
it'll match the actual source code for the default value.

##### 3.2.6.2: Trait Implementations for `GenericParamDefKind`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::GenericParamDefKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.7: `enum ItemEnum`

```rust
pub enum ItemEnum {
    Module(rustdoc_types::Module),
    ExternCrate { name: String, rename: option::Option<String> },
    Use(rustdoc_types::Use),
    Union(rustdoc_types::Union),
    Struct(rustdoc_types::Struct),
    StructField(rustdoc_types::Type),
    Enum(rustdoc_types::Enum),
    Variant(rustdoc_types::Variant),
    Function(rustdoc_types::Function),
    Trait(rustdoc_types::Trait),
    TraitAlias(rustdoc_types::TraitAlias),
    Impl(rustdoc_types::Impl),
    TypeAlias(rustdoc_types::TypeAlias),
    Constant { type_: rustdoc_types::Type, const_: rustdoc_types::Constant },
    Static(rustdoc_types::Static),
    ExternType,
    Macro(String),
    ProcMacro(rustdoc_types::ProcMacro),
    Primitive(rustdoc_types::Primitive),
    AssocConst { type_: rustdoc_types::Type, value: option::Option<String> },
    AssocType { generics: rustdoc_types::Generics, bounds: Vec<rustdoc_types::GenericBound>, type_: option::Option<rustdoc_types::Type> },
}
```

Specific fields of an item.

Part of [`Item`].

##### 3.2.7.2: Variants

###### 3.2.7.2.2: `Module(rustdoc_types::Module)`

A module declaration, e.g. `mod foo;` or `mod foo {}`

###### 3.2.7.2.3: `ExternCrate { name: String, rename: option::Option<String> }`

A crate imported via the `extern crate` syntax.

####### 3.2.7.2.3.2: Fields

######## 3.2.7.2.3.2.2: `name`

The name of the imported crate.

######## 3.2.7.2.3.2.3: `rename`

If the crate is renamed, this is its name in the crate.

###### 3.2.7.2.4: `Use(rustdoc_types::Use)`

An import of 1 or more items into scope, using the `use` keyword.

###### 3.2.7.2.5: `Union(rustdoc_types::Union)`

A `union` declaration.

###### 3.2.7.2.6: `Struct(rustdoc_types::Struct)`

A `struct` declaration.

###### 3.2.7.2.7: `StructField(rustdoc_types::Type)`

A field of a struct.

###### 3.2.7.2.8: `Enum(rustdoc_types::Enum)`

An `enum` declaration.

###### 3.2.7.2.9: `Variant(rustdoc_types::Variant)`

A variant of a enum.

###### 3.2.7.2.10: `Function(rustdoc_types::Function)`

A function declaration (including methods and other associated functions)

###### 3.2.7.2.11: `Trait(rustdoc_types::Trait)`

A `trait` declaration.

###### 3.2.7.2.12: `TraitAlias(rustdoc_types::TraitAlias)`

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

###### 3.2.7.2.13: `Impl(rustdoc_types::Impl)`

An `impl` block.

###### 3.2.7.2.14: `TypeAlias(rustdoc_types::TypeAlias)`

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

###### 3.2.7.2.15: `Constant { type_: rustdoc_types::Type, const_: rustdoc_types::Constant }`

The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

####### 3.2.7.2.15.2: Fields

######## 3.2.7.2.15.2.2: `type_`

The type of the constant.

######## 3.2.7.2.15.2.3: `const_`

The declared constant itself.

###### 3.2.7.2.16: `Static(rustdoc_types::Static)`

A declaration of a `static`.

###### 3.2.7.2.17: `ExternType`

`type`s from an `extern` block.

See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

###### 3.2.7.2.18: `Macro(String)`

A macro_rules! declarative macro. Contains a single string with the source
representation of the macro with the patterns stripped.

###### 3.2.7.2.19: `ProcMacro(rustdoc_types::ProcMacro)`

A procedural macro.

###### 3.2.7.2.20: `Primitive(rustdoc_types::Primitive)`

A primitive type, e.g. `u32`.

[`Item`]s of this kind only come from the core library.

###### 3.2.7.2.21: `AssocConst { type_: rustdoc_types::Type, value: option::Option<String> }`

An associated constant of a trait or a type.

####### 3.2.7.2.21.2: Fields

######## 3.2.7.2.21.2.2: `type_`

The type of the constant.

######## 3.2.7.2.21.2.3: `value`

Inside a trait declaration, this is the default value for the associated constant,
if provided.
Inside an `impl` block, this is the value assigned to the associated constant,
and will always be present.

The representation is implementation-defined and not guaranteed to be representative of
either the resulting value or of the source code.

```rust
const X: usize = 640 * 1024;
//               ^^^^^^^^^^
```

###### 3.2.7.2.22: `AssocType { generics: rustdoc_types::Generics, bounds: Vec<rustdoc_types::GenericBound>, type_: option::Option<rustdoc_types::Type> }`

An associated type of a trait or a type.

####### 3.2.7.2.22.2: Fields

######## 3.2.7.2.22.2.2: `generics`

The generic parameters and where clauses on ahis associated type.

######## 3.2.7.2.22.2.3: `bounds`

The bounds for this associated type. e.g.
```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
//                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
}
```

######## 3.2.7.2.22.2.4: `type_`

Inside a trait declaration, this is the default for the associated type, if provided.
Inside an impl block, this is the type assigned to the associated type, and will always
be present.

```rust
type X = usize;
//       ^^^^^
```

##### 3.2.7.2: Trait Implementations for `ItemEnum`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::ItemEnum {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.8: `enum ItemKind`

```rust
pub enum ItemKind {
    Module,
    ExternCrate,
    Use,
    Struct,
    StructField,
    Union,
    Enum,
    Variant,
    Function,
    TypeAlias,
    Constant,
    Trait,
    TraitAlias,
    Impl,
    Static,
    ExternType,
    Macro,
    ProcAttribute,
    ProcDerive,
    AssocConst,
    AssocType,
    Primitive,
    Keyword,
}
```

The fundamental kind of an item. Unlike [`ItemEnum`], this does not carry any additional info.

Part of [`ItemSummary`].

##### 3.2.8.2: Variants

###### 3.2.8.2.2: `Module`

A module declaration, e.g. `mod foo;` or `mod foo {}`

###### 3.2.8.2.3: `ExternCrate`

A crate imported via the `extern crate` syntax.

###### 3.2.8.2.4: `Use`

An import of 1 or more items into scope, using the `use` keyword.

###### 3.2.8.2.5: `Struct`

A `struct` declaration.

###### 3.2.8.2.6: `StructField`

A field of a struct.

###### 3.2.8.2.7: `Union`

A `union` declaration.

###### 3.2.8.2.8: `Enum`

An `enum` declaration.

###### 3.2.8.2.9: `Variant`

A variant of a enum.

###### 3.2.8.2.10: `Function`

A function declaration, e.g. `fn f() {}`

###### 3.2.8.2.11: `TypeAlias`

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

###### 3.2.8.2.12: `Constant`

The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

###### 3.2.8.2.13: `Trait`

A `trait` declaration.

###### 3.2.8.2.14: `TraitAlias`

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

###### 3.2.8.2.15: `Impl`

An `impl` block.

###### 3.2.8.2.16: `Static`

A `static` declaration.

###### 3.2.8.2.17: `ExternType`

`type`s from an `extern` block.

See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

###### 3.2.8.2.18: `Macro`

A macro declaration.

Corresponds to either `ItemEnum::Macro(_)`
or `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Bang })`

###### 3.2.8.2.19: `ProcAttribute`

A procedural macro attribute.

Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Attr })`

###### 3.2.8.2.20: `ProcDerive`

A procedural macro usable in the `#[derive()]` attribute.

Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Derive })`

###### 3.2.8.2.21: `AssocConst`

An associated constant of a trait or a type.

###### 3.2.8.2.22: `AssocType`

An associated type of a trait or a type.

###### 3.2.8.2.23: `Primitive`

A primitive type, e.g. `u32`.

[`Item`]s of this kind only come from the core library.

###### 3.2.8.2.24: `Keyword`

A keyword declaration.

[`Item`]s of this kind only come from the come library and exist solely
to carry documentation for the respective keywords.

##### 3.2.8.2: Trait Implementations for `ItemKind`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Copy`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::ItemKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.9: `enum MacroKind`

```rust
pub enum MacroKind {
    Bang,
    Attr,
    Derive,
}
```

The way a [`ProcMacro`] is declared to be used.

##### 3.2.9.2: Variants

###### 3.2.9.2.2: `Bang`

A bang macro `foo!()`.

###### 3.2.9.2.3: `Attr`

An attribute macro `#[foo]`.

###### 3.2.9.2.4: `Derive`

A derive macro `#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]`

##### 3.2.9.2: Trait Implementations for `MacroKind`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Copy`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::MacroKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.10: `enum PreciseCapturingArg`

```rust
pub enum PreciseCapturingArg {
    Lifetime(String),
    Param(String),
}
```

One precise capturing argument. See [the rust reference](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing).

##### 3.2.10.2: Variants

###### 3.2.10.2.2: `Lifetime(String)`

A lifetime.
```rust
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
//                                                        ^^

###### 3.2.10.2.3: `Param(String)`

A type or constant parameter.
```rust
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
//                                                            ^  ^

##### 3.2.10.2: Trait Implementations for `PreciseCapturingArg`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::PreciseCapturingArg {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.11: `enum StructKind`

```rust
pub enum StructKind {
    Unit,
    Tuple(Vec<option::Option<rustdoc_types::Id>>),
    Plain { fields: Vec<rustdoc_types::Id>, has_stripped_fields: bool },
}
```

The kind of a [`Struct`] and the data specific to it, i.e. fields.

##### 3.2.11.2: Variants

###### 3.2.11.2.2: `Unit`

A struct with no fields and no parentheses.

```rust
pub struct Unit;
```

###### 3.2.11.2.3: `Tuple(Vec<option::Option<rustdoc_types::Id>>)`

A struct with unnamed fields.

All [`Id`]'s will point to [`ItemEnum::StructField`].
Unlike most of JSON, private and `#[doc(hidden)]` fields will be given as `None`
instead of being omitted, because order matters.

```rust
pub struct TupleStruct(i32);
pub struct EmptyTupleStruct();
```

###### 3.2.11.2.4: `Plain { fields: Vec<rustdoc_types::Id>, has_stripped_fields: bool }`

A struct with named fields.

```rust
pub struct PlainStruct { x: i32 }
pub struct EmptyPlainStruct {}
```

####### 3.2.11.2.4.2: Fields

######## 3.2.11.2.4.2.2: `fields`

The list of fields in the struct.

All of the corresponding [`Item`]s are of kind [`ItemEnum::StructField`].

######## 3.2.11.2.4.2.3: `has_stripped_fields`

Whether any fields have been removed from the result, due to being private or hidden.

##### 3.2.11.2: Trait Implementations for `StructKind`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::StructKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.12: `enum Term`

```rust
pub enum Term {
    Type(rustdoc_types::Type),
    Constant(rustdoc_types::Constant),
}
```

Either a type or a constant, usually stored as the right-hand side of an equation in places like
[`AssocItemConstraint`]

##### 3.2.12.2: Variants

###### 3.2.12.2.2: `Type(rustdoc_types::Type)`

A type.

```rust
fn f(x: impl IntoIterator<Item = u32>) {}
//                               ^^^
```

###### 3.2.12.2.3: `Constant(rustdoc_types::Constant)`

A constant.

```ignore (incomplete feature in the snippet)
trait Foo {
    const BAR: usize;
}

fn f(x: impl Foo<BAR = 42>) {}
//                     ^^
```

##### 3.2.12.2: Trait Implementations for `Term`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Term {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.13: `enum TraitBoundModifier`

```rust
pub enum TraitBoundModifier {
    None,
    Maybe,
    MaybeConst,
}
```

A set of modifiers applied to a trait.

##### 3.2.13.2: Variants

###### 3.2.13.2.2: `None`

Marks the absence of a modifier.

###### 3.2.13.2.3: `Maybe`

Indicates that the trait bound relaxes a trait bound applied to a parameter by default,
e.g. `T: Sized?`, the `Sized` trait is required for all generic type parameters by default
unless specified otherwise with this modifier.

###### 3.2.13.2.4: `MaybeConst`

Indicates that the trait bound must be applicable in both a run-time and a compile-time
context.

##### 3.2.13.2: Trait Implementations for `TraitBoundModifier`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Copy`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::TraitBoundModifier {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.14: `enum Type`

```rust
pub enum Type {
    ResolvedPath(rustdoc_types::Path),
    DynTrait(rustdoc_types::DynTrait),
    Generic(String),
    Primitive(String),
    FunctionPointer(Box<rustdoc_types::FunctionPointer>),
    Tuple(Vec<rustdoc_types::Type>),
    Slice(Box<rustdoc_types::Type>),
    Array { type_: Box<rustdoc_types::Type>, len: String },
    Pat { type_: Box<rustdoc_types::Type> },
    ImplTrait(Vec<rustdoc_types::GenericBound>),
    Infer,
    RawPointer { is_mutable: bool, type_: Box<rustdoc_types::Type> },
    BorrowedRef { lifetime: option::Option<String>, is_mutable: bool, type_: Box<rustdoc_types::Type> },
    QualifiedPath { name: String, args: Box<rustdoc_types::GenericArgs>, self_type: Box<rustdoc_types::Type>, trait_: option::Option<rustdoc_types::Path> },
}
```

A type.

##### 3.2.14.2: Variants

###### 3.2.14.2.2: `ResolvedPath(rustdoc_types::Path)`

Structs, enums, unions and type aliases, e.g. `std::option::Option<u32>`

###### 3.2.14.2.3: `DynTrait(rustdoc_types::DynTrait)`

Dynamic trait object type (`dyn Trait`).

###### 3.2.14.2.4: `Generic(String)`

Parameterized types. The contained string is the name of the parameter.

###### 3.2.14.2.5: `Primitive(String)`

Built-in numeric types (e.g. `u32`, `f32`), `bool`, `char`.

###### 3.2.14.2.6: `FunctionPointer(Box<rustdoc_types::FunctionPointer>)`

A function pointer type, e.g. `fn(u32) -> u32`, `extern "C" fn() -> *const u8`

###### 3.2.14.2.7: `Tuple(Vec<rustdoc_types::Type>)`

A tuple type, e.g. `(String, u32, Box<usize>)`

###### 3.2.14.2.8: `Slice(Box<rustdoc_types::Type>)`

An unsized slice type, e.g. `[u32]`.

###### 3.2.14.2.9: `Array { type_: Box<rustdoc_types::Type>, len: String }`

An array type, e.g. `[u32; 15]`

####### 3.2.14.2.9.2: Fields

######## 3.2.14.2.9.2.2: `type_`

The type of the contained element.

######## 3.2.14.2.9.2.3: `len`

The stringified expression that is the length of the array.

Keep in mind that it's not guaranteed to match the actual source code of the expression.

###### 3.2.14.2.10: `Pat { type_: Box<rustdoc_types::Type> }`

A pattern type, e.g. `u32 is 1..`

See [the tracking issue](https://github.com/rust-lang/rust/issues/123646)

####### 3.2.14.2.10.2: Fields

######## 3.2.14.2.10.2.2: `type_`

The base type, e.g. the `u32` in `u32 is 1..`


_[Private fields hidden]_
###### 3.2.14.2.11: `ImplTrait(Vec<rustdoc_types::GenericBound>)`

An opaque type that satisfies a set of bounds, `impl TraitA + TraitB + ...`

###### 3.2.14.2.12: `Infer`

A type that's left to be inferred, `_`

###### 3.2.14.2.13: `RawPointer { is_mutable: bool, type_: Box<rustdoc_types::Type> }`

A raw pointer type, e.g. `*mut u32`, `*const u8`, etc.

####### 3.2.14.2.13.2: Fields

######## 3.2.14.2.13.2.2: `is_mutable`

This is `true` for `*mut _` and `false` for `*const _`.

######## 3.2.14.2.13.2.3: `type_`

The type of the pointee.

###### 3.2.14.2.14: `BorrowedRef { lifetime: option::Option<String>, is_mutable: bool, type_: Box<rustdoc_types::Type> }`

`&'a mut String`, `&str`, etc.

####### 3.2.14.2.14.2: Fields

######## 3.2.14.2.14.2.2: `lifetime`

The name of the lifetime of the reference, if provided.

######## 3.2.14.2.14.2.3: `is_mutable`

This is `true` for `&mut i32` and `false` for `&i32`

######## 3.2.14.2.14.2.4: `type_`

The type of the pointee, e.g. the `i32` in `&'a mut i32`

###### 3.2.14.2.15: `QualifiedPath { name: String, args: Box<rustdoc_types::GenericArgs>, self_type: Box<rustdoc_types::Type>, trait_: option::Option<rustdoc_types::Path> }`

Associated types like `<Type as Trait>::Name` and `T::Item` where
`T: Iterator` or inherent associated types like `Struct::Name`.

####### 3.2.14.2.15.2: Fields

######## 3.2.14.2.15.2.2: `name`

The name of the associated type in the parent type.

```ignore (incomplete expression)
<core::array::IntoIter<u32, 42> as Iterator>::Item
//                                            ^^^^
```

######## 3.2.14.2.15.2.3: `args`

The generic arguments provided to the associated type.

```ignore (incomplete expression)
<core::slice::IterMut<'static, u32> as BetterIterator>::Item<'static>
//                                                          ^^^^^^^^^
```

######## 3.2.14.2.15.2.4: `self_type`

The type with which this type is associated.

```ignore (incomplete expression)
<core::array::IntoIter<u32, 42> as Iterator>::Item
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

######## 3.2.14.2.15.2.5: `trait_`

`None` iff this is an *inherent* associated type.

##### 3.2.14.2: Trait Implementations for `Type`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Type {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.15: `enum VariantKind`

```rust
pub enum VariantKind {
    Plain,
    Tuple(Vec<option::Option<rustdoc_types::Id>>),
    Struct { fields: Vec<rustdoc_types::Id>, has_stripped_fields: bool },
}
```

The kind of an [`Enum`] [`Variant`] and the data specific to it, i.e. fields.

##### 3.2.15.2: Variants

###### 3.2.15.2.2: `Plain`

A variant with no parentheses

```rust
enum Demo {
    PlainVariant,
    PlainWithDiscriminant = 1,
}
```

###### 3.2.15.2.3: `Tuple(Vec<option::Option<rustdoc_types::Id>>)`

A variant with unnamed fields.

All [`Id`]'s will point to [`ItemEnum::StructField`].
Unlike most of JSON, `#[doc(hidden)]` fields will be given as `None`
instead of being omitted, because order matters.

```rust
enum Demo {
    TupleVariant(i32),
    EmptyTupleVariant(),
}
```

###### 3.2.15.2.4: `Struct { fields: Vec<rustdoc_types::Id>, has_stripped_fields: bool }`

A variant with named fields.

```rust
enum Demo {
    StructVariant { x: i32 },
    EmptyStructVariant {},
}
```

####### 3.2.15.2.4.2: Fields

######## 3.2.15.2.4.2.2: `fields`

The list of variants in the enum.
All of the corresponding [`Item`]s are of kind [`ItemEnum::Variant`].

######## 3.2.15.2.4.2.3: `has_stripped_fields`

Whether any variants have been removed from the result, due to being private or hidden.

##### 3.2.15.2: Trait Implementations for `VariantKind`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::VariantKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.16: `enum Visibility`

```rust
pub enum Visibility {
    Public,
    Default,
    Crate,
    Restricted { parent: rustdoc_types::Id, path: String },
}
```

Visibility of an [`Item`].

##### 3.2.16.2: Variants

###### 3.2.16.2.2: `Public`

Explicitly public visibility set with `pub`.

###### 3.2.16.2.3: `Default`

For the most part items are private by default. The exceptions are associated items of
public traits and variants of public enums.

###### 3.2.16.2.4: `Crate`

Explicitly crate-wide visibility set with `pub(crate)`

###### 3.2.16.2.5: `Restricted { parent: rustdoc_types::Id, path: String }`

For `pub(in path)` visibility.

####### 3.2.16.2.5.2: Fields

######## 3.2.16.2.5.2.2: `parent`

ID of the module to which this visibility restricts items.

######## 3.2.16.2.5.2.3: `path`

The path with which [`parent`] was referenced
(like `super::super` or `crate::foo::bar`).

[`parent`]: Visibility::Restricted::parent

##### 3.2.16.2: Trait Implementations for `Visibility`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Visibility {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 3.2.17: `enum WherePredicate`

```rust
pub enum WherePredicate {
    BoundPredicate { type_: rustdoc_types::Type, bounds: Vec<rustdoc_types::GenericBound>, generic_params: Vec<rustdoc_types::GenericParamDef> },
    LifetimePredicate { lifetime: String, outlives: Vec<String> },
    EqPredicate { lhs: rustdoc_types::Type, rhs: rustdoc_types::Term },
}
```

One `where` clause.
```rust
fn default<T>() -> T where T: Default { T::default() }
//                         ^^^^^^^^^^
```

##### 3.2.17.2: Variants

###### 3.2.17.2.2: `BoundPredicate { type_: rustdoc_types::Type, bounds: Vec<rustdoc_types::GenericBound>, generic_params: Vec<rustdoc_types::GenericParamDef> }`

A type is expected to comply with a set of bounds

####### 3.2.17.2.2.2: Fields

######## 3.2.17.2.2.2.2: `type_`

The type that's being constrained.

```rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                              ^
```

######## 3.2.17.2.2.2.3: `bounds`

The set of bounds that constrain the type.

```rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                                 ^^^^^^^^
```

######## 3.2.17.2.2.2.4: `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)
```rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                  ^^^^^^^
```

###### 3.2.17.2.3: `LifetimePredicate { lifetime: String, outlives: Vec<String> }`

A lifetime is expected to outlive other lifetimes.

####### 3.2.17.2.3.2: Fields

######## 3.2.17.2.3.2.2: `lifetime`

The name of the lifetime.

######## 3.2.17.2.3.2.3: `outlives`

The lifetimes that must be encompassed by the lifetime.

###### 3.2.17.2.4: `EqPredicate { lhs: rustdoc_types::Type, rhs: rustdoc_types::Term }`

A type must exactly equal another type.

####### 3.2.17.2.4.2: Fields

######## 3.2.17.2.4.2.2: `lhs`

The left side of the equation.

######## 3.2.17.2.4.2.3: `rhs`

The right side of the equation.

##### 3.2.17.2: Trait Implementations for `WherePredicate`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::WherePredicate {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


### 3.3: Constants

#### 3.3.1: `const FORMAT_VERSION`

The version of JSON output that this crate represents.

This integer is incremented with every breaking change to the API,
and is returned along with the JSON blob as [`Crate::format_version`].
Consuming code should assert that this value matches the format version(s) that it supports.

# cargo_manifest API (0.19.1)


## 1: Module: `cargo_manifest`

cargo-manifest
==============================================================================

[`serde`](https://serde.rs) definitions to read and write
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) files.


Description
------------------------------------------------------------------------------

This Rust crate contains various structs and enums to represent the contents of
a `Cargo.toml` file. These definitions can be used with [`serde`](https://serde.rs)
and the [`toml`](https://crates.io/crates/toml) crate to read and write
`Cargo.toml` manifest files.

This crate also to some degree supports post-processing of the data to emulate
Cargo's workspace inheritance and `autobins` features. This is used for example
by crates.io to extract whether a crate contains a library or executable
binaries.

> [!NOTE]
> The cargo team regularly adds new features to the `Cargo.toml` file
> definition. This crate aims to keep up-to-date with these changes. You should
> keep this crate up-to-date to correctly parse all fields in modern
> `Cargo.toml` files.


Installation
------------------------------------------------------------------------------

```sh
cargo add cargo-manifest
```


Usage
------------------------------------------------------------------------------

```rust
use cargo_manifest::Manifest;

let manifest = Manifest::from_path("Cargo.toml").unwrap();
```

see [docs.rs](https://docs.rs/cargo-manifest) for more information.


Users
------------------------------------------------------------------------------

- [cargo-chef](https://crates.io/crates/cargo-chef)
- [crates.io](https://github.com/rust-lang/crates.io) is using this crate for
  server-side validation of `Cargo.toml` files.


Alternatives
------------------------------------------------------------------------------

This crate is a fork of the [`cargo_toml`](https://crates.io/crates/cargo_toml)
project. There are only some minor differences between these projects at this
point, you will need to evaluate which one fits your needs better.

There is also [`cargo-util-schemas`](https://crates.io/crates/cargo-util-schemas)
now, which is maintained by the cargo team themselves. This crate was extracted
from the cargo codebase and is used inside the `cargo` binary itself. It is
kept up-to-date with the latest changes to the `Cargo.toml` file format, but is
currently lacking some of the post-processing features that `cargo-manifest`
provides.


License
------------------------------------------------------------------------------

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.


### 1.1: Structs

#### 1.1.1: `struct Badge`

```rust
pub struct Badge {
    pub repository: String,
    pub branch: String,
    pub service: option::Option<String>,
    pub id: option::Option<String>,
    pub project_name: option::Option<String>,
}
```

##### 1.1.1.1: Trait Implementations for `Badge`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Badge {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.1.2: `struct Badges`

```rust
pub struct Badges {
    pub appveyor: option::Option<cargo_manifest::Badge>,
    pub circle_ci: option::Option<cargo_manifest::Badge>,
    pub gitlab: option::Option<cargo_manifest::Badge>,
    pub travis_ci: option::Option<cargo_manifest::Badge>,
    pub codecov: option::Option<cargo_manifest::Badge>,
    pub coveralls: option::Option<cargo_manifest::Badge>,
    pub is_it_maintained_issue_resolution: option::Option<cargo_manifest::Badge>,
    pub is_it_maintained_open_issues: option::Option<cargo_manifest::Badge>,
    pub maintenance: cargo_manifest::Maintenance,
}
```

##### 1.1.2.1: Fields

###### 1.1.2.1.1: `appveyor`

Appveyor: `repository` is required. `branch` is optional; default is `master`
`service` is optional; valid values are `github` (default), `bitbucket`, and
`gitlab`; `id` is optional; you can specify the appveyor project id if you
want to use that instead. `project_name` is optional; use when the repository
name differs from the appveyor project name.

###### 1.1.2.1.2: `circle_ci`

Circle CI: `repository` is required. `branch` is optional; default is `master`

###### 1.1.2.1.3: `gitlab`

GitLab: `repository` is required. `branch` is optional; default is `master`

###### 1.1.2.1.4: `travis_ci`

Travis CI: `repository` in format "\<user>/\<project>" is required.
`branch` is optional; default is `master`

###### 1.1.2.1.5: `codecov`

Codecov: `repository` is required. `branch` is optional; default is `master`
`service` is optional; valid values are `github` (default), `bitbucket`, and
`gitlab`.

###### 1.1.2.1.6: `coveralls`

Coveralls: `repository` is required. `branch` is optional; default is `master`
`service` is optional; valid values are `github` (default) and `bitbucket`.

###### 1.1.2.1.7: `is_it_maintained_issue_resolution`

Is it maintained resolution time: `repository` is required.

###### 1.1.2.1.8: `is_it_maintained_open_issues`

Is it maintained percentage of open issues: `repository` is required.

###### 1.1.2.1.9: `maintenance`

Maintenance: `status` is required. Available options are `actively-developed`,
`passively-maintained`, `as-is`, `experimental`, `looking-for-maintainer`,
`deprecated`, and the default `none`, which displays no badge on crates.io.

##### 1.1.2.2: Trait Implementations for `Badges`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Badges {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.1.3: `struct DependencyDetail`

```rust
pub struct DependencyDetail {
    pub version: option::Option<String>,
    pub registry: option::Option<String>,
    pub registry_index: option::Option<String>,
    pub path: option::Option<String>,
    pub git: option::Option<String>,
    pub branch: option::Option<String>,
    pub tag: option::Option<String>,
    pub rev: option::Option<String>,
    pub features: option::Option<Vec<String>>,
    pub optional: option::Option<bool>,
    pub default_features: option::Option<bool>,
    pub package: option::Option<String>,
}
```

##### 1.1.3.1: Trait Implementations for `DependencyDetail`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::DependencyDetail {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.1.4: `struct Filesystem<'a>`

```rust
pub struct Filesystem<'a> {}
```

_[Private fields hidden]_

A [AbstractFilesystem] implementation that reads from the actual filesystem
within the given root path.

##### 1.1.4.2: `impl<'a> cargo_manifest::afs::Filesystem<'a>`

###### 1.1.4.2.2: `fn new(path: &'a path::Path) -> Self`


##### 1.1.4.2: Trait Implementations for `Filesystem`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `cargo_manifest::afs::AbstractFilesystem`

- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)

#### 1.1.5: `struct InheritedDependencyDetail`

```rust
pub struct InheritedDependencyDetail {
    pub workspace: {id:589},
    pub features: option::Option<Vec<String>>,
    pub optional: option::Option<bool>,
}
```

When a dependency is defined as `{ workspace = true }`,
and workspace data hasn't been applied yet.

##### 1.1.5.1: Trait Implementations for `InheritedDependencyDetail`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::InheritedDependencyDetail {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.1.6: `struct Maintenance`

```rust
pub struct Maintenance {
    pub status: cargo_manifest::MaintenanceStatus,
}
```

##### 1.1.6.1: Trait Implementations for `Maintenance`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Copy`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Maintenance {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.1.7: `struct Manifest<PackageMetadata = toml::value::Value, WorkspaceMetadata = toml::value::Value>`

```rust
pub struct Manifest<PackageMetadata = toml::value::Value, WorkspaceMetadata = toml::value::Value> {
    pub package: option::Option<cargo_manifest::Package<PackageMetadata>>,
    pub cargo_features: option::Option<Vec<String>>,
    pub workspace: option::Option<cargo_manifest::Workspace<WorkspaceMetadata>>,
    pub dependencies: option::Option<cargo_manifest::DepsSet>,
    pub dev_dependencies: option::Option<cargo_manifest::DepsSet>,
    pub build_dependencies: option::Option<cargo_manifest::DepsSet>,
    pub target: option::Option<cargo_manifest::TargetDepsSet>,
    pub features: option::Option<cargo_manifest::FeatureSet>,
    pub bin: Vec<cargo_manifest::Product>,
    pub bench: Vec<cargo_manifest::Product>,
    pub test: Vec<cargo_manifest::Product>,
    pub example: Vec<cargo_manifest::Product>,
    pub patch: option::Option<cargo_manifest::PatchSet>,
    pub lib: option::Option<cargo_manifest::Product>,
    pub profile: option::Option<cargo_manifest::Profiles>,
    pub badges: option::Option<cargo_manifest::Badges>,
}
```

The top-level `Cargo.toml` structure

The `Metadata` is a type for `[package.metadata]` table. You can replace it with
your own struct type if you use the metadata and don't want to use the catch-all `Value` type.

##### 1.1.7.1: Fields

###### 1.1.7.1.1: `bin`

Note that due to autobins feature this is not the complete list
unless you run `complete_from_path`

###### 1.1.7.1.2: `lib`

Note that due to autolibs feature this is not the complete list
unless you run `complete_from_path`

##### 1.1.7.3: `impl cargo_manifest::Manifest<toml::value::Value>`

###### 1.1.7.3.2: `fn from_slice(cargo_toml_content: &[u8]) -> result::Result<Self, cargo_manifest::error::Error>`

Parse contents of a `Cargo.toml` file already loaded as a byte slice.

It does not call `complete_from_path`, so may be missing implicit data.

###### 1.1.7.3.3: `fn from_path<impl impl AsRef<Path>: convert::AsRef<path::Path>>(cargo_toml_path: impl convert::AsRef<path::Path>) -> result::Result<Self, cargo_manifest::error::Error>`

Parse contents from a `Cargo.toml` file on disk.

Calls `complete_from_path`.

##### 1.1.7.4: `impl<Metadata: for<'a> serde::de::Deserialize<'a>> cargo_manifest::Manifest<Metadata>`

###### 1.1.7.4.2: `fn from_slice_with_metadata(cargo_toml_content: &[u8]) -> result::Result<Self, cargo_manifest::error::Error>`

Parse `Cargo.toml`, and parse its `[package.metadata]` into a custom Serde-compatible type.

It does not call `complete_from_path`, so may be missing implicit data.

###### 1.1.7.4.3: `fn from_path_with_metadata<impl impl AsRef<Path>: convert::AsRef<path::Path>>(cargo_toml_path: impl convert::AsRef<path::Path>) -> result::Result<Self, cargo_manifest::error::Error>`

Parse contents from `Cargo.toml` file on disk, with custom Serde-compatible metadata type.

Calls `complete_from_path`

###### 1.1.7.4.4: `fn complete_from_path(self: &mut Self, path: &path::Path) -> result::Result<(), cargo_manifest::error::Error>`

`Cargo.toml` may not contain explicit information about `[lib]`, `[[bin]]` and
`[package].build`, which are inferred based on files on disk.

This scans the disk to make the data in the manifest as complete as possible.

###### 1.1.7.4.5: `fn complete_from_abstract_filesystem<FS: cargo_manifest::afs::AbstractFilesystem>(self: &mut Self, fs: &FS) -> result::Result<(), cargo_manifest::error::Error>`

`Cargo.toml` may not contain explicit information about `[lib]`, `[[bin]]` and
`[package].build`, which are inferred based on files on disk.

You can provide any implementation of directory scan, which doesn't have to
be reading straight from disk (might scan a tarball or a git repo, for example).

###### 1.1.7.4.6: `fn autobins(self: &Self) -> bool`


###### 1.1.7.4.7: `fn autoexamples(self: &Self) -> bool`


###### 1.1.7.4.8: `fn autotests(self: &Self) -> bool`


###### 1.1.7.4.9: `fn autobenches(self: &Self) -> bool`


##### 1.1.7.4: Trait Implementations for `Manifest`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `str::traits::FromStr`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de, PackageMetadata, WorkspaceMetadata> serde::de::Deserialize<'de> for cargo_manifest::Manifest<PackageMetadata, WorkspaceMetadata>
      where
        PackageMetadata: serde::de::Deserialize<'de>,
        WorkspaceMetadata: serde::de::Deserialize<'de> {

        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```

- `serde::ser::Serialize`

    ```rust
    impl<PackageMetadata, WorkspaceMetadata> serde::ser::Serialize for cargo_manifest::Manifest<PackageMetadata, WorkspaceMetadata>
      where
        PackageMetadata: serde::ser::Serialize,
        WorkspaceMetadata: serde::ser::Serialize {

        pub fn serialize<__S> where __S: serde::ser::Serializer(self: &Self, __serializer: __S) -> result::Result<<__S as serde::ser::Serializer>::Ok, <__S as serde::ser::Serializer>::Error> { ... }
    }
    ```


#### 1.1.8: `struct Package<Metadata = toml::value::Value>`

```rust
pub struct Package<Metadata = toml::value::Value> {
    pub name: String,
    pub edition: option::Option<cargo_manifest::MaybeInherited<cargo_manifest::Edition>>,
    pub version: option::Option<cargo_manifest::MaybeInherited<String>>,
    pub build: option::Option<cargo_manifest::StringOrBool>,
    pub workspace: option::Option<String>,
    pub authors: option::Option<cargo_manifest::MaybeInherited<Vec<String>>>,
    pub links: option::Option<String>,
    pub description: option::Option<cargo_manifest::MaybeInherited<String>>,
    pub homepage: option::Option<cargo_manifest::MaybeInherited<String>>,
    pub documentation: option::Option<cargo_manifest::MaybeInherited<String>>,
    pub readme: option::Option<cargo_manifest::MaybeInherited<cargo_manifest::StringOrBool>>,
    pub keywords: option::Option<cargo_manifest::MaybeInherited<Vec<String>>>,
    pub categories: option::Option<cargo_manifest::MaybeInherited<Vec<String>>>,
    pub license: option::Option<cargo_manifest::MaybeInherited<String>>,
    pub license_file: option::Option<cargo_manifest::MaybeInherited<String>>,
    pub repository: option::Option<cargo_manifest::MaybeInherited<String>>,
    pub metadata: option::Option<Metadata>,
    pub rust_version: option::Option<cargo_manifest::MaybeInherited<String>>,
    pub exclude: option::Option<cargo_manifest::MaybeInherited<Vec<String>>>,
    pub include: option::Option<cargo_manifest::MaybeInherited<Vec<String>>>,
    pub default_run: option::Option<String>,
    pub autolib: option::Option<bool>,
    pub autobins: option::Option<bool>,
    pub autoexamples: option::Option<bool>,
    pub autotests: option::Option<bool>,
    pub autobenches: option::Option<bool>,
    pub publish: option::Option<cargo_manifest::MaybeInherited<cargo_manifest::Publish>>,
    pub resolver: option::Option<cargo_manifest::Resolver>,
}
```

You can replace `Metadata` type with your own
to parse into something more useful than a generic toml `Value`

##### 1.1.8.1: Fields

###### 1.1.8.1.1: `name`

Careful: some names are uppercase

###### 1.1.8.1.2: `version`

The version of the package (e.g. "1.9.0").

Use [Package::version()] to get the effective value, with the default
value of "0.0.0" applied.

###### 1.1.8.1.3: `authors`

e.g. ["Author <e@mail>", "etc"]

###### 1.1.8.1.4: `description`

A short blurb about the package. This is not rendered in any format when
uploaded to crates.io (aka this is not markdown).

###### 1.1.8.1.5: `readme`

This points to a file under the package root (relative to this `Cargo.toml`).

###### 1.1.8.1.6: `categories`

This is a list of up to five categories where this crate would fit.
e.g. ["command-line-utilities", "development-tools::cargo-plugins"]

###### 1.1.8.1.7: `license`

e.g. "MIT"

###### 1.1.8.1.8: `rust_version`

e.g. "1.63.0"

###### 1.1.8.1.9: `default_run`

The default binary to run by cargo run.

###### 1.1.8.1.10: `autolib`

Disables library auto discovery.

###### 1.1.8.1.11: `autobins`

Disables binary auto discovery.

Use [Manifest::autobins()] to get the effective value.

###### 1.1.8.1.12: `autoexamples`

Disables example auto discovery.

Use [Manifest::autoexamples()] to get the effective value.

###### 1.1.8.1.13: `autotests`

Disables test auto discovery.

Use [Manifest::autotests()] to get the effective value.

###### 1.1.8.1.14: `autobenches`

Disables bench auto discovery.

Use [Manifest::autobenches()] to get the effective value.

##### 1.1.8.3: `impl<Metadata> cargo_manifest::Package<Metadata>`

###### 1.1.8.3.2: `fn new(name: String, version: String) -> Self`


###### 1.1.8.3.3: `fn version(self: &Self) -> cargo_manifest::MaybeInherited<&str>`

Returns the effective version of the package.

If the version is not set, it defaults to "0.0.0"
(see <https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field>).

##### 1.1.8.3: Trait Implementations for `Package`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `PartialEq`
- `StructuralPartialEq`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de, Metadata> serde::de::Deserialize<'de> for cargo_manifest::Package<Metadata> where Metadata: serde::de::Deserialize<'de> {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```

- `serde::ser::Serialize`

    ```rust
    impl<Metadata> serde::ser::Serialize for cargo_manifest::Package<Metadata> where Metadata: serde::ser::Serialize {
        pub fn serialize<__S> where __S: serde::ser::Serializer(self: &Self, __serializer: __S) -> result::Result<<__S as serde::ser::Serializer>::Ok, <__S as serde::ser::Serializer>::Error> { ... }
    }
    ```


#### 1.1.9: `struct Product`

```rust
pub struct Product {
    pub path: option::Option<String>,
    pub name: option::Option<String>,
    pub test: bool,
    pub doctest: bool,
    pub bench: bool,
    pub doc: bool,
    pub plugin: bool,
    pub proc_macro: bool,
    pub harness: bool,
    pub edition: option::Option<cargo_manifest::Edition>,
    pub required_features: Vec<String>,
    pub crate_type: option::Option<Vec<String>>,
}
```

Cargo uses the term "target" for both "target platform" and "build target" (the thing to build),
which makes it ambigous.
Here Cargo's bin/lib **target** is renamed to **product**.

##### 1.1.9.1: Fields

###### 1.1.9.1.1: `path`

This field points at where the crate is located, relative to the `Cargo.toml`.

###### 1.1.9.1.2: `name`

The name of a product is the name of the library or binary that will be generated.
This is defaulted to the name of the package, with any dashes replaced
with underscores. (Rust `extern crate` declarations reference this name;
therefore the value must be a valid Rust identifier to be usable.)

###### 1.1.9.1.3: `test`

A flag for enabling unit tests for this product. This is used by `cargo test`.

###### 1.1.9.1.4: `doctest`

A flag for enabling documentation tests for this product. This is only relevant
for libraries, it has no effect on other sections. This is used by
`cargo test`.

###### 1.1.9.1.5: `bench`

A flag for enabling benchmarks for this product. This is used by `cargo bench`.

###### 1.1.9.1.6: `doc`

A flag for enabling documentation of this product. This is used by `cargo doc`.

###### 1.1.9.1.7: `plugin`

If the product is meant to be a compiler plugin, this field must be set to true
for Cargo to correctly compile it and make it available for all dependencies.

###### 1.1.9.1.8: `proc_macro`

If the product is meant to be a "macros 1.1" procedural macro, this field must
be set to true.

###### 1.1.9.1.9: `harness`

If set to false, `cargo test` will omit the `--test` flag to rustc, which
stops it from generating a test harness. This is useful when the binary being
built manages the test runner itself.

###### 1.1.9.1.10: `edition`

If set then a product can be configured to use a different edition than the
`[package]` is configured to use, perhaps only compiling a library with the
2018 edition or only compiling one unit test with the 2015 edition. By default
all products are compiled with the edition specified in `[package]`.

###### 1.1.9.1.11: `required_features`

The required-features field specifies which features the product needs in order to be built.
If any of the required features are not selected, the product will be skipped.
This is only relevant for the `[[bin]]`, `[[bench]]`, `[[test]]`, and `[[example]]` sections,
it has no effect on `[lib]`.

###### 1.1.9.1.12: `crate_type`

The available options are "dylib", "rlib", "staticlib", "cdylib", and "proc-macro".

##### 1.1.9.2: Trait Implementations for `Product`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Product {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.1.10: `struct Profile`

```rust
pub struct Profile {
    pub opt_level: option::Option<toml::value::Value>,
    pub debug: option::Option<toml::value::Value>,
    pub rpath: option::Option<bool>,
    pub inherits: option::Option<String>,
    pub lto: option::Option<toml::value::Value>,
    pub debug_assertions: option::Option<bool>,
    pub codegen_units: option::Option<u16>,
    pub panic: option::Option<String>,
    pub incremental: option::Option<bool>,
    pub overflow_checks: option::Option<bool>,
    pub strip: option::Option<cargo_manifest::StripSetting>,
    pub package: collections::btree::map::BTreeMap<String, toml::value::Value>,
    pub split_debuginfo: option::Option<String>,
    pub build_override: option::Option<toml::value::Value>,
}
```

##### 1.1.10.1: Fields

###### 1.1.10.1.1: `build_override`

profile overrides

##### 1.1.10.2: Trait Implementations for `Profile`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Profile {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.1.11: `struct Profiles`

```rust
pub struct Profiles {
    pub release: option::Option<cargo_manifest::Profile>,
    pub dev: option::Option<cargo_manifest::Profile>,
    pub test: option::Option<cargo_manifest::Profile>,
    pub bench: option::Option<cargo_manifest::Profile>,
    pub doc: option::Option<cargo_manifest::Profile>,
    pub custom: collections::btree::map::BTreeMap<String, cargo_manifest::Profile>,
}
```

##### 1.1.11.1: Trait Implementations for `Profiles`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Profiles {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.1.12: `struct Target`

```rust
pub struct Target {
    pub dependencies: cargo_manifest::DepsSet,
    pub dev_dependencies: cargo_manifest::DepsSet,
    pub build_dependencies: cargo_manifest::DepsSet,
}
```

##### 1.1.12.1: Trait Implementations for `Target`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Target {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.1.13: `struct Workspace<Metadata = toml::value::Value>`

```rust
pub struct Workspace<Metadata = toml::value::Value> {
    pub members: Vec<String>,
    pub default_members: option::Option<Vec<String>>,
    pub exclude: option::Option<Vec<String>>,
    pub resolver: option::Option<cargo_manifest::Resolver>,
    pub dependencies: option::Option<cargo_manifest::DepsSet>,
    pub package: option::Option<cargo_manifest::WorkspacePackage>,
    pub metadata: option::Option<Metadata>,
}
```

##### 1.1.13.1: Trait Implementations for `Workspace`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de, Metadata> serde::de::Deserialize<'de> for cargo_manifest::Workspace<Metadata> where Metadata: serde::de::Deserialize<'de> {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```

- `serde::ser::Serialize`

    ```rust
    impl<Metadata> serde::ser::Serialize for cargo_manifest::Workspace<Metadata> where Metadata: serde::ser::Serialize {
        pub fn serialize<__S> where __S: serde::ser::Serializer(self: &Self, __serializer: __S) -> result::Result<<__S as serde::ser::Serializer>::Ok, <__S as serde::ser::Serializer>::Error> { ... }
    }
    ```


#### 1.1.14: `struct WorkspacePackage`

```rust
pub struct WorkspacePackage {
    pub edition: option::Option<cargo_manifest::Edition>,
    pub version: option::Option<String>,
    pub authors: option::Option<Vec<String>>,
    pub description: option::Option<String>,
    pub homepage: option::Option<String>,
    pub documentation: option::Option<String>,
    pub readme: option::Option<cargo_manifest::StringOrBool>,
    pub keywords: option::Option<Vec<String>>,
    pub categories: option::Option<Vec<String>>,
    pub license: option::Option<String>,
    pub license_file: option::Option<String>,
    pub publish: option::Option<cargo_manifest::Publish>,
    pub exclude: option::Option<Vec<String>>,
    pub include: option::Option<Vec<String>>,
    pub repository: option::Option<String>,
    pub rust_version: option::Option<String>,
}
```

The workspace.package table is where you define keys that can be inherited by members of a
workspace. These keys can be inherited by defining them in the member package with
`{key}.workspace = true`.

See <https://doc.rust-lang.org/nightly/cargo/reference/workspaces.html#the-package-table>
for more details.

##### 1.1.14.1: Fields

###### 1.1.14.1.1: `version`

e.g. "1.9.0"

###### 1.1.14.1.2: `authors`

e.g. ["Author <e@mail>", "etc"]

###### 1.1.14.1.3: `description`

A short blurb about the package. This is not rendered in any format when
uploaded to crates.io (aka this is not markdown).

###### 1.1.14.1.4: `readme`

This points to a file under the package root (relative to this `Cargo.toml`).

###### 1.1.14.1.5: `categories`

This is a list of up to five categories where this crate would fit.
e.g. ["command-line-utilities", "development-tools::cargo-plugins"]

###### 1.1.14.1.6: `license`

e.g. "MIT"

###### 1.1.14.1.7: `rust_version`

e.g. "1.63.0"

##### 1.1.14.2: Trait Implementations for `WorkspacePackage`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::WorkspacePackage {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


### 1.2: Enums

#### 1.2.1: `enum Dependency`

```rust
pub enum Dependency {
    Simple(String),
    Inherited(cargo_manifest::InheritedDependencyDetail),
    Detailed(cargo_manifest::DependencyDetail),
}
```

##### 1.2.1.2: `impl cargo_manifest::Dependency`

###### 1.2.1.2.2: `fn detail(self: &Self) -> option::Option<&cargo_manifest::DependencyDetail>`


###### 1.2.1.2.3: `fn simplify(self: Self) -> Self`

Simplifies `Dependency::Detailed` to `Dependency::Simple` if only the
`version` field inside the `DependencyDetail` struct is set.

###### 1.2.1.2.4: `fn req(self: &Self) -> &str`


###### 1.2.1.2.5: `fn req_features(self: &Self) -> &[String]`


###### 1.2.1.2.6: `fn optional(self: &Self) -> bool`


###### 1.2.1.2.7: `fn package(self: &Self) -> option::Option<&str>`


###### 1.2.1.2.8: `fn git(self: &Self) -> option::Option<&str>`


###### 1.2.1.2.9: `fn is_crates_io(self: &Self) -> bool`


##### 1.2.1.2: Trait Implementations for `Dependency`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Dependency {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.2.2: `enum Edition`

```rust
pub enum Edition {
    E2015,
    E2018,
    E2021,
    E2024,
}
```

##### 1.2.2.2: `impl cargo_manifest::Edition`

###### 1.2.2.2.2: `fn as_str(self: &Self) -> &'static str`


##### 1.2.2.2: Trait Implementations for `Edition`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Copy`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Edition {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.2.3: `enum Error`

```rust
pub enum Error {
    Parse(toml::de::Error),
    Io(io::error::Error),
    Utf8(str::error::Utf8Error),
    Other(String),
}
```

##### 1.2.3.1: Trait Implementations for `Error`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Display`
- `convert::From<io::error::Error>`
- `convert::From<str::error::Utf8Error>`
- `convert::From<toml::de::Error>`
- `error::Error`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `ToString` (`where T: Display + ?Sized`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)

#### 1.2.4: `enum MaintenanceStatus`

```rust
pub enum MaintenanceStatus {
    None,
    ActivelyDeveloped,
    PassivelyMaintained,
    AsIs,
    Experimental,
    LookingForMaintainer,
    Deprecated,
}
```

##### 1.2.4.1: Trait Implementations for `MaintenanceStatus`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Copy`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::MaintenanceStatus {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.2.5: `enum MaybeInherited<T>`

```rust
pub enum MaybeInherited<T> {
    Inherited { workspace: {id:589} },
    Local(T),
}
```

Used as a wrapper for properties that may be inherited by workspace-level settings.
It currently does not support more complex interactions (e.g. specifying part of the property
in the local manifest while inheriting another part of it from the workspace manifest, as it
happens for dependency features).

See [`cargo`'s documentation](https://doc.rust-lang.org/nightly/cargo/reference/workspaces.html#workspaces)
for more details.

##### 1.2.5.2: `impl<T> cargo_manifest::MaybeInherited<T>`

###### 1.2.5.2.2: `fn inherited() -> Self`


###### 1.2.5.2.3: `fn as_local(self: Self) -> option::Option<T>`


###### 1.2.5.2.4: `fn as_ref(self: &Self) -> cargo_manifest::MaybeInherited<&T>`

```rust
pub const fn as_ref(self: &Self) -> cargo_manifest::MaybeInherited<&T> { ... }
```

##### 1.2.5.2: Trait Implementations for `MaybeInherited`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de, T> serde::de::Deserialize<'de> for cargo_manifest::MaybeInherited<T> where T: serde::de::Deserialize<'de> {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```

- `serde::ser::Serialize`

    ```rust
    impl<T> serde::ser::Serialize for cargo_manifest::MaybeInherited<T> where T: serde::ser::Serialize {
        pub fn serialize<__S> where __S: serde::ser::Serializer(self: &Self, __serializer: __S) -> result::Result<<__S as serde::ser::Serializer>::Ok, <__S as serde::ser::Serializer>::Error> { ... }
    }
    ```


#### 1.2.6: `enum Publish`

```rust
pub enum Publish {
    Flag(bool),
    Registry(Vec<String>),
}
```

##### 1.2.6.1: Trait Implementations for `Publish`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `PartialEq<bool>`
- `PartialEq<cargo_manifest::Publish>`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Publish {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.2.7: `enum Resolver`

```rust
pub enum Resolver {
    V1,
    V2,
    V3,
}
```

##### 1.2.7.1: Trait Implementations for `Resolver`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Copy`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::Resolver {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.2.8: `enum StringOrBool`

```rust
pub enum StringOrBool {
    String(String),
    Bool(bool),
}
```

##### 1.2.8.1: Trait Implementations for `StringOrBool`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::StringOrBool {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### 1.2.9: `enum StripSetting`

```rust
pub enum StripSetting {
    None,
    Debuginfo,
    Symbols,
}
```

##### 1.2.9.2: Variants

###### 1.2.9.2.2: `None`

false

###### 1.2.9.2.3: `Symbols`

true

##### 1.2.9.2: Trait Implementations for `StripSetting`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `Clone`
- `Debug`
- `Eq`
- `Ord`
- `PartialEq`
- `PartialOrd`
- `StructuralPartialEq`
- `convert::TryFrom<toml::value::Value>`
- `serde::ser::Serialize`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)
- `equivalent::Comparable<K>`

    ```rust
    where
        Q: Ord + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `equivalent::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `hashbrown::Equivalent<K>`

    ```rust
    where
        Q: Eq + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

- `serde::de::DeserializeOwned` (`where T: for<'de> serde::de::Deserialize<'de>`)
- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for cargo_manifest::StripSetting {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


### 1.3: Traits

#### 1.3.1: `trait AbstractFilesystem`

```rust
pub trait AbstractFilesystem {
    pub fn file_names_in<T: convert::AsRef<path::Path>>(self: &Self, rel_path: T) -> io::error::Result<collections::btree::set::BTreeSet<Box<str>>>;;
}
```

A trait for abstracting over filesystem operations.

This trait is primarily used for target auto-discovery in the
[`complete_from_abstract_filesystem()`](crate::Manifest::complete_from_abstract_filesystem) method.

##### 1.3.1.2: Associated Items

###### 1.3.1.2.2: Associated Functions

####### 1.3.1.2.2.2: `fn file_names_in<T: convert::AsRef<path::Path>>(self: &Self, rel_path: T) -> io::error::Result<collections::btree::set::BTreeSet<Box<str>>>`

Returns a set of file and folder names in the given directory.

This method should return a [std::io::ErrorKind::NotFound] error if the
directory does not exist.

##### 1.3.1.3: Implementors

###### 1.3.1.3.2: `impl cargo_manifest::afs::AbstractFilesystem for cargo_manifest::afs::Filesystem<'_>`


### 1.4: Type Aliases

#### 1.4.1: `type DepsSet`


#### 1.4.2: `type FeatureSet`


#### 1.4.3: `type PatchSet`


#### 1.4.4: `type TargetDepsSet`


## 2: Module: `{id:54}`



