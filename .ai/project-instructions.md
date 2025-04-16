
# rustdoc-types API 0.39

## Item: rustdoc_types::VariantKind (Enum)
------------------------------------
The kind of an [`Enum`] [`Variant`] and the data specific to it, i.e. fields.

## Item: Unknown Path (ID: Id(1441)) (Struct Field)
---------------------------------------------
The type of the pointee, e.g. the `i32` in `&'a mut i32`

## Item: Unknown Path (ID: Id(1544)) (Struct Field)
---------------------------------------------
List of argument names and their type.

Note that not all names will be valid identifiers, as some of
them may be patterns.

## Item: rustdoc_types::TraitAlias (Struct)
-------------------------------------
A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

## Item: rustdoc_types::Function (Struct)
-----------------------------------
A function declaration (including methods and other associated functions).

## Item: Unknown Path (ID: Id(1236)) (Struct Field)
---------------------------------------------
The left side of the equation.

## Item: Unknown Path (ID: Id(1478)) (Struct Field)
---------------------------------------------
The path of the type.

This will be the path that is *used* (not where it is defined), so
multiple `Path`s may have different values for this field even if
they all refer to the same item. e.g.

```rust
pub type Vec1 = std::vec::Vec<i32>; // path: "std::vec::Vec"
pub type Vec2 = Vec<i32>; // path: "Vec"
pub type Vec3 = std::prelude::v1::Vec<i32>; // path: "std::prelude::v1::Vec"
```

## Item: rustdoc_types::Type::FunctionPointer (Variant)
-------------------------------------------------
A function pointer type, e.g. `fn(u32) -> u32`, `extern "C" fn() -> *const u8`

## Item: Unknown Path (ID: Id(1512)) (Struct Field)
---------------------------------------------
Used for Higher-Rank Trait Bounds (HRTBs)

```ignore (incomplete expression)
   for<'c> fn(val: &'c i32) -> i32
// ^^^^^^^
```

## Item: rustdoc_types::Abi::C (Variant)
----------------------------------
Can be specified as `extern "C"` or, as a shorthand, just `extern`.

## Item: rustdoc_types::ItemKind::ExternType (Variant)
------------------------------------------------
`type`s from an `extern` block.

See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

## Item: rustdoc_types::Module (Struct)
---------------------------------
A module declaration, e.g. `mod foo;` or `mod foo {}`.

## Item: rustdoc_types::GenericArg::Lifetime (Variant)
------------------------------------------------
A lifetime argument.
```text
std::borrow::Cow<'static, str>
                 ^^^^^^^
```

## Item: Unknown Path (ID: Id(694)) (Struct Field)
--------------------------------------------
The generic parameters and where clauses on ahis associated type.

## Item: Unknown Path (ID: Id(1577)) (Struct Field)
---------------------------------------------
Whether the trait is marked `auto` and is thus implemented automatically
for all applicable types.

## Item: Unknown Path (ID: Id(3)) (Struct Field)
------------------------------------------
The id of the root [`Module`] item of the local crate.

## Item: rustdoc_types::Deprecation (Struct)
--------------------------------------
Information about the deprecation of an [`Item`].

## Item: rustdoc_types::Visibility (Enum)
-----------------------------------
Visibility of an [`Item`].

## Item: rustdoc_types::ItemEnum::Module (Variant)
--------------------------------------------
A module declaration, e.g. `mod foo;` or `mod foo {}`

## Item: Unknown Path (ID: Id(798)) (Struct Field)
--------------------------------------------
The generic parameters and where clauses on this struct.

## Item: rustdoc_types::GenericArgs::AngleBracketed (Variant)
-------------------------------------------------------
`<'a, 32, B: Copy, C = u32>`

## Item: Unknown Path (ID: Id(866)) (Struct Field)
--------------------------------------------
Information about the type parameters and `where` clauses of the enum.

## Item: Unknown Path (ID: Id(1005)) (Struct Field)
---------------------------------------------
Is this function unsafe?

## Item: Unknown Path (ID: Id(1686)) (Struct Field)
---------------------------------------------
May be different from the last segment of `source` when renaming imports:
`use source as name;`

## Item: rustdoc_types::ItemEnum::StructField (Variant)
-------------------------------------------------
A field of a struct.

## Item: rustdoc_types::ItemKind (Enum)
---------------------------------
The fundamental kind of an item. Unlike [`ItemEnum`], this does not carry any additional info.

Part of [`ItemSummary`].

## Item: Unknown Path (ID: Id(1513)) (Struct Field)
---------------------------------------------
The core properties of the function, such as the ABI it conforms to, whether it's unsafe, etc.

## Item: rustdoc_types::MacroKind::Bang (Variant)
-------------------------------------------
A bang macro `foo!()`.

## Item: rustdoc_types::PreciseCapturingArg::Param (Variant)
------------------------------------------------------
A type or constant parameter.
```rust
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
//                                                            ^  ^

## Item: Unknown Path (ID: Id(483)) (Struct Field)
--------------------------------------------
Arguments provided to the associated type/constant.

## Item: Unknown Path (ID: Id(696)) (Struct Field)
--------------------------------------------
The bounds for this associated type. e.g.
```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
//                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
}
```

## Item: rustdoc_types::Type::Infer (Variant)
---------------------------------------
A type that's left to be inferred, `_`

## Item: Unknown Path (ID: Id(765)) (Struct Field)
--------------------------------------------
All impls (both of traits and inherent) for this union.

All of the corresponding [`Item`]s are of kind [`ItemEnum::Impl`].

## Item: rustdoc_types::WherePredicate::LifetimePredicate (Variant)
-------------------------------------------------------------
A lifetime is expected to outlive other lifetimes.

## Item: rustdoc_types::Enum (Struct)
-------------------------------
An `enum`.

## Item: Unknown Path (ID: Id(0)) (Struct Field)
------------------------------------------
A single version number to be used in the future when making backwards incompatible changes
to the JSON output.

## Item: rustdoc_types::Static (Struct)
---------------------------------
A `static` declaration.

## Item: Unknown Path (ID: Id(1436)) (Struct Field)
---------------------------------------------
This is `true` for `*mut _` and `false` for `*const _`.

## Item: Unknown Path (ID: Id(166)) (Struct Field)
--------------------------------------------
This mapping resolves [intra-doc links](https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md) from the docstring to their IDs

## Item: Unknown Path (ID: Id(126)) (Struct Field)
--------------------------------------------
Whether this item is a struct, trait, macro, etc.

## Item: Unknown Path (ID: Id(1579)) (Struct Field)
---------------------------------------------
Whether the trait is [dyn compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)[^1].

[^1]: Formerly known as "object safe".

## Item: Unknown Path (ID: Id(170)) (Struct Field)
--------------------------------------------
The type-specific fields describing this item.

## Item: rustdoc_types::FunctionSignature (Struct)
--------------------------------------------
The signature of a function.

## Item: Unknown Path (ID: Id(1234)) (Struct Field)
---------------------------------------------
The lifetimes that must be encompassed by the lifetime.

## Item: rustdoc_types::VariantKind::Struct (Variant)
-----------------------------------------------
A variant with named fields.

```rust
enum Demo {
    StructVariant { x: i32 },
    EmptyStructVariant {},
}
```

## Item: Unknown Path (ID: Id(1650)) (Struct Field)
---------------------------------------------
The type that the impl block is for.

## Item: rustdoc_types::PreciseCapturingArg (Enum)
--------------------------------------------
One precise capturing argument. See [the rust reference](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing).

## Item: Unknown Path (ID: Id(1787)) (Struct Field)
---------------------------------------------
The type referred to by this alias.

## Item: Unknown Path (ID: Id(731)) (Struct Field)
--------------------------------------------
If `true`, this module is not part of the public API, but it contains
items that are re-exported as public API.

## Item: rustdoc_types::Visibility::Public (Variant)
----------------------------------------------
Explicitly public visibility set with `pub`.

## Item: Unknown Path (ID: Id(1192)) (Struct Field)
---------------------------------------------
Bounds applied directly to the type. Note that the bounds from `where` clauses
that constrain this parameter won't appear here.

```rust
fn default2<T: Default>() -> [T; 2] where T: Clone { todo!() }
//             ^^^^^^^
```

## Item: rustdoc_types::Crate (Struct)
--------------------------------
The root of the emitted JSON blob.

It contains all type/documentation information
about the language items in the local crate, as well as info about external items to allow
tools to find or link to them.

## Item: Unknown Path (ID: Id(41)) (Function)
---------------------------------------
Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
<code>[From]&lt;T&gt; for U</code> chooses to do.

## Item: rustdoc_types::ItemKind::Keyword (Variant)
---------------------------------------------
A keyword declaration.

[`Item`]s of this kind only come from the come library and exist solely
to carry documentation for the respective keywords.

## Item: Unknown Path (ID: Id(1648)) (Struct Field)
---------------------------------------------
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

## Item: rustdoc_types::WherePredicate::EqPredicate (Variant)
-------------------------------------------------------
A type must exactly equal another type.

## Item: Unknown Path (ID: Id(1819)) (Struct Field)
---------------------------------------------
The type of the static.

## Item: rustdoc_types::GenericArg::Const (Variant)
---------------------------------------------
A constant as a generic argument.
```text
core::array::IntoIter<u32, { 640 * 1024 }>
                           ^^^^^^^^^^^^^^
```

## Item: rustdoc_types::ItemEnum (Enum)
---------------------------------
Specific fields of an item.

Part of [`Item`].

## Item: rustdoc_types::ItemSummary (Struct)
--------------------------------------
Information about an external (not defined in the local crate) [`Item`].

For external items, you don't get the same level of
information. This struct should contain enough to generate a link/reference to the item in
question, or can be used by a tool that takes the json output of multiple crates to find
the actual item definition with all the relevant info.

## Item: rustdoc_types::Type::Tuple (Variant)
---------------------------------------
A tuple type, e.g. `(String, u32, Box<usize>)`

## Item: Unknown Path (ID: Id(1652)) (Struct Field)
---------------------------------------------
Whether this is a negative impl (e.g. `!Sized` or `!Send`).

## Item: Unknown Path (ID: Id(1820)) (Struct Field)
---------------------------------------------
This is `true` for mutable statics, declared as `static mut X: T = f();`

## Item: rustdoc_types::WherePredicate (Enum)
---------------------------------------
One `where` clause.
```rust
fn default<T>() -> T where T: Default { T::default() }
//                         ^^^^^^^^^^
```

## Item: Unknown Path (ID: Id(1546)) (Struct Field)
---------------------------------------------
Whether the function accepts an arbitrary amount of trailing arguments the C way.

```ignore (incomplete code)
fn printf(fmt: &str, ...);
```

## Item: rustdoc_types::ItemKind::Function (Variant)
----------------------------------------------
A function declaration, e.g. `fn f() {}`

## Item: rustdoc_types::ItemKind::Variant (Variant)
---------------------------------------------
A variant of a enum.

## Item: rustdoc_types::ItemKind::Constant (Variant)
----------------------------------------------
The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

## Item: Unknown Path (ID: Id(269)) (Struct Field)
--------------------------------------------
ID of the module to which this visibility restricts items.

## Item: Unknown Path (ID: Id(1479)) (Struct Field)
---------------------------------------------
The ID of the type.

## Item: rustdoc_types::GenericArg::Infer (Variant)
---------------------------------------------
A generic argument that's explicitly set to be inferred.
```text
std::vec::Vec::<_>::new()
                ^
```

## Item: Unknown Path (ID: Id(160)) (Struct Field)
--------------------------------------------
Some items such as impls don't have names.

## Item: Unknown Path (ID: Id(642)) (Struct Field)
--------------------------------------------
If the crate is renamed, this is its name in the crate.

## Item: Unknown Path (ID: Id(200)) (Struct Field)
--------------------------------------------
The path to the source file for this span relative to the path `rustdoc` was invoked with.

## Item: rustdoc_types::Variant (Struct)
----------------------------------
A variant of an enum.

## Item: Unknown Path (ID: Id(1007)) (Struct Field)
---------------------------------------------
The ABI used by the function.

## Item: Unknown Path (ID: Id(1091)) (Struct Field)
---------------------------------------------
Information about the function’s type parameters and `where` clauses.

## Item: rustdoc_types::Type (Enum)
-----------------------------
A type.

## Item: Unknown Path (ID: Id(1651)) (Struct Field)
---------------------------------------------
The list of associated items contained in this impl block.

## Item: rustdoc_types::PolyTrait (Struct)
------------------------------------
A trait and potential HRTBs

## Item: Unknown Path (ID: Id(44)) (Function)
---------------------------------------
Returns the argument unchanged.

## Item: Unknown Path (ID: Id(168)) (Struct Field)
--------------------------------------------
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

## Item: rustdoc_types::GenericBound::Outlives (Variant)
--------------------------------------------------
A lifetime bound, e.g.
```rust
fn f<'a, T>(x: &'a str, y: &T) where T: 'a {}
//                                     ^^^
```

## Item: Unknown Path (ID: Id(484)) (Struct Field)
--------------------------------------------
The kind of bound applied to the associated type/constant.

## Item: rustdoc_types::ItemKind::ExternCrate (Variant)
-------------------------------------------------
A crate imported via the `extern crate` syntax.

## Item: Unknown Path (ID: Id(1582)) (Struct Field)
---------------------------------------------
Constraints that must be met by the implementor of the trait.

## Item: rustdoc_types::GenericParamDefKind::Const (Variant)
------------------------------------------------------
Denotes a constant parameter.

## Item: Unknown Path (ID: Id(868)) (Struct Field)
--------------------------------------------
The list of variants in the enum.

All of the corresponding [`Item`]s are of kind [`ItemEnum::Variant`]

## Item: Unknown Path (ID: Id(1821)) (Struct Field)
---------------------------------------------
The stringified expression for the initial value.

It's not guaranteed that it'll match the actual source code for the initial value.

## Item: rustdoc_types::Term (Enum)
-----------------------------
Either a type or a constant, usually stored as the right-hand side of an equation in places like
[`AssocItemConstraint`]

## Item: Unknown Path (ID: Id(1615)) (Struct Field)
---------------------------------------------
The bounds that are associated with the alias.

## Item: rustdoc_types::ItemKind::Enum (Variant)
------------------------------------------
An `enum` declaration.

## Item: Unknown Path (ID: Id(202)) (Struct Field)
--------------------------------------------
Zero indexed Line and Column of the first character of the `Span`

## Item: rustdoc_types::ItemEnum::TypeAlias (Variant)
-----------------------------------------------
A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

## Item: rustdoc_types::Type::Pat (Variant)
-------------------------------------
A pattern type, e.g. `u32 is 1..`

See [the tracking issue](https://github.com/rust-lang/rust/issues/123646)

## Item: rustdoc_types::Use (Struct)
------------------------------
A `use` statement.

## Item: Unknown Path (ID: Id(1193)) (Struct Field)
---------------------------------------------
The default type for this parameter, if provided, e.g.

```rust
trait PartialEq<Rhs = Self> {}
//                    ^^^^
```

## Item: rustdoc_types::ItemKind::Use (Variant)
-----------------------------------------
An import of 1 or more items into scope, using the `use` keyword.

## Item: Unknown Path (ID: Id(869)) (Struct Field)
--------------------------------------------
`impl`s for the enum.

## Item: Unknown Path (ID: Id(1197)) (Struct Field)
---------------------------------------------
The stringified expression for the default value, if provided. It's not guaranteed that
it'll match the actual source code for the default value.

## Item: rustdoc_types::AssocItemConstraintKind::Equality (Variant)
-------------------------------------------------------------
The required value/type is specified exactly. e.g.
```text
Iterator<Item = u32, IntoIter: DoubleEndedIterator>
         ^^^^^^^^^^
```

## Item: Unknown Path (ID: Id(1190)) (Struct Field)
---------------------------------------------
Lifetimes that this lifetime parameter is required to outlive.

```rust
fn f<'a, 'b, 'resource: 'a + 'b>(a: &'a str, b: &'b str, res: &'resource str) {}
//                      ^^^^^^^
```

## Item: rustdoc_types::GenericBound::Use (Variant)
---------------------------------------------
`use<'a, T>` precise-capturing bound syntax

## Item: rustdoc_types::MacroKind::Derive (Variant)
---------------------------------------------
A derive macro `#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]`

## Item: Unknown Path (ID: Id(1230)) (Struct Field)
---------------------------------------------
The set of bounds that constrain the type.

```rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                                 ^^^^^^^^
```

## Item: Unknown Path (ID: Id(1687)) (Struct Field)
---------------------------------------------
The ID of the item being imported. Will be `None` in case of re-exports of primitives:
```rust
pub use i32 as my_i32;
```

## Item: rustdoc_types::ItemEnum::AssocConst (Variant)
------------------------------------------------
An associated constant of a trait or a type.

## Item: Unknown Path (ID: Id(450)) (Struct Field)
--------------------------------------------
The value of the evaluated expression for this constant, which is only computed for numeric
types.

## Item: rustdoc_types::Type::QualifiedPath (Variant)
-----------------------------------------------
Associated types like `<Type as Trait>::Name` and `T::Item` where
`T: Iterator` or inherent associated types like `Struct::Name`.

## Item: rustdoc_types::Abi::Win64 (Variant)
--------------------------------------
Can be specified as `extern "win64"`.

## Item: rustdoc_types::ItemKind::AssocConst (Variant)
------------------------------------------------
An associated constant of a trait or a type.

## Item: Unknown Path (ID: Id(1158)) (Struct Field)
---------------------------------------------
The kind of the parameter and data specific to a particular parameter kind, e.g. type
bounds.

## Item: Unknown Path (ID: Id(1580)) (Struct Field)
---------------------------------------------
Associated [`Item`]s that can/must be implemented by the `impl` blocks.

## Item: Unknown Path (ID: Id(89)) (Struct Field)
-------------------------------------------
The name of the crate.

Note: This is the [*crate* name][crate-name], which may not be the same as the
[*package* name][package-name]. For example, for <https://crates.io/crates/regex-syntax>,
this field will be `regex_syntax` (which uses an `_`, not a `-`).

[crate-name]: https://doc.rust-lang.org/stable/cargo/reference/cargo-targets.html#the-name-field
[package-name]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-name-field

## Item: Unknown Path (ID: Id(1006)) (Struct Field)
---------------------------------------------
Is this function async?

## Item: Unknown Path (ID: Id(1788)) (Struct Field)
---------------------------------------------
Information about the type parameters and `where` clauses of the alias.

## Item: rustdoc_types::ProcMacro (Struct)
------------------------------------
A procedural macro.

## Item: rustdoc_types::ItemEnum::Macro (Variant)
-------------------------------------------
A macro_rules! declarative macro. Contains a single string with the source
representation of the macro with the patterns stripped.

## Item: Unknown Path (ID: Id(124)) (Struct Field)
--------------------------------------------
The list of path components for the fully qualified path of this item (e.g.
`["std", "io", "lazy", "Lazy"]` for `std::io::lazy::Lazy`).

Note that items can appear in multiple paths, and the one chosen is implementation
defined. Currently, this is the full path to where the item was defined. Eg
[`String`] is currently `["alloc", "string", "String"]` and [`HashMap`][`std::collections::HashMap`]
is `["std", "collections", "hash", "map", "HashMap"]`, but this is subject to change.

## Item: Unknown Path (ID: Id(1444)) (Struct Field)
---------------------------------------------
The generic arguments provided to the associated type.

```ignore (incomplete expression)
<core::slice::IterMut<'static, u32> as BetterIterator>::Item<'static>
//                                                          ^^^^^^^^^
```

## Item: rustdoc_types::TypeAlias (Struct)
------------------------------------
A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

## Item: rustdoc_types::ItemEnum::Struct (Variant)
--------------------------------------------
A `struct` declaration.

## Item: rustdoc_types::ItemKind::TraitAlias (Variant)
------------------------------------------------
A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

## Item: rustdoc_types::ItemEnum::Variant (Variant)
---------------------------------------------
A variant of a enum.

## Item: rustdoc_types::ItemEnum::Union (Variant)
-------------------------------------------
A `union` declaration.

## Item: Unknown Path (ID: Id(1649)) (Struct Field)
---------------------------------------------
The trait being implemented or `None` if the impl is inherent, which means
`impl Struct {}` as opposed to `impl Trait for Struct {}`.

## Item: Unknown Path (ID: Id(729)) (Struct Field)
--------------------------------------------
Whether this is the root item of a crate.

This item doesn't correspond to any construction in the source code and is generated by the
compiler.

## Item: rustdoc_types::Impl (Struct)
-------------------------------
An `impl` block.

## Item: Unknown Path (ID: Id(1445)) (Struct Field)
---------------------------------------------
The type with which this type is associated.

```ignore (incomplete expression)
<core::array::IntoIter<u32, 42> as Iterator>::Item
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

## Item: Unknown Path (ID: Id(1822)) (Struct Field)
---------------------------------------------
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

## Item: Unknown Path (ID: Id(5)) (Struct Field)
------------------------------------------
The version string given to `--crate-version`, if any.

## Item: rustdoc_types::ItemKind::Struct (Variant)
--------------------------------------------
A `struct` declaration.

## Item: Unknown Path (ID: Id(203)) (Struct Field)
--------------------------------------------
Zero indexed Line and Column of the last character of the `Span`

## Item: Unknown Path (ID: Id(1614)) (Struct Field)
---------------------------------------------
Information about the type parameters and `where` clauses of the alias.

## Item: Unknown Path (ID: Id(1581)) (Struct Field)
---------------------------------------------
Information about the type parameters and `where` clauses of the trait.

## Item: rustdoc_types::ItemKind::TypeAlias (Variant)
-----------------------------------------------
A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

## Item: rustdoc_types (Module)
-------------------------
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

## Item: Unknown Path (ID: Id(235)) (Struct Field)
--------------------------------------------
The reason for deprecation and/or what alternatives to use.

## Item: rustdoc_types::ItemEnum::Enum (Variant)
------------------------------------------
An `enum` declaration.

## Item: Unknown Path (ID: Id(676)) (Struct Field)
--------------------------------------------
The type of the constant.

## Item: Unknown Path (ID: Id(1578)) (Struct Field)
---------------------------------------------
Whether the trait is marked as `unsafe`.

## Item: rustdoc_types::Abi (Enum)
----------------------------
The ABI (Application Binary Interface) used by a function.

If a variant has an `unwind` field, this means the ABI that it represents can be specified in 2
ways: `extern "_"` and `extern "_-unwind"`, and a value of `true` for that field signifies the
latter variant.

See the [Rustonomicon section](https://doc.rust-lang.org/nightly/nomicon/ffi.html#ffi-and-unwinding)
on unwinding for more info.

## Item: rustdoc_types::Visibility::Crate (Variant)
---------------------------------------------
Explicitly crate-wide visibility set with `pub(crate)`

## Item: rustdoc_types::Span (Struct)
-------------------------------
A range of source code.

## Item: rustdoc_types::ItemEnum::Function (Variant)
----------------------------------------------
A function declaration (including methods and other associated functions)

## Item: Unknown Path (ID: Id(482)) (Struct Field)
--------------------------------------------
The name of the associated type/constant.

## Item: Unknown Path (ID: Id(1854)) (Struct Field)
---------------------------------------------
The implementations, inherent and of traits, on the primitive type.

## Item: rustdoc_types::ItemKind::ProcDerive (Variant)
------------------------------------------------
A procedural macro usable in the `#[derive()]` attribute.

Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Derive })`

## Item: rustdoc_types::ItemKind::Module (Variant)
--------------------------------------------
A module declaration, e.g. `mod foo;` or `mod foo {}`

## Item: rustdoc_types::TraitBoundModifier (Enum)
-------------------------------------------
A set of modifiers applied to a trait.

## Item: rustdoc_types::ItemKind::Macro (Variant)
-------------------------------------------
A macro declaration.

Corresponds to either `ItemEnum::Macro(_)`
or `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Bang })`

## Item: Unknown Path (ID: Id(1093)) (Struct Field)
---------------------------------------------
Whether the function has a body, i.e. an implementation.

## Item: Unknown Path (ID: Id(1430)) (Struct Field)
---------------------------------------------
The base type, e.g. the `u32` in `u32 is 1..`

## Item: rustdoc_types::ItemEnum::ProcMacro (Variant)
-----------------------------------------------
A procedural macro.

## Item: Unknown Path (ID: Id(938)) (Struct Field)
--------------------------------------------
Whether any variants have been removed from the result, due to being private or hidden.

## Item: rustdoc_types::ItemKind::Primitive (Variant)
-----------------------------------------------
A primitive type, e.g. `u32`.

[`Item`]s of this kind only come from the core library.

## Item: Unknown Path (ID: Id(336)) (Struct Field)
--------------------------------------------
The path to the trait.

## Item: Unknown Path (ID: Id(1269)) (Struct Field)
---------------------------------------------
The full path to the trait.

## Item: rustdoc_types::FunctionHeader (Struct)
-----------------------------------------
A set of fundamental properties of a function.

## Item: Unknown Path (ID: Id(763)) (Struct Field)
--------------------------------------------
Whether any fields have been removed from the result, due to being private or hidden.

## Item: Unknown Path (ID: Id(270)) (Struct Field)
--------------------------------------------
The path with which [`parent`] was referenced
(like `super::super` or `crate::foo::bar`).

[`parent`]: Visibility::Restricted::parent

## Item: rustdoc_types::GenericArgs::Parenthesized (Variant)
------------------------------------------------------
`Fn(A, B) -> C`

## Item: Unknown Path (ID: Id(1196)) (Struct Field)
---------------------------------------------
The type of the constant as declared.

## Item: Unknown Path (ID: Id(9)) (Struct Field)
------------------------------------------
A collection of all items in the local crate as well as some external traits and their
items that are referenced locally.

## Item: Unknown Path (ID: Id(304)) (Struct Field)
--------------------------------------------
The lifetime of the whole dyn object
```text
dyn Debug + 'static
            ^^^^^^^
            |
            this part
```

## Item: rustdoc_types::ItemEnum::ExternCrate (Variant)
-------------------------------------------------
A crate imported via the `extern crate` syntax.

## Item: rustdoc_types::Abi::Cdecl (Variant)
--------------------------------------
Can be specified as `extern "cdecl"`.

## Item: rustdoc_types::AssocItemConstraintKind (Enum)
------------------------------------------------
The way in which an associate type/constant is bound.

## Item: Unknown Path (ID: Id(677)) (Struct Field)
--------------------------------------------
The declared constant itself.

## Item: rustdoc_types::FunctionPointer (Struct)
------------------------------------------
A type that is a function pointer.

## Item: Unknown Path (ID: Id(796)) (Struct Field)
--------------------------------------------
The kind of the struct (e.g. unit, tuple-like or struct-like) and the data specific to it,
i.e. fields.

## Item: Unknown Path (ID: Id(1237)) (Struct Field)
---------------------------------------------
The right side of the equation.

## Item: rustdoc_types::GenericArgs (Enum)
------------------------------------
A set of generic arguments provided to a path segment, e.g.

```text
std::option::Option::<u32>::None
                     ^^^^^
```

## Item: rustdoc_types::Path (Struct)
-------------------------------
A type that has a simple path to it. This is the kind of type of structs, unions, enums, etc.

## Item: rustdoc_types::StructKind (Enum)
-----------------------------------
The kind of a [`Struct`] and the data specific to it, i.e. fields.

## Item: Unknown Path (ID: Id(1271)) (Struct Field)
---------------------------------------------
The context for which a trait is supposed to be used, e.g. `const

## Item: Unknown Path (ID: Id(375)) (Struct Field)
--------------------------------------------
The input types, enclosed in parentheses.

## Item: Unknown Path (ID: Id(451)) (Struct Field)
--------------------------------------------
Whether this constant is a bool, numeric, string, or char literal.

## Item: Unknown Path (ID: Id(1004)) (Struct Field)
---------------------------------------------
Is this function marked as `const`?

## Item: Unknown Path (ID: Id(163)) (Struct Field)
--------------------------------------------
By default all documented items are public, but you can tell rustdoc to output private items
so this field is needed to differentiate.

## Item: rustdoc_types::Visibility::Default (Variant)
-----------------------------------------------
For the most part items are private by default. The exceptions are associated items of
public traits and variants of public enums.

## Item: rustdoc_types::ItemEnum::ExternType (Variant)
------------------------------------------------
`type`s from an `extern` block.

See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

## Item: Unknown Path (ID: Id(1229)) (Struct Field)
---------------------------------------------
The type that's being constrained.

```rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                              ^
```

## Item: Unknown Path (ID: Id(1231)) (Struct Field)
---------------------------------------------
Used for Higher-Rank Trait Bounds (HRTBs)
```rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                  ^^^^^^^
```

## Item: Unknown Path (ID: Id(1853)) (Struct Field)
---------------------------------------------
The name of the type.

## Item: rustdoc_types::Type::ImplTrait (Variant)
-------------------------------------------
An opaque type that satisfies a set of bounds, `impl TraitA + TraitB + ...`

## Item: Unknown Path (ID: Id(1124)) (Struct Field)
---------------------------------------------
A list of generic parameter definitions (e.g. `<T: Clone + Hash, U: Copy>`).

## Item: Unknown Path (ID: Id(937)) (Struct Field)
--------------------------------------------
The list of variants in the enum.
All of the corresponding [`Item`]s are of kind [`ItemEnum::Variant`].

## Item: rustdoc_types::Type::ResolvedPath (Variant)
----------------------------------------------
Structs, enums, unions and type aliases, e.g. `std::option::Option<u32>`

## Item: Unknown Path (ID: Id(302)) (Struct Field)
--------------------------------------------
All the traits implemented. One of them is the vtable, and the rest must be auto traits.

## Item: Unknown Path (ID: Id(691)) (Struct Field)
--------------------------------------------
The type of the constant.

## Item: Unknown Path (ID: Id(1233)) (Struct Field)
---------------------------------------------
The name of the lifetime.

## Item: Unknown Path (ID: Id(1480)) (Struct Field)
---------------------------------------------
Generic arguments to the type.

```ignore (incomplete expression)
std::borrow::Cow<'static, str>
//              ^^^^^^^^^^^^^^
```

## Item: rustdoc_types::Type::Generic (Variant)
-----------------------------------------
Parameterized types. The contained string is the name of the parameter.

## Item: Unknown Path (ID: Id(1511)) (Struct Field)
---------------------------------------------
The signature of the function.

## Item: Unknown Path (ID: Id(641)) (Struct Field)
--------------------------------------------
The name of the imported crate.

## Item: rustdoc_types::ItemEnum::Impl (Variant)
------------------------------------------
An `impl` block.

## Item: rustdoc_types::Primitive (Struct)
------------------------------------
A primitive type declaration. Declarations of this kind can only come from the core library.

## Item: Unknown Path (ID: Id(1125)) (Struct Field)
---------------------------------------------
A list of where predicates (e.g. `where T: Iterator, T::Item: Copy`).

## Item: Unknown Path (ID: Id(692)) (Struct Field)
--------------------------------------------
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

## Item: rustdoc_types::Trait (Struct)
--------------------------------
A `trait` declaration.

## Item: Unknown Path (ID: Id(762)) (Struct Field)
--------------------------------------------
The generic parameters and where clauses on this union.

## Item: rustdoc_types::Struct (Struct)
---------------------------------
A `struct`.

## Item: Unknown Path (ID: Id(1439)) (Struct Field)
---------------------------------------------
The name of the lifetime of the reference, if provided.

## Item: rustdoc_types::PreciseCapturingArg::Lifetime (Variant)
---------------------------------------------------------
A lifetime.
```rust
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
//                                                        ^^

## Item: rustdoc_types::ItemEnum::Constant (Variant)
----------------------------------------------
The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

## Item: Unknown Path (ID: Id(970)) (Struct Field)
--------------------------------------------
The expression that produced the discriminant.

Unlike `value`, this preserves the original formatting (eg suffixes,
hexadecimal, and underscores), making it unsuitable to be machine
interpreted.

In some cases, when the value is too complex, this may be `"{ _ }"`.
When this occurs is unstable, and may change without notice.

## Item: Unknown Path (ID: Id(1270)) (Struct Field)
---------------------------------------------
Used for Higher-Rank Trait Bounds (HRTBs)
```text
where F: for<'a, 'b> Fn(&'a u8, &'b u8)
         ^^^^^^^^^^^
         |
         this part
```

## Item: rustdoc_types::MacroKind (Enum)
----------------------------------
The way a [`ProcMacro`] is declared to be used.

## Item: Unknown Path (ID: Id(90)) (Struct Field)
-------------------------------------------
The root URL at which the crate's documentation lives.

## Item: Unknown Path (ID: Id(833)) (Struct Field)
--------------------------------------------
The list of fields in the struct.

All of the corresponding [`Item`]s are of kind [`ItemEnum::StructField`].

## Item: rustdoc_types::Abi::Stdcall (Variant)
----------------------------------------
Can be specified as `extern "stdcall"`.

## Item: Unknown Path (ID: Id(1446)) (Struct Field)
---------------------------------------------
`None` iff this is an *inherent* associated type.

## Item: rustdoc_types::MacroKind::Attr (Variant)
-------------------------------------------
An attribute macro `#[foo]`.

## Item: Unknown Path (ID: Id(799)) (Struct Field)
--------------------------------------------
All impls (both of traits and inherent) for this struct.
All of the corresponding [`Item`]s are of kind [`ItemEnum::Impl`].

## Item: Unknown Path (ID: Id(1583)) (Struct Field)
---------------------------------------------
The implementations of the trait.

## Item: Unknown Path (ID: Id(14)) (Struct Field)
-------------------------------------------
Maps `crate_id` of items to a crate name and html_root_url if it exists.

## Item: rustdoc_types::Visibility::Restricted (Variant)
--------------------------------------------------
For `pub(in path)` visibility.

## Item: Unknown Path (ID: Id(973)) (Struct Field)
--------------------------------------------
The numerical value of the discriminant. Stored as a string due to
JSON's poor support for large integers, and the fact that it would need
to store from [`i128::MIN`] to [`u128::MAX`].

## Item: rustdoc_types::ItemKind::Union (Variant)
-------------------------------------------
A `union` declaration.

## Item: rustdoc_types::StructKind::Tuple (Variant)
---------------------------------------------
A struct with unnamed fields.

All [`Id`]'s will point to [`ItemEnum::StructField`].
Unlike most of JSON, private and `#[doc(hidden)]` fields will be given as `None`
instead of being omitted, because order matters.

```rust
pub struct TupleStruct(i32);
pub struct EmptyTupleStruct();
```

## Item: Unknown Path (ID: Id(158)) (Struct Field)
--------------------------------------------
The unique identifier of this item. Can be used to find this item in various mappings.

## Item: rustdoc_types::GenericArg (Enum)
-----------------------------------
One argument in a list of generic arguments to a path segment.

Part of [`GenericArgs`].

## Item: Unknown Path (ID: Id(1437)) (Struct Field)
---------------------------------------------
The type of the pointee.

## Item: Unknown Path (ID: Id(697)) (Struct Field)
--------------------------------------------
Inside a trait declaration, this is the default for the associated type, if provided.
Inside an impl block, this is the type assigned to the associated type, and will always
be present.

```rust
type X = usize;
//       ^^^^^
```

## Item: rustdoc_types::DynTrait (Struct)
-----------------------------------
Dynamic trait object type (`dyn Trait`).

## Item: rustdoc_types::ItemKind::ProcAttribute (Variant)
---------------------------------------------------
A procedural macro attribute.

Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Attr })`

## Item: rustdoc_types::ItemKind::Impl (Variant)
------------------------------------------
An `impl` block.

## Item: rustdoc_types::Type::BorrowedRef (Variant)
---------------------------------------------
`&'a mut String`, `&str`, etc.

## Item: rustdoc_types::Discriminant (Struct)
---------------------------------------
The value that distinguishes a variant in an [`Enum`] from other variants.

## Item: rustdoc_types::ItemEnum::AssocType (Variant)
-----------------------------------------------
An associated type of a trait or a type.

## Item: Unknown Path (ID: Id(377)) (Struct Field)
--------------------------------------------
The output type provided after the `->`, if present.

## Item: rustdoc_types::GenericArgs::ReturnTypeNotation (Variant)
-----------------------------------------------------------
`T::method(..)`

## Item: rustdoc_types::VariantKind::Tuple (Variant)
----------------------------------------------
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

## Item: rustdoc_types::Term::Constant (Variant)
------------------------------------------
A constant.

```ignore (incomplete feature in the snippet)
trait Foo {
    const BAR: usize;
}

fn f(x: impl Foo<BAR = 42>) {}
//                     ^^
```

## Item: rustdoc_types::Constant (Struct)
-----------------------------------
A constant.

## Item: rustdoc_types::Generics (Struct)
-----------------------------------
Generic parameters accepted by an item and `where` clauses imposed on it and the parameters.

## Item: rustdoc_types::TraitBoundModifier::MaybeConst (Variant)
----------------------------------------------------------
Indicates that the trait bound must be applicable in both a run-time and a compile-time
context.

## Item: rustdoc_types::StructKind::Plain (Variant)
---------------------------------------------
A struct with named fields.

```rust
pub struct PlainStruct { x: i32 }
pub struct EmptyPlainStruct {}
```

## Item: rustdoc_types::Item (Struct)
-------------------------------
Anything that can hold documentation - modules, structs, enums, functions, traits, etc.

The `Item` data type holds fields that can apply to any of these,
and leaves kind-specific details (like function args or enum variants) to the `inner` field.

## Item: Unknown Path (ID: Id(1647)) (Struct Field)
---------------------------------------------
Information about the impl’s type parameters and `where` clauses.

## Item: rustdoc_types::Term::Type (Variant)
--------------------------------------
A type.

```rust
fn f(x: impl IntoIterator<Item = u32>) {}
//                               ^^^
```

## Item: Unknown Path (ID: Id(1722)) (Struct Field)
---------------------------------------------
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

## Item: rustdoc_types::ItemEnum::Use (Variant)
-----------------------------------------
An import of 1 or more items into scope, using the `use` keyword.

## Item: rustdoc_types::AssocItemConstraintKind::Constraint (Variant)
---------------------------------------------------------------
The type is required to satisfy a set of bounds.
```text
Iterator<Item = u32, IntoIter: DoubleEndedIterator>
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

## Item: rustdoc_types::Type::Slice (Variant)
---------------------------------------
An unsized slice type, e.g. `[u32]`.

## Item: Unknown Path (ID: Id(370)) (Struct Field)
--------------------------------------------
The list of each argument on this type.
```text
<'a, 32, B: Copy, C = u32>
 ^^^^^^
```

## Item: rustdoc_types::ItemKind::Static (Variant)
--------------------------------------------
A `static` declaration.

## Item: rustdoc_types::ItemEnum::Static (Variant)
--------------------------------------------
A declaration of a `static`.

## Item: Unknown Path (ID: Id(12)) (Struct Field)
-------------------------------------------
Maps IDs to fully qualified paths and other info helpful for generating links.

## Item: Unknown Path (ID: Id(8)) (Struct Field)
------------------------------------------
Whether or not the output includes private items.

## Item: Unknown Path (ID: Id(161)) (Struct Field)
--------------------------------------------
The source location of this item (absent if it came from a macro expansion or inline
assembly).

## Item: rustdoc_types::Abi::Rust (Variant)
-------------------------------------
The default ABI, but that can also be written explicitly with `extern "Rust"`.

## Item: rustdoc_types::ItemEnum::Primitive (Variant)
-----------------------------------------------
A primitive type, e.g. `u32`.

[`Item`]s of this kind only come from the core library.

## Item: rustdoc_types::Abi::System (Variant)
---------------------------------------
Can be specified as `extern "system"`.

## Item: Unknown Path (ID: Id(372)) (Struct Field)
--------------------------------------------
Associated type or constant bindings (e.g. `Item=i32` or `Item: Clone`) for this type.

## Item: Unknown Path (ID: Id(1653)) (Struct Field)
---------------------------------------------
Whether this is an impl that’s implied by the compiler
(for autotraits, e.g. `Send` or `Sync`).

## Item: rustdoc_types::Type::RawPointer (Variant)
--------------------------------------------
A raw pointer type, e.g. `*mut u32`, `*const u8`, etc.

## Item: rustdoc_types::Type::DynTrait (Variant)
------------------------------------------
Dynamic trait object type (`dyn Trait`).

## Item: Unknown Path (ID: Id(1440)) (Struct Field)
---------------------------------------------
This is `true` for `&mut i32` and `false` for `&i32`

## Item: Unknown Path (ID: Id(1428)) (Struct Field)
---------------------------------------------
The stringified expression that is the length of the array.

Keep in mind that it's not guaranteed to match the actual source code of the expression.

## Item: rustdoc_types::ItemEnum::Trait (Variant)
-------------------------------------------
A `trait` declaration.

## Item: rustdoc_types::GenericParamDef (Struct)
------------------------------------------
One generic parameter accepted by an item.

## Item: Unknown Path (ID: Id(234)) (Struct Field)
--------------------------------------------
Usually a version number when this [`Item`] first became deprecated.

## Item: Unknown Path (ID: Id(730)) (Struct Field)
--------------------------------------------
[`Item`]s declared inside this module.

## Item: rustdoc_types::TraitBoundModifier::None (Variant)
----------------------------------------------------
Marks the absence of a modifier.

## Item: rustdoc_types::Abi::Aapcs (Variant)
--------------------------------------
Can be specified as `extern "aapcs"`.

## Item: Unknown Path (ID: Id(1092)) (Struct Field)
---------------------------------------------
Information about core properties of the function, e.g. whether it's `const`, its ABI, etc.

## Item: Unknown Path (ID: Id(902)) (Struct Field)
--------------------------------------------
The discriminant, if explicitly specified.

## Item: rustdoc_types::ItemEnum::TraitAlias (Variant)
------------------------------------------------
A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

## Item: rustdoc_types::GenericArg::Type (Variant)
--------------------------------------------
A type argument.
```text
std::borrow::Cow<'static, str>
                          ^^^
```

## Item: rustdoc_types::GenericBound (Enum)
-------------------------------------
Either a trait bound or a lifetime bound.

## Item: Unknown Path (ID: Id(1157)) (Struct Field)
---------------------------------------------
Name of the parameter.
```rust
fn f<'resource, Resource>(x: &'resource Resource) {}
//    ^^^^^^^^  ^^^^^^^^
```

## Item: Unknown Path (ID: Id(123)) (Struct Field)
--------------------------------------------
Can be used to look up the name and html_root_url of the crate this item came from in the
`external_crates` map.

## Item: Unknown Path (ID: Id(1194)) (Struct Field)
---------------------------------------------
This is normally `false`, which means that this generic parameter is
declared in the Rust source text.

If it is `true`, this generic parameter has been introduced by the
compiler behind the scenes.

# Example

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

## Item: rustdoc_types::ExternalCrate (Struct)
----------------------------------------
Metadata of a crate, either the same crate on which `rustdoc` was invoked, or its dependency.

## Item: Unknown Path (ID: Id(1443)) (Struct Field)
---------------------------------------------
The name of the associated type in the parent type.

```ignore (incomplete expression)
<core::array::IntoIter<u32, 42> as Iterator>::Item
//                                            ^^^^
```

## Item: rustdoc_types::GenericParamDefKind::Type (Variant)
-----------------------------------------------------
Denotes a type parameter.

## Item: rustdoc_types::Abi::Other (Variant)
--------------------------------------
Any other ABI, including unstable ones.

## Item: rustdoc_types::Id (Struct)
-----------------------------
An opaque identifier for an item.

It can be used to lookup in [`Crate::index`] or [`Crate::paths`] to resolve it
to an [`Item`].

Id's are only valid within a single JSON blob. They cannot be used to
resolve references between the JSON output's for different crates.

Rustdoc makes no guarantees about the inner value of Id's. Applications
should treat them as opaque keys to lookup items, and avoid attempting
to parse them, or otherwise depend on any implementation details.

## Item: rustdoc_types::FORMAT_VERSION (Constant)
-------------------------------------------
The version of JSON output that this crate represents.

This integer is incremented with every breaking change to the API,
and is returned along with the JSON blob as [`Crate::format_version`].
Consuming code should assert that this value matches the format version(s) that it supports.

## Item: rustdoc_types::ItemKind::Trait (Variant)
-------------------------------------------
A `trait` declaration.

## Item: rustdoc_types::TraitBoundModifier::Maybe (Variant)
-----------------------------------------------------
Indicates that the trait bound relaxes a trait bound applied to a parameter by default,
e.g. `T: Sized?`, the `Sized` trait is required for all generic type parameters by default
unless specified otherwise with this modifier.

## Item: rustdoc_types::AssocItemConstraint (Struct)
----------------------------------------------
Describes a bound applied to an associated type/constant.

Example:
```text
IntoIterator<Item = u32, IntoIter: Clone>
             ^^^^^^^^^^  ^^^^^^^^^^^^^^^
```

## Item: Unknown Path (ID: Id(1688)) (Struct Field)
---------------------------------------------
Whether this statement is a wildcard `use`, e.g. `use source::*;`

## Item: Unknown Path (ID: Id(338)) (Struct Field)
--------------------------------------------
Used for Higher-Rank Trait Bounds (HRTBs)
```text
dyn for<'a> Fn() -> &'a i32"
    ^^^^^^^
```

## Item: Unknown Path (ID: Id(165)) (Struct Field)
--------------------------------------------
The full markdown docstring of this item. Absent if there is no documentation at all,
Some("") if there is some documentation but it is empty (EG `#[doc = ""]`).

## Item: Unknown Path (ID: Id(1089)) (Struct Field)
---------------------------------------------
Information about the function signature, or declaration.

## Item: Unknown Path (ID: Id(834)) (Struct Field)
--------------------------------------------
Whether any fields have been removed from the result, due to being private or hidden.

## Item: rustdoc_types::WherePredicate::BoundPredicate (Variant)
----------------------------------------------------------
A type is expected to comply with a set of bounds

## Item: Unknown Path (ID: Id(1719)) (Struct Field)
---------------------------------------------
How this macro is supposed to be called: `foo!()`, `#[foo]` or `#[derive(foo)]`

## Item: Unknown Path (ID: Id(449)) (Struct Field)
--------------------------------------------
The stringified expression of this constant. Note that its mapping to the original
source code is unstable and it's not guaranteed that it'll match the source code.

## Item: Unknown Path (ID: Id(764)) (Struct Field)
--------------------------------------------
The list of fields in the union.

All of the corresponding [`Item`]s are of kind [`ItemEnum::StructField`].

## Item: Unknown Path (ID: Id(1646)) (Struct Field)
---------------------------------------------
Whether this impl is for an unsafe trait.

## Item: rustdoc_types::Abi::SysV64 (Variant)
---------------------------------------
Can be specified as `extern "sysv64"`.

## Item: Unknown Path (ID: Id(867)) (Struct Field)
--------------------------------------------
Whether any variants have been removed from the result, due to being private or hidden.

## Item: Unknown Path (ID: Id(1545)) (Struct Field)
---------------------------------------------
The output type, if specified.

## Item: rustdoc_types::VariantKind::Plain (Variant)
----------------------------------------------
A variant with no parentheses

```rust
enum Demo {
    PlainVariant,
    PlainWithDiscriminant = 1,
}
```

## Item: rustdoc_types::Type::Primitive (Variant)
-------------------------------------------
Built-in numeric types (e.g. `u32`, `f32`), `bool`, `char`.

## Item: rustdoc_types::GenericParamDefKind::Lifetime (Variant)
---------------------------------------------------------
Denotes a lifetime parameter.

## Item: rustdoc_types::StructKind::Unit (Variant)
--------------------------------------------
A struct with no fields and no parentheses.

```rust
pub struct Unit;
```

## Item: Unknown Path (ID: Id(167)) (Struct Field)
--------------------------------------------
Information about the item’s deprecation, if present.

## Item: rustdoc_types::Abi::Fastcall (Variant)
-----------------------------------------
Can be specified as `extern "fastcall"`.

## Item: rustdoc_types::GenericParamDefKind (Enum)
--------------------------------------------
The kind of a [`GenericParamDef`].

## Item: rustdoc_types::ItemKind::AssocType (Variant)
-----------------------------------------------
An associated type of a trait or a type.

## Item: Unknown Path (ID: Id(1685)) (Struct Field)
---------------------------------------------
The full path being imported.

## Item: Unknown Path (ID: Id(1427)) (Struct Field)
---------------------------------------------
The type of the contained element.

## Item: rustdoc_types::GenericBound::TraitBound (Variant)
----------------------------------------------------
A trait bound.

## Item: rustdoc_types::ItemKind::StructField (Variant)
-------------------------------------------------
A field of a struct.

## Item: Unknown Path (ID: Id(900)) (Struct Field)
--------------------------------------------
Whether the variant is plain, a tuple-like, or struct-like. Contains the fields.

## Item: rustdoc_types::Type::Array (Variant)
---------------------------------------
An array type, e.g. `[u32; 15]`

## Item: rustdoc_types::Union (Struct)
--------------------------------
A `union`.

## Item: Unknown Path (ID: Id(159)) (Struct Field)
--------------------------------------------
This can be used as a key to the `external_crates` map of [`Crate`] to see which crate
this item came from.

--- End of Docstrings (313 items found) ---
