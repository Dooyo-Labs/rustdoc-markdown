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

````rust
let json_string = std::fs::read_to_string("./target/doc/rustdoc_types.json")?;
let krate: rustdoc_types::Crate = serde_json::from_str(&json_string)?;

println!("the index has {} items", krate.index.len());
````

For performance sensitive crates, consider turning on the `rustc-hash`
feature. This switches all data structures from `std::collections::HashMap` to
`rustc-hash::FxHashMap` which improves performance when reading big JSON files
(like `aws_sdk_rs`'s).

`cargo-semver-checks` benchmarked this change with `aws_sdk_ec2`'s JSON and
[observed a -3% improvement to the runtime][csc benchmarks]. The performance
here depends on how much time you spend querying the `HashMap`s, so as always,
measure first.

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
1. Run `cargo test`
1. Run `./clgen.sh <old_version> <new_version>`
1. Follow printed instructions to commit and push.

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

[csc benchmarks]: https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc/topic/rustc-hash.20and.20performance.20of.20rustdoc-types/near/474855731


## 3: Common Traits

The following traits are commonly implemented by types in this crate. Unless otherwise noted, you can assume these traits are implemented:

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `PartialEq`
- `StructuralPartialEq`
- `serde::ser::Serialize`

- `serde::de::Deserialize<'de>`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

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


## 4: Module: `rustdoc_types`

Rustdoc's JSON output interface

These types are the public API exposed through the `--output-format json` flag. The \[`Crate`\]
struct is the root of the JSON blob and all other items are contained within.

We expose a `rustc-hash` feature that is disabled by default. This feature switches the
\[`std::collections::HashMap`\] for \[`rustc_hash::FxHashMap`\] to improve the performance of said
`HashMap` in specific situations.

`cargo-semver-checks` for example, saw a [-3% improvement][1] when benchmarking using the
`aws_sdk_ec2` JSON output (~500MB of JSON). As always, we recommend measuring the impact before
turning this feature on, as [`FxHashMap`][2] only concerns itself with hash speed, and may
increase the number of collisions.

[1]: https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc/topic/rustc-hash.20and.20performance.20of.20rustdoc-types/near/474855731
[2]: https://crates.io/crates/rustc-hash


### 4.1: Structs

#### 4.1.1: `struct rustdoc_types::AssocItemConstraint`

```rust
pub struct AssocItemConstraint {
    pub name: String,
    pub args: rustdoc_types::GenericArgs,
    pub binding: rustdoc_types::AssocItemConstraintKind,
}
```

Describes a bound applied to an associated type/constant.

Example:

````text
IntoIterator<Item = u32, IntoIter: Clone>
             ^^^^^^^^^^  ^^^^^^^^^^^^^^^
````

##### 4.1.1.1: Fields

###### 4.1.1.1.1: `name`

The name of the associated type/constant.

###### 4.1.1.1.2: `args`

Arguments provided to the associated type/constant.

###### 4.1.1.1.3: `binding`

The kind of bound applied to the associated type/constant.


#### 4.1.2: `struct rustdoc_types::Constant`

```rust
pub struct Constant {
    pub expr: String,
    pub value: option::Option<String>,
    pub is_literal: bool,
}
```

A constant.

##### 4.1.2.1: Fields

###### 4.1.2.1.1: `expr`

The stringified expression of this constant. Note that its mapping to the original
source code is unstable and it's not guaranteed that it'll match the source code.

###### 4.1.2.1.2: `value`

The value of the evaluated expression for this constant, which is only computed for numeric
types.

###### 4.1.2.1.3: `is_literal`

Whether this constant is a bool, numeric, string, or char literal.


#### 4.1.3: `struct rustdoc_types::Crate`

```rust
pub struct Crate {
    pub root: rustdoc_types::Id,
    pub crate_version: option::Option<String>,
    pub includes_private: bool,
    pub index: collections::hash::map::HashMap<rustdoc_types::Id, rustdoc_types::Item>,
    pub paths: collections::hash::map::HashMap<rustdoc_types::Id, rustdoc_types::ItemSummary>,
    pub external_crates: collections::hash::map::HashMap<u32, rustdoc_types::ExternalCrate>,
    pub format_version: u32,
}
```

The root of the emitted JSON blob.

It contains all type/documentation information
about the language items in the local crate, as well as info about external items to allow
tools to find or link to them.

##### 4.1.3.1: Fields

###### 4.1.3.1.1: `root`

The id of the root \[`Module`\] item of the local crate.

###### 4.1.3.1.2: `crate_version`

The version string given to `--crate-version`, if any.

###### 4.1.3.1.3: `includes_private`

Whether or not the output includes private items.

###### 4.1.3.1.4: `index`

A collection of all items in the local crate as well as some external traits and their
items that are referenced locally.

###### 4.1.3.1.5: `paths`

Maps IDs to fully qualified paths and other info helpful for generating links.

###### 4.1.3.1.6: `external_crates`

Maps `crate_id` of items to a crate name and html_root_url if it exists.

###### 4.1.3.1.7: `format_version`

A single version number to be used in the future when making backwards incompatible changes
to the JSON output.

##### 4.1.3.2: Trait Implementations for `Crate`

**(Note: Does not implement common trait(s): `Hash`)**


#### 4.1.4: `struct rustdoc_types::Deprecation`

```rust
pub struct Deprecation {
    pub since: option::Option<String>,
    pub note: option::Option<String>,
}
```

Information about the deprecation of an \[`Item`\].

##### 4.1.4.1: Fields

###### 4.1.4.1.1: `since`

Usually a version number when this \[`Item`\] first became deprecated.

###### 4.1.4.1.2: `note`

The reason for deprecation and/or what alternatives to use.


#### 4.1.5: `struct rustdoc_types::Discriminant`

```rust
pub struct Discriminant {
    pub expr: String,
    pub value: String,
}
```

The value that distinguishes a variant in an \[`Enum`\] from other variants.

##### 4.1.5.1: Fields

###### 4.1.5.1.1: `expr`

The expression that produced the discriminant.

Unlike `value`, this preserves the original formatting (eg suffixes,
hexadecimal, and underscores), making it unsuitable to be machine
interpreted.

In some cases, when the value is too complex, this may be `"{ _ }"`.
When this occurs is unstable, and may change without notice.

###### 4.1.5.1.2: `value`

The numerical value of the discriminant. Stored as a string due to
JSON's poor support for large integers, and the fact that it would need
to store from \[`i128::MIN`\] to \[`u128::MAX`\].


#### 4.1.6: `struct rustdoc_types::DynTrait`

```rust
pub struct DynTrait {
    pub traits: Vec<rustdoc_types::PolyTrait>,
    pub lifetime: option::Option<String>,
}
```

Dynamic trait object type (`dyn Trait`).

##### 4.1.6.1: Fields

###### 4.1.6.1.1: `traits`

All the traits implemented. One of them is the vtable, and the rest must be auto traits.

###### 4.1.6.1.2: `lifetime`

The lifetime of the whole dyn object

````text
dyn Debug + 'static
            ^^^^^^^
            |
            this part
````


#### 4.1.7: `struct rustdoc_types::Enum`

```rust
pub struct Enum {
    pub generics: rustdoc_types::Generics,
    pub has_stripped_variants: bool,
    pub variants: Vec<rustdoc_types::Id>,
    pub impls: Vec<rustdoc_types::Id>,
}
```

An `enum`.

##### 4.1.7.1: Fields

###### 4.1.7.1.1: `generics`

Information about the type parameters and `where` clauses of the enum.

###### 4.1.7.1.2: `has_stripped_variants`

Whether any variants have been removed from the result, due to being private or hidden.

###### 4.1.7.1.3: `variants`

The list of variants in the enum.

All of the corresponding \[`Item`\]s are of kind \[`ItemEnum::Variant`\]

###### 4.1.7.1.4: `impls`

`impl`s for the enum.


#### 4.1.8: `struct rustdoc_types::ExternalCrate`

```rust
pub struct ExternalCrate {
    pub name: String,
    pub html_root_url: option::Option<String>,
}
```

Metadata of a crate, either the same crate on which `rustdoc` was invoked, or its dependency.

##### 4.1.8.1: Fields

###### 4.1.8.1.1: `name`

The name of the crate.

Note: This is the [*crate* name][crate-name], which may not be the same as the
[*package* name][package-name]. For example, for <https://crates.io/crates/regex-syntax>,
this field will be `regex_syntax` (which uses an `_`, not a `-`).

[crate-name]: https://doc.rust-lang.org/stable/cargo/reference/cargo-targets.html#the-name-field
[package-name]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-name-field

###### 4.1.8.1.2: `html_root_url`

The root URL at which the crate's documentation lives.


#### 4.1.9: `struct rustdoc_types::Function`

```rust
pub struct Function {
    pub sig: rustdoc_types::FunctionSignature,
    pub generics: rustdoc_types::Generics,
    pub header: rustdoc_types::FunctionHeader,
    pub has_body: bool,
}
```

A function declaration (including methods and other associated functions).

##### 4.1.9.1: Fields

###### 4.1.9.1.1: `sig`

Information about the function signature, or declaration.

###### 4.1.9.1.2: `generics`

Information about the function’s type parameters and `where` clauses.

###### 4.1.9.1.3: `header`

Information about core properties of the function, e.g. whether it's `const`, its ABI, etc.

###### 4.1.9.1.4: `has_body`

Whether the function has a body, i.e. an implementation.


#### 4.1.10: `struct rustdoc_types::FunctionHeader`

```rust
pub struct FunctionHeader {
    pub is_const: bool,
    pub is_unsafe: bool,
    pub is_async: bool,
    pub abi: rustdoc_types::Abi,
}
```

A set of fundamental properties of a function.

##### 4.1.10.1: Fields

###### 4.1.10.1.1: `is_const`

Is this function marked as `const`?

###### 4.1.10.1.2: `is_unsafe`

Is this function unsafe?

###### 4.1.10.1.3: `is_async`

Is this function async?

###### 4.1.10.1.4: `abi`

The ABI used by the function.


#### 4.1.11: `struct rustdoc_types::FunctionPointer`

```rust
pub struct FunctionPointer {
    pub sig: rustdoc_types::FunctionSignature,
    pub generic_params: Vec<rustdoc_types::GenericParamDef>,
    pub header: rustdoc_types::FunctionHeader,
}
```

A type that is a function pointer.

##### 4.1.11.1: Fields

###### 4.1.11.1.1: `sig`

The signature of the function.

###### 4.1.11.1.2: `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)

````ignore (incomplete expression)
   for<'c> fn(val: &'c i32) -> i32
// ^^^^^^^
````

###### 4.1.11.1.3: `header`

The core properties of the function, such as the ABI it conforms to, whether it's unsafe, etc.


#### 4.1.12: `struct rustdoc_types::FunctionSignature`

```rust
pub struct FunctionSignature {
    pub inputs: Vec<(String, rustdoc_types::Type)>,
    pub output: option::Option<rustdoc_types::Type>,
    pub is_c_variadic: bool,
}
```

The signature of a function.

##### 4.1.12.1: Fields

###### 4.1.12.1.1: `inputs`

List of argument names and their type.

Note that not all names will be valid identifiers, as some of
them may be patterns.

###### 4.1.12.1.2: `output`

The output type, if specified.

###### 4.1.12.1.3: `is_c_variadic`

Whether the function accepts an arbitrary amount of trailing arguments the C way.

````ignore (incomplete code)
fn printf(fmt: &str, ...);
````


#### 4.1.13: `struct rustdoc_types::GenericParamDef`

```rust
pub struct GenericParamDef {
    pub name: String,
    pub kind: rustdoc_types::GenericParamDefKind,
}
```

One generic parameter accepted by an item.

##### 4.1.13.1: Fields

###### 4.1.13.1.1: `name`

Name of the parameter.

````rust
fn f<'resource, Resource>(x: &'resource Resource) {}
//    ^^^^^^^^  ^^^^^^^^
````

###### 4.1.13.1.2: `kind`

The kind of the parameter and data specific to a particular parameter kind, e.g. type
bounds.


#### 4.1.14: `struct rustdoc_types::Generics`

```rust
pub struct Generics {
    pub params: Vec<rustdoc_types::GenericParamDef>,
    pub where_predicates: Vec<rustdoc_types::WherePredicate>,
}
```

Generic parameters accepted by an item and `where` clauses imposed on it and the parameters.

##### 4.1.14.1: Fields

###### 4.1.14.1.1: `params`

A list of generic parameter definitions (e.g. `<T: Clone + Hash, U: Copy>`).

###### 4.1.14.1.2: `where_predicates`

A list of where predicates (e.g. `where T: Iterator, T::Item: Copy`).


#### 4.1.15: `struct rustdoc_types::Id`

```rust
pub struct Id(pub u32);
```

An opaque identifier for an item.

It can be used to lookup in \[`Crate::index`\] or \[`Crate::paths`\] to resolve it
to an \[`Item`\].

Id's are only valid within a single JSON blob. They cannot be used to
resolve references between the JSON output's for different crates.

Rustdoc makes no guarantees about the inner value of Id's. Applications
should treat them as opaque keys to lookup items, and avoid attempting
to parse them, or otherwise depend on any implementation details.

##### 4.1.15.1: Trait Implementations for `Id`

- `Copy`

#### 4.1.16: `struct rustdoc_types::Impl`

```rust
pub struct Impl {
    pub is_unsafe: bool,
    pub generics: rustdoc_types::Generics,
    pub provided_trait_methods: Vec<String>,
    #[serde(rename = "trait")] pub trait_: option::Option<rustdoc_types::Path>,
    #[serde(rename = "for")] pub for_: rustdoc_types::Type,
    pub items: Vec<rustdoc_types::Id>,
    pub is_negative: bool,
    pub is_synthetic: bool,
    pub blanket_impl: option::Option<rustdoc_types::Type>,
}
```

An `impl` block.

##### 4.1.16.1: Fields

###### 4.1.16.1.1: `is_unsafe`

Whether this impl is for an unsafe trait.

###### 4.1.16.1.2: `generics`

Information about the impl’s type parameters and `where` clauses.

###### 4.1.16.1.3: `provided_trait_methods`

The list of the names of all the trait methods that weren't mentioned in this impl but
were provided by the trait itself.

For example, for this impl of the \[`PartialEq`\] trait:

````rust
struct Foo;

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool { todo!() }
}
````

This field will be `["ne"]`, as it has a default implementation defined for it.

###### 4.1.16.1.4: `trait_`

The trait being implemented or `None` if the impl is inherent, which means
`impl Struct {}` as opposed to `impl Trait for Struct {}`.

###### 4.1.16.1.5: `for_`

The type that the impl block is for.

###### 4.1.16.1.6: `items`

The list of associated items contained in this impl block.

###### 4.1.16.1.7: `is_negative`

Whether this is a negative impl (e.g. `!Sized` or `!Send`).

###### 4.1.16.1.8: `is_synthetic`

Whether this is an impl that’s implied by the compiler
(for autotraits, e.g. `Send` or `Sync`).


#### 4.1.17: `struct rustdoc_types::Item`

```rust
pub struct Item {
    pub id: rustdoc_types::Id,
    pub crate_id: u32,
    pub name: option::Option<String>,
    pub span: option::Option<rustdoc_types::Span>,
    pub visibility: rustdoc_types::Visibility,
    pub docs: option::Option<String>,
    pub links: collections::hash::map::HashMap<String, rustdoc_types::Id>,
    pub attrs: Vec<String>,
    pub deprecation: option::Option<rustdoc_types::Deprecation>,
    pub inner: rustdoc_types::ItemEnum,
}
```

Anything that can hold documentation - modules, structs, enums, functions, traits, etc.

The `Item` data type holds fields that can apply to any of these,
and leaves kind-specific details (like function args or enum variants) to the `inner` field.

##### 4.1.17.1: Fields

###### 4.1.17.1.1: `id`

The unique identifier of this item. Can be used to find this item in various mappings.

###### 4.1.17.1.2: `crate_id`

This can be used as a key to the `external_crates` map of \[`Crate`\] to see which crate
this item came from.

###### 4.1.17.1.3: `name`

Some items such as impls don't have names.

###### 4.1.17.1.4: `span`

The source location of this item (absent if it came from a macro expansion or inline
assembly).

###### 4.1.17.1.5: `visibility`

By default all documented items are public, but you can tell rustdoc to output private items
so this field is needed to differentiate.

###### 4.1.17.1.6: `docs`

The full markdown docstring of this item. Absent if there is no documentation at all,
Some("") if there is some documentation but it is empty (EG `#[doc = ""]`).

###### 4.1.17.1.7: `links`

This mapping resolves [intra-doc links](https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md) from the docstring to their IDs

###### 4.1.17.1.8: `attrs`

Attributes on this item.

Does not include `#[deprecated]` attributes: see the \[`Self::deprecation`\] field instead.

Some attributes appear in pretty-printed Rust form, regardless of their formatting
in the original source code. For example:

* `#[non_exhaustive]` and `#[must_use]` are represented as themselves.
* `#[no_mangle]` and `#[export_name]` are also represented as themselves.
* `#[repr(C)]` and other reprs also appear as themselves,
  though potentially with a different order: e.g. `repr(i8, C)` may become `repr(C, i8)`.
  Multiple repr attributes on the same item may be combined into an equivalent single attr.

Other attributes may appear debug-printed. For example:

* `#[inline]` becomes something similar to `#[attr="Inline(Hint)"]`.

As an internal implementation detail subject to change, this debug-printing format
is currently equivalent to the HIR pretty-printing of parsed attributes.

###### 4.1.17.1.9: `deprecation`

Information about the item’s deprecation, if present.

###### 4.1.17.1.10: `inner`

The type-specific fields describing this item.

##### 4.1.17.2: Trait Implementations for `Item`

**(Note: Does not implement common trait(s): `Hash`)**


#### 4.1.18: `struct rustdoc_types::ItemSummary`

```rust
pub struct ItemSummary {
    pub crate_id: u32,
    pub path: Vec<String>,
    pub kind: rustdoc_types::ItemKind,
}
```

Information about an external (not defined in the local crate) \[`Item`\].

For external items, you don't get the same level of
information. This struct should contain enough to generate a link/reference to the item in
question, or can be used by a tool that takes the json output of multiple crates to find
the actual item definition with all the relevant info.

##### 4.1.18.1: Fields

###### 4.1.18.1.1: `crate_id`

Can be used to look up the name and html_root_url of the crate this item came from in the
`external_crates` map.

###### 4.1.18.1.2: `path`

The list of path components for the fully qualified path of this item (e.g.
`["std", "io", "lazy", "Lazy"]` for `std::io::lazy::Lazy`).

Note that items can appear in multiple paths, and the one chosen is implementation
defined. Currently, this is the full path to where the item was defined. Eg
\[`String`\] is currently `["alloc", "string", "String"]` and \[`HashMap`\]\[`std::collections::HashMap`\]
is `["std", "collections", "hash", "map", "HashMap"]`, but this is subject to change.

###### 4.1.18.1.3: `kind`

Whether this item is a struct, trait, macro, etc.


#### 4.1.19: `struct rustdoc_types::Module`

```rust
pub struct Module {
    pub is_crate: bool,
    pub items: Vec<rustdoc_types::Id>,
    pub is_stripped: bool,
}
```

A module declaration, e.g. `mod foo;` or `mod foo {}`.

##### 4.1.19.1: Fields

###### 4.1.19.1.1: `is_crate`

Whether this is the root item of a crate.

This item doesn't correspond to any construction in the source code and is generated by the
compiler.

###### 4.1.19.1.2: `items`

\[`Item`\]s declared inside this module.

###### 4.1.19.1.3: `is_stripped`

If `true`, this module is not part of the public API, but it contains
items that are re-exported as public API.


#### 4.1.20: `struct rustdoc_types::Path`

```rust
pub struct Path {
    pub path: String,
    pub id: rustdoc_types::Id,
    pub args: option::Option<Box<rustdoc_types::GenericArgs>>,
}
```

A type that has a simple path to it. This is the kind of type of structs, unions, enums, etc.

##### 4.1.20.1: Fields

###### 4.1.20.1.1: `path`

The path of the type.

This will be the path that is *used* (not where it is defined), so
multiple `Path`s may have different values for this field even if
they all refer to the same item. e.g.

````rust
pub type Vec1 = std::vec::Vec<i32>; // path: "std::vec::Vec"
pub type Vec2 = Vec<i32>; // path: "Vec"
pub type Vec3 = std::prelude::v1::Vec<i32>; // path: "std::prelude::v1::Vec"
````

###### 4.1.20.1.2: `id`

The ID of the type.

###### 4.1.20.1.3: `args`

Generic arguments to the type.

````ignore (incomplete expression)
std::borrow::Cow<'static, str>
//              ^^^^^^^^^^^^^^
````


#### 4.1.21: `struct rustdoc_types::PolyTrait`

```rust
pub struct PolyTrait {
    #[serde(rename = "trait")] pub trait_: rustdoc_types::Path,
    pub generic_params: Vec<rustdoc_types::GenericParamDef>,
}
```

A trait and potential HRTBs

##### 4.1.21.1: Fields

###### 4.1.21.1.1: `trait_`

The path to the trait.

###### 4.1.21.1.2: `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)

````text
dyn for<'a> Fn() -> &'a i32"
    ^^^^^^^
````


#### 4.1.22: `struct rustdoc_types::Primitive`

```rust
pub struct Primitive {
    pub name: String,
    pub impls: Vec<rustdoc_types::Id>,
}
```

A primitive type declaration. Declarations of this kind can only come from the core library.

##### 4.1.22.1: Fields

###### 4.1.22.1.1: `name`

The name of the type.

###### 4.1.22.1.2: `impls`

The implementations, inherent and of traits, on the primitive type.


#### 4.1.23: `struct rustdoc_types::ProcMacro`

```rust
pub struct ProcMacro {
    pub kind: rustdoc_types::MacroKind,
    pub helpers: Vec<String>,
}
```

A procedural macro.

##### 4.1.23.1: Fields

###### 4.1.23.1.1: `kind`

How this macro is supposed to be called: `foo!()`, `#[foo]` or `#[derive(foo)]`

###### 4.1.23.1.2: `helpers`

Helper attributes defined by a macro to be used inside it.

Defined only for derive macros.

E.g. the \[`Default`\] derive macro defines a `#[default]` helper attribute so that one can
do:

````rust
#[derive(Default)]
enum Option<T> {
    #[default]
    None,
    Some(T),
}
````


#### 4.1.24: `struct rustdoc_types::Span`

```rust
pub struct Span {
    pub filename: path::PathBuf,
    pub begin: (usize, usize),
    pub end: (usize, usize),
}
```

A range of source code.

##### 4.1.24.1: Fields

###### 4.1.24.1.1: `filename`

The path to the source file for this span relative to the path `rustdoc` was invoked with.

###### 4.1.24.1.2: `begin`

Zero indexed Line and Column of the first character of the `Span`

###### 4.1.24.1.3: `end`

Zero indexed Line and Column of the last character of the `Span`


#### 4.1.25: `struct rustdoc_types::Static`

```rust
pub struct Static {
    #[serde(rename = "type")] pub type_: rustdoc_types::Type,
    pub is_mutable: bool,
    pub expr: String,
    pub is_unsafe: bool,
}
```

A `static` declaration.

##### 4.1.25.1: Fields

###### 4.1.25.1.1: `type_`

The type of the static.

###### 4.1.25.1.2: `is_mutable`

This is `true` for mutable statics, declared as `static mut X: T = f();`

###### 4.1.25.1.3: `expr`

The stringified expression for the initial value.

It's not guaranteed that it'll match the actual source code for the initial value.

###### 4.1.25.1.4: `is_unsafe`

Is the static `unsafe`?

This is only true if it's in an `extern` block, and not explicity marked
as `safe`.

````rust
unsafe extern {
    static A: i32;      // unsafe
    safe static B: i32; // safe
}

static C: i32 = 0;     // safe
static mut D: i32 = 0; // safe
````


#### 4.1.26: `struct rustdoc_types::Struct`

```rust
pub struct Struct {
    pub kind: rustdoc_types::StructKind,
    pub generics: rustdoc_types::Generics,
    pub impls: Vec<rustdoc_types::Id>,
}
```

A `struct`.

##### 4.1.26.1: Fields

###### 4.1.26.1.1: `kind`

The kind of the struct (e.g. unit, tuple-like or struct-like) and the data specific to it,
i.e. fields.

###### 4.1.26.1.2: `generics`

The generic parameters and where clauses on this struct.

###### 4.1.26.1.3: `impls`

All impls (both of traits and inherent) for this struct.
All of the corresponding \[`Item`\]s are of kind \[`ItemEnum::Impl`\].


#### 4.1.27: `struct rustdoc_types::Trait`

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

##### 4.1.27.1: Fields

###### 4.1.27.1.1: `is_auto`

Whether the trait is marked `auto` and is thus implemented automatically
for all applicable types.

###### 4.1.27.1.2: `is_unsafe`

Whether the trait is marked as `unsafe`.

###### 4.1.27.1.3: `is_dyn_compatible`

Whether the trait is [dyn compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)\[^1\].

\[^1\]: Formerly known as "object safe".

###### 4.1.27.1.4: `items`

Associated \[`Item`\]s that can/must be implemented by the `impl` blocks.

###### 4.1.27.1.5: `generics`

Information about the type parameters and `where` clauses of the trait.

###### 4.1.27.1.6: `bounds`

Constraints that must be met by the implementor of the trait.

###### 4.1.27.1.7: `implementations`

The implementations of the trait.


#### 4.1.28: `struct rustdoc_types::TraitAlias`

```rust
pub struct TraitAlias {
    pub generics: rustdoc_types::Generics,
    pub params: Vec<rustdoc_types::GenericBound>,
}
```

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

##### 4.1.28.1: Fields

###### 4.1.28.1.1: `generics`

Information about the type parameters and `where` clauses of the alias.

###### 4.1.28.1.2: `params`

The bounds that are associated with the alias.


#### 4.1.29: `struct rustdoc_types::TypeAlias`

```rust
pub struct TypeAlias {
    #[serde(rename = "type")] pub type_: rustdoc_types::Type,
    pub generics: rustdoc_types::Generics,
}
```

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

##### 4.1.29.1: Fields

###### 4.1.29.1.1: `type_`

The type referred to by this alias.

###### 4.1.29.1.2: `generics`

Information about the type parameters and `where` clauses of the alias.


#### 4.1.30: `struct rustdoc_types::Union`

```rust
pub struct Union {
    pub generics: rustdoc_types::Generics,
    pub has_stripped_fields: bool,
    pub fields: Vec<rustdoc_types::Id>,
    pub impls: Vec<rustdoc_types::Id>,
}
```

A `union`.

##### 4.1.30.1: Fields

###### 4.1.30.1.1: `generics`

The generic parameters and where clauses on this union.

###### 4.1.30.1.2: `has_stripped_fields`

Whether any fields have been removed from the result, due to being private or hidden.

###### 4.1.30.1.3: `fields`

The list of fields in the union.

All of the corresponding \[`Item`\]s are of kind \[`ItemEnum::StructField`\].

###### 4.1.30.1.4: `impls`

All impls (both of traits and inherent) for this union.

All of the corresponding \[`Item`\]s are of kind \[`ItemEnum::Impl`\].


#### 4.1.31: `struct rustdoc_types::Use`

```rust
#[serde(rename_all = "snake_case")]
pub struct Use {
    pub source: String,
    pub name: String,
    pub id: option::Option<rustdoc_types::Id>,
    pub is_glob: bool,
}
```

A `use` statement.

##### 4.1.31.1: Fields

###### 4.1.31.1.1: `source`

The full path being imported.

###### 4.1.31.1.2: `name`

May be different from the last segment of `source` when renaming imports:
`use source as name;`

###### 4.1.31.1.3: `id`

The ID of the item being imported. Will be `None` in case of re-exports of primitives:

````rust
pub use i32 as my_i32;
````

###### 4.1.31.1.4: `is_glob`

Whether this statement is a wildcard `use`, e.g. `use source::*;`


#### 4.1.32: `struct rustdoc_types::Variant`

```rust
pub struct Variant {
    pub kind: rustdoc_types::VariantKind,
    pub discriminant: option::Option<rustdoc_types::Discriminant>,
}
```

A variant of an enum.

##### 4.1.32.1: Fields

###### 4.1.32.1.1: `kind`

Whether the variant is plain, a tuple-like, or struct-like. Contains the fields.

###### 4.1.32.1.2: `discriminant`

The discriminant, if explicitly specified.


### 4.2: Enums

#### 4.2.1: `enum rustdoc_types::Abi`

```rust
pub enum Abi {
    Rust,
    C{ unwind: bool },
    Cdecl{ unwind: bool },
    Stdcall{ unwind: bool },
    Fastcall{ unwind: bool },
    Aapcs{ unwind: bool },
    Win64{ unwind: bool },
    SysV64{ unwind: bool },
    System{ unwind: bool },
    Other(String),
}
```

The ABI (Application Binary Interface) used by a function.

If a variant has an `unwind` field, this means the ABI that it represents can be specified in 2
ways: `extern "_"` and `extern "_-unwind"`, and a value of `true` for that field signifies the
latter variant.

See the [Rustonomicon section](https://doc.rust-lang.org/nightly/nomicon/ffi.html#ffi-and-unwinding)
on unwinding for more info.

##### 4.2.1.1: Variants

###### 4.2.1.1.1: `Rust`

The default ABI, but that can also be written explicitly with `extern "Rust"`.

###### 4.2.1.1.2: `C { unwind: bool }`

Can be specified as `extern "C"` or, as a shorthand, just `extern`.

###### 4.2.1.1.3: `Cdecl { unwind: bool }`

Can be specified as `extern "cdecl"`.

###### 4.2.1.1.4: `Stdcall { unwind: bool }`

Can be specified as `extern "stdcall"`.

###### 4.2.1.1.5: `Fastcall { unwind: bool }`

Can be specified as `extern "fastcall"`.

###### 4.2.1.1.6: `Aapcs { unwind: bool }`

Can be specified as `extern "aapcs"`.

###### 4.2.1.1.7: `Win64 { unwind: bool }`

Can be specified as `extern "win64"`.

###### 4.2.1.1.8: `SysV64 { unwind: bool }`

Can be specified as `extern "sysv64"`.

###### 4.2.1.1.9: `System { unwind: bool }`

Can be specified as `extern "system"`.

###### 4.2.1.1.10: `Other(String)`

Any other ABI, including unstable ones.


#### 4.2.2: `enum rustdoc_types::AssocItemConstraintKind`

```rust
#[serde(rename_all = "snake_case")]
pub enum AssocItemConstraintKind {
    Equality(rustdoc_types::Term),
    Constraint(Vec<rustdoc_types::GenericBound>),
}
```

The way in which an associate type/constant is bound.

##### 4.2.2.1: Variants

###### 4.2.2.1.1: `Equality(rustdoc_types::Term)`

The required value/type is specified exactly. e.g.

````text
Iterator<Item = u32, IntoIter: DoubleEndedIterator>
         ^^^^^^^^^^
````

###### 4.2.2.1.2: `Constraint(Vec<rustdoc_types::GenericBound>)`

The type is required to satisfy a set of bounds.

````text
Iterator<Item = u32, IntoIter: DoubleEndedIterator>
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
````


#### 4.2.3: `enum rustdoc_types::GenericArg`

```rust
#[serde(rename_all = "snake_case")]
pub enum GenericArg {
    Lifetime(String),
    Type(rustdoc_types::Type),
    Const(rustdoc_types::Constant),
    Infer,
}
```

One argument in a list of generic arguments to a path segment.

Part of \[`GenericArgs`\].

##### 4.2.3.1: Variants

###### 4.2.3.1.1: `Lifetime(String)`

A lifetime argument.

````text
std::borrow::Cow<'static, str>
                 ^^^^^^^
````

###### 4.2.3.1.2: `Type(rustdoc_types::Type)`

A type argument.

````text
std::borrow::Cow<'static, str>
                          ^^^
````

###### 4.2.3.1.3: `Const(rustdoc_types::Constant)`

A constant as a generic argument.

````text
core::array::IntoIter<u32, { 640 * 1024 }>
                           ^^^^^^^^^^^^^^
````

###### 4.2.3.1.4: `Infer`

A generic argument that's explicitly set to be inferred.

````text
std::vec::Vec::<_>::new()
                ^
````


#### 4.2.4: `enum rustdoc_types::GenericArgs`

```rust
#[serde(rename_all = "snake_case")]
pub enum GenericArgs {
    AngleBracketed{ args: Vec<rustdoc_types::GenericArg>, constraints: Vec<rustdoc_types::AssocItemConstraint> },
    Parenthesized{ inputs: Vec<rustdoc_types::Type>, output: option::Option<rustdoc_types::Type> },
    ReturnTypeNotation,
}
```

A set of generic arguments provided to a path segment, e.g.

````text
std::option::Option::<u32>::None
                     ^^^^^
````

##### 4.2.4.1: Variants

###### 4.2.4.1.1: `AngleBracketed { args: Vec<rustdoc_types::GenericArg>, constraints: Vec<rustdoc_types::AssocItemConstraint> }`

`<'a, 32, B: Copy, C = u32>`

####### 4.2.4.1.1.1: Fields

######## 4.2.4.1.1.1.1: `args`

The list of each argument on this type.

````text
<'a, 32, B: Copy, C = u32>
 ^^^^^^
````

######## 4.2.4.1.1.1.2: `constraints`

Associated type or constant bindings (e.g. `Item=i32` or `Item: Clone`) for this type.

###### 4.2.4.1.2: `Parenthesized { inputs: Vec<rustdoc_types::Type>, output: option::Option<rustdoc_types::Type> }`

`Fn(A, B) -> C`

####### 4.2.4.1.2.1: Fields

######## 4.2.4.1.2.1.1: `inputs`

The input types, enclosed in parentheses.

######## 4.2.4.1.2.1.2: `output`

The output type provided after the `->`, if present.

###### 4.2.4.1.3: `ReturnTypeNotation`

`T::method(..)`


#### 4.2.5: `enum rustdoc_types::GenericBound`

```rust
#[serde(rename_all = "snake_case")]
pub enum GenericBound {
    TraitBound{ #[serde(rename = "trait")] trait_: rustdoc_types::Path, generic_params: Vec<rustdoc_types::GenericParamDef>, modifier: rustdoc_types::TraitBoundModifier },
    Outlives(String),
    Use(Vec<rustdoc_types::PreciseCapturingArg>),
}
```

Either a trait bound or a lifetime bound.

##### 4.2.5.1: Variants

###### 4.2.5.1.1: `TraitBound { trait_: rustdoc_types::Path, generic_params: Vec<rustdoc_types::GenericParamDef>, modifier: rustdoc_types::TraitBoundModifier }`

A trait bound.

####### 4.2.5.1.1.1: Fields

######## 4.2.5.1.1.1.1: `trait_`

The full path to the trait.

######## 4.2.5.1.1.1.2: `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)

````text
where F: for<'a, 'b> Fn(&'a u8, &'b u8)
         ^^^^^^^^^^^
         |
         this part
````

######## 4.2.5.1.1.1.3: `modifier`

The context for which a trait is supposed to be used, e.g. \`const

###### 4.2.5.1.2: `Outlives(String)`

A lifetime bound, e.g.

````rust
fn f<'a, T>(x: &'a str, y: &T) where T: 'a {}
//                                     ^^^
````

###### 4.2.5.1.3: `Use(Vec<rustdoc_types::PreciseCapturingArg>)`

`use<'a, T>` precise-capturing bound syntax


#### 4.2.6: `enum rustdoc_types::GenericParamDefKind`

```rust
#[serde(rename_all = "snake_case")]
pub enum GenericParamDefKind {
    Lifetime{ outlives: Vec<String> },
    Type{ bounds: Vec<rustdoc_types::GenericBound>, default: option::Option<rustdoc_types::Type>, is_synthetic: bool },
    Const{ #[serde(rename = "type")] type_: rustdoc_types::Type, default: option::Option<String> },
}
```

The kind of a \[`GenericParamDef`\].

##### 4.2.6.1: Variants

###### 4.2.6.1.1: `Lifetime { outlives: Vec<String> }`

Denotes a lifetime parameter.

####### 4.2.6.1.1.1: Fields

######## 4.2.6.1.1.1.1: `outlives`

Lifetimes that this lifetime parameter is required to outlive.

````rust
fn f<'a, 'b, 'resource: 'a + 'b>(a: &'a str, b: &'b str, res: &'resource str) {}
//                      ^^^^^^^
````

###### 4.2.6.1.2: `Type { bounds: Vec<rustdoc_types::GenericBound>, default: option::Option<rustdoc_types::Type>, is_synthetic: bool }`

Denotes a type parameter.

####### 4.2.6.1.2.1: Fields

######## 4.2.6.1.2.1.1: `bounds`

Bounds applied directly to the type. Note that the bounds from `where` clauses
that constrain this parameter won't appear here.

````rust
fn default2<T: Default>() -> [T; 2] where T: Clone { todo!() }
//             ^^^^^^^
````

######## 4.2.6.1.2.1.2: `default`

The default type for this parameter, if provided, e.g.

````rust
trait PartialEq<Rhs = Self> {}
//                    ^^^^
````

######## 4.2.6.1.2.1.3: `is_synthetic`

This is normally `false`, which means that this generic parameter is
declared in the Rust source text.

If it is `true`, this generic parameter has been introduced by the
compiler behind the scenes.

###### Example

Consider

````ignore (pseudo-rust)
pub fn f(_: impl Trait) {}
````

The compiler will transform this behind the scenes to

````ignore (pseudo-rust)
pub fn f<impl Trait: Trait>(_: impl Trait) {}
````

In this example, the generic parameter named `impl Trait` (and which
is bound by `Trait`) is synthetic, because it was not originally in
the Rust source text.

###### 4.2.6.1.3: `Const { type_: rustdoc_types::Type, default: option::Option<String> }`

Denotes a constant parameter.

####### 4.2.6.1.3.1: Fields

######## 4.2.6.1.3.1.1: `type_`

The type of the constant as declared.

######## 4.2.6.1.3.1.2: `default`

The stringified expression for the default value, if provided. It's not guaranteed that
it'll match the actual source code for the default value.


#### 4.2.7: `enum rustdoc_types::ItemEnum`

```rust
#[serde(rename_all = "snake_case")]
pub enum ItemEnum {
    Module(rustdoc_types::Module),
    ExternCrate{ name: String, rename: option::Option<String> },
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
    Constant{ #[serde(rename = "type")] type_: rustdoc_types::Type, #[serde(rename = "const")] const_: rustdoc_types::Constant },
    Static(rustdoc_types::Static),
    ExternType,
    Macro(String),
    ProcMacro(rustdoc_types::ProcMacro),
    Primitive(rustdoc_types::Primitive),
    AssocConst{ #[serde(rename = "type")] type_: rustdoc_types::Type, value: option::Option<String> },
    AssocType{ generics: rustdoc_types::Generics, bounds: Vec<rustdoc_types::GenericBound>, #[serde(rename = "type")] type_: option::Option<rustdoc_types::Type> },
}
```

Specific fields of an item.

Part of \[`Item`\].

##### 4.2.7.1: Variants

###### 4.2.7.1.1: `Module(rustdoc_types::Module)`

A module declaration, e.g. `mod foo;` or `mod foo {}`

###### 4.2.7.1.2: `ExternCrate { name: String, rename: option::Option<String> }`

A crate imported via the `extern crate` syntax.

####### 4.2.7.1.2.1: Fields

######## 4.2.7.1.2.1.1: `name`

The name of the imported crate.

######## 4.2.7.1.2.1.2: `rename`

If the crate is renamed, this is its name in the crate.

###### 4.2.7.1.3: `Use(rustdoc_types::Use)`

An import of 1 or more items into scope, using the `use` keyword.

###### 4.2.7.1.4: `Union(rustdoc_types::Union)`

A `union` declaration.

###### 4.2.7.1.5: `Struct(rustdoc_types::Struct)`

A `struct` declaration.

###### 4.2.7.1.6: `StructField(rustdoc_types::Type)`

A field of a struct.

###### 4.2.7.1.7: `Enum(rustdoc_types::Enum)`

An `enum` declaration.

###### 4.2.7.1.8: `Variant(rustdoc_types::Variant)`

A variant of a enum.

###### 4.2.7.1.9: `Function(rustdoc_types::Function)`

A function declaration (including methods and other associated functions)

###### 4.2.7.1.10: `Trait(rustdoc_types::Trait)`

A `trait` declaration.

###### 4.2.7.1.11: `TraitAlias(rustdoc_types::TraitAlias)`

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

###### 4.2.7.1.12: `Impl(rustdoc_types::Impl)`

An `impl` block.

###### 4.2.7.1.13: `TypeAlias(rustdoc_types::TypeAlias)`

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

###### 4.2.7.1.14: `Constant { type_: rustdoc_types::Type, const_: rustdoc_types::Constant }`

The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

####### 4.2.7.1.14.1: Fields

######## 4.2.7.1.14.1.1: `type_`

The type of the constant.

######## 4.2.7.1.14.1.2: `const_`

The declared constant itself.

###### 4.2.7.1.15: `Static(rustdoc_types::Static)`

A declaration of a `static`.

###### 4.2.7.1.16: `ExternType`

`type`s from an `extern` block.

See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

###### 4.2.7.1.17: `Macro(String)`

A macro_rules! declarative macro. Contains a single string with the source
representation of the macro with the patterns stripped.

###### 4.2.7.1.18: `ProcMacro(rustdoc_types::ProcMacro)`

A procedural macro.

###### 4.2.7.1.19: `Primitive(rustdoc_types::Primitive)`

A primitive type, e.g. `u32`.

\[`Item`\]s of this kind only come from the core library.

###### 4.2.7.1.20: `AssocConst { type_: rustdoc_types::Type, value: option::Option<String> }`

An associated constant of a trait or a type.

####### 4.2.7.1.20.1: Fields

######## 4.2.7.1.20.1.1: `type_`

The type of the constant.

######## 4.2.7.1.20.1.2: `value`

Inside a trait declaration, this is the default value for the associated constant,
if provided.
Inside an `impl` block, this is the value assigned to the associated constant,
and will always be present.

The representation is implementation-defined and not guaranteed to be representative of
either the resulting value or of the source code.

````rust
const X: usize = 640 * 1024;
//               ^^^^^^^^^^
````

###### 4.2.7.1.21: `AssocType { generics: rustdoc_types::Generics, bounds: Vec<rustdoc_types::GenericBound>, type_: option::Option<rustdoc_types::Type> }`

An associated type of a trait or a type.

####### 4.2.7.1.21.1: Fields

######## 4.2.7.1.21.1.1: `generics`

The generic parameters and where clauses on ahis associated type.

######## 4.2.7.1.21.1.2: `bounds`

The bounds for this associated type. e.g.

````rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
//                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
}
````

######## 4.2.7.1.21.1.3: `type_`

Inside a trait declaration, this is the default for the associated type, if provided.
Inside an impl block, this is the type assigned to the associated type, and will always
be present.

````rust
type X = usize;
//       ^^^^^
````


#### 4.2.8: `enum rustdoc_types::ItemKind`

```rust
#[serde(rename_all = "snake_case")]
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

The fundamental kind of an item. Unlike \[`ItemEnum`\], this does not carry any additional info.

Part of \[`ItemSummary`\].

##### 4.2.8.1: Variants

###### 4.2.8.1.1: `Module`

A module declaration, e.g. `mod foo;` or `mod foo {}`

###### 4.2.8.1.2: `ExternCrate`

A crate imported via the `extern crate` syntax.

###### 4.2.8.1.3: `Use`

An import of 1 or more items into scope, using the `use` keyword.

###### 4.2.8.1.4: `Struct`

A `struct` declaration.

###### 4.2.8.1.5: `StructField`

A field of a struct.

###### 4.2.8.1.6: `Union`

A `union` declaration.

###### 4.2.8.1.7: `Enum`

An `enum` declaration.

###### 4.2.8.1.8: `Variant`

A variant of a enum.

###### 4.2.8.1.9: `Function`

A function declaration, e.g. `fn f() {}`

###### 4.2.8.1.10: `TypeAlias`

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

###### 4.2.8.1.11: `Constant`

The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

###### 4.2.8.1.12: `Trait`

A `trait` declaration.

###### 4.2.8.1.13: `TraitAlias`

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

###### 4.2.8.1.14: `Impl`

An `impl` block.

###### 4.2.8.1.15: `Static`

A `static` declaration.

###### 4.2.8.1.16: `ExternType`

`type`s from an `extern` block.

See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

###### 4.2.8.1.17: `Macro`

A macro declaration.

Corresponds to either `ItemEnum::Macro(_)`
or `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Bang })`

###### 4.2.8.1.18: `ProcAttribute`

A procedural macro attribute.

Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Attr })`

###### 4.2.8.1.19: `ProcDerive`

A procedural macro usable in the `#[derive()]` attribute.

Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Derive })`

###### 4.2.8.1.20: `AssocConst`

An associated constant of a trait or a type.

###### 4.2.8.1.21: `AssocType`

An associated type of a trait or a type.

###### 4.2.8.1.22: `Primitive`

A primitive type, e.g. `u32`.

\[`Item`\]s of this kind only come from the core library.

###### 4.2.8.1.23: `Keyword`

A keyword declaration.

\[`Item`\]s of this kind only come from the come library and exist solely
to carry documentation for the respective keywords.

##### 4.2.8.2: Trait Implementations for `ItemKind`

- `Copy`

#### 4.2.9: `enum rustdoc_types::MacroKind`

```rust
#[serde(rename_all = "snake_case")]
pub enum MacroKind {
    Bang,
    Attr,
    Derive,
}
```

The way a \[`ProcMacro`\] is declared to be used.

##### 4.2.9.1: Variants

###### 4.2.9.1.1: `Bang`

A bang macro `foo!()`.

###### 4.2.9.1.2: `Attr`

An attribute macro `#[foo]`.

###### 4.2.9.1.3: `Derive`

A derive macro `#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]`

##### 4.2.9.2: Trait Implementations for `MacroKind`

- `Copy`

#### 4.2.10: `enum rustdoc_types::PreciseCapturingArg`

```rust
#[serde(rename_all = "snake_case")]
pub enum PreciseCapturingArg {
    Lifetime(String),
    Param(String),
}
```

One precise capturing argument. See [the rust reference](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing).

##### 4.2.10.1: Variants

###### 4.2.10.1.1: `Lifetime(String)`

A lifetime.

````rust
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
//                                                        ^^
````

###### 4.2.10.1.2: `Param(String)`

A type or constant parameter.

````rust
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
//                                                            ^  ^
````


#### 4.2.11: `enum rustdoc_types::StructKind`

```rust
#[serde(rename_all = "snake_case")]
pub enum StructKind {
    Unit,
    Tuple(Vec<option::Option<rustdoc_types::Id>>),
    Plain{ fields: Vec<rustdoc_types::Id>, has_stripped_fields: bool },
}
```

The kind of a \[`Struct`\] and the data specific to it, i.e. fields.

##### 4.2.11.1: Variants

###### 4.2.11.1.1: `Unit`

A struct with no fields and no parentheses.

````rust
pub struct Unit;
````

###### 4.2.11.1.2: `Tuple(Vec<option::Option<rustdoc_types::Id>>)`

A struct with unnamed fields.

All \[`Id`\]'s will point to \[`ItemEnum::StructField`\].
Unlike most of JSON, private and `#[doc(hidden)]` fields will be given as `None`
instead of being omitted, because order matters.

````rust
pub struct TupleStruct(i32);
pub struct EmptyTupleStruct();
````

###### 4.2.11.1.3: `Plain { fields: Vec<rustdoc_types::Id>, has_stripped_fields: bool }`

A struct with named fields.

````rust
pub struct PlainStruct { x: i32 }
pub struct EmptyPlainStruct {}
````

####### 4.2.11.1.3.1: Fields

######## 4.2.11.1.3.1.1: `fields`

The list of fields in the struct.

All of the corresponding \[`Item`\]s are of kind \[`ItemEnum::StructField`\].

######## 4.2.11.1.3.1.2: `has_stripped_fields`

Whether any fields have been removed from the result, due to being private or hidden.


#### 4.2.12: `enum rustdoc_types::Term`

```rust
#[serde(rename_all = "snake_case")]
pub enum Term {
    Type(rustdoc_types::Type),
    Constant(rustdoc_types::Constant),
}
```

Either a type or a constant, usually stored as the right-hand side of an equation in places like
\[`AssocItemConstraint`\]

##### 4.2.12.1: Variants

###### 4.2.12.1.1: `Type(rustdoc_types::Type)`

A type.

````rust
fn f(x: impl IntoIterator<Item = u32>) {}
//                               ^^^
````

###### 4.2.12.1.2: `Constant(rustdoc_types::Constant)`

A constant.

````ignore (incomplete feature in the snippet)
trait Foo {
    const BAR: usize;
}

fn f(x: impl Foo<BAR = 42>) {}
//                     ^^
````


#### 4.2.13: `enum rustdoc_types::TraitBoundModifier`

```rust
#[serde(rename_all = "snake_case")]
pub enum TraitBoundModifier {
    None,
    Maybe,
    MaybeConst,
}
```

A set of modifiers applied to a trait.

##### 4.2.13.1: Variants

###### 4.2.13.1.1: `None`

Marks the absence of a modifier.

###### 4.2.13.1.2: `Maybe`

Indicates that the trait bound relaxes a trait bound applied to a parameter by default,
e.g. `T: Sized?`, the `Sized` trait is required for all generic type parameters by default
unless specified otherwise with this modifier.

###### 4.2.13.1.3: `MaybeConst`

Indicates that the trait bound must be applicable in both a run-time and a compile-time
context.

##### 4.2.13.2: Trait Implementations for `TraitBoundModifier`

- `Copy`

#### 4.2.14: `enum rustdoc_types::Type`

```rust
#[serde(rename_all = "snake_case")]
pub enum Type {
    ResolvedPath(rustdoc_types::Path),
    DynTrait(rustdoc_types::DynTrait),
    Generic(String),
    Primitive(String),
    FunctionPointer(Box<rustdoc_types::FunctionPointer>),
    Tuple(Vec<rustdoc_types::Type>),
    Slice(Box<rustdoc_types::Type>),
    Array{ #[serde(rename = "type")] type_: Box<rustdoc_types::Type>, len: String },
    Pat{ #[serde(rename = "type")] type_: Box<rustdoc_types::Type> },
    ImplTrait(Vec<rustdoc_types::GenericBound>),
    Infer,
    RawPointer{ is_mutable: bool, #[serde(rename = "type")] type_: Box<rustdoc_types::Type> },
    BorrowedRef{ lifetime: option::Option<String>, is_mutable: bool, #[serde(rename = "type")] type_: Box<rustdoc_types::Type> },
    QualifiedPath{ name: String, args: Box<rustdoc_types::GenericArgs>, self_type: Box<rustdoc_types::Type>, #[serde(rename = "trait")] trait_: option::Option<rustdoc_types::Path> },
}
```

A type.

##### 4.2.14.1: Variants

###### 4.2.14.1.1: `ResolvedPath(rustdoc_types::Path)`

Structs, enums, unions and type aliases, e.g. `std::option::Option<u32>`

###### 4.2.14.1.2: `DynTrait(rustdoc_types::DynTrait)`

Dynamic trait object type (`dyn Trait`).

###### 4.2.14.1.3: `Generic(String)`

Parameterized types. The contained string is the name of the parameter.

###### 4.2.14.1.4: `Primitive(String)`

Built-in numeric types (e.g. `u32`, `f32`), `bool`, `char`.

###### 4.2.14.1.5: `FunctionPointer(Box<rustdoc_types::FunctionPointer>)`

A function pointer type, e.g. `fn(u32) -> u32`, `extern "C" fn() -> *const u8`

###### 4.2.14.1.6: `Tuple(Vec<rustdoc_types::Type>)`

A tuple type, e.g. `(String, u32, Box<usize>)`

###### 4.2.14.1.7: `Slice(Box<rustdoc_types::Type>)`

An unsized slice type, e.g. `[u32]`.

###### 4.2.14.1.8: `Array { type_: Box<rustdoc_types::Type>, len: String }`

An array type, e.g. `[u32; 15]`

####### 4.2.14.1.8.1: Fields

######## 4.2.14.1.8.1.1: `type_`

The type of the contained element.

######## 4.2.14.1.8.1.2: `len`

The stringified expression that is the length of the array.

Keep in mind that it's not guaranteed to match the actual source code of the expression.

###### 4.2.14.1.9: `Pat { type_: Box<rustdoc_types::Type> }`

A pattern type, e.g. `u32 is 1..`

See [the tracking issue](https://github.com/rust-lang/rust/issues/123646)

####### 4.2.14.1.9.1: Fields

######## 4.2.14.1.9.1.1: `type_`

The base type, e.g. the `u32` in `u32 is 1..`


_[Private fields hidden]_
###### 4.2.14.1.10: `ImplTrait(Vec<rustdoc_types::GenericBound>)`

An opaque type that satisfies a set of bounds, `impl TraitA + TraitB + ...`

###### 4.2.14.1.11: `Infer`

A type that's left to be inferred, `_`

###### 4.2.14.1.12: `RawPointer { is_mutable: bool, type_: Box<rustdoc_types::Type> }`

A raw pointer type, e.g. `*mut u32`, `*const u8`, etc.

####### 4.2.14.1.12.1: Fields

######## 4.2.14.1.12.1.1: `is_mutable`

This is `true` for `*mut _` and `false` for `*const _`.

######## 4.2.14.1.12.1.2: `type_`

The type of the pointee.

###### 4.2.14.1.13: `BorrowedRef { lifetime: option::Option<String>, is_mutable: bool, type_: Box<rustdoc_types::Type> }`

`&'a mut String`, `&str`, etc.

####### 4.2.14.1.13.1: Fields

######## 4.2.14.1.13.1.1: `lifetime`

The name of the lifetime of the reference, if provided.

######## 4.2.14.1.13.1.2: `is_mutable`

This is `true` for `&mut i32` and `false` for `&i32`

######## 4.2.14.1.13.1.3: `type_`

The type of the pointee, e.g. the `i32` in `&'a mut i32`

###### 4.2.14.1.14: `QualifiedPath { name: String, args: Box<rustdoc_types::GenericArgs>, self_type: Box<rustdoc_types::Type>, trait_: option::Option<rustdoc_types::Path> }`

Associated types like `<Type as Trait>::Name` and `T::Item` where
`T: Iterator` or inherent associated types like `Struct::Name`.

####### 4.2.14.1.14.1: Fields

######## 4.2.14.1.14.1.1: `name`

The name of the associated type in the parent type.

````ignore (incomplete expression)
<core::array::IntoIter<u32, 42> as Iterator>::Item
//                                            ^^^^
````

######## 4.2.14.1.14.1.2: `args`

The generic arguments provided to the associated type.

````ignore (incomplete expression)
<core::slice::IterMut<'static, u32> as BetterIterator>::Item<'static>
//                                                          ^^^^^^^^^
````

######## 4.2.14.1.14.1.3: `self_type`

The type with which this type is associated.

````ignore (incomplete expression)
<core::array::IntoIter<u32, 42> as Iterator>::Item
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
````

######## 4.2.14.1.14.1.4: `trait_`

`None` iff this is an *inherent* associated type.


#### 4.2.15: `enum rustdoc_types::VariantKind`

```rust
#[serde(rename_all = "snake_case")]
pub enum VariantKind {
    Plain,
    Tuple(Vec<option::Option<rustdoc_types::Id>>),
    Struct{ fields: Vec<rustdoc_types::Id>, has_stripped_fields: bool },
}
```

The kind of an \[`Enum`\] \[`Variant`\] and the data specific to it, i.e. fields.

##### 4.2.15.1: Variants

###### 4.2.15.1.1: `Plain`

A variant with no parentheses

````rust
enum Demo {
    PlainVariant,
    PlainWithDiscriminant = 1,
}
````

###### 4.2.15.1.2: `Tuple(Vec<option::Option<rustdoc_types::Id>>)`

A variant with unnamed fields.

All \[`Id`\]'s will point to \[`ItemEnum::StructField`\].
Unlike most of JSON, `#[doc(hidden)]` fields will be given as `None`
instead of being omitted, because order matters.

````rust
enum Demo {
    TupleVariant(i32),
    EmptyTupleVariant(),
}
````

###### 4.2.15.1.3: `Struct { fields: Vec<rustdoc_types::Id>, has_stripped_fields: bool }`

A variant with named fields.

````rust
enum Demo {
    StructVariant { x: i32 },
    EmptyStructVariant {},
}
````

####### 4.2.15.1.3.1: Fields

######## 4.2.15.1.3.1.1: `fields`

The list of variants in the enum.
All of the corresponding \[`Item`\]s are of kind \[`ItemEnum::Variant`\].

######## 4.2.15.1.3.1.2: `has_stripped_fields`

Whether any variants have been removed from the result, due to being private or hidden.


#### 4.2.16: `enum rustdoc_types::Visibility`

```rust
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Public,
    Default,
    Crate,
    Restricted{ parent: rustdoc_types::Id, path: String },
}
```

Visibility of an \[`Item`\].

##### 4.2.16.1: Variants

###### 4.2.16.1.1: `Public`

Explicitly public visibility set with `pub`.

###### 4.2.16.1.2: `Default`

For the most part items are private by default. The exceptions are associated items of
public traits and variants of public enums.

###### 4.2.16.1.3: `Crate`

Explicitly crate-wide visibility set with `pub(crate)`

###### 4.2.16.1.4: `Restricted { parent: rustdoc_types::Id, path: String }`

For `pub(in path)` visibility.

####### 4.2.16.1.4.1: Fields

######## 4.2.16.1.4.1.1: `parent`

ID of the module to which this visibility restricts items.

######## 4.2.16.1.4.1.2: `path`

The path with which [`parent`] was referenced
(like `super::super` or `crate::foo::bar`).

[`parent`]: Visibility::Restricted::parent


#### 4.2.17: `enum rustdoc_types::WherePredicate`

```rust
#[serde(rename_all = "snake_case")]
pub enum WherePredicate {
    BoundPredicate{ #[serde(rename = "type")] type_: rustdoc_types::Type, bounds: Vec<rustdoc_types::GenericBound>, generic_params: Vec<rustdoc_types::GenericParamDef> },
    LifetimePredicate{ lifetime: String, outlives: Vec<String> },
    EqPredicate{ lhs: rustdoc_types::Type, rhs: rustdoc_types::Term },
}
```

One `where` clause.

````rust
fn default<T>() -> T where T: Default { T::default() }
//                         ^^^^^^^^^^
````

##### 4.2.17.1: Variants

###### 4.2.17.1.1: `BoundPredicate { type_: rustdoc_types::Type, bounds: Vec<rustdoc_types::GenericBound>, generic_params: Vec<rustdoc_types::GenericParamDef> }`

A type is expected to comply with a set of bounds

####### 4.2.17.1.1.1: Fields

######## 4.2.17.1.1.1.1: `type_`

The type that's being constrained.

````rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                              ^
````

######## 4.2.17.1.1.1.2: `bounds`

The set of bounds that constrain the type.

````rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                                 ^^^^^^^^
````

######## 4.2.17.1.1.1.3: `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)

````rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                  ^^^^^^^
````

###### 4.2.17.1.2: `LifetimePredicate { lifetime: String, outlives: Vec<String> }`

A lifetime is expected to outlive other lifetimes.

####### 4.2.17.1.2.1: Fields

######## 4.2.17.1.2.1.1: `lifetime`

The name of the lifetime.

######## 4.2.17.1.2.1.2: `outlives`

The lifetimes that must be encompassed by the lifetime.

###### 4.2.17.1.3: `EqPredicate { lhs: rustdoc_types::Type, rhs: rustdoc_types::Term }`

A type must exactly equal another type.

####### 4.2.17.1.3.1: Fields

######## 4.2.17.1.3.1.1: `lhs`

The left side of the equation.

######## 4.2.17.1.3.1.2: `rhs`

The right side of the equation.


### 4.3: Constants

#### 4.3.1: `const FORMAT_VERSION`

The version of JSON output that this crate represents.

This integer is incremented with every breaking change to the API,
and is returned along with the JSON blob as \[`Crate::format_version`\].
Consuming code should assert that this value matches the format version(s) that it supports.


## 5: Examples Appendix

### 5.1: `simple_usage.rs`

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let json_string = std::fs::read_to_string("./target/doc/rustdoc_types.json")?;
    let krate: rustdoc_types::Crate = serde_json::from_str(&json_string)?;
    println!("the index has {} items", krate.index.len());

    Ok(())
}

```

# cargo_manifest API (0.19.1)

Helper crate to parse and manipulate manifests - `Cargo.toml` files.

## 1: Manifest

- Repository: <https://github.com/LukeMathWalker/cargo-manifest>
- Categories: rust-patterns, parser-implementations
- License: Apache-2.0 OR MIT
- edition: `2021`

### 1.1: Features

- None


## 2: README

### cargo-manifest

[`serde`](https://serde.rs) definitions to read and write
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) files.

#### Description

This Rust crate contains various structs and enums to represent the contents of
a `Cargo.toml` file. These definitions can be used with [`serde`](https://serde.rs)
and the [`toml`](https://crates.io/crates/toml) crate to read and write
`Cargo.toml` manifest files.

This crate also to some degree supports post-processing of the data to emulate
Cargo's workspace inheritance and `autobins` features. This is used for example
by crates.io to extract whether a crate contains a library or executable
binaries.

 > 
 > \[!NOTE\]
 > The cargo team regularly adds new features to the `Cargo.toml` file
 > definition. This crate aims to keep up-to-date with these changes. You should
 > keep this crate up-to-date to correctly parse all fields in modern
 > `Cargo.toml` files.

#### Installation

````sh
cargo add cargo-manifest
````

#### Usage

````rust
use cargo_manifest::Manifest;

let manifest = Manifest::from_path("Cargo.toml").unwrap();
````

see [docs.rs](https://docs.rs/cargo-manifest) for more information.

#### Users

* [cargo-chef](https://crates.io/crates/cargo-chef)
* [crates.io](https://github.com/rust-lang/crates.io) is using this crate for
  server-side validation of `Cargo.toml` files.

#### Alternatives

This crate is a fork of the [`cargo_toml`](https://crates.io/crates/cargo_toml)
project. There are only some minor differences between these projects at this
point, you will need to evaluate which one fits your needs better.

There is also [`cargo-util-schemas`](https://crates.io/crates/cargo-util-schemas)
now, which is maintained by the cargo team themselves. This crate was extracted
from the cargo codebase and is used inside the `cargo` binary itself. It is
kept up-to-date with the latest changes to the `Cargo.toml` file format, but is
currently lacking some of the post-processing features that `cargo-manifest`
provides.

#### License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)

* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.


## 3: Common Traits

The following traits are commonly implemented by types in this crate. Unless otherwise noted, you can assume these traits are implemented:

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `StructuralPartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `serde::de::Deserialize<'de>`

- `Freeze`
- `Send`
- `Sync`
- `Unpin`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::Into<U>` (`where U: convert::From<T>`)
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


## 4: Module: `cargo_manifest`

#### cargo-manifest

[`serde`](https://serde.rs) definitions to read and write
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) files.

##### Description

This Rust crate contains various structs and enums to represent the contents of
a `Cargo.toml` file. These definitions can be used with [`serde`](https://serde.rs)
and the [`toml`](https://crates.io/crates/toml) crate to read and write
`Cargo.toml` manifest files.

This crate also to some degree supports post-processing of the data to emulate
Cargo's workspace inheritance and `autobins` features. This is used for example
by crates.io to extract whether a crate contains a library or executable
binaries.

 > 
 > \[!NOTE\]
 > The cargo team regularly adds new features to the `Cargo.toml` file
 > definition. This crate aims to keep up-to-date with these changes. You should
 > keep this crate up-to-date to correctly parse all fields in modern
 > `Cargo.toml` files.

##### Installation

````sh
cargo add cargo-manifest
````

##### Usage

````rust
use cargo_manifest::Manifest;

let manifest = Manifest::from_path("Cargo.toml").unwrap();
````

see [docs.rs](https://docs.rs/cargo-manifest) for more information.

##### Users

* [cargo-chef](https://crates.io/crates/cargo-chef)
* [crates.io](https://github.com/rust-lang/crates.io) is using this crate for
  server-side validation of `Cargo.toml` files.

##### Alternatives

This crate is a fork of the [`cargo_toml`](https://crates.io/crates/cargo_toml)
project. There are only some minor differences between these projects at this
point, you will need to evaluate which one fits your needs better.

There is also [`cargo-util-schemas`](https://crates.io/crates/cargo-util-schemas)
now, which is maintained by the cargo team themselves. This crate was extracted
from the cargo codebase and is used inside the `cargo` binary itself. It is
kept up-to-date with the latest changes to the `Cargo.toml` file format, but is
currently lacking some of the post-processing features that `cargo-manifest`
provides.

##### License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)

* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.


### 4.1: Structs

#### 4.1.1: `struct cargo_manifest::Badge`

```rust
#[serde(rename_all = "kebab-case")]
pub struct Badge {
    pub repository: String,
    #[serde(default = "default_master")] pub branch: String,
    pub service: option::Option<String>,
    pub id: option::Option<String>,
    #[serde(alias = "project_name")] pub project_name: option::Option<String>,
}
```

##### 4.1.1.1: Trait Implementations for `Badge`

**(Note: Does not implement common trait(s): `default::Default`)**

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.2: `struct cargo_manifest::Badges`

```rust
#[serde(rename_all = "kebab-case")]
pub struct Badges {
    #[serde(default, deserialize_with = "ok_or_default")] pub appveyor: option::Option<cargo_manifest::Badge>,
    #[serde(default, deserialize_with = "ok_or_default")] pub circle_ci: option::Option<cargo_manifest::Badge>,
    #[serde(default, deserialize_with = "ok_or_default")] pub gitlab: option::Option<cargo_manifest::Badge>,
    #[serde(default, deserialize_with = "ok_or_default")] pub travis_ci: option::Option<cargo_manifest::Badge>,
    #[serde(default, deserialize_with = "ok_or_default")] pub codecov: option::Option<cargo_manifest::Badge>,
    #[serde(default, deserialize_with = "ok_or_default")] pub coveralls: option::Option<cargo_manifest::Badge>,
    #[serde(default, deserialize_with = "ok_or_default")] pub is_it_maintained_issue_resolution: option::Option<cargo_manifest::Badge>,
    #[serde(default, deserialize_with = "ok_or_default")] pub is_it_maintained_open_issues: option::Option<cargo_manifest::Badge>,
    #[serde(default, deserialize_with = "ok_or_default")] pub maintenance: cargo_manifest::Maintenance,
}
```

##### 4.1.2.1: Fields

###### 4.1.2.1.1: `appveyor`

Appveyor: `repository` is required. `branch` is optional; default is `master`
`service` is optional; valid values are `github` (default), `bitbucket`, and
`gitlab`; `id` is optional; you can specify the appveyor project id if you
want to use that instead. `project_name` is optional; use when the repository
name differs from the appveyor project name.

###### 4.1.2.1.2: `circle_ci`

Circle CI: `repository` is required. `branch` is optional; default is `master`

###### 4.1.2.1.3: `gitlab`

GitLab: `repository` is required. `branch` is optional; default is `master`

###### 4.1.2.1.4: `travis_ci`

Travis CI: `repository` in format "\<user>/\<project>" is required.
`branch` is optional; default is `master`

###### 4.1.2.1.5: `codecov`

Codecov: `repository` is required. `branch` is optional; default is `master`
`service` is optional; valid values are `github` (default), `bitbucket`, and
`gitlab`.

###### 4.1.2.1.6: `coveralls`

Coveralls: `repository` is required. `branch` is optional; default is `master`
`service` is optional; valid values are `github` (default) and `bitbucket`.

###### 4.1.2.1.7: `is_it_maintained_issue_resolution`

Is it maintained resolution time: `repository` is required.

###### 4.1.2.1.8: `is_it_maintained_open_issues`

Is it maintained percentage of open issues: `repository` is required.

###### 4.1.2.1.9: `maintenance`

Maintenance: `status` is required. Available options are `actively-developed`,
`passively-maintained`, `as-is`, `experimental`, `looking-for-maintainer`,
`deprecated`, and the default `none`, which displays no badge on crates.io.

##### 4.1.2.2: Trait Implementations for `Badges`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.3: `struct cargo_manifest::DependencyDetail`

```rust
#[serde(rename_all = "kebab-case")]
pub struct DependencyDetail {
    #[serde(skip_serializing_if = "Option::is_none")] pub version: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub registry: option::Option<String>,
    #[serde(alias = "registry_index")] pub registry_index: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub path: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub git: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub branch: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub tag: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub rev: option::Option<String>,
    #[serde(default)] #[serde(skip_serializing_if = "Option::is_none")] pub features: option::Option<Vec<String>>,
    #[serde(default)] #[serde(skip_serializing_if = "Option::is_none")] pub optional: option::Option<bool>,
    #[serde(default, alias = "default_features")] #[serde(skip_serializing_if = "Option::is_none")] pub default_features: option::Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub package: option::Option<String>,
}
```

##### 4.1.3.1: Trait Implementations for `DependencyDetail`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.4: `struct cargo_manifest::Filesystem<'a>`

```rust
pub struct Filesystem<'a> {}
```

_[Private fields hidden]_

A \[AbstractFilesystem\] implementation that reads from the actual filesystem
within the given root path.

##### 4.1.4.1: `impl<'a> cargo_manifest::afs::Filesystem<'a>`

###### 4.1.4.2.1: `fn new(path: &'a path::Path) -> Self`


##### 4.1.4.2: Trait Implementations for `Filesystem`

**(Note: Does not implement common trait(s): `Clone`, `CloneToUninit`, `Debug`, `Eq`, `PartialEq`, `StructuralPartialEq`, `ToOwned`, `default::Default`, `equivalent::Equivalent<K>`, `hashbrown::Equivalent<K>`, `serde::de::Deserialize<'de>`, `serde::de::DeserializeOwned`, `serde::ser::Serialize`)**

- `cargo_manifest::afs::AbstractFilesystem`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.5: `struct cargo_manifest::InheritedDependencyDetail`

```rust
#[serde(rename_all = "kebab-case")]
pub struct InheritedDependencyDetail {
    pub workspace: {id:589},
    #[serde(default, skip_serializing_if = "Option::is_none")] pub features: option::Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub optional: option::Option<bool>,
}
```

When a dependency is defined as `{ workspace = true }`,
and workspace data hasn't been applied yet.

##### 4.1.5.1: Trait Implementations for `InheritedDependencyDetail`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.6: `struct cargo_manifest::Maintenance`

```rust
pub struct Maintenance {
    pub status: cargo_manifest::MaintenanceStatus,
}
```

##### 4.1.6.1: Trait Implementations for `Maintenance`

- `Copy`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.7: `struct cargo_manifest::Manifest<PackageMetadata = toml::value::Value, WorkspaceMetadata = toml::value::Value>`

```rust
#[serde(rename_all = "kebab-case")]
pub struct Manifest<PackageMetadata = toml::value::Value, WorkspaceMetadata = toml::value::Value> {
    #[serde(skip_serializing_if = "Option::is_none")] pub package: option::Option<cargo_manifest::Package<PackageMetadata>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub cargo_features: option::Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub workspace: option::Option<cargo_manifest::Workspace<WorkspaceMetadata>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub dependencies: option::Option<cargo_manifest::DepsSet>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "dev_dependencies")] pub dev_dependencies: option::Option<cargo_manifest::DepsSet>,
    #[serde(skip_serializing_if = "Option::is_none", alias =
"build_dependencies")] pub build_dependencies: option::Option<cargo_manifest::DepsSet>,
    #[serde(skip_serializing_if = "Option::is_none")] pub target: option::Option<cargo_manifest::TargetDepsSet>,
    #[serde(skip_serializing_if = "Option::is_none")] pub features: option::Option<cargo_manifest::FeatureSet>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub bin: Vec<cargo_manifest::Product>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub bench: Vec<cargo_manifest::Product>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub test: Vec<cargo_manifest::Product>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub example: Vec<cargo_manifest::Product>,
    #[serde(skip_serializing_if = "Option::is_none")] pub patch: option::Option<cargo_manifest::PatchSet>,
    #[serde(skip_serializing_if = "Option::is_none")] pub lib: option::Option<cargo_manifest::Product>,
    #[serde(skip_serializing_if = "Option::is_none")] pub profile: option::Option<cargo_manifest::Profiles>,
    #[serde(skip_serializing_if = "Option::is_none")] pub badges: option::Option<cargo_manifest::Badges>,
}
```

The top-level `Cargo.toml` structure

The `Metadata` is a type for `[package.metadata]` table. You can replace it with
your own struct type if you use the metadata and don't want to use the catch-all `Value` type.

##### 4.1.7.1: Fields

###### 4.1.7.1.1: `bin`

Note that due to autobins feature this is not the complete list
unless you run `complete_from_path`

###### 4.1.7.1.2: `lib`

Note that due to autolibs feature this is not the complete list
unless you run `complete_from_path`

##### 4.1.7.2: `impl cargo_manifest::Manifest<toml::value::Value>`

###### 4.1.7.3.1: `fn from_slice(cargo_toml_content: &[u8]) -> result::Result<Self, cargo_manifest::error::Error>`

Parse contents of a `Cargo.toml` file already loaded as a byte slice.

It does not call `complete_from_path`, so may be missing implicit data.

###### 4.1.7.3.2: `fn from_path<impl impl AsRef<Path>: convert::AsRef<path::Path>>(cargo_toml_path: impl convert::AsRef<path::Path>) -> result::Result<Self, cargo_manifest::error::Error>`

Parse contents from a `Cargo.toml` file on disk.

Calls `complete_from_path`.

##### 4.1.7.3: `impl<Metadata: for<'a> serde::de::Deserialize<'a>> cargo_manifest::Manifest<Metadata>`

###### 4.1.7.4.1: `fn from_slice_with_metadata(cargo_toml_content: &[u8]) -> result::Result<Self, cargo_manifest::error::Error>`

Parse `Cargo.toml`, and parse its `[package.metadata]` into a custom Serde-compatible type.

It does not call `complete_from_path`, so may be missing implicit data.

###### 4.1.7.4.2: `fn from_path_with_metadata<impl impl AsRef<Path>: convert::AsRef<path::Path>>(cargo_toml_path: impl convert::AsRef<path::Path>) -> result::Result<Self, cargo_manifest::error::Error>`

Parse contents from `Cargo.toml` file on disk, with custom Serde-compatible metadata type.

Calls `complete_from_path`

###### 4.1.7.4.3: `fn complete_from_path(self: &mut Self, path: &path::Path) -> result::Result<(), cargo_manifest::error::Error>`

`Cargo.toml` may not contain explicit information about `[lib]`, `[[bin]]` and
`[package].build`, which are inferred based on files on disk.

This scans the disk to make the data in the manifest as complete as possible.

###### 4.1.7.4.4: `fn complete_from_abstract_filesystem<FS: cargo_manifest::afs::AbstractFilesystem>(self: &mut Self, fs: &FS) -> result::Result<(), cargo_manifest::error::Error>`

`Cargo.toml` may not contain explicit information about `[lib]`, `[[bin]]` and
`[package].build`, which are inferred based on files on disk.

You can provide any implementation of directory scan, which doesn't have to
be reading straight from disk (might scan a tarball or a git repo, for example).

###### 4.1.7.4.5: `fn autobins(self: &Self) -> bool`


###### 4.1.7.4.6: `fn autoexamples(self: &Self) -> bool`


###### 4.1.7.4.7: `fn autotests(self: &Self) -> bool`


###### 4.1.7.4.8: `fn autobenches(self: &Self) -> bool`


##### 4.1.7.4: Trait Implementations for `Manifest`

**(Note: Does not implement common trait(s): `Eq`, `equivalent::Equivalent<K>`, `hashbrown::Equivalent<K>`)**

- `Clone`
- `Debug`
- `PartialEq`
- `serde::ser::Serialize`
- `str::traits::FromStr`

    ```rust
    impl str::traits::FromStr for cargo_manifest::Manifest<toml::value::Value> {
        type Err = cargo_manifest::error::Error;
    }
    ```

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.8: `struct cargo_manifest::Package<Metadata = toml::value::Value>`

```rust
pub struct Package<Metadata = toml::value::Value> {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub edition: option::Option<cargo_manifest::MaybeInherited<cargo_manifest::Edition>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub version: option::Option<cargo_manifest::MaybeInherited<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub build: option::Option<cargo_manifest::StringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub workspace: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub authors: option::Option<cargo_manifest::MaybeInherited<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub links: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub description: option::Option<cargo_manifest::MaybeInherited<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub homepage: option::Option<cargo_manifest::MaybeInherited<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub documentation: option::Option<cargo_manifest::MaybeInherited<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub readme: option::Option<cargo_manifest::MaybeInherited<cargo_manifest::StringOrBool>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub keywords: option::Option<cargo_manifest::MaybeInherited<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub categories: option::Option<cargo_manifest::MaybeInherited<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub license: option::Option<cargo_manifest::MaybeInherited<String>>,
    #[serde(rename = "license-file")] #[serde(skip_serializing_if = "Option::is_none")] pub license_file: option::Option<cargo_manifest::MaybeInherited<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub repository: option::Option<cargo_manifest::MaybeInherited<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub metadata: option::Option<Metadata>,
    #[serde(rename = "rust-version")] pub rust_version: option::Option<cargo_manifest::MaybeInherited<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub exclude: option::Option<cargo_manifest::MaybeInherited<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub include: option::Option<cargo_manifest::MaybeInherited<Vec<String>>>,
    #[serde(rename = "default-run")] #[serde(skip_serializing_if = "Option::is_none")] pub default_run: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub autolib: option::Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub autobins: option::Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub autoexamples: option::Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub autotests: option::Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub autobenches: option::Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub publish: option::Option<cargo_manifest::MaybeInherited<cargo_manifest::Publish>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub resolver: option::Option<cargo_manifest::Resolver>,
}
```

You can replace `Metadata` type with your own
to parse into something more useful than a generic toml `Value`

##### 4.1.8.1: Fields

###### 4.1.8.1.1: `name`

Careful: some names are uppercase

###### 4.1.8.1.2: `version`

The version of the package (e.g. "1.9.0").

Use \[Package::version()\] to get the effective value, with the default
value of "0.0.0" applied.

###### 4.1.8.1.3: `authors`

e.g. \["Author <e@mail>", "etc"\]

###### 4.1.8.1.4: `description`

A short blurb about the package. This is not rendered in any format when
uploaded to crates.io (aka this is not markdown).

###### 4.1.8.1.5: `readme`

This points to a file under the package root (relative to this `Cargo.toml`).

###### 4.1.8.1.6: `categories`

This is a list of up to five categories where this crate would fit.
e.g. \["command-line-utilities", "development-tools::cargo-plugins"\]

###### 4.1.8.1.7: `license`

e.g. "MIT"

###### 4.1.8.1.8: `rust_version`

e.g. "1.63.0"

###### 4.1.8.1.9: `default_run`

The default binary to run by cargo run.

###### 4.1.8.1.10: `autolib`

Disables library auto discovery.

###### 4.1.8.1.11: `autobins`

Disables binary auto discovery.

Use \[Manifest::autobins()\] to get the effective value.

###### 4.1.8.1.12: `autoexamples`

Disables example auto discovery.

Use \[Manifest::autoexamples()\] to get the effective value.

###### 4.1.8.1.13: `autotests`

Disables test auto discovery.

Use \[Manifest::autotests()\] to get the effective value.

###### 4.1.8.1.14: `autobenches`

Disables bench auto discovery.

Use \[Manifest::autobenches()\] to get the effective value.

##### 4.1.8.2: `impl<Metadata> cargo_manifest::Package<Metadata>`

###### 4.1.8.3.1: `fn new(name: String, version: String) -> Self`


###### 4.1.8.3.2: `fn version(self: &Self) -> cargo_manifest::MaybeInherited<&str>`

Returns the effective version of the package.

If the version is not set, it defaults to "0.0.0"
(see <https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field>).

##### 4.1.8.3: Trait Implementations for `Package`

**(Note: Does not implement common trait(s): `Eq`, `default::Default`, `equivalent::Equivalent<K>`, `hashbrown::Equivalent<K>`)**

- `Clone`
- `Debug`
- `PartialEq`
- `serde::ser::Serialize`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.9: `struct cargo_manifest::Product`

```rust
#[serde(rename_all = "kebab-case")]
pub struct Product {
    pub path: option::Option<String>,
    pub name: option::Option<String>,
    #[serde(default = "default_true", skip_serializing_if = "is_true")] pub test: bool,
    #[serde(default = "default_true", skip_serializing_if = "is_true")] pub doctest: bool,
    #[serde(default = "default_true", skip_serializing_if = "is_true")] pub bench: bool,
    #[serde(default = "default_true", skip_serializing_if = "is_true")] pub doc: bool,
    #[serde(default, skip_serializing_if = "is_false")] pub plugin: bool,
    #[serde(default, alias = "proc_macro", skip_serializing_if = "is_false")] pub proc_macro: bool,
    #[serde(default = "default_true", skip_serializing_if = "is_true")] pub harness: bool,
    #[serde(default)] pub edition: option::Option<cargo_manifest::Edition>,
    #[serde(default, alias = "required_features")] pub required_features: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "crate_type")] pub crate_type: option::Option<Vec<String>>,
}
```

Cargo uses the term "target" for both "target platform" and "build target" (the thing to build),
which makes it ambigous.
Here Cargo's bin/lib **target** is renamed to **product**.

##### 4.1.9.1: Fields

###### 4.1.9.1.1: `path`

This field points at where the crate is located, relative to the `Cargo.toml`.

###### 4.1.9.1.2: `name`

The name of a product is the name of the library or binary that will be generated.
This is defaulted to the name of the package, with any dashes replaced
with underscores. (Rust `extern crate` declarations reference this name;
therefore the value must be a valid Rust identifier to be usable.)

###### 4.1.9.1.3: `test`

A flag for enabling unit tests for this product. This is used by `cargo test`.

###### 4.1.9.1.4: `doctest`

A flag for enabling documentation tests for this product. This is only relevant
for libraries, it has no effect on other sections. This is used by
`cargo test`.

###### 4.1.9.1.5: `bench`

A flag for enabling benchmarks for this product. This is used by `cargo bench`.

###### 4.1.9.1.6: `doc`

A flag for enabling documentation of this product. This is used by `cargo doc`.

###### 4.1.9.1.7: `plugin`

If the product is meant to be a compiler plugin, this field must be set to true
for Cargo to correctly compile it and make it available for all dependencies.

###### 4.1.9.1.8: `proc_macro`

If the product is meant to be a "macros 1.1" procedural macro, this field must
be set to true.

###### 4.1.9.1.9: `harness`

If set to false, `cargo test` will omit the `--test` flag to rustc, which
stops it from generating a test harness. This is useful when the binary being
built manages the test runner itself.

###### 4.1.9.1.10: `edition`

If set then a product can be configured to use a different edition than the
`[package]` is configured to use, perhaps only compiling a library with the
2018 edition or only compiling one unit test with the 2015 edition. By default
all products are compiled with the edition specified in `[package]`.

###### 4.1.9.1.11: `required_features`

The required-features field specifies which features the product needs in order to be built.
If any of the required features are not selected, the product will be skipped.
This is only relevant for the `[[bin]]`, `[[bench]]`, `[[test]]`, and `[[example]]` sections,
it has no effect on `[lib]`.

###### 4.1.9.1.12: `crate_type`

The available options are "dylib", "rlib", "staticlib", "cdylib", and "proc-macro".

##### 4.1.9.2: Trait Implementations for `Product`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.10: `struct cargo_manifest::Profile`

```rust
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    #[serde(alias = "opt_level")] pub opt_level: option::Option<toml::value::Value>,
    pub debug: option::Option<toml::value::Value>,
    pub rpath: option::Option<bool>,
    pub inherits: option::Option<String>,
    pub lto: option::Option<toml::value::Value>,
    #[serde(alias = "debug_assertions")] pub debug_assertions: option::Option<bool>,
    #[serde(alias = "codegen_units")] pub codegen_units: option::Option<u16>,
    pub panic: option::Option<String>,
    pub incremental: option::Option<bool>,
    #[serde(alias = "overflow_checks")] pub overflow_checks: option::Option<bool>,
    pub strip: option::Option<cargo_manifest::StripSetting>,
    #[serde(default)] pub package: collections::btree::map::BTreeMap<String, toml::value::Value>,
    pub split_debuginfo: option::Option<String>,
    pub build_override: option::Option<toml::value::Value>,
}
```

##### 4.1.10.1: Fields

###### 4.1.10.1.1: `build_override`

profile overrides

##### 4.1.10.2: Trait Implementations for `Profile`

**(Note: Does not implement common trait(s): `Eq`, `default::Default`, `equivalent::Equivalent<K>`, `hashbrown::Equivalent<K>`)**

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.11: `struct cargo_manifest::Profiles`

```rust
pub struct Profiles {
    pub release: option::Option<cargo_manifest::Profile>,
    pub dev: option::Option<cargo_manifest::Profile>,
    pub test: option::Option<cargo_manifest::Profile>,
    pub bench: option::Option<cargo_manifest::Profile>,
    pub doc: option::Option<cargo_manifest::Profile>,
    #[serde(flatten)] pub custom: collections::btree::map::BTreeMap<String, cargo_manifest::Profile>,
}
```

##### 4.1.11.1: Trait Implementations for `Profiles`

**(Note: Does not implement common trait(s): `Eq`, `equivalent::Equivalent<K>`, `hashbrown::Equivalent<K>`)**

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.12: `struct cargo_manifest::Target`

```rust
#[serde(rename_all = "kebab-case")]
pub struct Target {
    #[serde(default)] pub dependencies: cargo_manifest::DepsSet,
    #[serde(default, alias = "dev_dependencies")] pub dev_dependencies: cargo_manifest::DepsSet,
    #[serde(default, alias = "build_dependencies")] pub build_dependencies: cargo_manifest::DepsSet,
}
```

##### 4.1.12.1: Trait Implementations for `Target`

**(Note: Does not implement common trait(s): `default::Default`)**

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.13: `struct cargo_manifest::Workspace<Metadata = toml::value::Value>`

```rust
#[serde(rename_all = "kebab-case")]
pub struct Workspace<Metadata = toml::value::Value> {
    #[serde(default)] pub members: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", alias = "default_members")] pub default_members: option::Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub exclude: option::Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub resolver: option::Option<cargo_manifest::Resolver>,
    #[serde(skip_serializing_if = "Option::is_none")] pub dependencies: option::Option<cargo_manifest::DepsSet>,
    #[serde(skip_serializing_if = "Option::is_none")] pub package: option::Option<cargo_manifest::WorkspacePackage>,
    #[serde(skip_serializing_if = "Option::is_none")] pub metadata: option::Option<Metadata>,
}
```

##### 4.1.13.1: Trait Implementations for `Workspace`

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `default::Default`
- `serde::ser::Serialize`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.14: `struct cargo_manifest::WorkspacePackage`

```rust
#[serde(rename_all = "kebab-case")]
pub struct WorkspacePackage {
    #[serde(skip_serializing_if = "Option::is_none")] pub edition: option::Option<cargo_manifest::Edition>,
    pub version: option::Option<String>,
    pub authors: option::Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub description: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub homepage: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub documentation: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub readme: option::Option<cargo_manifest::StringOrBool>,
    #[serde(skip_serializing_if = "Option::is_none")] pub keywords: option::Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub categories: option::Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub license: option::Option<String>,
    #[serde(rename = "license-file")] #[serde(skip_serializing_if = "Option::is_none")] pub license_file: option::Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub publish: option::Option<cargo_manifest::Publish>,
    #[serde(skip_serializing_if = "Option::is_none")] pub exclude: option::Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub include: option::Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub repository: option::Option<String>,
    #[serde(rename = "rust-version")] pub rust_version: option::Option<String>,
}
```

The workspace.package table is where you define keys that can be inherited by members of a
workspace. These keys can be inherited by defining them in the member package with
`{key}.workspace = true`.

See <https://doc.rust-lang.org/nightly/cargo/reference/workspaces.html#the-package-table>
for more details.

##### 4.1.14.1: Fields

###### 4.1.14.1.1: `version`

e.g. "1.9.0"

###### 4.1.14.1.2: `authors`

e.g. \["Author <e@mail>", "etc"\]

###### 4.1.14.1.3: `description`

A short blurb about the package. This is not rendered in any format when
uploaded to crates.io (aka this is not markdown).

###### 4.1.14.1.4: `readme`

This points to a file under the package root (relative to this `Cargo.toml`).

###### 4.1.14.1.5: `categories`

This is a list of up to five categories where this crate would fit.
e.g. \["command-line-utilities", "development-tools::cargo-plugins"\]

###### 4.1.14.1.6: `license`

e.g. "MIT"

###### 4.1.14.1.7: `rust_version`

e.g. "1.63.0"

##### 4.1.14.2: Trait Implementations for `WorkspacePackage`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

### 4.2: Enums

#### 4.2.1: `enum cargo_manifest::Dependency`

```rust
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Inherited(cargo_manifest::InheritedDependencyDetail),
    Detailed(cargo_manifest::DependencyDetail),
}
```

##### 4.2.1.1: `impl cargo_manifest::Dependency`

###### 4.2.1.2.1: `fn detail(self: &Self) -> option::Option<&cargo_manifest::DependencyDetail>`


###### 4.2.1.2.2: `fn simplify(self: Self) -> Self`

Simplifies `Dependency::Detailed` to `Dependency::Simple` if only the
`version` field inside the `DependencyDetail` struct is set.

###### 4.2.1.2.3: `fn req(self: &Self) -> &str`


###### 4.2.1.2.4: `fn req_features(self: &Self) -> &[String]`


###### 4.2.1.2.5: `fn optional(self: &Self) -> bool`


###### 4.2.1.2.6: `fn package(self: &Self) -> option::Option<&str>`


###### 4.2.1.2.7: `fn git(self: &Self) -> option::Option<&str>`


###### 4.2.1.2.8: `fn is_crates_io(self: &Self) -> bool`


##### 4.2.1.2: Trait Implementations for `Dependency`

**(Note: Does not implement common trait(s): `default::Default`)**

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.2: `enum cargo_manifest::Edition`

```rust
pub enum Edition {
    #[serde(rename = "2015")] #[default] E2015,
    #[serde(rename = "2018")] E2018,
    #[serde(rename = "2021")] E2021,
    #[serde(rename = "2024")] E2024,
}
```

##### 4.2.2.1: `impl cargo_manifest::Edition`

###### 4.2.2.2.1: `fn as_str(self: &Self) -> &'static str`


##### 4.2.2.2: Trait Implementations for `Edition`

- `Copy`
- `Hash`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.3: `enum cargo_manifest::Error`

```rust
pub enum Error {
    #[error(transparent)] Parse(#[from] toml::de::Error),
    #[error(transparent)] Io(#[from] io::error::Error),
    #[error(transparent)] Utf8(#[from] str::error::Utf8Error),
    #[error("{0}")] Other(String),
}
```

##### 4.2.3.1: Trait Implementations for `Error`

**(Note: Does not implement common trait(s): `Eq`, `PartialEq`, `StructuralPartialEq`, `default::Default`, `equivalent::Equivalent<K>`, `hashbrown::Equivalent<K>`, `serde::de::Deserialize<'de>`, `serde::de::DeserializeOwned`, `serde::ser::Serialize`)**

- `Display`
- `error::Error`

- `convert::From<io::error::Error>`
- `convert::From<str::error::Utf8Error>`
- `convert::From<toml::de::Error>`

- `!RefUnwindSafe`
- `!UnwindSafe`

- `ToString` (`where T: Display + ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.4: `enum cargo_manifest::MaintenanceStatus`

```rust
#[serde(rename_all = "kebab-case")]
pub enum MaintenanceStatus {
    #[default] None,
    ActivelyDeveloped,
    PassivelyMaintained,
    AsIs,
    Experimental,
    LookingForMaintainer,
    Deprecated,
}
```

##### 4.2.4.1: Trait Implementations for `MaintenanceStatus`

- `Copy`
- `Hash`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.5: `enum cargo_manifest::MaybeInherited<T>`

```rust
#[serde(untagged)]
pub enum MaybeInherited<T> {
    Inherited{ workspace: {id:589} },
    Local(T),
}
```

Used as a wrapper for properties that may be inherited by workspace-level settings.
It currently does not support more complex interactions (e.g. specifying part of the property
in the local manifest while inheriting another part of it from the workspace manifest, as it
happens for dependency features).

See [`cargo`'s documentation](https://doc.rust-lang.org/nightly/cargo/reference/workspaces.html#workspaces)
for more details.

##### 4.2.5.1: `impl<T> cargo_manifest::MaybeInherited<T>`

###### 4.2.5.2.1: `fn inherited() -> Self`


###### 4.2.5.2.2: `fn as_local(self: Self) -> option::Option<T>`


###### 4.2.5.2.3: `fn as_ref(self: &Self) -> cargo_manifest::MaybeInherited<&T>`

```rust
pub const fn as_ref(self: &Self) -> cargo_manifest::MaybeInherited<&T> { ... }
```

##### 4.2.5.2: Trait Implementations for `MaybeInherited`

**(Note: Does not implement common trait(s): `default::Default`)**

- `Clone`
- `Debug`
- `Eq`
- `PartialEq`
- `serde::ser::Serialize`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.6: `enum cargo_manifest::Publish`

```rust
#[serde(untagged)]
pub enum Publish {
    Flag(bool),
    Registry(Vec<String>),
}
```

##### 4.2.6.1: Trait Implementations for `Publish`

- `PartialEq<bool>`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.7: `enum cargo_manifest::Resolver`

```rust
pub enum Resolver {
    #[serde(rename = "1")] V1,
    #[serde(rename = "2")] V2,
    #[serde(rename = "3")] V3,
}
```

##### 4.2.7.1: Trait Implementations for `Resolver`

- `Copy`
- `Hash`

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.8: `enum cargo_manifest::StringOrBool`

```rust
#[serde(untagged)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}
```

##### 4.2.8.1: Trait Implementations for `StringOrBool`

**(Note: Does not implement common trait(s): `default::Default`)**

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.9: `enum cargo_manifest::StripSetting`

```rust
#[serde(try_from = "toml::Value")]
pub enum StripSetting {
    None,
    Debuginfo,
    Symbols,
}
```

##### 4.2.9.1: Variants

###### 4.2.9.1.1: `None`

false

###### 4.2.9.1.2: `Symbols`

true

##### 4.2.9.2: Trait Implementations for `StripSetting`

**(Note: Does not implement common trait(s): `default::Default`)**

- `Ord`
- `PartialOrd`

- `convert::TryFrom<toml::value::Value>`

    ```rust
    impl convert::TryFrom<toml::value::Value> for cargo_manifest::StripSetting {
        type Error = cargo_manifest::error::Error;
    }
    ```

- `RefUnwindSafe`
- `UnwindSafe`

- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `equivalent::Comparable<K>`

    ```rust
    where
        Q: Ord + ?Sized,
        K: borrow::Borrow<Q> + ?Sized
    ```

### 4.3: Traits

#### 4.3.1: `trait cargo_manifest::AbstractFilesystem`

```rust
pub trait AbstractFilesystem {
    pub fn file_names_in<T: convert::AsRef<path::Path>>(self: &Self, rel_path: T) -> io::error::Result<collections::btree::set::BTreeSet<Box<str>>>;;
}
```

A trait for abstracting over filesystem operations.

This trait is primarily used for target auto-discovery in the
[`complete_from_abstract_filesystem()`](crate::Manifest::complete_from_abstract_filesystem) method.

##### 4.3.1.1: Required Methods

###### 4.3.1.1.1: `fn file_names_in<T: convert::AsRef<path::Path>>(self: &Self, rel_path: T) -> io::error::Result<collections::btree::set::BTreeSet<Box<str>>>`

Returns a set of file and folder names in the given directory.

This method should return a \[std::io::ErrorKind::NotFound\] error if the
directory does not exist.

##### 4.3.1.2: Implementors

###### 4.3.1.2.1: `impl cargo_manifest::afs::AbstractFilesystem for cargo_manifest::afs::Filesystem<'_>`


### 4.4: Type Aliases

#### 4.4.1: `type DepsSet`


#### 4.4.2: `type FeatureSet`


#### 4.4.3: `type PatchSet`


#### 4.4.4: `type TargetDepsSet`


## 5: Module: `cargo_manifest::afs`


### 5.1: Re-exports

- `struct cargo_manifest::afs::Filesystem<'a>` (See section 4.1.4: for details)
- `trait cargo_manifest::afs::AbstractFilesystem` (See section 4.3.1: for details)

# pulldown_cmark API (0.13.0)

A pull parser for CommonMark

## 1: Manifest

- Repository: <https://github.com/raphlinus/pulldown-cmark>
- Categories: text-processing
- License: MIT
- rust-version: `1.71.1`
- edition: `2021`

### 1.1: Features

- `default`
- `gen-tests`
- `html`
- `simd`


## 2: README

### pulldown-cmark

[![Tests](https://github.com/pulldown-cmark/pulldown-cmark/actions/workflows/rust.yml/badge.svg)](https://github.com/pulldown-cmark/pulldown-cmark/actions/workflows/rust.yml)
[![Docs](https://docs.rs/pulldown-cmark/badge.svg)](https://docs.rs/pulldown-cmark)
[![Crates.io](https://img.shields.io/crates/v/pulldown-cmark.svg?maxAge=2592000)](https://crates.io/crates/pulldown-cmark)

[Documentation](https://docs.rs/pulldown-cmark/)

This library is a pull parser for [CommonMark](http://commonmark.org/), written
in [Rust](http://www.rust-lang.org/). It comes with a simple command-line tool,
useful for rendering to HTML, and is also designed to be easy to use from as
a library.

It is designed to be:

* Fast; a bare minimum of allocation and copying
* Safe; written in pure Rust with no unsafe blocks (except in the opt-in SIMD feature)
* Versatile; in particular source-maps are supported
* Correct; the goal is 100% compliance with the [CommonMark spec](http://spec.commonmark.org/)

Further, it optionally supports parsing footnotes,
[Github flavored tables](https://github.github.com/gfm/#tables-extension-),
[Github flavored task lists](https://github.github.com/gfm/#task-list-items-extension-) and
[strikethrough](https://github.github.com/gfm/#strikethrough-extension-).

Rustc 1.71.1 or newer is required to build the crate.

#### Example

Example usage:

````rust
// Create parser with example Markdown text.
let markdown_input = "hello world";
let parser = pulldown_cmark::Parser::new(markdown_input);

// Write to a new String buffer.
let mut html_output = String::new();
pulldown_cmark::html::push_html(&mut html_output, parser);
assert_eq!(&html_output, "<p>hello world</p>\n");
````

#### Why a pull parser?

There are many parsers for Markdown and its variants, but to my knowledge none
use pull parsing. Pull parsing has become popular for XML, especially for
memory-conscious applications, because it uses dramatically less memory than
constructing a document tree, but is much easier to use than push parsers. Push
parsers are notoriously difficult to use, and also often error-prone because of
the need for user to delicately juggle state in a series of callbacks.

In a clean design, the parsing and rendering stages are neatly separated, but
this is often sacrificed in the name of performance and expedience. Many Markdown
implementations mix parsing and rendering together, and even designs that try
to separate them (such as the popular [hoedown](https://github.com/hoedown/hoedown)),
make the assumption that the rendering process can be fully represented as a
serialized string.

Pull parsing is in some sense the most versatile architecture. It's possible to
drive a push interface, also with minimal memory, and quite straightforward to
construct an AST. Another advantage is that source-map information (the mapping
between parsed blocks and offsets within the source text) is readily available;
you can call `into_offset_iter()` to create an iterator that yields `(Event, Range)`
pairs, where the second element is the event's corresponding range in the source
document.

While manipulating ASTs is the most flexible way to transform documents,
operating on iterators is surprisingly easy, and quite efficient. Here, for
example, is the code to transform soft line breaks into hard breaks:

````rust
let parser = parser.map(|event| match event {
	Event::SoftBreak => Event::HardBreak,
	_ => event
});
````

Or expanding an abbreviation in text:

````rust
let parser = parser.map(|event| match event {
	Event::Text(text) => Event::Text(text.replace("abbr", "abbreviation").into()),
	_ => event
});
````

Another simple example is code to determine the max nesting level:

````rust
let mut max_nesting = 0;
let mut level = 0;
for event in parser {
	match event {
		Event::Start(_) => {
			level += 1;
			max_nesting = std::cmp::max(max_nesting, level);
		}
		Event::End(_) => level -= 1,
		_ => ()
	}
}
````

Note that consecutive text events can happen due to the manner in which the
parser evaluates the source. A utility `TextMergeStream` exists to improve
the comfort of iterating the events:

````rust
use pulldown_cmark::{Event, Parser, Options};

let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";

let iterator = TextMergeStream::new(Parser::new(markdown_input));

for event in iterator {
    match event {
        Event::Text(text) => println!("{}", text),
        _ => {}
    }
}
````

There are some basic but fully functional examples of the usage of the crate in the
`examples` directory of this repository.

#### Using Rust idiomatically

A lot of the internal scanning code is written at a pretty low level (it
pretty much scans byte patterns for the bits of syntax), but the external
interface is designed to be idiomatic Rust.

Pull parsers are at heart an iterator of events (start and end tags, text,
and other bits and pieces). The parser data structure implements the
Rust Iterator trait directly, and Event is an enum. Thus, you can use the
full power and expressivity of Rust's iterator infrastructure, including
for loops and `map` (as in the examples above), collecting the events into
a vector (for recording, playback, and manipulation), and more.

Further, the `Text` event (representing text) is a small copy-on-write string.
The vast majority of text fragments are just
slices of the source document. For these, copy-on-write gives a convenient
representation that requires no allocation or copying, but allocated
strings are available when they're needed. Thus, when rendering text to
HTML, most text is copied just once, from the source document to the
HTML buffer.

When using the pulldown-cmark's own HTML renderer, make sure to write to a buffered
target like a `Vec<u8>` or `String`. Since it performs many (very) small writes, writing
directly to stdout, files, or sockets is detrimental to performance. Such writers can
be wrapped in a [`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html).

#### Build options

By default, the binary is built as well. If you don't want/need it, then build like this:

````bash
> cargo build --no-default-features
````

Or add this package as dependency of your project using `cargo add`:

````bash
> cargo add pulldown-cmark --no-default-features
````

SIMD accelerated scanners are available for the x64 platform from version 0.5 onwards. To
enable them, build with simd feature:

````bash
> cargo build --release --features simd
````

Or add this package as dependency of your project with the feature using `cargo add`:

````bash
> cargo add pulldown-cmark --no-default-features --features=simd
````

For a higher release performance you may want this configuration in your profile release:

````
lto = true
codegen-units = 1
panic = "abort"
````

#### Authors

The main author is Raph Levien. The implementation of the new design (v0.3+) was
completed by Marcus Klaas de Vries. Since 2023, the development has been driven
by Martín Pozo, Michael Howell, Roope Salmi and Martin Geisler.

#### License

This software is under the MIT license. See details in [license file](./LICENSE).

#### Contributions

We gladly accept contributions via GitHub pull requests. Please see
[CONTRIBUTING.md](CONTRIBUTING.md) for more details.


## 3: Common Traits

The following traits are commonly implemented by types in this crate. Unless otherwise noted, you can assume these traits are implemented:

- `Clone`
- `Debug`
- `PartialEq`
- `StructuralPartialEq`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)


## 4: Module: `pulldown_cmark`

Pull parser for [CommonMark](https://commonmark.org). This crate provides a [Parser](struct.Parser.html) struct
which is an iterator over [Event](enum.Event.html)s. This iterator can be used
directly, or to output HTML using the [HTML module](html/index.html).

By default, only CommonMark features are enabled. To use extensions like tables,
footnotes or task lists, enable them by setting the corresponding flags in the
[Options](struct.Options.html) struct.

#### Example

````rust
use pulldown_cmark::{Parser, Options};

let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";

// Set up options and parser. Strikethroughs are not part of the CommonMark standard
// and we therefore must enable it explicitly.
let mut options = Options::empty();
options.insert(Options::ENABLE_STRIKETHROUGH);
let parser = Parser::new_ext(markdown_input, options);

# #[cfg(feature = "html")] {
// Write to String buffer.
let mut html_output = String::new();
pulldown_cmark::html::push_html(&mut html_output, parser);

// Check that the output is what we expected.
let expected_html = "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
assert_eq!(expected_html, &html_output);
# }
````

Note that consecutive text events can happen due to the manner in which the
parser evaluates the source. A utility `TextMergeStream` exists to improve
the comfort of iterating the events:

````rust
use pulldown_cmark::{Event, Parser, TextMergeStream};

let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";

let iterator = TextMergeStream::new(Parser::new(markdown_input));

for event in iterator {
    match event {
        Event::Text(text) => println!("{}", text),
        _ => {}
    }
}
````


### 4.1: Structs

#### 4.1.1: `struct pulldown_cmark::BrokenLink<'a>`

```rust
pub struct BrokenLink<'a> {
    pub span: range::Range<usize>,
    pub link_type: pulldown_cmark::LinkType,
    pub reference: pulldown_cmark::strings::CowStr<'a>,
}
```

##### 4.1.1.1: `impl<'a> pulldown_cmark::parse::BrokenLink<'a>`

###### 4.1.1.2.1: `fn into_static(self: Self) -> pulldown_cmark::parse::BrokenLink<'static>`

Moves the link into version with a static lifetime.

The `reference` member is cloned to a Boxed or Inline version.

##### 4.1.1.2: Trait Implementations for `BrokenLink`

**(Note: Does not implement common trait(s): `Clone`, `CloneToUninit`, `PartialEq`, `StructuralPartialEq`, `ToOwned`)**

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.2: `struct pulldown_cmark::DefaultBrokenLinkCallback`

```rust
pub struct DefaultBrokenLinkCallback;
```

Broken link callback that does nothing.

##### 4.1.2.1: Trait Implementations for `DefaultBrokenLinkCallback`

**(Note: Does not implement common trait(s): `Clone`, `CloneToUninit`, `PartialEq`, `StructuralPartialEq`, `ToOwned`)**

- `pulldown_cmark::parse::BrokenLinkCallback<'input>`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.3: `struct pulldown_cmark::InlineStr`

```rust
pub struct InlineStr {}
```

_[Private fields hidden]_

An inline string that can contain almost three words
of utf-8 text.

##### 4.1.3.1: Trait Implementations for `InlineStr`

**(Note: Does not implement common trait(s): `StructuralPartialEq`)**

- `Copy`
- `Display`
- `Eq`
- `Hash`

- `convert::AsRef<str>`
- `convert::From<char>`
- `convert::TryFrom<&str>`

    ```rust
    impl convert::TryFrom<&str> for pulldown_cmark::strings::InlineStr {
        type Error = pulldown_cmark::strings::StringTooLongError;
    }
    ```
- `deref::Deref`

    ```rust
    impl deref::Deref for pulldown_cmark::strings::InlineStr {
        type Target = str;
    }
    ```

- `ToString` (`where T: Display + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `deref::Receiver`

    ```rust
    where
        P: deref::Deref<Target = T> + ?Sized,
        T: ?Sized
    ```

#### 4.1.4: `struct pulldown_cmark::InvalidHeadingLevel`

```rust
pub struct InvalidHeadingLevel();
```

Returned when trying to convert a `usize` into a `Heading` but it fails
because the usize isn't a valid heading level

##### 4.1.4.1: Trait Implementations for `InvalidHeadingLevel`

- `Copy`
- `Eq`
- `Hash`
- `Ord`
- `PartialOrd`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.5: `struct pulldown_cmark::OffsetIter<'a, F = pulldown_cmark::parse::DefaultBrokenLinkCallback>`

```rust
pub struct OffsetIter<'a, F = pulldown_cmark::parse::DefaultBrokenLinkCallback> {}
```

_[Private fields hidden]_

Markdown event and source range iterator.

Generates tuples where the first element is the markdown event and the second
is a the corresponding range in the source string.

Constructed from a `Parser` using its
[`into_offset_iter`](struct.Parser.html#method.into_offset_iter) method.

##### 4.1.5.1: `impl<'a, F: pulldown_cmark::parse::BrokenLinkCallback<'a>> pulldown_cmark::parse::OffsetIter<'a, F>`

###### 4.1.5.2.1: `fn reference_definitions(self: &Self) -> &pulldown_cmark::parse::RefDefs<'_>`

Returns a reference to the internal reference definition tracker.

##### 4.1.5.2: Trait Implementations for `OffsetIter`

**(Note: Does not implement common trait(s): `Clone`, `CloneToUninit`, `PartialEq`, `StructuralPartialEq`, `ToOwned`)**

- `Debug`
- `iter::traits::iterator::Iterator`

    ```rust
    impl<'a, F: pulldown_cmark::parse::BrokenLinkCallback<'a>> iter::traits::iterator::Iterator for pulldown_cmark::parse::OffsetIter<'a, F> {
        type Item = (pulldown_cmark::Event<'a>, range::Range<usize>);
    }
    ```

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `iter::traits::collect::IntoIterator` (`where I: iter::traits::iterator::Iterator`)

#### 4.1.6: `struct pulldown_cmark::Options`

```rust
pub struct Options();
```

Option struct containing flags for enabling extra features
that are not part of the CommonMark spec.

##### 4.1.6.1: `impl pulldown_cmark::Options`

###### 4.1.6.2.1: `const ENABLE_TABLES`

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.2: `const ENABLE_FOOTNOTES`

GitHub-compatible footnote syntax.

Footnotes are referenced with the syntax `[^IDENT]`,
and defined with an identifier followed by a colon at top level.

---

````markdown
Footnote referenced [^1].

[^1]: footnote defined
````

Footnote referenced \[^1\].

\[^1\]: footnote defined

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.3: `const ENABLE_STRIKETHROUGH`

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.4: `const ENABLE_TASKLISTS`

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.5: `const ENABLE_SMART_PUNCTUATION`

Enables replacement of ASCII punctuation characters with
Unicode ligatures and smart quotes.

This includes replacing `--` with `—`, `---` with `—`, `...` with `…`,
`"quote"` with `“quote”`, and `'quote'` with `‘quote’`.

The replacement takes place during the parsing of the document.

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.6: `const ENABLE_HEADING_ATTRIBUTES`

Extension to allow headings to have ID and classes.

`# text { #id .class1 .class2 myattr other_attr=myvalue }`
is interpreted as a level 1 heading
with the content `text`, ID `id`, classes `class1` and `class2` and
custom attributes `myattr` (without value) and
`other_attr` with value `myvalue`.
Note that ID, classes, and custom attributes should be space-separated.

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.7: `const ENABLE_YAML_STYLE_METADATA_BLOCKS`

Metadata blocks in YAML style, i.e.:

* starting with a `---` line
* ending with a `---` or `...` line

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.8: `const ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS`

Metadata blocks delimited by:

* `+++` line at start
* `+++` line at end

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.9: `const ENABLE_OLD_FOOTNOTES`

Older footnote syntax. This flag implies `ENABLE_FOOTNOTES`, changing it to use an
older syntax instead of the new, default, GitHub-compatible syntax.

New syntax is different from the old syntax regarding
indentation, nesting, and footnote references with no definition:

````markdown
[^1]: In new syntax, this is two footnote definitions.
[^2]: In old syntax, this is a single footnote definition with two lines.

[^3]:

    In new syntax, this is a footnote with two paragraphs.

    In old syntax, this is a footnote followed by a code block.

In new syntax, this undefined footnote definition renders as
literal text [^4]. In old syntax, it creates a dangling link.
````

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.10: `const ENABLE_MATH`

With this feature enabled, two events `Event::InlineMath` and `Event::DisplayMath`
are emitted that conventionally contain TeX formulas.

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.11: `const ENABLE_GFM`

Misc GitHub Flavored Markdown features not supported in CommonMark.
The following features are currently behind this tag:

* Blockquote tags (\[!NOTE\], \[!TIP\], \[!IMPORTANT\], \[!WARNING\], \[!CAUTION\]).

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.12: `const ENABLE_DEFINITION_LIST`

Commonmark-HS-Extensions compatible definition lists.

````markdown
title 1
  : definition 1
title 2
  : definition 2
````

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.13: `const ENABLE_SUPERSCRIPT`

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.14: `const ENABLE_SUBSCRIPT`

_Type: `Self`_
_Default: `_`_

###### 4.1.6.2.15: `const ENABLE_WIKILINKS`

Obsidian-style Wikilinks.

_Type: `Self`_
_Default: `_`_

##### 4.1.6.2: `impl pulldown_cmark::Options`

###### 4.1.6.3.1: `#[inline] fn empty() -> Self`

```rust
#[inline] pub const fn empty() -> Self { ... }
```

Get a flags value with all bits unset.

###### 4.1.6.3.2: `#[inline] fn all() -> Self`

```rust
#[inline] pub const fn all() -> Self { ... }
```

Get a flags value with all known bits set.

###### 4.1.6.3.3: `#[inline] fn bits(self: &Self) -> u32`

```rust
#[inline] pub const fn bits(self: &Self) -> u32 { ... }
```

Get the underlying bits value.

The returned value is exactly the bits set in this flags value.

###### 4.1.6.3.4: `#[inline] fn from_bits(bits: u32) -> option::Option<Self>`

```rust
#[inline] pub const fn from_bits(bits: u32) -> option::Option<Self> { ... }
```

Convert from a bits value.

This method will return `None` if any unknown bits are set.

###### 4.1.6.3.5: `#[inline] fn from_bits_truncate(bits: u32) -> Self`

```rust
#[inline] pub const fn from_bits_truncate(bits: u32) -> Self { ... }
```

Convert from a bits value, unsetting any unknown bits.

###### 4.1.6.3.6: `#[inline] fn from_bits_retain(bits: u32) -> Self`

```rust
#[inline] pub const fn from_bits_retain(bits: u32) -> Self { ... }
```

Convert from a bits value exactly.

###### 4.1.6.3.7: `#[inline] fn from_name(name: &str) -> option::Option<Self>`

```rust
#[inline] pub fn from_name(name: &str) -> option::Option<Self> { ... }
```

Get a flags value with the bits of a flag with the given name set.

This method will return `None` if `name` is empty or doesn't
correspond to any named flag.

###### 4.1.6.3.8: `#[inline] fn is_empty(self: &Self) -> bool`

```rust
#[inline] pub const fn is_empty(self: &Self) -> bool { ... }
```

Whether all bits in this flags value are unset.

###### 4.1.6.3.9: `#[inline] fn is_all(self: &Self) -> bool`

```rust
#[inline] pub const fn is_all(self: &Self) -> bool { ... }
```

Whether all known bits in this flags value are set.

###### 4.1.6.3.10: `#[inline] fn intersects(self: &Self, other: Self) -> bool`

```rust
#[inline] pub const fn intersects(self: &Self, other: Self) -> bool { ... }
```

Whether any set bits in a source flags value are also set in a target flags value.

###### 4.1.6.3.11: `#[inline] fn contains(self: &Self, other: Self) -> bool`

```rust
#[inline] pub const fn contains(self: &Self, other: Self) -> bool { ... }
```

Whether all set bits in a source flags value are also set in a target flags value.

###### 4.1.6.3.12: `#[inline] fn insert(self: &mut Self, other: Self)`

```rust
#[inline] pub fn insert(self: &mut Self, other: Self) { ... }
```

The bitwise or (`|`) of the bits in two flags values.

###### 4.1.6.3.13: `#[inline] fn remove(self: &mut Self, other: Self)`

```rust
#[inline] pub fn remove(self: &mut Self, other: Self) { ... }
```

The intersection of a source flags value with the complement of a target flags value (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set.
`remove` won't truncate `other`, but the `!` operator will.

###### 4.1.6.3.14: `#[inline] fn toggle(self: &mut Self, other: Self)`

```rust
#[inline] pub fn toggle(self: &mut Self, other: Self) { ... }
```

The bitwise exclusive-or (`^`) of the bits in two flags values.

###### 4.1.6.3.15: `#[inline] fn set(self: &mut Self, other: Self, value: bool)`

```rust
#[inline] pub fn set(self: &mut Self, other: Self, value: bool) { ... }
```

Call `insert` when `value` is `true` or `remove` when `value` is `false`.

###### 4.1.6.3.16: `#[inline] #[must_use] fn intersection(self: Self, other: Self) -> Self`

```rust
#[inline] #[must_use] pub const fn intersection(self: Self, other: Self) -> Self { ... }
```

The bitwise and (`&`) of the bits in two flags values.

###### 4.1.6.3.17: `#[inline] #[must_use] fn union(self: Self, other: Self) -> Self`

```rust
#[inline] #[must_use] pub const fn union(self: Self, other: Self) -> Self { ... }
```

The bitwise or (`|`) of the bits in two flags values.

###### 4.1.6.3.18: `#[inline] #[must_use] fn difference(self: Self, other: Self) -> Self`

```rust
#[inline] #[must_use] pub const fn difference(self: Self, other: Self) -> Self { ... }
```

The intersection of a source flags value with the complement of a target flags value (`&!`).

This method is not equivalent to `self & !other` when `other` has unknown bits set.
`difference` won't truncate `other`, but the `!` operator will.

###### 4.1.6.3.19: `#[inline] #[must_use] fn symmetric_difference(self: Self, other: Self) -> Self`

```rust
#[inline] #[must_use] pub const fn symmetric_difference(self: Self, other: Self) -> Self { ... }
```

The bitwise exclusive-or (`^`) of the bits in two flags values.

###### 4.1.6.3.20: `#[inline] #[must_use] fn complement(self: Self) -> Self`

```rust
#[inline] #[must_use] pub const fn complement(self: Self) -> Self { ... }
```

The bitwise negation (`!`) of the bits in a flags value, truncating the result.

##### 4.1.6.3: `impl pulldown_cmark::Options`

###### 4.1.6.4.1: `#[inline] fn iter(self: &Self) -> bitflags::iter::Iter<pulldown_cmark::Options>`

```rust
#[inline] pub const fn iter(self: &Self) -> bitflags::iter::Iter<pulldown_cmark::Options> { ... }
```

Yield a set of contained flags values.

Each yielded flags value will correspond to a defined named flag. Any unknown bits
will be yielded together as a final flags value.

###### 4.1.6.4.2: `#[inline] fn iter_names(self: &Self) -> bitflags::iter::IterNames<pulldown_cmark::Options>`

```rust
#[inline] pub const fn iter_names(self: &Self) -> bitflags::iter::IterNames<pulldown_cmark::Options> { ... }
```

Yield a set of contained named flags values.

This method is like [`iter`](#method.iter), except only yields bits in contained named flags.
Any unknown bits, or bits not corresponding to a contained flag will not be yielded.

##### 4.1.6.4: Trait Implementations for `Options`

- `Binary`
- `Copy`
- `Eq`
- `Hash`
- `LowerHex`
- `Octal`
- `Ord`
- `PartialOrd`
- `UpperHex`
- `arith::SubAssign`
- `bit::BitAndAssign`
- `bit::BitOrAssign`
- `bit::BitXorAssign`

- `arith::Sub`

    ```rust
    impl arith::Sub for pulldown_cmark::Options {
        type Output = pulldown_cmark::Options;
    }
    ```
- `bit::BitAnd`

    ```rust
    impl bit::BitAnd for pulldown_cmark::Options {
        type Output = pulldown_cmark::Options;
    }
    ```
- `bit::BitOr`

    ```rust
    impl bit::BitOr for pulldown_cmark::Options {
        type Output = pulldown_cmark::Options;
    }
    ```
- `bit::BitXor`

    ```rust
    impl bit::BitXor for pulldown_cmark::Options {
        type Output = pulldown_cmark::Options;
    }
    ```
- `bit::Not`

    ```rust
    impl bit::Not for pulldown_cmark::Options {
        type Output = pulldown_cmark::Options;
    }
    ```
- `bitflags::traits::Flags`

    ```rust
    impl bitflags::traits::Flags for pulldown_cmark::Options {
        const FLAGS: &'static [bitflags::traits::Flag<pulldown_cmark::Options>] = _;
        type Bits = u32;
    }
    ```
- `bitflags::traits::PublicFlags`

    ```rust
    impl bitflags::traits::PublicFlags for pulldown_cmark::Options {
        type Primitive = u32;
        type Internal = {id:872};
    }
    ```
- `iter::traits::collect::Extend<pulldown_cmark::Options>`
- `iter::traits::collect::FromIterator<pulldown_cmark::Options>`
- `iter::traits::collect::IntoIterator`

    ```rust
    impl iter::traits::collect::IntoIterator for pulldown_cmark::Options {
        type Item = pulldown_cmark::Options;
        type IntoIter = bitflags::iter::Iter<pulldown_cmark::Options>;
    }
    ```

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.7: `struct pulldown_cmark::Parser<'input, F = pulldown_cmark::parse::DefaultBrokenLinkCallback>`

```rust
pub struct Parser<'input, F = pulldown_cmark::parse::DefaultBrokenLinkCallback> {}
```

_[Private fields hidden]_

Markdown event iterator.

##### 4.1.7.1: `impl<'input> pulldown_cmark::parse::Parser<'input, pulldown_cmark::parse::DefaultBrokenLinkCallback>`

###### 4.1.7.2.1: `fn new(text: &'input str) -> Self`

Creates a new event iterator for a markdown string without any options enabled.

###### 4.1.7.2.2: `fn new_ext(text: &'input str, options: pulldown_cmark::Options) -> Self`

Creates a new event iterator for a markdown string with given options.

##### 4.1.7.2: `impl<'input, F: pulldown_cmark::parse::BrokenLinkCallback<'input>> pulldown_cmark::parse::Parser<'input, F>`

###### 4.1.7.3.1: `fn new_with_broken_link_callback(text: &'input str, options: pulldown_cmark::Options, broken_link_callback: option::Option<F>) -> Self`

In case the parser encounters any potential links that have a broken
reference (e.g `[foo]` when there is no `[foo]: ` entry at the bottom)
the provided callback will be called with the reference name,
and the returned pair will be used as the link URL and title if it is not
`None`.

###### 4.1.7.3.2: `fn reference_definitions(self: &Self) -> &pulldown_cmark::parse::RefDefs<'_>`

Returns a reference to the internal `RefDefs` object, which provides access
to the internal map of reference definitions.

###### 4.1.7.3.3: `fn into_offset_iter(self: Self) -> pulldown_cmark::parse::OffsetIter<'input, F>`

Consumes the event iterator and produces an iterator that produces
`(Event, Range)` pairs, where the `Range` value maps to the corresponding
range in the markdown source.

##### 4.1.7.3: Trait Implementations for `Parser`

**(Note: Does not implement common trait(s): `Clone`, `CloneToUninit`, `PartialEq`, `StructuralPartialEq`, `ToOwned`)**

- `iter::traits::iterator::Iterator`

    ```rust
    impl<'a, F: pulldown_cmark::parse::BrokenLinkCallback<'a>> iter::traits::iterator::Iterator for pulldown_cmark::parse::Parser<'a, F> {
        type Item = pulldown_cmark::Event<'a>;
    }
    ```
- `iter::traits::marker::FusedIterator`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `iter::traits::collect::IntoIterator` (`where I: iter::traits::iterator::Iterator`)

#### 4.1.8: `struct pulldown_cmark::RefDefs<'input>`

```rust
pub struct RefDefs<'input>();
```

Keeps track of the reference definitions defined in the document.

##### 4.1.8.1: `impl<'input, 'b, 's> pulldown_cmark::parse::RefDefs<'input> where 's: 'b`

###### 4.1.8.2.1: `fn get(self: &'s Self, key: &'b str) -> option::Option<&'b pulldown_cmark::parse::LinkDef<'input>>`

Performs a lookup on reference label using unicode case folding.

###### 4.1.8.2.2: `fn iter(self: &'s Self) -> impl iter::traits::iterator::Iterator<Item = (&'s str, &'s pulldown_cmark::parse::LinkDef<'input>)>`

Provides an iterator over all the document's reference definitions.

##### 4.1.8.2: Trait Implementations for `RefDefs`

**(Note: Does not implement common trait(s): `PartialEq`, `StructuralPartialEq`)**

- `default::Default`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.1.9: `struct pulldown_cmark::TextMergeStream<'a, I>`

```rust
pub struct TextMergeStream<'a, I> {}
```

_[Private fields hidden]_

Merge consecutive `Event::Text` events into only one.

##### 4.1.9.1: `impl<'a, I> pulldown_cmark::utils::TextMergeStream<'a, I>
  where
    I: iter::traits::iterator::Iterator<Item = pulldown_cmark::Event<'a>>`

###### 4.1.9.2.1: `fn new(iter: I) -> Self`


##### 4.1.9.2: Trait Implementations for `TextMergeStream`

**(Note: Does not implement common trait(s): `Clone`, `CloneToUninit`, `PartialEq`, `StructuralPartialEq`, `ToOwned`)**

- `Debug`
- `iter::traits::iterator::Iterator`

    ```rust
    impl<'a, I> iter::traits::iterator::Iterator for pulldown_cmark::utils::TextMergeStream<'a, I>
      where
        I: iter::traits::iterator::Iterator<Item = pulldown_cmark::Event<'a>> {
    
        type Item = pulldown_cmark::Event<'a>;
    }
    ```

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `iter::traits::collect::IntoIterator` (`where I: iter::traits::iterator::Iterator`)

#### 4.1.10: `struct pulldown_cmark::TextMergeWithOffset<'a, I>`

```rust
pub struct TextMergeWithOffset<'a, I> {}
```

_[Private fields hidden]_

Merge consecutive `Event::Text` events into only one, with offsets.

Compatible with with [`OffsetIter`](crate::OffsetIter).

##### 4.1.10.1: `impl<'a, I> pulldown_cmark::utils::TextMergeWithOffset<'a, I>
  where
    I: iter::traits::iterator::Iterator<Item = (pulldown_cmark::Event<'a>, range::Range<usize>)>`

###### 4.1.10.2.1: `fn new(iter: I) -> Self`


##### 4.1.10.2: Trait Implementations for `TextMergeWithOffset`

**(Note: Does not implement common trait(s): `Clone`, `CloneToUninit`, `PartialEq`, `StructuralPartialEq`, `ToOwned`)**

- `Debug`
- `iter::traits::iterator::Iterator`

    ```rust
    impl<'a, I> iter::traits::iterator::Iterator for pulldown_cmark::utils::TextMergeWithOffset<'a, I>
      where
        I: iter::traits::iterator::Iterator<Item = (pulldown_cmark::Event<'a>, range::Range<usize>)> {
    
        type Item = (pulldown_cmark::Event<'a>, range::Range<usize>);
    }
    ```

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `iter::traits::collect::IntoIterator` (`where I: iter::traits::iterator::Iterator`)

### 4.2: Enums

#### 4.2.1: `enum pulldown_cmark::Alignment`

```rust
#[<cfg_attr>(feature = "serde", derive(Serialize, Deserialize))]
pub enum Alignment {
    None,
    Left,
    Center,
    Right,
}
```

Table column text alignment.

##### 4.2.1.1: Variants

###### 4.2.1.1.1: `None`

Default text alignment.

##### 4.2.1.2: Trait Implementations for `Alignment`

- `Copy`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.2: `enum pulldown_cmark::BlockQuoteKind`

```rust
#[<cfg_attr>(feature = "serde", derive(Serialize, Deserialize))]
pub enum BlockQuoteKind {
    Note,
    Tip,
    Important,
    Warning,
    Caution,
}
```

BlockQuote kind (Note, Tip, Important, Warning, Caution).

##### 4.2.2.1: Trait Implementations for `BlockQuoteKind`

- `Copy`
- `Eq`
- `Hash`
- `Ord`
- `PartialOrd`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.3: `enum pulldown_cmark::CodeBlockKind<'a>`

```rust
#[<cfg_attr>(feature = "serde", derive(Serialize, Deserialize))]
pub enum CodeBlockKind<'a> {
    Indented,
    #[<cfg_attr>(feature = "serde", serde(borrow))] Fenced(pulldown_cmark::strings::CowStr<'a>),
}
```

Codeblock kind.

##### 4.2.3.1: Variants

###### 4.2.3.1.1: `Fenced(pulldown_cmark::strings::CowStr<'a>)`

The value contained in the tag describes the language of the code, which may be empty.

##### 4.2.3.2: `impl<'a> pulldown_cmark::CodeBlockKind<'a>`

###### 4.2.3.3.1: `fn is_indented(self: &Self) -> bool`


###### 4.2.3.3.2: `fn is_fenced(self: &Self) -> bool`


###### 4.2.3.3.3: `fn into_static(self: Self) -> pulldown_cmark::CodeBlockKind<'static>`


##### 4.2.3.3: Trait Implementations for `CodeBlockKind`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.4: `enum pulldown_cmark::CowStr<'a>`

```rust
pub enum CowStr<'a> {
    Boxed(Box<str>),
    Borrowed(&'a str),
    Inlined(pulldown_cmark::strings::InlineStr),
}
```

A copy-on-write string that can be owned, borrowed
or inlined.

It is three words long.

##### 4.2.4.1: Variants

###### 4.2.4.1.1: `Boxed(Box<str>)`

An owned, immutable string.

###### 4.2.4.1.2: `Borrowed(&'a str)`

A borrowed string.

###### 4.2.4.1.3: `Inlined(pulldown_cmark::strings::InlineStr)`

A short inline string.

##### 4.2.4.2: `impl<'a> pulldown_cmark::strings::CowStr<'a>`

###### 4.2.4.3.1: `fn into_string(self: Self) -> String`


###### 4.2.4.3.2: `fn into_static(self: Self) -> pulldown_cmark::strings::CowStr<'static>`


##### 4.2.4.3: Trait Implementations for `CowStr`

**(Note: Does not implement common trait(s): `StructuralPartialEq`)**

- `Display`
- `Eq`
- `Hash`

- `borrow::Borrow<str>`
- `convert::AsRef<str>`
- `convert::From<&'a str>`
- `convert::From<Cow<'a, char>>`
- `convert::From<Cow<'a, str>>`
- `convert::From<String>`
- `convert::From<char>`
- `deref::Deref`

    ```rust
    impl<'a> deref::Deref for pulldown_cmark::strings::CowStr<'a> {
        type Target = str;
    }
    ```

- `ToString` (`where T: Display + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `deref::Receiver`

    ```rust
    where
        P: deref::Deref<Target = T> + ?Sized,
        T: ?Sized
    ```

#### 4.2.5: `enum pulldown_cmark::Event<'a>`

```rust
#[<cfg_attr>(feature = "serde", derive(Serialize, Deserialize))]
pub enum Event<'a> {
    #[<cfg_attr>(feature = "serde", serde(borrow))] Start(pulldown_cmark::Tag<'a>),
    End(pulldown_cmark::TagEnd),
    #[<cfg_attr>(feature = "serde", serde(borrow))] Text(pulldown_cmark::strings::CowStr<'a>),
    #[<cfg_attr>(feature = "serde", serde(borrow))] Code(pulldown_cmark::strings::CowStr<'a>),
    #[<cfg_attr>(feature = "serde", serde(borrow))] InlineMath(pulldown_cmark::strings::CowStr<'a>),
    #[<cfg_attr>(feature = "serde", serde(borrow))] DisplayMath(pulldown_cmark::strings::CowStr<'a>),
    #[<cfg_attr>(feature = "serde", serde(borrow))] Html(pulldown_cmark::strings::CowStr<'a>),
    #[<cfg_attr>(feature = "serde", serde(borrow))] InlineHtml(pulldown_cmark::strings::CowStr<'a>),
    #[<cfg_attr>(feature = "serde", serde(borrow))] FootnoteReference(pulldown_cmark::strings::CowStr<'a>),
    SoftBreak,
    HardBreak,
    Rule,
    TaskListMarker(bool),
}
```

Markdown events that are generated in a preorder traversal of the document
tree, with additional `End` events whenever all of an inner node's children
have been visited.

##### 4.2.5.1: Variants

###### 4.2.5.1.1: `Start(pulldown_cmark::Tag<'a>)`

Start of a tagged element. Events that are yielded after this event
and before its corresponding `End` event are inside this element.
Start and end events are guaranteed to be balanced.

###### 4.2.5.1.2: `End(pulldown_cmark::TagEnd)`

End of a tagged element.

###### 4.2.5.1.3: `Text(pulldown_cmark::strings::CowStr<'a>)`

A text node.

All text, outside and inside \[`Tag`\]s.

###### 4.2.5.1.4: `Code(pulldown_cmark::strings::CowStr<'a>)`

An [inline code node](https://spec.commonmark.org/0.31.2/#code-spans).

````markdown
`code`
````

###### 4.2.5.1.5: `InlineMath(pulldown_cmark::strings::CowStr<'a>)`

An inline math environment node.
Requires \[`Options::ENABLE_MATH`\].

````markdown
$math$
````

###### 4.2.5.1.6: `DisplayMath(pulldown_cmark::strings::CowStr<'a>)`

A display math environment node.
Requires \[`Options::ENABLE_MATH`\].

````markdown
$$math$$
````

###### 4.2.5.1.7: `Html(pulldown_cmark::strings::CowStr<'a>)`

An HTML node.

A line of HTML inside \[`Tag::HtmlBlock`\] includes the line break.

###### 4.2.5.1.8: `InlineHtml(pulldown_cmark::strings::CowStr<'a>)`

An [inline HTML node](https://spec.commonmark.org/0.31.2/#raw-html).

Contains only the tag itself, e.g. `<open-tag>`, `</close-tag>` or `<!-- comment -->`.

**Note**: Under some conditions HTML can also be parsed as an HTML Block, see \[`Tag::HtmlBlock`\] for details.

###### 4.2.5.1.9: `FootnoteReference(pulldown_cmark::strings::CowStr<'a>)`

A reference to a footnote with given label, which may or may not be defined
by an event with a \[`Tag::FootnoteDefinition`\] tag. Definitions and references to them may
occur in any order. Only parsed and emitted with \[`Options::ENABLE_FOOTNOTES`\] or \[`Options::ENABLE_OLD_FOOTNOTES`\].

````markdown
[^1]
````

###### 4.2.5.1.10: `SoftBreak`

A [soft line break](https://spec.commonmark.org/0.31.2/#soft-line-breaks).

Any line break that isn't a [`HardBreak`](Self::HardBreak), or the end of e.g. a paragraph.

###### 4.2.5.1.11: `HardBreak`

A [hard line break](https://spec.commonmark.org/0.31.2/#hard-line-breaks).

A line ending that is either preceded by at least two spaces or `\`.

````markdown
hard··
line\
breaks
````

*`·` is a space*

###### 4.2.5.1.12: `Rule`

A horizontal ruler.

````markdown
***
···---
_·_··_····_··
````

*`·` is any whitespace*

###### 4.2.5.1.13: `TaskListMarker(bool)`

A task list marker, rendered as a checkbox in HTML. Contains a true when it is checked.
Only parsed and emitted with \[`Options::ENABLE_TASKLISTS`\].

````markdown
- [ ] unchecked
- [x] checked
````

##### 4.2.5.2: `impl<'a> pulldown_cmark::Event<'a>`

###### 4.2.5.3.1: `fn into_static(self: Self) -> pulldown_cmark::Event<'static>`


##### 4.2.5.3: Trait Implementations for `Event`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.6: `enum pulldown_cmark::HeadingLevel`

```rust
#[<cfg_attr>(feature = "serde", derive(Serialize, Deserialize))]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}
```

##### 4.2.6.1: Trait Implementations for `HeadingLevel`

- `Copy`
- `Display`
- `Eq`
- `Hash`
- `Ord`
- `PartialOrd`

- `convert::TryFrom<usize>`

    ```rust
    impl convert::TryFrom<usize> for pulldown_cmark::HeadingLevel {
        type Error = pulldown_cmark::InvalidHeadingLevel;
    }
    ```

- `ToString` (`where T: Display + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.7: `enum pulldown_cmark::LinkType`

```rust
#[<cfg_attr>(feature = "serde", derive(Serialize, Deserialize))]
pub enum LinkType {
    Inline,
    Reference,
    ReferenceUnknown,
    Collapsed,
    CollapsedUnknown,
    Shortcut,
    ShortcutUnknown,
    Autolink,
    Email,
    WikiLink{ has_pothole: bool },
}
```

Type specifier for inline links. See [the Tag::Link](enum.Tag.html#variant.Link) for more information.

##### 4.2.7.1: Variants

###### 4.2.7.1.1: `Inline`

Inline link like `[foo](bar)`

###### 4.2.7.1.2: `Reference`

Reference link like `[foo][bar]`

###### 4.2.7.1.3: `ReferenceUnknown`

Reference without destination in the document, but resolved by the broken_link_callback

###### 4.2.7.1.4: `Collapsed`

Collapsed link like `[foo][]`

###### 4.2.7.1.5: `CollapsedUnknown`

Collapsed link without destination in the document, but resolved by the broken_link_callback

###### 4.2.7.1.6: `Shortcut`

Shortcut link like `[foo]`

###### 4.2.7.1.7: `ShortcutUnknown`

Shortcut without destination in the document, but resolved by the broken_link_callback

###### 4.2.7.1.8: `Autolink`

Autolink like `<http://foo.bar/baz>`

###### 4.2.7.1.9: `Email`

Email address in autolink like `<john@example.org>`

###### 4.2.7.1.10: `WikiLink { has_pothole: bool }`

Wikilink link like `[[foo]]` or `[[foo|bar]]`

####### 4.2.7.1.10.1: Fields

######## 4.2.7.1.10.1.1: `has_pothole`

`true` if the wikilink was piped.

* `true` - `[[foo|bar]]`
* `false` - `[[foo]]`

##### 4.2.7.2: Trait Implementations for `LinkType`

- `Copy`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.8: `enum pulldown_cmark::MetadataBlockKind`

```rust
#[<cfg_attr>(feature = "serde", derive(Serialize, Deserialize))]
pub enum MetadataBlockKind {
    YamlStyle,
    PlusesStyle,
}
```

Metadata block kind.

##### 4.2.8.1: Trait Implementations for `MetadataBlockKind`

- `Copy`
- `Eq`
- `Hash`
- `Ord`
- `PartialOrd`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.9: `enum pulldown_cmark::Tag<'a>`

```rust
#[<cfg_attr>(feature = "serde", derive(Serialize, Deserialize))]
pub enum Tag<'a> {
    Paragraph,
    Heading{ level: pulldown_cmark::HeadingLevel, id: option::Option<pulldown_cmark::strings::CowStr<'a>>, classes: Vec<pulldown_cmark::strings::CowStr<'a>>, attrs: Vec<(pulldown_cmark::strings::CowStr<'a>, option::Option<pulldown_cmark::strings::CowStr<'a>>)> },
    BlockQuote(option::Option<pulldown_cmark::BlockQuoteKind>),
    CodeBlock(pulldown_cmark::CodeBlockKind<'a>),
    HtmlBlock,
    List(option::Option<u64>),
    Item,
    #[<cfg_attr>(feature = "serde", serde(borrow))] FootnoteDefinition(pulldown_cmark::strings::CowStr<'a>),
    DefinitionList,
    DefinitionListTitle,
    DefinitionListDefinition,
    Table(Vec<pulldown_cmark::Alignment>),
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    Superscript,
    Subscript,
    Link{ link_type: pulldown_cmark::LinkType, dest_url: pulldown_cmark::strings::CowStr<'a>, title: pulldown_cmark::strings::CowStr<'a>, id: pulldown_cmark::strings::CowStr<'a> },
    Image{ link_type: pulldown_cmark::LinkType, dest_url: pulldown_cmark::strings::CowStr<'a>, title: pulldown_cmark::strings::CowStr<'a>, id: pulldown_cmark::strings::CowStr<'a> },
    MetadataBlock(pulldown_cmark::MetadataBlockKind),
}
```

Tags for elements that can contain other elements.

##### 4.2.9.1: Variants

###### 4.2.9.1.1: `Paragraph`

A paragraph of text and other inline elements.

###### 4.2.9.1.2: `Heading { level: pulldown_cmark::HeadingLevel, id: option::Option<pulldown_cmark::strings::CowStr<'a>>, classes: Vec<pulldown_cmark::strings::CowStr<'a>>, attrs: Vec<(pulldown_cmark::strings::CowStr<'a>, option::Option<pulldown_cmark::strings::CowStr<'a>>)> }`

A heading, with optional identifier, classes and custom attributes.
The identifier is prefixed with `#` and the last one in the attributes
list is chosen, classes are prefixed with `.` and custom attributes
have no prefix and can optionally have a value (`myattr` or `myattr=myvalue`).

`id`, `classes` and `attrs` are only parsed and populated with \[`Options::ENABLE_HEADING_ATTRIBUTES`\], `None` or empty otherwise.

####### 4.2.9.1.2.1: Fields

######## 4.2.9.1.2.1.1: `attrs`

The first item of the tuple is the attr and second one the value.

###### 4.2.9.1.3: `BlockQuote(option::Option<pulldown_cmark::BlockQuoteKind>)`

A block quote.

The `BlockQuoteKind` is only parsed & populated with \[`Options::ENABLE_GFM`\], `None` otherwise.

````markdown
> regular quote

> [!NOTE]
> note quote
````

###### 4.2.9.1.4: `CodeBlock(pulldown_cmark::CodeBlockKind<'a>)`

A code block.

###### 4.2.9.1.5: `HtmlBlock`

An HTML block.

A line that begins with some predefined tags (HTML block tags) (see [CommonMark Spec](https://spec.commonmark.org/0.31.2/#html-blocks) for more details) or any tag that is followed only by whitespace.

Most HTML blocks end on an empty line, though some e.g. `<pre>` like `<script>` or `<!-- Comments -->` don't.

````markdown
<body> Is HTML block even though here is non-whitespace.
Block ends on an empty line.

<some-random-tag>
This is HTML block.

<pre> Doesn't end on empty lines.

This is still the same block.</pre>
````

###### 4.2.9.1.6: `List(option::Option<u64>)`

A list. If the list is ordered the field indicates the number of the first item.
Contains only list items.

###### 4.2.9.1.7: `Item`

A list item.

###### 4.2.9.1.8: `FootnoteDefinition(pulldown_cmark::strings::CowStr<'a>)`

A footnote definition. The value contained is the footnote's label by which it can
be referred to.

Only parsed and emitted with \[`Options::ENABLE_FOOTNOTES`\] or \[`Options::ENABLE_OLD_FOOTNOTES`\].

###### 4.2.9.1.9: `DefinitionList`

Only parsed and emitted with \[`Options::ENABLE_DEFINITION_LIST`\].

###### 4.2.9.1.10: `DefinitionListTitle`

Only parsed and emitted with \[`Options::ENABLE_DEFINITION_LIST`\].

###### 4.2.9.1.11: `DefinitionListDefinition`

Only parsed and emitted with \[`Options::ENABLE_DEFINITION_LIST`\].

###### 4.2.9.1.12: `Table(Vec<pulldown_cmark::Alignment>)`

A table. Contains a vector describing the text-alignment for each of its columns.
Only parsed and emitted with \[`Options::ENABLE_TABLES`\].

###### 4.2.9.1.13: `TableHead`

A table header. Contains only `TableCell`s. Note that the table body starts immediately
after the closure of the `TableHead` tag. There is no `TableBody` tag.
Only parsed and emitted with \[`Options::ENABLE_TABLES`\].

###### 4.2.9.1.14: `TableRow`

A table row. Is used both for header rows as body rows. Contains only `TableCell`s.
Only parsed and emitted with \[`Options::ENABLE_TABLES`\].

###### 4.2.9.1.15: `TableCell`

Only parsed and emitted with \[`Options::ENABLE_TABLES`\].

###### 4.2.9.1.16: `Emphasis`

[Emphasis](https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis).

````markdown
half*emph* _strong_ _multi _level__
````

###### 4.2.9.1.17: `Strong`

[Strong emphasis](https://spec.commonmark.org/0.31.2/#emphasis-and-strong-emphasis).

````markdown
half**strong** __strong__ __multi __level____
````

###### 4.2.9.1.18: `Strikethrough`

Only parsed and emitted with \[`Options::ENABLE_STRIKETHROUGH`\].

````markdown
~strike through~
````

###### 4.2.9.1.19: `Superscript`

Only parsed and emitted with \[`Options::ENABLE_SUPERSCRIPT`\].

````markdown
^superscript^
````

###### 4.2.9.1.20: `Subscript`

Only parsed and emitted with \[`Options::ENABLE_SUBSCRIPT`\], if disabled `~something~` is parsed as [`Strikethrough`](Self::Strikethrough).

````markdown
~subscript~ ~~if also enabled this is strikethrough~~
````

###### 4.2.9.1.21: `Link { link_type: pulldown_cmark::LinkType, dest_url: pulldown_cmark::strings::CowStr<'a>, title: pulldown_cmark::strings::CowStr<'a>, id: pulldown_cmark::strings::CowStr<'a> }`

A link.

####### 4.2.9.1.21.1: Fields

######## 4.2.9.1.21.1.1: `id`

Identifier of reference links, e.g. `world` in the link `[hello][world]`.

###### 4.2.9.1.22: `Image { link_type: pulldown_cmark::LinkType, dest_url: pulldown_cmark::strings::CowStr<'a>, title: pulldown_cmark::strings::CowStr<'a>, id: pulldown_cmark::strings::CowStr<'a> }`

An image. The first field is the link type, the second the destination URL and the third is a title,
the fourth is the link identifier.

####### 4.2.9.1.22.1: Fields

######## 4.2.9.1.22.1.1: `id`

Identifier of reference links, e.g. `world` in the link `[hello][world]`.

###### 4.2.9.1.23: `MetadataBlock(pulldown_cmark::MetadataBlockKind)`

A metadata block.
Only parsed and emitted with \[`Options::ENABLE_YAML_STYLE_METADATA_BLOCKS`\]
or \[`Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS`\].

##### 4.2.9.2: `impl<'a> pulldown_cmark::Tag<'a>`

###### 4.2.9.3.1: `fn to_end(self: &Self) -> pulldown_cmark::TagEnd`


###### 4.2.9.3.2: `fn into_static(self: Self) -> pulldown_cmark::Tag<'static>`


##### 4.2.9.3: Trait Implementations for `Tag`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

#### 4.2.10: `enum pulldown_cmark::TagEnd`

```rust
#[<cfg_attr>(feature = "serde", derive(Serialize, Deserialize))]
pub enum TagEnd {
    Paragraph,
    Heading(pulldown_cmark::HeadingLevel),
    BlockQuote(option::Option<pulldown_cmark::BlockQuoteKind>),
    CodeBlock,
    HtmlBlock,
    List(bool),
    Item,
    FootnoteDefinition,
    DefinitionList,
    DefinitionListTitle,
    DefinitionListDefinition,
    Table,
    TableHead,
    TableRow,
    TableCell,
    Emphasis,
    Strong,
    Strikethrough,
    Superscript,
    Subscript,
    Link,
    Image,
    MetadataBlock(pulldown_cmark::MetadataBlockKind),
}
```

The end of a `Tag`.

##### 4.2.10.1: Variants

###### 4.2.10.1.1: `List(bool)`

A list, `true` for ordered lists.

##### 4.2.10.2: Trait Implementations for `TagEnd`

- `Copy`
- `Eq`
- `Hash`
- `Ord`
- `PartialOrd`

- `convert::From<pulldown_cmark::Tag<'a>>`

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)

### 4.3: Traits

#### 4.3.1: `trait pulldown_cmark::BrokenLinkCallback<'input>`

```rust
pub trait BrokenLinkCallback<'input> {
    pub fn handle_broken_link(self: &mut Self, link: pulldown_cmark::parse::BrokenLink<'input>) -> option::Option<(pulldown_cmark::strings::CowStr<'input>, pulldown_cmark::strings::CowStr<'input>)>;;
}
```

Trait for broken link callbacks.

See \[Parser::new_with_broken_link_callback\].
Automatically implemented for closures with the appropriate signature.

##### 4.3.1.1: Implementors

###### 4.3.1.1.1: `impl<'input, T> pulldown_cmark::parse::BrokenLinkCallback<'input> for T`

```rust
where
    T: function::FnMut<(pulldown_cmark::parse::BrokenLink<'input>) -> option::Option<(pulldown_cmark::strings::CowStr<'input>, pulldown_cmark::strings::CowStr<'input>)>>
```

###### 4.3.1.1.2: `impl<'input> pulldown_cmark::parse::BrokenLinkCallback<'input> for Box<dyn pulldown_cmark::parse::BrokenLinkCallback<'input>>`

###### 4.3.1.1.3: `impl<'input> pulldown_cmark::parse::BrokenLinkCallback<'input> for pulldown_cmark::parse::DefaultBrokenLinkCallback`


## 5: Module: `pulldown_cmark::html`

HTML renderer that takes an iterator of events as input.


### 5.1: Functions

#### 5.1.1: `fn push_html<'a, I>(s: &mut String, iter: I)`

```rust
pub fn push_html<'a, I>
  where
    I: iter::traits::iterator::Iterator<Item = pulldown_cmark::Event<'a>>(s: &mut String, iter: I) { ... }
```

Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
push it to a `String`.

###### Examples

````
use pulldown_cmark::{html, Parser};

let markdown_str = r#"
hello
=====

* alpha
* beta
"#;
let parser = Parser::new(markdown_str);

let mut html_buf = String::new();
html::push_html(&mut html_buf, parser);

assert_eq!(html_buf, r#"<h1>hello</h1>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
"#);
````


#### 5.1.2: `fn write_html_fmt<'a, I, W>(writer: W, iter: I) -> Result`

```rust
pub fn write_html_fmt<'a, I, W>
  where
    I: iter::traits::iterator::Iterator<Item = pulldown_cmark::Event<'a>>,
    W: Write(writer: W, iter: I) -> Result { ... }
```

Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
write it into Unicode-accepting buffer or stream.

###### Examples

````
use pulldown_cmark::{html, Parser};

let markdown_str = r#"
hello
=====

* alpha
* beta
"#;
let mut buf = String::new();
let parser = Parser::new(markdown_str);

html::write_html_fmt(&mut buf, parser);

assert_eq!(buf, r#"<h1>hello</h1>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
"#);
````


#### 5.1.3: `fn write_html_io<'a, I, W>(writer: W, iter: I) -> io::error::Result<()>`

```rust
pub fn write_html_io<'a, I, W>
  where
    I: iter::traits::iterator::Iterator<Item = pulldown_cmark::Event<'a>>,
    W: io::Write(writer: W, iter: I) -> io::error::Result<()> { ... }
```

Iterate over an `Iterator` of `Event`s, generate HTML for each `Event`, and
write it out to an I/O stream.

**Note**: using this function with an unbuffered writer like a file or socket
will result in poor performance. Wrap these in a
[`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html) to
prevent unnecessary slowdowns.

###### Examples

````
use pulldown_cmark::{html, Parser};
use std::io::Cursor;

let markdown_str = r#"
hello
=====

* alpha
* beta
"#;
let mut bytes = Vec::new();
let parser = Parser::new(markdown_str);

html::write_html_io(Cursor::new(&mut bytes), parser);

assert_eq!(&String::from_utf8_lossy(&bytes)[..], r#"<h1>hello</h1>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
"#);
````


## 6: Module: `pulldown_cmark::utils`

Miscellaneous utilities to increase comfort.
Special thanks to:

* <https://github.com/BenjaminRi/Redwood-Wiki/blob/master/src/markdown_utils.rs>.
  Its author authorized the use of this GPL code in this project in
  <https://github.com/raphlinus/pulldown-cmark/issues/507>.

* <https://gist.github.com/rambip/a507c312ed61c99c24b2a54f98325721>.
  Its author proposed the solution in
  <https://github.com/raphlinus/pulldown-cmark/issues/708>.

### 6.1: Common Traits

In addition to the crate's 'Common Traits', the following traits are commonly implemented by types in this module. Unless otherwise noted, you can assume these traits are implemented:

- `Debug`
- `iter::traits::iterator::Iterator`

    ```rust
    impl<'a, I> iter::traits::iterator::Iterator for pulldown_cmark::utils::TextMergeStream<'a, I>
      where
        I: iter::traits::iterator::Iterator<Item = pulldown_cmark::Event<'a>> {
    
        type Item = pulldown_cmark::Event<'a>;
    }
    ```

- `borrow::Borrow<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `iter::traits::collect::IntoIterator` (`where I: iter::traits::iterator::Iterator`)

### 6.2: Re-exports

- `struct pulldown_cmark::utils::TextMergeStream<'a, I>` (See section 4.1.9: for details)
- `struct pulldown_cmark::utils::TextMergeWithOffset<'a, I>` (See section 4.1.10: for details)


## 7: Examples Appendix

### 7.1: `broken-link-callbacks.rs`

```rust
use pulldown_cmark::{html, BrokenLink, Options, Parser};

fn main() {
    let input: &str = "Hello world, check out [my website][].";
    println!("Parsing the following markdown string:\n{}", input);

    // Setup callback that sets the URL and title when it encounters
    // a reference to our home page.
    let callback = |broken_link: BrokenLink| {
        if broken_link.reference.as_ref() == "my website" {
            println!(
                "Replacing the markdown `{}` of type {:?} with a working link",
                &input[broken_link.span], broken_link.link_type,
            );
            Some(("http://example.com".into(), "my example website".into()))
        } else {
            None
        }
    };

    // Create a parser with our callback function for broken links.
    let parser = Parser::new_with_broken_link_callback(input, Options::empty(), Some(callback));

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    // Check that the output is what we expected.
    let expected_html: &str =
        "<p>Hello world, check out <a href=\"http://example.com\" title=\"my example website\">my website</a>.</p>\n";
    assert_eq!(expected_html, &html_output);

    // Write result to stdout.
    println!("\nHTML output:\n{}", &html_output);
}

```

### 7.2: `event-filter.rs`

```rust
use std::io::Write as _;

use pulldown_cmark::{html, Event, Options, Parser, Tag, TagEnd};

fn main() {
    let markdown_input: &str = "This is Peter on ![holiday in Greece](pearl_beach.jpg).";
    println!("Parsing the following markdown string:\n{}", markdown_input);

    // Set up parser. We can treat is as any other iterator. We replace Peter by John
    // and image by its alt text.
    let parser = Parser::new_ext(markdown_input, Options::empty())
        .map(|event| match event {
            Event::Text(text) => Event::Text(text.replace("Peter", "John").into()),
            _ => event,
        })
        .filter(|event| match event {
            Event::Start(Tag::Image { .. }) | Event::End(TagEnd::Image) => false,
            _ => true,
        });

    // Write to anything implementing the `Write` trait. This could also be a file
    // or network socket.
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"\nHTML output:\n").unwrap();
    html::write_html_io(&mut handle, parser).unwrap();
}

```

### 7.3: `events.rs`

```rust
use std::io::Read;

use pulldown_cmark::{Event, Parser};

/// Show all events from the text on stdin.
fn main() {
    let mut text = String::new();
    std::io::stdin().read_to_string(&mut text).unwrap();

    eprintln!("{text:?} -> [");
    let mut width = 0;
    for event in Parser::new(&text) {
        if let Event::End(_) = event {
            width -= 2;
        }
        eprintln!("  {:width$}{event:?}", "");
        if let Event::Start(_) = event {
            width += 2;
        }
    }
    eprintln!("]");
}

```

### 7.4: `footnote-rewrite.rs`

```rust
use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::Write as _;

use pulldown_cmark::{html, CowStr, Event, Options, Parser, Tag, TagEnd};

/// This example shows how to do footnotes as bottom-notes, in the style of GitHub.
fn main() {
    let markdown_input: &str = "This is an [^a] footnote [^a].\n\n[^a]: footnote contents";
    println!("Parsing the following markdown string:\n{}", markdown_input);

    // To generate this style, you have to collect the footnotes at the end, while parsing.
    // You also need to count usages.
    let mut footnotes = Vec::new();
    let mut in_footnote = Vec::new();
    let mut footnote_numbers = HashMap::new();
    // ENABLE_FOOTNOTES is used in this example, but ENABLE_OLD_FOOTNOTES would work, too.
    let parser = Parser::new_ext(markdown_input, Options::ENABLE_FOOTNOTES)
        .filter_map(|event| {
            match event {
                Event::Start(Tag::FootnoteDefinition(_)) => {
                    in_footnote.push(vec![event]);
                    None
                }
                Event::End(TagEnd::FootnoteDefinition) => {
                    let mut f = in_footnote.pop().unwrap();
                    f.push(event);
                    footnotes.push(f);
                    None
                }
                Event::FootnoteReference(name) => {
                    let n = footnote_numbers.len() + 1;
                    let (n, nr) = footnote_numbers.entry(name.clone()).or_insert((n, 0usize));
                    *nr += 1;
                    let html = Event::Html(format!(r##"<sup class="footnote-reference" id="fr-{name}-{nr}"><a href="#fn-{name}">[{n}]</a></sup>"##).into());
                    if in_footnote.is_empty() {
                        Some(html)
                    } else {
                        in_footnote.last_mut().unwrap().push(html);
                        None
                    }
                }
                _ if !in_footnote.is_empty() => {
                    in_footnote.last_mut().unwrap().push(event);
                    None
                }
                _ => Some(event),
            }
        });

    // Write to anything implementing the `Write` trait. This could also be a file
    // or network socket.
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"\nHTML output:\n").unwrap();
    html::write_html_io(&mut handle, parser).unwrap();

    // To make the footnotes look right, we need to sort them by their appearance order, not by
    // the in-tree order of their actual definitions. Unused items are omitted entirely.
    //
    // For example, this code:
    //
    //     test [^1] [^2]
    //     [^2]: second used, first defined
    //     [^1]: test
    //
    // Gets rendered like *this* if you copy it into a GitHub comment box:
    //
    //     <p>test <sup>[1]</sup> <sup>[2]</sup></p>
    //     <hr>
    //     <ol>
    //     <li>test ↩</li>
    //     <li>second used, first defined ↩</li>
    //     </ol>
    if !footnotes.is_empty() {
        footnotes.retain(|f| match f.first() {
            Some(Event::Start(Tag::FootnoteDefinition(name))) => {
                footnote_numbers.get(name).unwrap_or(&(0, 0)).1 != 0
            }
            _ => false,
        });
        footnotes.sort_by_cached_key(|f| match f.first() {
            Some(Event::Start(Tag::FootnoteDefinition(name))) => {
                footnote_numbers.get(name).unwrap_or(&(0, 0)).0
            }
            _ => unreachable!(),
        });
        handle
            .write_all(b"<hr><ol class=\"footnotes-list\">\n")
            .unwrap();
        html::write_html_io(
            &mut handle,
            footnotes.into_iter().flat_map(|fl| {
                // To write backrefs, the name needs kept until the end of the footnote definition.
                let mut name = CowStr::from("");
                // Backrefs are included in the final paragraph of the footnote, if it's normal text.
                // For example, this DOM can be produced:
                //
                // Markdown:
                //
                //     five [^feet].
                //
                //     [^feet]:
                //         A foot is defined, in this case, as 0.3048 m.
                //
                //         Historically, the foot has not been defined this way, corresponding to many
                //         subtly different units depending on the location.
                //
                // HTML:
                //
                //     <p>five <sup class="footnote-reference" id="fr-feet-1"><a href="#fn-feet">[1]</a></sup>.</p>
                //
                //     <ol class="footnotes-list">
                //     <li id="fn-feet">
                //     <p>A foot is defined, in this case, as 0.3048 m.</p>
                //     <p>Historically, the foot has not been defined this way, corresponding to many
                //     subtly different units depending on the location. <a href="#fr-feet-1">↩</a></p>
                //     </li>
                //     </ol>
                //
                // This is mostly a visual hack, so that footnotes use less vertical space.
                //
                // If there is no final paragraph, such as a tabular, list, or image footnote, it gets
                // pushed after the last tag instead.
                let mut has_written_backrefs = false;
                let fl_len = fl.len();
                let footnote_numbers = &footnote_numbers;
                fl.into_iter().enumerate().map(move |(i, f)| match f {
                    Event::Start(Tag::FootnoteDefinition(current_name)) => {
                        name = current_name;
                        has_written_backrefs = false;
                        Event::Html(format!(r##"<li id="fn-{name}">"##).into())
                    }
                    Event::End(TagEnd::FootnoteDefinition) | Event::End(TagEnd::Paragraph)
                        if !has_written_backrefs && i >= fl_len - 2 =>
                    {
                        let usage_count = footnote_numbers.get(&name).unwrap().1;
                        let mut end = String::with_capacity(
                            name.len() + (r##" <a href="#fr--1">↩</a></li>"##.len() * usage_count),
                        );
                        for usage in 1..=usage_count {
                            if usage == 1 {
                                write!(&mut end, r##" <a href="#fr-{name}-{usage}">↩</a>"##)
                                    .unwrap();
                            } else {
                                write!(&mut end, r##" <a href="#fr-{name}-{usage}">↩{usage}</a>"##)
                                    .unwrap();
                            }
                        }
                        has_written_backrefs = true;
                        if f == Event::End(TagEnd::FootnoteDefinition) {
                            end.push_str("</li>\n");
                        } else {
                            end.push_str("</p>\n");
                        }
                        Event::Html(end.into())
                    }
                    Event::End(TagEnd::FootnoteDefinition) => Event::Html("</li>\n".into()),
                    Event::FootnoteReference(_) => unreachable!("converted to HTML earlier"),
                    f => f,
                })
            }),
        )
        .unwrap();
        handle.write_all(b"</ol>\n").unwrap();
    }
}

```

### 7.5: `normalize-wikilink.rs`

```rust
use pulldown_cmark::{html, CowStr, Event, LinkType, Options, Parser, Tag};
use regex::RegexBuilder;
use std::io::Write;

/// This example demonstrates how to normalize the href of a wikilink. The
/// details of this implementation can be tweaked for different use cases.
fn main() {
    let markdown_input: &str = r#"
Example provided by [[https://example.org/]].
Some people might prefer the wikilink syntax for autolinks.

Wanna go for a [[Wiki Walk]]?"#;

    let parser = Parser::new_ext(markdown_input, Options::ENABLE_WIKILINKS).map(|event| {
        if let Event::Start(Tag::Link {
            link_type: LinkType::WikiLink { has_pothole },
            dest_url,
            title,
            id,
        }) = event
        {
            let new_link = normalize_wikilink(dest_url);
            Event::Start(Tag::Link {
                link_type: LinkType::WikiLink { has_pothole },
                dest_url: new_link,
                title,
                id,
            })
        } else {
            event
        }
    });

    // Write to anything implementing the `Write` trait. This could also be a file
    // or network socket.
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"\nHTML output:\n").unwrap();
    html::write_html_io(&mut handle, parser).unwrap();
}

/// Performs wikilink normalization.
fn normalize_wikilink(link: CowStr) -> CowStr {
    // your wiki is stored at "/wiki"
    let prefix: &str = "/wiki";
    if link.is_empty() {
        return link;
    }

    // check if the link is absolute, if it is, return as is
    // according to RFC 3986; https://www.rfc-editor.org/rfc/rfc3986
    let is_absolute = RegexBuilder::new("^(?:[a-z][a-z0-9+\\-.]*:)?//")
        .case_insensitive(true)
        .build()
        .expect("valid regex");

    if is_absolute.is_match(&link) {
        return link;
    }

    let mut result = String::with_capacity(link.len() + 2);
    let mut i = 0;
    let mut mark = 0;
    let mut in_whitespace = false;

    result.push_str(prefix);

    if !link.starts_with('/') {
        result.push('/');
    }

    while i < link.len() {
        if !in_whitespace && link.as_bytes()[i].is_ascii_whitespace() {
            in_whitespace = true;
            result.push_str(&link[mark..i]);
        } else if in_whitespace && !link.as_bytes()[i].is_ascii_whitespace() {
            result.push('_');
            mark = i;
            in_whitespace = false;
        }

        i += 1;
    }

    if !in_whitespace {
        result.push_str(&link[mark..]);
    }
    if !result.ends_with('/') {
        result.push('/');
    }
    result.into()
}

```

### 7.6: `parser-map-event-print.rs`

```rust
use pulldown_cmark::{html, Event, Parser};

fn main() {
    let markdown_input = "# Example Heading\nExample paragraph with **lorem** _ipsum_ text.";
    println!(
        "\nParsing the following markdown string:\n{}\n",
        markdown_input
    );

    // Set up the parser. We can treat is as any other iterator.
    // For each event, we print its details, such as the tag or string.
    // This filter simply returns the same event without any changes;
    // you can compare the `event-filter` example which alters the output.
    let parser = Parser::new(markdown_input).map(|event| {
        match &event {
            Event::Start(tag) => println!("Start: {:?}", tag),
            Event::End(tag) => println!("End: {:?}", tag),
            Event::Html(s) => println!("Html: {:?}", s),
            Event::InlineHtml(s) => println!("InlineHtml: {:?}", s),
            Event::Text(s) => println!("Text: {:?}", s),
            Event::Code(s) => println!("Code: {:?}", s),
            Event::DisplayMath(s) => println!("DisplayMath: {:?}", s),
            Event::InlineMath(s) => println!("Math: {:?}", s),
            Event::FootnoteReference(s) => println!("FootnoteReference: {:?}", s),
            Event::TaskListMarker(b) => println!("TaskListMarker: {:?}", b),
            Event::SoftBreak => println!("SoftBreak"),
            Event::HardBreak => println!("HardBreak"),
            Event::Rule => println!("Rule"),
        };
        event
    });

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    println!("\nHTML output:\n{}\n", &html_output);
}

```

### 7.7: `parser-map-tag-print.rs`

```rust
use pulldown_cmark::{Event, Options, Parser, Tag};

fn main() {
    let markdown_input = concat!(
        "# My Heading\n",
        "\n",
        "My paragraph.\n",
        "\n",
        "* a\n",
        "* b\n",
        "* c\n",
        "\n",
        "1. d\n",
        "2. e\n",
        "3. f\n",
        "\n",
        "> my block quote\n",
        "\n",
        "```\n",
        "my code block\n",
        "```\n",
        "\n",
        "*emphasis*\n",
        "**strong**\n",
        "~~strikethrough~~\n",
        "[My Link](http://example.com)\n",
        "![My Image](http://example.com/image.jpg)\n",
        "\n",
        "| a | b |\n",
        "| - | - |\n",
        "| c | d |\n",
        "\n",
        "hello[^1]\n",
        "[^1]: my footnote\n",
    );
    println!(
        "\nParsing the following markdown string:\n{}\n",
        markdown_input
    );

    // Set up the parser. We can treat is as any other iterator.
    // For each event, we print its details, such as the tag or string.
    // This filter simply returns the same event without any changes;
    // you can compare the `event-filter` example which alters the output.
    let parser = Parser::new_ext(markdown_input, Options::all()).map(|event| {
        match &event {
            Event::Start(tag) => match tag {
                Tag::HtmlBlock => println!("HtmlBlock"),
                Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                } => println!(
                    "Heading heading_level: {} fragment identifier: {:?} classes: {:?} attrs: {:?}",
                    level, id, classes, attrs
                ),
                Tag::Paragraph => println!("Paragraph"),
                Tag::List(ordered_list_first_item_number) => println!(
                    "List ordered_list_first_item_number: {:?}",
                    ordered_list_first_item_number
                ),
                Tag::DefinitionList => println!("Definition list"),
                Tag::DefinitionListTitle => println!("Definition title (definition list item)"),
                Tag::DefinitionListDefinition => println!("Definition (definition list item)"),
                Tag::Item => println!("Item (this is a list item)"),
                Tag::Emphasis => println!("Emphasis (this is a span tag)"),
                Tag::Superscript => println!("Superscript (this is a span tag)"),
                Tag::Subscript => println!("Subscript (this is a span tag)"),
                Tag::Strong => println!("Strong (this is a span tag)"),
                Tag::Strikethrough => println!("Strikethrough (this is a span tag)"),
                Tag::BlockQuote(kind) => println!("BlockQuote ({:?})", kind),
                Tag::CodeBlock(code_block_kind) => {
                    println!("CodeBlock code_block_kind: {:?}", code_block_kind)
                }
                Tag::Link {
                    link_type,
                    dest_url,
                    title,
                    id,
                } => println!(
                    "Link link_type: {:?} url: {} title: {} id: {}",
                    link_type, dest_url, title, id
                ),
                Tag::Image {
                    link_type,
                    dest_url,
                    title,
                    id,
                } => println!(
                    "Image link_type: {:?} url: {} title: {} id: {}",
                    link_type, dest_url, title, id
                ),
                Tag::Table(column_text_alignment_list) => println!(
                    "Table column_text_alignment_list: {:?}",
                    column_text_alignment_list
                ),
                Tag::TableHead => println!("TableHead (contains TableRow tags"),
                Tag::TableRow => println!("TableRow (contains TableCell tags)"),
                Tag::TableCell => println!("TableCell (contains inline tags)"),
                Tag::FootnoteDefinition(label) => println!("FootnoteDefinition label: {}", label),
                Tag::MetadataBlock(kind) => println!("MetadataBlock: {:?}", kind),
            },
            _ => (),
        };
        event
    });

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    println!("\nHTML output:\n{}\n", &html_output);
}

```

### 7.8: `string-to-string.rs`

```rust
use pulldown_cmark::{html, Options, Parser};

fn main() {
    let markdown_input: &str = "Hello world, this is a ~~complicated~~ *very simple* example.";
    println!("Parsing the following markdown string:\n{}", markdown_input);

    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    // Check that the output is what we expected.
    let expected_html: &str =
        "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
    assert_eq!(expected_html, &html_output);

    // Write result to stdout.
    println!("\nHTML output:\n{}", &html_output);
}

```

# pulldown_cmark_to_cmark API (21.0.0)

Convert pulldown-cmark Events back to the string they were parsed from

## 1: Manifest

- Homepage: <https://github.com/Byron/pulldown-cmark-to-cmark>
- Repository: <https://github.com/Byron/pulldown-cmark-to-cmark>
- License: Apache-2.0
- rust-version: `1.71.1`
- edition: `2018`

### 1.1: Features

- None


## 2: README

[![Crates.io](https://img.shields.io/crates/v/pulldown-cmark-to-cmark)](https://crates.io/crates/pulldown-cmark-to-cmark)
![Rust](https://github.com/Byron/pulldown-cmark-to-cmark/workflows/Rust/badge.svg)

A utility library which translates [`Event`][pdcm-event] back to markdown.
It's the prerequisite for writing markdown filters which can work as
[mdbook-preprocessors][mdbook-prep].

This library takes great pride in supporting **everything that `pulldown-cmark`** supports,
including *tables* and *footnotes* and *codeblocks in codeblocks*,
while assuring *quality* with a powerful test suite.

##### How to use

Please have a look at the [`stupicat`-example][sc-example] for a complete tour
of the API, or have a look at the [api-docs][api].

It's easiest to get this library into your `Cargo.toml` using `cargo-add`:

````
cargo add pulldown-cmark-to-cmark
````

##### Supported Rust Versions

`pulldown-cmark-to-cmark` follows the MSRV (minimum supported rust version) policy of [`pulldown-cmark`]. The current MSRV is 1.71.1.

##### Friends of this project

* [**termbook**](https://github.com/Byron/termbook)
  * A runner for `mdbooks` to keep your documentation tested.
* [**Share Secrets Safely**](https://github.com/Byron/share-secrets-safely)
  * share secrets within teams to avoid plain-text secrets from day one

##### Maintenance Guide

###### Making a new release

* **Assure all documentation is up-to-date and tests are green**
* update the `version` in `Cargo.toml` and `git commit`
* run `cargo release --no-dev-version`

[pdcm-event]: https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.Event.html
[mdbook-prep]: https://rust-lang.github.io/mdBook/for_developers/preprocessors.html
[sc-example]: https://github.com/Byron/pulldown-cmark-to-cmark/blob/76667725b61be24890fbdfed5e7ecdb4c1ad1dc8/examples/stupicat.rs#L21
[api]: https://docs.rs/crate/pulldown-cmark-to-cmark
[`pulldown-cmark`]: https://github.com/pulldown-cmark/pulldown-cmark


## 3: Common Traits

The following traits are commonly implemented by types in this crate. Unless otherwise noted, you can assume these traits are implemented:

- `Clone`
- `Debug`
- `Eq`
- `Hash`
- `Ord`
- `PartialEq`
- `PartialOrd`
- `StructuralPartialEq`

- `Freeze`
- `RefUnwindSafe`
- `Send`
- `Sync`
- `Unpin`
- `UnwindSafe`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)


## 4: Module: `pulldown_cmark_to_cmark`


### 4.1: Structs

#### 4.1.1: `struct pulldown_cmark_to_cmark::Heading<'a>`

```rust
pub struct Heading<'a> {}
```

_[Private fields hidden]_

##### 4.1.1.1: Trait Implementations for `Heading`

- `convert::From<T>`

#### 4.1.2: `struct pulldown_cmark_to_cmark::Options<'a>`

```rust
pub struct Options<'a> {
    pub newlines_after_headline: usize,
    pub newlines_after_paragraph: usize,
    pub newlines_after_codeblock: usize,
    pub newlines_after_htmlblock: usize,
    pub newlines_after_table: usize,
    pub newlines_after_rule: usize,
    pub newlines_after_list: usize,
    pub newlines_after_blockquote: usize,
    pub newlines_after_rest: usize,
    pub newlines_after_metadata: usize,
    pub code_block_token_count: usize,
    pub code_block_token: char,
    pub list_token: char,
    pub ordered_list_token: char,
    pub increment_ordered_list_bullets: bool,
    pub emphasis_token: char,
    pub strong_token: &'a str,
}
```

Configuration for the \[`cmark_with_options()`\] and \[`cmark_resume_with_options()`\] functions.
The defaults should provide decent spacing and most importantly, will
provide a faithful rendering of your markdown document particularly when
rendering it to HTML.

It's best used with its `Options::default()` implementation.

##### 4.1.2.1: Fields

###### 4.1.2.1.1: `newlines_after_metadata`

The amount of newlines placed after TOML or YAML metadata blocks at the beginning of a document.

###### 4.1.2.1.2: `code_block_token_count`

Token count for fenced code block. An appropriate value of this field can be decided by
\[`calculate_code_block_token_count()`\].
Note that the default value is `4` which allows for one level of nested code-blocks,
which is typically a safe value for common kinds of markdown documents.

##### 4.1.2.2: `impl pulldown_cmark_to_cmark::Options<'_>`

###### 4.1.2.3.1: `fn special_characters(self: &Self) -> Cow<'static, str>`


##### 4.1.2.3: Trait Implementations for `Options`

- `default::Default`

- `convert::From<T>`

#### 4.1.3: `struct pulldown_cmark_to_cmark::State<'a>`

```rust
#[non_exhaustive]
pub struct State<'a> {
    pub newlines_before_start: usize,
    pub list_stack: Vec<option::Option<u64>>,
    pub padding: Vec<Cow<'a, str>>,
    pub table_alignments: Vec<pulldown_cmark_to_cmark::Alignment>,
    pub table_headers: Vec<String>,
    pub text_for_header: option::Option<String>,
    pub code_block: option::Option<pulldown_cmark_to_cmark::CodeBlockKind>,
    pub last_was_text_without_trailing_newline: bool,
    pub last_was_paragraph_start: bool,
    pub next_is_link_like: bool,
    pub link_stack: Vec<pulldown_cmark_to_cmark::LinkCategory<'a>>,
    pub image_stack: Vec<pulldown_cmark_to_cmark::ImageLink<'a>>,
    pub current_heading: option::Option<pulldown_cmark_to_cmark::Heading<'a>>,
    pub in_table_cell: bool,
    pub current_shortcut_text: option::Option<String>,
    pub shortcuts: Vec<(String, String, String)>,
    pub last_event_end_index: usize,
}
```

The state of the \[`cmark_resume()`\] and \[`cmark_resume_with_options()`\] functions.
This does not only allow introspection, but enables the user
to halt the serialization at any time, and resume it later.

##### 4.1.3.1: Fields

###### 4.1.3.1.1: `newlines_before_start`

The amount of newlines to insert after `Event::Start(...)`

###### 4.1.3.1.2: `list_stack`

The lists and their types for which we have seen a `Event::Start(List(...))` tag

###### 4.1.3.1.3: `padding`

The computed padding and prefix to print after each newline.
This changes with the level of `BlockQuote` and `List` events.

###### 4.1.3.1.4: `table_alignments`

Keeps the current table alignments, if we are currently serializing a table.

###### 4.1.3.1.5: `table_headers`

Keeps the current table headers, if we are currently serializing a table.

###### 4.1.3.1.6: `text_for_header`

The last seen text when serializing a header

###### 4.1.3.1.7: `code_block`

Is set while we are handling text in a code block

###### 4.1.3.1.8: `last_was_text_without_trailing_newline`

True if the last event was text and the text does not have trailing newline. Used to inject additional newlines before code block end fence.

###### 4.1.3.1.9: `last_was_paragraph_start`

True if the last event was a paragraph start. Used to escape spaces at start of line (prevent spurrious indented code).

###### 4.1.3.1.10: `next_is_link_like`

True if the next event is a link, image, or footnote.

###### 4.1.3.1.11: `link_stack`

Currently open links

###### 4.1.3.1.12: `image_stack`

Currently open images

###### 4.1.3.1.13: `current_heading`

Keeps track of the last seen heading's id, classes, and attributes

###### 4.1.3.1.14: `in_table_cell`

True whenever between `Start(TableCell)` and `End(TableCell)`

###### 4.1.3.1.15: `current_shortcut_text`

Keeps track of the last seen shortcut/link

###### 4.1.3.1.16: `shortcuts`

A list of shortcuts seen so far for later emission

###### 4.1.3.1.17: `last_event_end_index`

Index into the `source` bytes of the end of the range corresponding to the last event.

It's used to see if the current event didn't capture some bytes because of a
skipped-over backslash.

##### 4.1.3.2: `impl pulldown_cmark_to_cmark::State<'_>`

###### 4.1.3.3.1: `fn is_in_code_block(self: &Self) -> bool`


##### 4.1.3.3: `impl pulldown_cmark_to_cmark::State<'_>`

###### 4.1.3.4.1: `fn finalize<F>(self: Self, formatter: F) -> result::Result<Self, pulldown_cmark_to_cmark::Error>`

```rust
pub fn finalize<F> where F: Write(self: Self, formatter: F) -> result::Result<Self, pulldown_cmark_to_cmark::Error> { ... }
```

##### 4.1.3.4: Trait Implementations for `State`

- `default::Default`

- `convert::From<T>`

### 4.2: Enums

#### 4.2.1: `enum pulldown_cmark_to_cmark::Alignment`

```rust
pub enum Alignment {
    None,
    Left,
    Center,
    Right,
}
```

Similar to \[Pulldown-Cmark-Alignment\]\[Alignment\], but with required
traits for comparison to allow testing.

##### 4.2.1.1: Trait Implementations for `Alignment`

- `Copy`

- `convert::From<&'a pulldown_cmark::Alignment>`

- `convert::From<T>`

#### 4.2.2: `enum pulldown_cmark_to_cmark::CodeBlockKind`

```rust
pub enum CodeBlockKind {
    Indented,
    Fenced,
}
```

##### 4.2.2.1: Trait Implementations for `CodeBlockKind`

- `convert::From<T>`

#### 4.2.3: `enum pulldown_cmark_to_cmark::Error`

```rust
pub enum Error {
    FormatFailed(Error),
    UnexpectedEvent,
}
```

The error returned by \[`cmark_resume_one_event`\] and
\[`cmark_resume_with_source_range_and_options`\].

##### 4.2.3.1: Trait Implementations for `Error`

**(Note: Does not implement common trait(s): `Clone`, `CloneToUninit`, `Eq`, `Hash`, `Ord`, `PartialEq`, `PartialOrd`, `StructuralPartialEq`, `ToOwned`)**

- `Display`
- `error::Error`

- `convert::From<Error>`

- `ToString` (`where T: Display + ?Sized`)
- `convert::From<T>`

#### 4.2.4: `enum pulldown_cmark_to_cmark::ImageLink<'a>`

```rust
pub enum ImageLink<'a> {
    Reference{ uri: Cow<'a, str>, title: Cow<'a, str>, id: Cow<'a, str> },
    Collapsed{ uri: Cow<'a, str>, title: Cow<'a, str> },
    Shortcut{ uri: Cow<'a, str>, title: Cow<'a, str> },
    Other{ uri: Cow<'a, str>, title: Cow<'a, str> },
}
```

##### 4.2.4.1: Trait Implementations for `ImageLink`

- `convert::From<T>`

#### 4.2.5: `enum pulldown_cmark_to_cmark::LinkCategory<'a>`

```rust
pub enum LinkCategory<'a> {
    AngleBracketed,
    Reference{ uri: Cow<'a, str>, title: Cow<'a, str>, id: Cow<'a, str> },
    Collapsed{ uri: Cow<'a, str>, title: Cow<'a, str> },
    Shortcut{ uri: Cow<'a, str>, title: Cow<'a, str> },
    Other{ uri: Cow<'a, str>, title: Cow<'a, str> },
}
```

##### 4.2.5.1: Trait Implementations for `LinkCategory`

- `convert::From<T>`

### 4.3: Functions

#### 4.3.1: `fn calculate_code_block_token_count<'a, I, E>(events: I) -> option::Option<usize>`

```rust
pub fn calculate_code_block_token_count<'a, I, E>
  where
    I: iter::traits::collect::IntoIterator<Item = E>,
    E: borrow::Borrow<pulldown_cmark::Event<'a>>(events: I) -> option::Option<usize> { ... }
```

Return the `<seen amount of consecutive fenced code-block tokens> + 1` that occur *within* a
fenced code-block `events`.

Use this function to obtain the correct value for `code_block_token_count` field of \[`Options`\]
to assure that the enclosing code-blocks remain functional as such.

Returns `None` if `events` didn't include any code-block, or the code-block didn't contain
a nested block. In that case, the correct amount of fenced code-block tokens is
\[`DEFAULT_CODE_BLOCK_TOKEN_COUNT`\].

````rust
use pulldown_cmark::Event;
use pulldown_cmark_to_cmark::*;

let events = &[Event::Text("text".into())];
let code_block_token_count = calculate_code_block_token_count(events).unwrap_or(DEFAULT_CODE_BLOCK_TOKEN_COUNT);
let options = Options {
    code_block_token_count,
    ..Default::default()
};
let mut buf = String::new();
cmark_with_options(events.iter(), &mut buf, options);
````


#### 4.3.2: `fn cmark<'a, I, E, F>(events: I, formatter: F) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error>`

```rust
pub fn cmark<'a, I, E, F>
  where
    I: iter::traits::iterator::Iterator<Item = E>,
    E: borrow::Borrow<pulldown_cmark::Event<'a>>,
    F: Write(events: I, formatter: F) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error> { ... }
```

As \[`cmark_with_options()`\], but with default \[`Options`\].


#### 4.3.3: `fn cmark_resume<'a, I, E, F>(events: I, formatter: F, state: option::Option<pulldown_cmark_to_cmark::State<'a>>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error>`

```rust
pub fn cmark_resume<'a, I, E, F>
  where
    I: iter::traits::iterator::Iterator<Item = E>,
    E: borrow::Borrow<pulldown_cmark::Event<'a>>,
    F: Write(events: I, formatter: F, state: option::Option<pulldown_cmark_to_cmark::State<'a>>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error> { ... }
```

As \[`cmark_resume_with_options()`\], but with default \[`Options`\].


#### 4.3.4: `fn cmark_resume_with_options<'a, I, E, F>(events: I, formatter: F, state: option::Option<pulldown_cmark_to_cmark::State<'a>>, options: pulldown_cmark_to_cmark::Options<'_>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error>`

```rust
pub fn cmark_resume_with_options<'a, I, E, F>
  where
    I: iter::traits::iterator::Iterator<Item = E>,
    E: borrow::Borrow<pulldown_cmark::Event<'a>>,
    F: Write(events: I, formatter: F, state: option::Option<pulldown_cmark_to_cmark::State<'a>>, options: pulldown_cmark_to_cmark::Options<'_>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error> { ... }
```

Serialize a stream of \[pulldown-cmark-Events\]\[Event\] into a string-backed buffer.

1. **events**
   * An iterator over \[`Events`\]\[Event\], for example as returned by the \[`Parser`\]\[pulldown_cmark::Parser\]
1. **formatter**
   * A format writer, can be a `String`.
1. **state**
   * The optional initial state of the serialization.
1. **options**
   * Customize the appearance of the serialization. All otherwise magic values are contained
     here.

*Returns* the \[`State`\] of the serialization on success. You can use it as initial state in the
next call if you are halting event serialization.

*Errors* if the underlying buffer fails (which is unlikely) or if the \[`Event`\] stream
cannot ever be produced by deserializing valid Markdown. Each failure mode corresponds to one
of \[`Error`\]'s variants.


#### 4.3.5: `fn cmark_resume_with_source_range<'a, I, E, F>(event_and_ranges: I, source: &'a str, formatter: F, state: option::Option<pulldown_cmark_to_cmark::State<'a>>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error>`

```rust
pub fn cmark_resume_with_source_range<'a, I, E, F>
  where
    I: iter::traits::iterator::Iterator<Item = (E, option::Option<range::Range<usize>>)>,
    E: borrow::Borrow<pulldown_cmark::Event<'a>>,
    F: Write(event_and_ranges: I, source: &'a str, formatter: F, state: option::Option<pulldown_cmark_to_cmark::State<'a>>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error> { ... }
```

As \[`cmark_resume_with_source_range_and_options`\], but with default \[`Options`\].


#### 4.3.6: `fn cmark_resume_with_source_range_and_options<'a, I, E, F>(event_and_ranges: I, source: &'a str, formatter: F, state: option::Option<pulldown_cmark_to_cmark::State<'a>>, options: pulldown_cmark_to_cmark::Options<'_>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error>`

```rust
pub fn cmark_resume_with_source_range_and_options<'a, I, E, F>
  where
    I: iter::traits::iterator::Iterator<Item = (E, option::Option<range::Range<usize>>)>,
    E: borrow::Borrow<pulldown_cmark::Event<'a>>,
    F: Write(event_and_ranges: I, source: &'a str, formatter: F, state: option::Option<pulldown_cmark_to_cmark::State<'a>>, options: pulldown_cmark_to_cmark::Options<'_>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error> { ... }
```

Serialize a stream of \[pulldown-cmark-Events\]\[Event\] while preserving the escape characters in `source`.
Each input \[Event\] is accompanied by an optional \[Range\] that maps it back to the `source` string.

Different from [`cmark_resume_with_options`](super::cmark_resume_with_options), which always escape
Markdown special characters like `#` or `[`, this function only escapes a special character if
it is escaped in `source`.

1. **source**
   * Markdown source from which `event_and_ranges` are created.
1. **event_and_ranges**
   * An iterator over \[`Event`\]-range pairs, for example as returned by \[`pulldown_cmark::OffsetIter`\].
     Must match what's provided in `source`.
1. **formatter**
   * A format writer, can be a `String`.
1. **state**
   * The optional initial state of the serialization, useful when the operation should be resumed.
1. **options**
   * Customize the appearance of the serialization. All otherwise magic values are contained
     here.

*Returns* the \[`State`\] of the serialization on success. You can use it as initial state in the
next call if you are halting event serialization.

*Errors* if the underlying buffer fails (which is unlikely) or if the \[`Event`\] stream
iterated over by `event_and_ranges` cannot ever be produced by deserializing valid Markdown.
Each failure mode corresponds to one of \[`Error`\]'s variants.


#### 4.3.7: `fn cmark_with_options<'a, I, E, F>(events: I, formatter: F, options: pulldown_cmark_to_cmark::Options<'_>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error>`

```rust
pub fn cmark_with_options<'a, I, E, F>
  where
    I: iter::traits::iterator::Iterator<Item = E>,
    E: borrow::Borrow<pulldown_cmark::Event<'a>>,
    F: Write(events: I, formatter: F, options: pulldown_cmark_to_cmark::Options<'_>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error> { ... }
```

As \[`cmark_resume_with_options()`\], but with the \[`State`\] finalized.


#### 4.3.8: `fn cmark_with_source_range<'a, I, E, F>(event_and_ranges: I, source: &'a str, formatter: F) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error>`

```rust
pub fn cmark_with_source_range<'a, I, E, F>
  where
    I: iter::traits::iterator::Iterator<Item = (E, option::Option<range::Range<usize>>)>,
    E: borrow::Borrow<pulldown_cmark::Event<'a>>,
    F: Write(event_and_ranges: I, source: &'a str, formatter: F) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error> { ... }
```

As \[`cmark_with_source_range_and_options`\], but with default \[`Options`\].


#### 4.3.9: `fn cmark_with_source_range_and_options<'a, I, E, F>(event_and_ranges: I, source: &'a str, formatter: F, options: pulldown_cmark_to_cmark::Options<'_>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error>`

```rust
pub fn cmark_with_source_range_and_options<'a, I, E, F>
  where
    I: iter::traits::iterator::Iterator<Item = (E, option::Option<range::Range<usize>>)>,
    E: borrow::Borrow<pulldown_cmark::Event<'a>>,
    F: Write(event_and_ranges: I, source: &'a str, formatter: F, options: pulldown_cmark_to_cmark::Options<'_>) -> result::Result<pulldown_cmark_to_cmark::State<'a>, pulldown_cmark_to_cmark::Error> { ... }
```

As \[`cmark_resume_with_source_range_and_options`\], but with the \[`State`\] finalized.


### 4.4: Constants

#### 4.4.1: `const DEFAULT_CODE_BLOCK_TOKEN_COUNT`

Thea mount of code-block tokens one needs to produce a valid fenced code-block.

# glob API (0.3.2)

Support for matching file paths against Unix shell style patterns.


## 1: Manifest

- Homepage: <https://github.com/rust-lang/glob>
- Repository: <https://github.com/rust-lang/glob>
- Categories: filesystem
- License: MIT OR Apache-2.0
- rust-version: `1.23.0`

### 1.1: Features

- None


## 2: README

### glob

Support for matching file paths against Unix shell style patterns.

[![Continuous integration](https://github.com/rust-lang/glob/actions/workflows/rust.yml/badge.svg)](https://github.com/rust-lang/glob/actions/workflows/rust.yml)

[Documentation](https://docs.rs/glob)

#### Usage

To use `glob`, add this to your `Cargo.toml`:

````toml
[dependencies]
glob = "0.3.1"
````

If you're using Rust 1.30 or earlier, or edition 2015, add this to your crate root:

````rust
extern crate glob;
````

#### Examples

Print all jpg files in /media/ and all of its subdirectories.

````rust
use glob::glob;

for entry in glob("/media/**/*.jpg").expect("Failed to read glob pattern") {
    match entry {
        Ok(path) => println!("{:?}", path.display()),
        Err(e) => println!("{:?}", e),
    }
}
````


## 3: Common Traits

The following traits are commonly implemented by types in this crate. Unless otherwise noted, you can assume these traits are implemented:

- `Debug`
- `Display`

- `Freeze`
- `Send`
- `Sync`
- `Unpin`

- `ToString` (`where T: Display + ?Sized`)
- `any::Any` (`where T: 'static + ?Sized`)
- `borrow::Borrow<T>` (`where T: ?Sized`)
- `borrow::BorrowMut<T>` (`where T: ?Sized`)
- `convert::From<T>`
- `convert::Into<U>` (`where U: convert::From<T>`)
- `convert::TryFrom<U>` (`where U: convert::Into<T>`)
- `convert::TryInto<U>` (`where U: convert::TryFrom<T>`)


## 4: Module: `glob`

Support for matching file paths against Unix shell style patterns.

The `glob` and `glob_with` functions allow querying the filesystem for all
files that match a particular pattern (similar to the libc `glob` function).
The methods on the `Pattern` type provide functionality for checking if
individual paths match a particular pattern (similar to the libc `fnmatch`
function).

For consistency across platforms, and for Windows support, this module
is implemented entirely in Rust rather than deferring to the libc
`glob`/`fnmatch` functions.

#### Examples

To print all jpg files in `/media/` and all of its subdirectories.

````rust,no_run
use glob::glob;

for entry in glob("/media/**/*.jpg").expect("Failed to read glob pattern") {
    match entry {
        Ok(path) => println!("{:?}", path.display()),
        Err(e) => println!("{:?}", e),
    }
}
````

To print all files containing the letter "a", case insensitive, in a `local`
directory relative to the current working directory. This ignores errors
instead of printing them.

````rust,no_run
use glob::glob_with;
use glob::MatchOptions;

let options = MatchOptions {
    case_sensitive: false,
    require_literal_separator: false,
    require_literal_leading_dot: false,
};
for entry in glob_with("local/*a*", options).unwrap() {
    if let Ok(path) = entry {
        println!("{:?}", path.display())
    }
}
````


### 4.1: Structs

#### 4.1.1: `struct glob::GlobError`

```rust
pub struct GlobError {}
```

_[Private fields hidden]_

A glob iteration error.

This is typically returned when a particular path cannot be read
to determine if its contents match the glob pattern. This is possible
if the program lacks the appropriate permissions, for example.

##### 4.1.1.1: `impl glob::GlobError`

###### 4.1.1.2.1: `fn path(self: &Self) -> &path::Path`

The Path that the error corresponds to.

###### 4.1.1.2.2: `fn error(self: &Self) -> &io::error::Error`

The error in question.

###### 4.1.1.2.3: `fn into_error(self: Self) -> io::error::Error`

Consumes self, returning the *raw* underlying `io::Error`

##### 4.1.1.2: Trait Implementations for `GlobError`

- `error::Error`

- `!RefUnwindSafe`
- `!UnwindSafe`

#### 4.1.2: `struct glob::MatchOptions`

```rust
#[allow(missing_copy_implementations)]
pub struct MatchOptions {
    pub case_sensitive: bool,
    pub require_literal_separator: bool,
    pub require_literal_leading_dot: bool,
}
```

Configuration options to modify the behaviour of `Pattern::matches_with(..)`.

##### 4.1.2.1: Fields

###### 4.1.2.1.1: `case_sensitive`

Whether or not patterns should be matched in a case-sensitive manner.
This currently only considers upper/lower case relationships between
ASCII characters, but in future this might be extended to work with
Unicode.

###### 4.1.2.1.2: `require_literal_separator`

Whether or not path-component separator characters (e.g. `/` on
Posix) must be matched by a literal `/`, rather than by `*` or `?` or
`[...]`.

###### 4.1.2.1.3: `require_literal_leading_dot`

Whether or not paths that contain components that start with a `.`
will require that `.` appears literally in the pattern; `*`, `?`, `**`,
or `[...]` will not match. This is useful because such files are
conventionally considered hidden on Unix systems and it might be
desirable to skip them when listing files.

##### 4.1.2.2: `impl glob::MatchOptions`

###### 4.1.2.3.1: `fn new() -> Self`

Constructs a new `MatchOptions` with default field values. This is used
when calling functions that do not take an explicit `MatchOptions`
parameter.

This function always returns this value:

````rust,ignore
MatchOptions {
    case_sensitive: true,
    require_literal_separator: false,
    require_literal_leading_dot: false
}
````

###### Note

The behavior of this method doesn't match `default()`'s. This returns
`case_sensitive` as `true` while `default()` does it as `false`.

##### 4.1.2.3: Trait Implementations for `MatchOptions`

**(Note: Does not implement common trait(s): `Display`, `ToString`)**

- `Clone`
- `Copy`
- `Eq`
- `Hash`
- `Ord`
- `PartialEq`
- `PartialOrd`
- `StructuralPartialEq`
- `default::Default`

- `RefUnwindSafe`
- `UnwindSafe`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)

#### 4.1.3: `struct glob::Paths`

```rust
pub struct Paths {}
```

_[Private fields hidden]_

An iterator that yields `Path`s from the filesystem that match a particular
pattern.

Note that it yields `GlobResult` in order to report any `IoErrors` that may
arise during iteration. If a directory matches but is unreadable,
thereby preventing its contents from being checked for matches, a
`GlobError` is returned to express this.

See the `glob` function for more details.

##### 4.1.3.1: Trait Implementations for `Paths`

**(Note: Does not implement common trait(s): `Display`, `ToString`)**

- `iter::traits::iterator::Iterator`

    ```rust
    impl iter::traits::iterator::Iterator for glob::Paths {
        type Item = result::Result<path::PathBuf, glob::GlobError>;
    }
    ```

- `!RefUnwindSafe`
- `!UnwindSafe`

- `iter::traits::collect::IntoIterator` (`where I: iter::traits::iterator::Iterator`)

#### 4.1.4: `struct glob::Pattern`

```rust
pub struct Pattern {}
```

_[Private fields hidden]_

A compiled Unix shell style pattern.

* `?` matches any single character.

* `*` matches any (possibly empty) sequence of characters.

* `**` matches the current directory and arbitrary
  subdirectories. To match files in arbitrary subdiretories, use
  `**/*`.
  
  This sequence **must** form a single path component, so both
  `**a` and `b**` are invalid and will result in an error.  A
  sequence of more than two consecutive `*` characters is also
  invalid.

* `[...]` matches any character inside the brackets.  Character sequences
  can also specify ranges of characters, as ordered by Unicode, so e.g.
  `[0-9]` specifies any character between 0 and 9 inclusive. An unclosed
  bracket is invalid.

* `[!...]` is the negation of `[...]`, i.e. it matches any characters
  **not** in the brackets.

* The metacharacters `?`, `*`, `[`, `]` can be matched by using brackets
  (e.g. `[?]`).  When a `]` occurs immediately following `[` or `[!` then it
  is interpreted as being part of, rather then ending, the character set, so
  `]` and NOT `]` can be matched by `[]]` and `[!]]` respectively.  The `-`
  character can be specified inside a character sequence pattern by placing
  it at the start or the end, e.g. `[abc-]`.

##### 4.1.4.1: `impl glob::Pattern`

###### 4.1.4.2.1: `fn new(pattern: &str) -> result::Result<Self, glob::PatternError>`

This function compiles Unix shell style patterns.

An invalid glob pattern will yield a `PatternError`.

###### 4.1.4.2.2: `fn escape(s: &str) -> String`

Escape metacharacters within the given string by surrounding them in
brackets. The resulting string will, when compiled into a `Pattern`,
match the input string and nothing else.

###### 4.1.4.2.3: `fn matches(self: &Self, str: &str) -> bool`

Return if the given `str` matches this `Pattern` using the default
match options (i.e. `MatchOptions::new()`).

###### Examples

````rust
use glob::Pattern;

assert!(Pattern::new("c?t").unwrap().matches("cat"));
assert!(Pattern::new("k[!e]tteh").unwrap().matches("kitteh"));
assert!(Pattern::new("d*g").unwrap().matches("doog"));
````

###### 4.1.4.2.4: `fn matches_path(self: &Self, path: &path::Path) -> bool`

Return if the given `Path`, when converted to a `str`, matches this
`Pattern` using the default match options (i.e. `MatchOptions::new()`).

###### 4.1.4.2.5: `fn matches_with(self: &Self, str: &str, options: glob::MatchOptions) -> bool`

Return if the given `str` matches this `Pattern` using the specified
match options.

###### 4.1.4.2.6: `fn matches_path_with(self: &Self, path: &path::Path, options: glob::MatchOptions) -> bool`

Return if the given `Path`, when converted to a `str`, matches this
`Pattern` using the specified match options.

###### 4.1.4.2.7: `fn as_str(self: &Self) -> &str`

Access the original glob pattern.

##### 4.1.4.2: Trait Implementations for `Pattern`

- `Clone`
- `Eq`
- `Hash`
- `Ord`
- `PartialEq`
- `PartialOrd`
- `StructuralPartialEq`
- `default::Default`

- `str::traits::FromStr`

    ```rust
    impl str::traits::FromStr for glob::Pattern {
        type Err = glob::PatternError;
    }
    ```

- `RefUnwindSafe`
- `UnwindSafe`

- `CloneToUninit` (`where T: Clone`)
- `ToOwned` (`where T: Clone`)

#### 4.1.5: `struct glob::PatternError`

```rust
#[allow(missing_copy_implementations)]
pub struct PatternError {
    pub pos: usize,
    pub msg: &'static str,
}
```

A pattern parsing error.

##### 4.1.5.1: Fields

###### 4.1.5.1.1: `pos`

The approximate character index of where the error occurred.

###### 4.1.5.1.2: `msg`

A message describing the error.

##### 4.1.5.2: Trait Implementations for `PatternError`

- `error::Error`

- `RefUnwindSafe`
- `UnwindSafe`

### 4.2: Functions

#### 4.2.1: `fn glob(pattern: &str) -> result::Result<glob::Paths, glob::PatternError>`

Return an iterator that produces all the `Path`s that match the given
pattern using default match options, which may be absolute or relative to
the current working directory.

This may return an error if the pattern is invalid.

This method uses the default match options and is equivalent to calling
`glob_with(pattern, MatchOptions::new())`. Use `glob_with` directly if you
want to use non-default match options.

When iterating, each result is a `GlobResult` which expresses the
possibility that there was an `IoError` when attempting to read the contents
of the matched path.  In other words, each item returned by the iterator
will either be an `Ok(Path)` if the path matched, or an `Err(GlobError)` if
the path (partially) matched *but* its contents could not be read in order
to determine if its contents matched.

See the `Paths` documentation for more information.

###### Examples

Consider a directory `/media/pictures` containing only the files
`kittens.jpg`, `puppies.jpg` and `hamsters.gif`:

````rust,no_run
use glob::glob;

for entry in glob("/media/pictures/*.jpg").unwrap() {
    match entry {
        Ok(path) => println!("{:?}", path.display()),

        // if the path matched but was unreadable,
        // thereby preventing its contents from matching
        Err(e) => println!("{:?}", e),
    }
}
````

The above code will print:

````ignore
/media/pictures/kittens.jpg
/media/pictures/puppies.jpg
````

If you want to ignore unreadable paths, you can use something like
`filter_map`:

````rust
use glob::glob;
use std::result::Result;

for path in glob("/media/pictures/*.jpg").unwrap().filter_map(Result::ok) {
    println!("{}", path.display());
}
````

Paths are yielded in alphabetical order.


#### 4.2.2: `fn glob_with(pattern: &str, options: glob::MatchOptions) -> result::Result<glob::Paths, glob::PatternError>`

Return an iterator that produces all the `Path`s that match the given
pattern using the specified match options, which may be absolute or relative
to the current working directory.

This may return an error if the pattern is invalid.

This function accepts Unix shell style patterns as described by
`Pattern::new(..)`.  The options given are passed through unchanged to
`Pattern::matches_with(..)` with the exception that
`require_literal_separator` is always set to `true` regardless of the value
passed to this function.

Paths are yielded in alphabetical order.


### 4.3: Type Aliases

#### 4.3.1: `type GlobResult`

An alias for a glob iteration result.

This represents either a matched path or a glob iteration error,
such as failing to read a particular directory's contents.

