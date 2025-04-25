
# rustdoc_types API (0.39.0)

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


## Modules

### `::` (Crate Root)


#### Structs

##### `struct AssocItemConstraint`

```rust
pub struct AssocItemConstraint {
    pub name: alloc::string::String,
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

###### Fields

####### `name`

The name of the associated type/constant.

####### `args`

Arguments provided to the associated type/constant.

####### `binding`

The kind of bound applied to the associated type/constant.

###### Trait Implementations for `AssocItemConstraint`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::AssocItemConstraint where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::AssocItemConstraint where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::AssocItemConstraint where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::AssocItemConstraint where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::AssocItemConstraint {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::AssocItemConstraint where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::AssocItemConstraint where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::AssocItemConstraint where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::AssocItemConstraint where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::AssocItemConstraint where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::AssocItemConstraint {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Constant`

```rust
pub struct Constant {
    pub expr: alloc::string::String,
    pub value: core::option::Option<alloc::string::String>,
    pub is_literal: bool,
}
```

A constant.

###### Fields

####### `expr`

The stringified expression of this constant. Note that its mapping to the original
source code is unstable and it's not guaranteed that it'll match the source code.

####### `value`

The value of the evaluated expression for this constant, which is only computed for numeric
types.

####### `is_literal`

Whether this constant is a bool, numeric, string, or char literal.

###### Trait Implementations for `Constant`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Constant where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Constant where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Constant where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Constant where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Constant {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Constant where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Constant where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Constant where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Constant where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Constant where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Constant {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Crate`

```rust
pub struct Crate {
    pub root: rustdoc_types::Id,
    pub crate_version: core::option::Option<alloc::string::String>,
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

###### Fields

####### `root`

The id of the root [`Module`] item of the local crate.

####### `crate_version`

The version string given to `--crate-version`, if any.

####### `includes_private`

Whether or not the output includes private items.

####### `index`

A collection of all items in the local crate as well as some external traits and their
items that are referenced locally.

####### `paths`

Maps IDs to fully qualified paths and other info helpful for generating links.

####### `external_crates`

Maps `crate_id` of items to a crate name and html_root_url if it exists.

####### `format_version`

A single version number to be used in the future when making backwards incompatible changes
to the JSON output.

###### Trait Implementations for `Crate`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Crate where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Crate where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Crate where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Crate where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Crate {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Crate where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Crate where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Crate where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Crate where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Crate where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Crate {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Deprecation`

```rust
pub struct Deprecation {
    pub since: core::option::Option<alloc::string::String>,
    pub note: core::option::Option<alloc::string::String>,
}
```

Information about the deprecation of an [`Item`].

###### Fields

####### `since`

Usually a version number when this [`Item`] first became deprecated.

####### `note`

The reason for deprecation and/or what alternatives to use.

###### Trait Implementations for `Deprecation`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Deprecation where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Deprecation where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Deprecation where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Deprecation where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Deprecation {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Deprecation where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Deprecation where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Deprecation where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Deprecation where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Deprecation where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Deprecation {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Discriminant`

```rust
pub struct Discriminant {
    pub expr: alloc::string::String,
    pub value: alloc::string::String,
}
```

The value that distinguishes a variant in an [`Enum`] from other variants.

###### Fields

####### `expr`

The expression that produced the discriminant.

Unlike `value`, this preserves the original formatting (eg suffixes,
hexadecimal, and underscores), making it unsuitable to be machine
interpreted.

In some cases, when the value is too complex, this may be `"{ _ }"`.
When this occurs is unstable, and may change without notice.

####### `value`

The numerical value of the discriminant. Stored as a string due to
JSON's poor support for large integers, and the fact that it would need
to store from [`i128::MIN`] to [`u128::MAX`].

###### Trait Implementations for `Discriminant`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Discriminant where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Discriminant where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Discriminant where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Discriminant where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Discriminant {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Discriminant where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Discriminant where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Discriminant where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Discriminant where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Discriminant where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Discriminant {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct DynTrait`

```rust
pub struct DynTrait {
    pub traits: alloc::vec::Vec<rustdoc_types::PolyTrait>,
    pub lifetime: core::option::Option<alloc::string::String>,
}
```

Dynamic trait object type (`dyn Trait`).

###### Fields

####### `traits`

All the traits implemented. One of them is the vtable, and the rest must be auto traits.

####### `lifetime`

The lifetime of the whole dyn object
```text
dyn Debug + 'static
            ^^^^^^^
            |
            this part
```

###### Trait Implementations for `DynTrait`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::DynTrait where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::DynTrait where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::DynTrait where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::DynTrait where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::DynTrait {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::DynTrait where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::DynTrait where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::DynTrait where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::DynTrait where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::DynTrait where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::DynTrait {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Enum`

```rust
pub struct Enum {
    pub generics: rustdoc_types::Generics,
    pub has_stripped_variants: bool,
    pub variants: alloc::vec::Vec<rustdoc_types::Id>,
    pub impls: alloc::vec::Vec<rustdoc_types::Id>,
}
```

An `enum`.

###### Fields

####### `generics`

Information about the type parameters and `where` clauses of the enum.

####### `has_stripped_variants`

Whether any variants have been removed from the result, due to being private or hidden.

####### `variants`

The list of variants in the enum.

All of the corresponding [`Item`]s are of kind [`ItemEnum::Variant`]

####### `impls`

`impl`s for the enum.

###### Trait Implementations for `Enum`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Enum where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Enum where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Enum where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Enum where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Enum {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Enum where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Enum where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Enum where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Enum where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Enum where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Enum {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct ExternalCrate`

```rust
pub struct ExternalCrate {
    pub name: alloc::string::String,
    pub html_root_url: core::option::Option<alloc::string::String>,
}
```

Metadata of a crate, either the same crate on which `rustdoc` was invoked, or its dependency.

###### Fields

####### `name`

The name of the crate.

Note: This is the [*crate* name][crate-name], which may not be the same as the
[*package* name][package-name]. For example, for <https://crates.io/crates/regex-syntax>,
this field will be `regex_syntax` (which uses an `_`, not a `-`).

[crate-name]: https://doc.rust-lang.org/stable/cargo/reference/cargo-targets.html#the-name-field
[package-name]: https://doc.rust-lang.org/stable/cargo/reference/manifest.html#the-name-field

####### `html_root_url`

The root URL at which the crate's documentation lives.

###### Trait Implementations for `ExternalCrate`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::ExternalCrate where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::ExternalCrate where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::ExternalCrate where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::ExternalCrate where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::ExternalCrate {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::ExternalCrate where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::ExternalCrate where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::ExternalCrate where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::ExternalCrate where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::ExternalCrate where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::ExternalCrate {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Function`

```rust
pub struct Function {
    pub sig: rustdoc_types::FunctionSignature,
    pub generics: rustdoc_types::Generics,
    pub header: rustdoc_types::FunctionHeader,
    pub has_body: bool,
}
```

A function declaration (including methods and other associated functions).

###### Fields

####### `sig`

Information about the function signature, or declaration.

####### `generics`

Information about the function’s type parameters and `where` clauses.

####### `header`

Information about core properties of the function, e.g. whether it's `const`, its ABI, etc.

####### `has_body`

Whether the function has a body, i.e. an implementation.

###### Trait Implementations for `Function`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Function where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Function where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Function where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Function where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Function {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Function where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Function where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Function where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Function where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Function where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Function {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct FunctionHeader`

```rust
pub struct FunctionHeader {
    pub is_const: bool,
    pub is_unsafe: bool,
    pub is_async: bool,
    pub abi: rustdoc_types::Abi,
}
```

A set of fundamental properties of a function.

###### Fields

####### `is_const`

Is this function marked as `const`?

####### `is_unsafe`

Is this function unsafe?

####### `is_async`

Is this function async?

####### `abi`

The ABI used by the function.

###### Trait Implementations for `FunctionHeader`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::FunctionHeader where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::FunctionHeader where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::FunctionHeader where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::FunctionHeader where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::FunctionHeader {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::FunctionHeader where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::FunctionHeader where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::FunctionHeader where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::FunctionHeader where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::FunctionHeader where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::FunctionHeader {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct FunctionPointer`

```rust
pub struct FunctionPointer {
    pub sig: rustdoc_types::FunctionSignature,
    pub generic_params: alloc::vec::Vec<rustdoc_types::GenericParamDef>,
    pub header: rustdoc_types::FunctionHeader,
}
```

A type that is a function pointer.

###### Fields

####### `sig`

The signature of the function.

####### `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)

```ignore (incomplete expression)
   for<'c> fn(val: &'c i32) -> i32
// ^^^^^^^
```

####### `header`

The core properties of the function, such as the ABI it conforms to, whether it's unsafe, etc.

###### Trait Implementations for `FunctionPointer`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::FunctionPointer where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::FunctionPointer where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::FunctionPointer where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::FunctionPointer where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::FunctionPointer {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::FunctionPointer where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::FunctionPointer where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::FunctionPointer where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::FunctionPointer where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::FunctionPointer where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::FunctionPointer {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct FunctionSignature`

```rust
pub struct FunctionSignature {
    pub inputs: alloc::vec::Vec<(alloc::string::String, rustdoc_types::Type)>,
    pub output: core::option::Option<rustdoc_types::Type>,
    pub is_c_variadic: bool,
}
```

The signature of a function.

###### Fields

####### `inputs`

List of argument names and their type.

Note that not all names will be valid identifiers, as some of
them may be patterns.

####### `output`

The output type, if specified.

####### `is_c_variadic`

Whether the function accepts an arbitrary amount of trailing arguments the C way.

```ignore (incomplete code)
fn printf(fmt: &str, ...);
```

###### Trait Implementations for `FunctionSignature`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::FunctionSignature where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::FunctionSignature where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::FunctionSignature where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::FunctionSignature where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::FunctionSignature {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::FunctionSignature where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::FunctionSignature where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::FunctionSignature where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::FunctionSignature where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::FunctionSignature where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::FunctionSignature {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct GenericParamDef`

```rust
pub struct GenericParamDef {
    pub name: alloc::string::String,
    pub kind: rustdoc_types::GenericParamDefKind,
}
```

One generic parameter accepted by an item.

###### Fields

####### `name`

Name of the parameter.
```rust
fn f<'resource, Resource>(x: &'resource Resource) {}
//    ^^^^^^^^  ^^^^^^^^
```

####### `kind`

The kind of the parameter and data specific to a particular parameter kind, e.g. type
bounds.

###### Trait Implementations for `GenericParamDef`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::GenericParamDef where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::GenericParamDef where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::GenericParamDef where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::GenericParamDef where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::GenericParamDef {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::GenericParamDef where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::GenericParamDef where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::GenericParamDef where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::GenericParamDef where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::GenericParamDef where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::GenericParamDef {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Generics`

```rust
pub struct Generics {
    pub params: alloc::vec::Vec<rustdoc_types::GenericParamDef>,
    pub where_predicates: alloc::vec::Vec<rustdoc_types::WherePredicate>,
}
```

Generic parameters accepted by an item and `where` clauses imposed on it and the parameters.

###### Fields

####### `params`

A list of generic parameter definitions (e.g. `<T: Clone + Hash, U: Copy>`).

####### `where_predicates`

A list of where predicates (e.g. `where T: Iterator, T::Item: Copy`).

###### Trait Implementations for `Generics`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Generics where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Generics where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Generics where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Generics where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Generics {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Generics where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Generics where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Generics where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Generics where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Generics where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Generics {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Id`

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

###### Trait Implementations for `Id`

- `Copy`
- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Id where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Id where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Id where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Id where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Id {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Id where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Id where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Id where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Id where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Id where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Id {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Impl`

```rust
pub struct Impl {
    pub is_unsafe: bool,
    pub generics: rustdoc_types::Generics,
    pub provided_trait_methods: alloc::vec::Vec<alloc::string::String>,
    pub trait_: core::option::Option<rustdoc_types::Path>,
    pub for_: rustdoc_types::Type,
    pub items: alloc::vec::Vec<rustdoc_types::Id>,
    pub is_negative: bool,
    pub is_synthetic: bool,
    pub blanket_impl: core::option::Option<rustdoc_types::Type>,
}
```

An `impl` block.

###### Fields

####### `is_unsafe`

Whether this impl is for an unsafe trait.

####### `generics`

Information about the impl’s type parameters and `where` clauses.

####### `provided_trait_methods`

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

####### `trait_`

The trait being implemented or `None` if the impl is inherent, which means
`impl Struct {}` as opposed to `impl Trait for Struct {}`.

####### `for_`

The type that the impl block is for.

####### `items`

The list of associated items contained in this impl block.

####### `is_negative`

Whether this is a negative impl (e.g. `!Sized` or `!Send`).

####### `is_synthetic`

Whether this is an impl that’s implied by the compiler
(for autotraits, e.g. `Send` or `Sync`).

###### Trait Implementations for `Impl`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Impl where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Impl where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Impl where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Impl where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Impl {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Impl where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Impl where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Impl where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Impl where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Impl where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Impl {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Item`

```rust
pub struct Item {
    pub id: rustdoc_types::Id,
    pub crate_id: u32,
    pub name: core::option::Option<alloc::string::String>,
    pub span: core::option::Option<rustdoc_types::Span>,
    pub visibility: rustdoc_types::Visibility,
    pub docs: core::option::Option<alloc::string::String>,
    pub links: rustc_hash::FxHashMap<alloc::string::String, rustdoc_types::Id>,
    pub attrs: alloc::vec::Vec<alloc::string::String>,
    pub deprecation: core::option::Option<rustdoc_types::Deprecation>,
    pub inner: rustdoc_types::ItemEnum,
}
```

Anything that can hold documentation - modules, structs, enums, functions, traits, etc.

The `Item` data type holds fields that can apply to any of these,
and leaves kind-specific details (like function args or enum variants) to the `inner` field.

###### Fields

####### `id`

The unique identifier of this item. Can be used to find this item in various mappings.

####### `crate_id`

This can be used as a key to the `external_crates` map of [`Crate`] to see which crate
this item came from.

####### `name`

Some items such as impls don't have names.

####### `span`

The source location of this item (absent if it came from a macro expansion or inline
assembly).

####### `visibility`

By default all documented items are public, but you can tell rustdoc to output private items
so this field is needed to differentiate.

####### `docs`

The full markdown docstring of this item. Absent if there is no documentation at all,
Some("") if there is some documentation but it is empty (EG `#[doc = ""]`).

####### `links`

This mapping resolves [intra-doc links](https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md) from the docstring to their IDs

####### `attrs`

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

####### `deprecation`

Information about the item’s deprecation, if present.

####### `inner`

The type-specific fields describing this item.

###### Trait Implementations for `Item`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Item where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Item where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Item where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Item where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Item {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Item where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Item where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Item where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Item where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Item where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Item {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct ItemSummary`

```rust
pub struct ItemSummary {
    pub crate_id: u32,
    pub path: alloc::vec::Vec<alloc::string::String>,
    pub kind: rustdoc_types::ItemKind,
}
```

Information about an external (not defined in the local crate) [`Item`].

For external items, you don't get the same level of
information. This struct should contain enough to generate a link/reference to the item in
question, or can be used by a tool that takes the json output of multiple crates to find
the actual item definition with all the relevant info.

###### Fields

####### `crate_id`

Can be used to look up the name and html_root_url of the crate this item came from in the
`external_crates` map.

####### `path`

The list of path components for the fully qualified path of this item (e.g.
`["std", "io", "lazy", "Lazy"]` for `std::io::lazy::Lazy`).

Note that items can appear in multiple paths, and the one chosen is implementation
defined. Currently, this is the full path to where the item was defined. Eg
[`String`] is currently `["alloc", "string", "String"]` and [`HashMap`][`std::collections::HashMap`]
is `["std", "collections", "hash", "map", "HashMap"]`, but this is subject to change.

####### `kind`

Whether this item is a struct, trait, macro, etc.

###### Trait Implementations for `ItemSummary`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::ItemSummary where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::ItemSummary where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::ItemSummary where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::ItemSummary where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::ItemSummary {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::ItemSummary where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::ItemSummary where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::ItemSummary where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::ItemSummary where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::ItemSummary where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::ItemSummary {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Module`

```rust
pub struct Module {
    pub is_crate: bool,
    pub items: alloc::vec::Vec<rustdoc_types::Id>,
    pub is_stripped: bool,
}
```

A module declaration, e.g. `mod foo;` or `mod foo {}`.

###### Fields

####### `is_crate`

Whether this is the root item of a crate.

This item doesn't correspond to any construction in the source code and is generated by the
compiler.

####### `items`

[`Item`]s declared inside this module.

####### `is_stripped`

If `true`, this module is not part of the public API, but it contains
items that are re-exported as public API.

###### Trait Implementations for `Module`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Module where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Module where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Module where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Module where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Module {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Module where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Module where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Module where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Module where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Module where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Module {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Path`

```rust
pub struct Path {
    pub path: alloc::string::String,
    pub id: rustdoc_types::Id,
    pub args: core::option::Option<alloc::boxed::Box<rustdoc_types::GenericArgs>>,
}
```

A type that has a simple path to it. This is the kind of type of structs, unions, enums, etc.

###### Fields

####### `path`

The path of the type.

This will be the path that is *used* (not where it is defined), so
multiple `Path`s may have different values for this field even if
they all refer to the same item. e.g.

```rust
pub type Vec1 = std::vec::Vec<i32>; // path: "std::vec::Vec"
pub type Vec2 = Vec<i32>; // path: "Vec"
pub type Vec3 = std::prelude::v1::Vec<i32>; // path: "std::prelude::v1::Vec"
```

####### `id`

The ID of the type.

####### `args`

Generic arguments to the type.

```ignore (incomplete expression)
std::borrow::Cow<'static, str>
//              ^^^^^^^^^^^^^^
```

###### Trait Implementations for `Path`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Path where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Path where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Path where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Path where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Path {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Path where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Path where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Path where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Path where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Path where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Path {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct PolyTrait`

```rust
pub struct PolyTrait {
    pub trait_: rustdoc_types::Path,
    pub generic_params: alloc::vec::Vec<rustdoc_types::GenericParamDef>,
}
```

A trait and potential HRTBs

###### Fields

####### `trait_`

The path to the trait.

####### `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)
```text
dyn for<'a> Fn() -> &'a i32"
    ^^^^^^^
```

###### Trait Implementations for `PolyTrait`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::PolyTrait where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::PolyTrait where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::PolyTrait where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::PolyTrait where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::PolyTrait {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::PolyTrait where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::PolyTrait where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::PolyTrait where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::PolyTrait where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::PolyTrait where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::PolyTrait {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Primitive`

```rust
pub struct Primitive {
    pub name: alloc::string::String,
    pub impls: alloc::vec::Vec<rustdoc_types::Id>,
}
```

A primitive type declaration. Declarations of this kind can only come from the core library.

###### Fields

####### `name`

The name of the type.

####### `impls`

The implementations, inherent and of traits, on the primitive type.

###### Trait Implementations for `Primitive`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Primitive where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Primitive where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Primitive where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Primitive where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Primitive {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Primitive where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Primitive where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Primitive where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Primitive where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Primitive where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Primitive {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct ProcMacro`

```rust
pub struct ProcMacro {
    pub kind: rustdoc_types::MacroKind,
    pub helpers: alloc::vec::Vec<alloc::string::String>,
}
```

A procedural macro.

###### Fields

####### `kind`

How this macro is supposed to be called: `foo!()`, `#[foo]` or `#[derive(foo)]`

####### `helpers`

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

###### Trait Implementations for `ProcMacro`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::ProcMacro where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::ProcMacro where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::ProcMacro where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::ProcMacro where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::ProcMacro {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::ProcMacro where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::ProcMacro where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::ProcMacro where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::ProcMacro where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::ProcMacro where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::ProcMacro {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Span`

```rust
pub struct Span {
    pub filename: std::path::PathBuf,
    pub begin: (usize, usize),
    pub end: (usize, usize),
}
```

A range of source code.

###### Fields

####### `filename`

The path to the source file for this span relative to the path `rustdoc` was invoked with.

####### `begin`

Zero indexed Line and Column of the first character of the `Span`

####### `end`

Zero indexed Line and Column of the last character of the `Span`

###### Trait Implementations for `Span`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Span where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Span where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Span where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Span where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Span {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Span where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Span where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Span where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Span where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Span where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Span {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Static`

```rust
pub struct Static {
    pub type_: rustdoc_types::Type,
    pub is_mutable: bool,
    pub expr: alloc::string::String,
    pub is_unsafe: bool,
}
```

A `static` declaration.

###### Fields

####### `type_`

The type of the static.

####### `is_mutable`

This is `true` for mutable statics, declared as `static mut X: T = f();`

####### `expr`

The stringified expression for the initial value.

It's not guaranteed that it'll match the actual source code for the initial value.

####### `is_unsafe`

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

###### Trait Implementations for `Static`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Static where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Static where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Static where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Static where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Static {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Static where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Static where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Static where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Static where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Static where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Static {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Struct`

```rust
pub struct Struct {
    pub kind: rustdoc_types::StructKind,
    pub generics: rustdoc_types::Generics,
    pub impls: alloc::vec::Vec<rustdoc_types::Id>,
}
```

A `struct`.

###### Fields

####### `kind`

The kind of the struct (e.g. unit, tuple-like or struct-like) and the data specific to it,
i.e. fields.

####### `generics`

The generic parameters and where clauses on this struct.

####### `impls`

All impls (both of traits and inherent) for this struct.
All of the corresponding [`Item`]s are of kind [`ItemEnum::Impl`].

###### Trait Implementations for `Struct`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Struct where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Struct where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Struct where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Struct where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Struct {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Struct where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Struct where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Struct where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Struct where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Struct where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Struct {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Trait`

```rust
pub struct Trait {
    pub is_auto: bool,
    pub is_unsafe: bool,
    pub is_dyn_compatible: bool,
    pub items: alloc::vec::Vec<rustdoc_types::Id>,
    pub generics: rustdoc_types::Generics,
    pub bounds: alloc::vec::Vec<rustdoc_types::GenericBound>,
    pub implementations: alloc::vec::Vec<rustdoc_types::Id>,
}
```

A `trait` declaration.

###### Fields

####### `is_auto`

Whether the trait is marked `auto` and is thus implemented automatically
for all applicable types.

####### `is_unsafe`

Whether the trait is marked as `unsafe`.

####### `is_dyn_compatible`

Whether the trait is [dyn compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)[^1].

[^1]: Formerly known as "object safe".

####### `items`

Associated [`Item`]s that can/must be implemented by the `impl` blocks.

####### `generics`

Information about the type parameters and `where` clauses of the trait.

####### `bounds`

Constraints that must be met by the implementor of the trait.

####### `implementations`

The implementations of the trait.

###### Trait Implementations for `Trait`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Trait where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Trait where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Trait where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Trait where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Trait {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Trait where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Trait where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Trait where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Trait where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Trait where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Trait {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct TraitAlias`

```rust
pub struct TraitAlias {
    pub generics: rustdoc_types::Generics,
    pub params: alloc::vec::Vec<rustdoc_types::GenericBound>,
}
```

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

###### Fields

####### `generics`

Information about the type parameters and `where` clauses of the alias.

####### `params`

The bounds that are associated with the alias.

###### Trait Implementations for `TraitAlias`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::TraitAlias where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::TraitAlias where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::TraitAlias where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::TraitAlias where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::TraitAlias {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::TraitAlias where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::TraitAlias where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::TraitAlias where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::TraitAlias where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::TraitAlias where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::TraitAlias {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct TypeAlias`

```rust
pub struct TypeAlias {
    pub type_: rustdoc_types::Type,
    pub generics: rustdoc_types::Generics,
}
```

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

###### Fields

####### `type_`

The type referred to by this alias.

####### `generics`

Information about the type parameters and `where` clauses of the alias.

###### Trait Implementations for `TypeAlias`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::TypeAlias where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::TypeAlias where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::TypeAlias where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::TypeAlias where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::TypeAlias {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::TypeAlias where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::TypeAlias where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::TypeAlias where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::TypeAlias where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::TypeAlias where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::TypeAlias {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Union`

```rust
pub struct Union {
    pub generics: rustdoc_types::Generics,
    pub has_stripped_fields: bool,
    pub fields: alloc::vec::Vec<rustdoc_types::Id>,
    pub impls: alloc::vec::Vec<rustdoc_types::Id>,
}
```

A `union`.

###### Fields

####### `generics`

The generic parameters and where clauses on this union.

####### `has_stripped_fields`

Whether any fields have been removed from the result, due to being private or hidden.

####### `fields`

The list of fields in the union.

All of the corresponding [`Item`]s are of kind [`ItemEnum::StructField`].

####### `impls`

All impls (both of traits and inherent) for this union.

All of the corresponding [`Item`]s are of kind [`ItemEnum::Impl`].

###### Trait Implementations for `Union`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Union where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Union where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Union where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Union where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Union {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Union where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Union where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Union where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Union where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Union where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Union {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Use`

```rust
pub struct Use {
    pub source: alloc::string::String,
    pub name: alloc::string::String,
    pub id: core::option::Option<rustdoc_types::Id>,
    pub is_glob: bool,
}
```

A `use` statement.

###### Fields

####### `source`

The full path being imported.

####### `name`

May be different from the last segment of `source` when renaming imports:
`use source as name;`

####### `id`

The ID of the item being imported. Will be `None` in case of re-exports of primitives:
```rust
pub use i32 as my_i32;
```

####### `is_glob`

Whether this statement is a wildcard `use`, e.g. `use source::*;`

###### Trait Implementations for `Use`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Use where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Use where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Use where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Use where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Use {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Use where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Use where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Use where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Use where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Use where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Use {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `struct Variant`

```rust
pub struct Variant {
    pub kind: rustdoc_types::VariantKind,
    pub discriminant: core::option::Option<rustdoc_types::Discriminant>,
}
```

A variant of an enum.

###### Fields

####### `kind`

Whether the variant is plain, a tuple-like, or struct-like. Contains the fields.

####### `discriminant`

The discriminant, if explicitly specified.

###### Trait Implementations for `Variant`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Variant where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Variant where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Variant where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Variant where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Variant {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Variant where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Variant where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Variant where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Variant where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Variant where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Variant {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### Enums

##### `enum Abi`

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
    Other(alloc::string::String),
}
```

The ABI (Application Binary Interface) used by a function.

If a variant has an `unwind` field, this means the ABI that it represents can be specified in 2
ways: `extern "_"` and `extern "_-unwind"`, and a value of `true` for that field signifies the
latter variant.

See the [Rustonomicon section](https://doc.rust-lang.org/nightly/nomicon/ffi.html#ffi-and-unwinding)
on unwinding for more info.

###### Variants

####### `Rust`

The default ABI, but that can also be written explicitly with `extern "Rust"`.

####### `C { unwind: bool }`

Can be specified as `extern "C"` or, as a shorthand, just `extern`.

####### `Cdecl { unwind: bool }`

Can be specified as `extern "cdecl"`.

####### `Stdcall { unwind: bool }`

Can be specified as `extern "stdcall"`.

####### `Fastcall { unwind: bool }`

Can be specified as `extern "fastcall"`.

####### `Aapcs { unwind: bool }`

Can be specified as `extern "aapcs"`.

####### `Win64 { unwind: bool }`

Can be specified as `extern "win64"`.

####### `SysV64 { unwind: bool }`

Can be specified as `extern "sysv64"`.

####### `System { unwind: bool }`

Can be specified as `extern "system"`.

####### `Other(alloc::string::String)`

Any other ABI, including unstable ones.

###### Trait Implementations for `Abi`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Abi where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Abi where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Abi where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Abi where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Abi {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Abi where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Abi where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Abi where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Abi where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Abi where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Abi {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum AssocItemConstraintKind`

```rust
pub enum AssocItemConstraintKind {
    Equality(rustdoc_types::Term),
    Constraint(alloc::vec::Vec<rustdoc_types::GenericBound>),
}
```

The way in which an associate type/constant is bound.

###### Variants

####### `Equality(rustdoc_types::Term)`

The required value/type is specified exactly. e.g.
```text
Iterator<Item = u32, IntoIter: DoubleEndedIterator>
         ^^^^^^^^^^
```

####### `Constraint(alloc::vec::Vec<rustdoc_types::GenericBound>)`

The type is required to satisfy a set of bounds.
```text
Iterator<Item = u32, IntoIter: DoubleEndedIterator>
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

###### Trait Implementations for `AssocItemConstraintKind`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::AssocItemConstraintKind where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::AssocItemConstraintKind where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::AssocItemConstraintKind where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::AssocItemConstraintKind where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::AssocItemConstraintKind {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::AssocItemConstraintKind where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::AssocItemConstraintKind where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::AssocItemConstraintKind where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::AssocItemConstraintKind where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::AssocItemConstraintKind where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::AssocItemConstraintKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum GenericArg`

```rust
pub enum GenericArg {
    Lifetime(alloc::string::String),
    Type(rustdoc_types::Type),
    Const(rustdoc_types::Constant),
    Infer,
}
```

One argument in a list of generic arguments to a path segment.

Part of [`GenericArgs`].

###### Variants

####### `Lifetime(alloc::string::String)`

A lifetime argument.
```text
std::borrow::Cow<'static, str>
                 ^^^^^^^
```

####### `Type(rustdoc_types::Type)`

A type argument.
```text
std::borrow::Cow<'static, str>
                          ^^^
```

####### `Const(rustdoc_types::Constant)`

A constant as a generic argument.
```text
core::array::IntoIter<u32, { 640 * 1024 }>
                           ^^^^^^^^^^^^^^
```

####### `Infer`

A generic argument that's explicitly set to be inferred.
```text
std::vec::Vec::<_>::new()
                ^
```

###### Trait Implementations for `GenericArg`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::GenericArg where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::GenericArg where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::GenericArg where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::GenericArg where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::GenericArg {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::GenericArg where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::GenericArg where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::GenericArg where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::GenericArg where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::GenericArg where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::GenericArg {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum GenericArgs`

```rust
pub enum GenericArgs {
    AngleBracketed { args: alloc::vec::Vec<rustdoc_types::GenericArg>, constraints: alloc::vec::Vec<rustdoc_types::AssocItemConstraint> },
    Parenthesized { inputs: alloc::vec::Vec<rustdoc_types::Type>, output: core::option::Option<rustdoc_types::Type> },
    ReturnTypeNotation,
}
```

A set of generic arguments provided to a path segment, e.g.

```text
std::option::Option::<u32>::None
                     ^^^^^
```

###### Variants

####### `AngleBracketed { args: alloc::vec::Vec<rustdoc_types::GenericArg>, constraints: alloc::vec::Vec<rustdoc_types::AssocItemConstraint> }`

`<'a, 32, B: Copy, C = u32>`

####### `Parenthesized { inputs: alloc::vec::Vec<rustdoc_types::Type>, output: core::option::Option<rustdoc_types::Type> }`

`Fn(A, B) -> C`

####### `ReturnTypeNotation`

`T::method(..)`

###### Trait Implementations for `GenericArgs`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::GenericArgs where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::GenericArgs where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::GenericArgs where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::GenericArgs where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::GenericArgs {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::GenericArgs where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::GenericArgs where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::GenericArgs where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::GenericArgs where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::GenericArgs where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::GenericArgs {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum GenericBound`

```rust
pub enum GenericBound {
    TraitBound { trait_: rustdoc_types::Path, generic_params: alloc::vec::Vec<rustdoc_types::GenericParamDef>, modifier: rustdoc_types::TraitBoundModifier },
    Outlives(alloc::string::String),
    Use(alloc::vec::Vec<rustdoc_types::PreciseCapturingArg>),
}
```

Either a trait bound or a lifetime bound.

###### Variants

####### `TraitBound { trait_: rustdoc_types::Path, generic_params: alloc::vec::Vec<rustdoc_types::GenericParamDef>, modifier: rustdoc_types::TraitBoundModifier }`

A trait bound.

####### `Outlives(alloc::string::String)`

A lifetime bound, e.g.
```rust
fn f<'a, T>(x: &'a str, y: &T) where T: 'a {}
//                                     ^^^
```

####### `Use(alloc::vec::Vec<rustdoc_types::PreciseCapturingArg>)`

`use<'a, T>` precise-capturing bound syntax

###### Trait Implementations for `GenericBound`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::GenericBound where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::GenericBound where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::GenericBound where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::GenericBound where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::GenericBound {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::GenericBound where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::GenericBound where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::GenericBound where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::GenericBound where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::GenericBound where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::GenericBound {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum GenericParamDefKind`

```rust
pub enum GenericParamDefKind {
    Lifetime { outlives: alloc::vec::Vec<alloc::string::String> },
    Type { bounds: alloc::vec::Vec<rustdoc_types::GenericBound>, default: core::option::Option<rustdoc_types::Type>, is_synthetic: bool },
    Const { type_: rustdoc_types::Type, default: core::option::Option<alloc::string::String> },
}
```

The kind of a [`GenericParamDef`].

###### Variants

####### `Lifetime { outlives: alloc::vec::Vec<alloc::string::String> }`

Denotes a lifetime parameter.

####### `Type { bounds: alloc::vec::Vec<rustdoc_types::GenericBound>, default: core::option::Option<rustdoc_types::Type>, is_synthetic: bool }`

Denotes a type parameter.

####### `Const { type_: rustdoc_types::Type, default: core::option::Option<alloc::string::String> }`

Denotes a constant parameter.

###### Trait Implementations for `GenericParamDefKind`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::GenericParamDefKind where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::GenericParamDefKind where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::GenericParamDefKind where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::GenericParamDefKind where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::GenericParamDefKind {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::GenericParamDefKind where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::GenericParamDefKind where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::GenericParamDefKind where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::GenericParamDefKind where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::GenericParamDefKind where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::GenericParamDefKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum ItemEnum`

```rust
pub enum ItemEnum {
    Module(rustdoc_types::Module),
    ExternCrate { name: alloc::string::String, rename: core::option::Option<alloc::string::String> },
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
    Macro(alloc::string::String),
    ProcMacro(rustdoc_types::ProcMacro),
    Primitive(rustdoc_types::Primitive),
    AssocConst { type_: rustdoc_types::Type, value: core::option::Option<alloc::string::String> },
    AssocType { generics: rustdoc_types::Generics, bounds: alloc::vec::Vec<rustdoc_types::GenericBound>, type_: core::option::Option<rustdoc_types::Type> },
}
```

Specific fields of an item.

Part of [`Item`].

###### Variants

####### `Module(rustdoc_types::Module)`

A module declaration, e.g. `mod foo;` or `mod foo {}`

####### `ExternCrate { name: alloc::string::String, rename: core::option::Option<alloc::string::String> }`

A crate imported via the `extern crate` syntax.

####### `Use(rustdoc_types::Use)`

An import of 1 or more items into scope, using the `use` keyword.

####### `Union(rustdoc_types::Union)`

A `union` declaration.

####### `Struct(rustdoc_types::Struct)`

A `struct` declaration.

####### `StructField(rustdoc_types::Type)`

A field of a struct.

####### `Enum(rustdoc_types::Enum)`

An `enum` declaration.

####### `Variant(rustdoc_types::Variant)`

A variant of a enum.

####### `Function(rustdoc_types::Function)`

A function declaration (including methods and other associated functions)

####### `Trait(rustdoc_types::Trait)`

A `trait` declaration.

####### `TraitAlias(rustdoc_types::TraitAlias)`

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

####### `Impl(rustdoc_types::Impl)`

An `impl` block.

####### `TypeAlias(rustdoc_types::TypeAlias)`

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

####### `Constant { type_: rustdoc_types::Type, const_: rustdoc_types::Constant }`

The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

####### `Static(rustdoc_types::Static)`

A declaration of a `static`.

####### `ExternType`

`type`s from an `extern` block.

See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

####### `Macro(alloc::string::String)`

A macro_rules! declarative macro. Contains a single string with the source
representation of the macro with the patterns stripped.

####### `ProcMacro(rustdoc_types::ProcMacro)`

A procedural macro.

####### `Primitive(rustdoc_types::Primitive)`

A primitive type, e.g. `u32`.

[`Item`]s of this kind only come from the core library.

####### `AssocConst { type_: rustdoc_types::Type, value: core::option::Option<alloc::string::String> }`

An associated constant of a trait or a type.

####### `AssocType { generics: rustdoc_types::Generics, bounds: alloc::vec::Vec<rustdoc_types::GenericBound>, type_: core::option::Option<rustdoc_types::Type> }`

An associated type of a trait or a type.

###### Trait Implementations for `ItemEnum`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::ItemEnum where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::ItemEnum where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::ItemEnum where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::ItemEnum where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::ItemEnum {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::ItemEnum where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::ItemEnum where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::ItemEnum where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::ItemEnum where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::ItemEnum where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::ItemEnum {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum ItemKind`

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

###### Variants

####### `Module`

A module declaration, e.g. `mod foo;` or `mod foo {}`

####### `ExternCrate`

A crate imported via the `extern crate` syntax.

####### `Use`

An import of 1 or more items into scope, using the `use` keyword.

####### `Struct`

A `struct` declaration.

####### `StructField`

A field of a struct.

####### `Union`

A `union` declaration.

####### `Enum`

An `enum` declaration.

####### `Variant`

A variant of a enum.

####### `Function`

A function declaration, e.g. `fn f() {}`

####### `TypeAlias`

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

####### `Constant`

The declaration of a constant, e.g. `const GREETING: &str = "Hi :3";`

####### `Trait`

A `trait` declaration.

####### `TraitAlias`

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

####### `Impl`

An `impl` block.

####### `Static`

A `static` declaration.

####### `ExternType`

`type`s from an `extern` block.

See [the tracking issue](https://github.com/rust-lang/rust/issues/43467)

####### `Macro`

A macro declaration.

Corresponds to either `ItemEnum::Macro(_)`
or `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Bang })`

####### `ProcAttribute`

A procedural macro attribute.

Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Attr })`

####### `ProcDerive`

A procedural macro usable in the `#[derive()]` attribute.

Corresponds to `ItemEnum::ProcMacro(ProcMacro { kind: MacroKind::Derive })`

####### `AssocConst`

An associated constant of a trait or a type.

####### `AssocType`

An associated type of a trait or a type.

####### `Primitive`

A primitive type, e.g. `u32`.

[`Item`]s of this kind only come from the core library.

####### `Keyword`

A keyword declaration.

[`Item`]s of this kind only come from the come library and exist solely
to carry documentation for the respective keywords.

###### Trait Implementations for `ItemKind`

- `Copy`
- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::ItemKind where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::ItemKind where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::ItemKind where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::ItemKind where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::ItemKind {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::ItemKind where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::ItemKind where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::ItemKind where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::ItemKind where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::ItemKind where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::ItemKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum MacroKind`

```rust
pub enum MacroKind {
    Bang,
    Attr,
    Derive,
}
```

The way a [`ProcMacro`] is declared to be used.

###### Variants

####### `Bang`

A bang macro `foo!()`.

####### `Attr`

An attribute macro `#[foo]`.

####### `Derive`

A derive macro `#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]`

###### Trait Implementations for `MacroKind`

- `Copy`
- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::MacroKind where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::MacroKind where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::MacroKind where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::MacroKind where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::MacroKind {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::MacroKind where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::MacroKind where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::MacroKind where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::MacroKind where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::MacroKind where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::MacroKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum PreciseCapturingArg`

```rust
pub enum PreciseCapturingArg {
    Lifetime(alloc::string::String),
    Param(alloc::string::String),
}
```

One precise capturing argument. See [the rust reference](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing).

###### Variants

####### `Lifetime(alloc::string::String)`

A lifetime.
```rust
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
//                                                        ^^

####### `Param(alloc::string::String)`

A type or constant parameter.
```rust
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
//                                                            ^  ^

###### Trait Implementations for `PreciseCapturingArg`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::PreciseCapturingArg where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::PreciseCapturingArg where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::PreciseCapturingArg where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::PreciseCapturingArg where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::PreciseCapturingArg {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::PreciseCapturingArg where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::PreciseCapturingArg where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::PreciseCapturingArg where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::PreciseCapturingArg where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::PreciseCapturingArg where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::PreciseCapturingArg {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum StructKind`

```rust
pub enum StructKind {
    Unit,
    Tuple(alloc::vec::Vec<core::option::Option<rustdoc_types::Id>>),
    Plain { fields: alloc::vec::Vec<rustdoc_types::Id>, has_stripped_fields: bool },
}
```

The kind of a [`Struct`] and the data specific to it, i.e. fields.

###### Variants

####### `Unit`

A struct with no fields and no parentheses.

```rust
pub struct Unit;
```

####### `Tuple(alloc::vec::Vec<core::option::Option<rustdoc_types::Id>>)`

A struct with unnamed fields.

All [`Id`]'s will point to [`ItemEnum::StructField`].
Unlike most of JSON, private and `#[doc(hidden)]` fields will be given as `None`
instead of being omitted, because order matters.

```rust
pub struct TupleStruct(i32);
pub struct EmptyTupleStruct();
```

####### `Plain { fields: alloc::vec::Vec<rustdoc_types::Id>, has_stripped_fields: bool }`

A struct with named fields.

```rust
pub struct PlainStruct { x: i32 }
pub struct EmptyPlainStruct {}
```

###### Trait Implementations for `StructKind`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::StructKind where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::StructKind where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::StructKind where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::StructKind where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::StructKind {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::StructKind where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::StructKind where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::StructKind where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::StructKind where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::StructKind where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::StructKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum Term`

```rust
pub enum Term {
    Type(rustdoc_types::Type),
    Constant(rustdoc_types::Constant),
}
```

Either a type or a constant, usually stored as the right-hand side of an equation in places like
[`AssocItemConstraint`]

###### Variants

####### `Type(rustdoc_types::Type)`

A type.

```rust
fn f(x: impl IntoIterator<Item = u32>) {}
//                               ^^^
```

####### `Constant(rustdoc_types::Constant)`

A constant.

```ignore (incomplete feature in the snippet)
trait Foo {
    const BAR: usize;
}

fn f(x: impl Foo<BAR = 42>) {}
//                     ^^
```

###### Trait Implementations for `Term`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Term where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Term where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Term where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Term where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Term {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Term where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Term where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Term where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Term where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Term where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Term {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum TraitBoundModifier`

```rust
pub enum TraitBoundModifier {
    None,
    Maybe,
    MaybeConst,
}
```

A set of modifiers applied to a trait.

###### Variants

####### `None`

Marks the absence of a modifier.

####### `Maybe`

Indicates that the trait bound relaxes a trait bound applied to a parameter by default,
e.g. `T: Sized?`, the `Sized` trait is required for all generic type parameters by default
unless specified otherwise with this modifier.

####### `MaybeConst`

Indicates that the trait bound must be applicable in both a run-time and a compile-time
context.

###### Trait Implementations for `TraitBoundModifier`

- `Copy`
- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::TraitBoundModifier where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::TraitBoundModifier where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::TraitBoundModifier where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::TraitBoundModifier where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::TraitBoundModifier {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::TraitBoundModifier where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::TraitBoundModifier where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::TraitBoundModifier where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::TraitBoundModifier where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::TraitBoundModifier where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::TraitBoundModifier {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum Type`

```rust
pub enum Type {
    ResolvedPath(rustdoc_types::Path),
    DynTrait(rustdoc_types::DynTrait),
    Generic(alloc::string::String),
    Primitive(alloc::string::String),
    FunctionPointer(alloc::boxed::Box<rustdoc_types::FunctionPointer>),
    Tuple(alloc::vec::Vec<rustdoc_types::Type>),
    Slice(alloc::boxed::Box<rustdoc_types::Type>),
    Array { type_: alloc::boxed::Box<rustdoc_types::Type>, len: alloc::string::String },
    Pat { type_: alloc::boxed::Box<rustdoc_types::Type> },
    ImplTrait(alloc::vec::Vec<rustdoc_types::GenericBound>),
    Infer,
    RawPointer { is_mutable: bool, type_: alloc::boxed::Box<rustdoc_types::Type> },
    BorrowedRef { lifetime: core::option::Option<alloc::string::String>, is_mutable: bool, type_: alloc::boxed::Box<rustdoc_types::Type> },
    QualifiedPath { name: alloc::string::String, args: alloc::boxed::Box<rustdoc_types::GenericArgs>, self_type: alloc::boxed::Box<rustdoc_types::Type>, trait_: core::option::Option<rustdoc_types::Path> },
}
```

A type.

###### Variants

####### `ResolvedPath(rustdoc_types::Path)`

Structs, enums, unions and type aliases, e.g. `std::option::Option<u32>`

####### `DynTrait(rustdoc_types::DynTrait)`

Dynamic trait object type (`dyn Trait`).

####### `Generic(alloc::string::String)`

Parameterized types. The contained string is the name of the parameter.

####### `Primitive(alloc::string::String)`

Built-in numeric types (e.g. `u32`, `f32`), `bool`, `char`.

####### `FunctionPointer(alloc::boxed::Box<rustdoc_types::FunctionPointer>)`

A function pointer type, e.g. `fn(u32) -> u32`, `extern "C" fn() -> *const u8`

####### `Tuple(alloc::vec::Vec<rustdoc_types::Type>)`

A tuple type, e.g. `(String, u32, Box<usize>)`

####### `Slice(alloc::boxed::Box<rustdoc_types::Type>)`

An unsized slice type, e.g. `[u32]`.

####### `Array { type_: alloc::boxed::Box<rustdoc_types::Type>, len: alloc::string::String }`

An array type, e.g. `[u32; 15]`

####### `Pat { type_: alloc::boxed::Box<rustdoc_types::Type> }`

A pattern type, e.g. `u32 is 1..`

See [the tracking issue](https://github.com/rust-lang/rust/issues/123646)

####### `ImplTrait(alloc::vec::Vec<rustdoc_types::GenericBound>)`

An opaque type that satisfies a set of bounds, `impl TraitA + TraitB + ...`

####### `Infer`

A type that's left to be inferred, `_`

####### `RawPointer { is_mutable: bool, type_: alloc::boxed::Box<rustdoc_types::Type> }`

A raw pointer type, e.g. `*mut u32`, `*const u8`, etc.

####### `BorrowedRef { lifetime: core::option::Option<alloc::string::String>, is_mutable: bool, type_: alloc::boxed::Box<rustdoc_types::Type> }`

`&'a mut String`, `&str`, etc.

####### `QualifiedPath { name: alloc::string::String, args: alloc::boxed::Box<rustdoc_types::GenericArgs>, self_type: alloc::boxed::Box<rustdoc_types::Type>, trait_: core::option::Option<rustdoc_types::Path> }`

Associated types like `<Type as Trait>::Name` and `T::Item` where
`T: Iterator` or inherent associated types like `Struct::Name`.

###### Trait Implementations for `Type`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Type where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Type where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Type where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Type where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Type {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Type where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Type where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Type where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Type where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Type where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Type {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum VariantKind`

```rust
pub enum VariantKind {
    Plain,
    Tuple(alloc::vec::Vec<core::option::Option<rustdoc_types::Id>>),
    Struct { fields: alloc::vec::Vec<rustdoc_types::Id>, has_stripped_fields: bool },
}
```

The kind of an [`Enum`] [`Variant`] and the data specific to it, i.e. fields.

###### Variants

####### `Plain`

A variant with no parentheses

```rust
enum Demo {
    PlainVariant,
    PlainWithDiscriminant = 1,
}
```

####### `Tuple(alloc::vec::Vec<core::option::Option<rustdoc_types::Id>>)`

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

####### `Struct { fields: alloc::vec::Vec<rustdoc_types::Id>, has_stripped_fields: bool }`

A variant with named fields.

```rust
enum Demo {
    StructVariant { x: i32 },
    EmptyStructVariant {},
}
```

###### Trait Implementations for `VariantKind`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::VariantKind where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::VariantKind where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::VariantKind where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::VariantKind where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::VariantKind {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::VariantKind where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::VariantKind where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::VariantKind where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::VariantKind where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::VariantKind where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::VariantKind {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum Visibility`

```rust
pub enum Visibility {
    Public,
    Default,
    Crate,
    Restricted { parent: rustdoc_types::Id, path: alloc::string::String },
}
```

Visibility of an [`Item`].

###### Variants

####### `Public`

Explicitly public visibility set with `pub`.

####### `Default`

For the most part items are private by default. The exceptions are associated items of
public traits and variants of public enums.

####### `Crate`

Explicitly crate-wide visibility set with `pub(crate)`

####### `Restricted { parent: rustdoc_types::Id, path: alloc::string::String }`

For `pub(in path)` visibility.

###### Trait Implementations for `Visibility`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::Visibility where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::Visibility where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::Visibility where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::Visibility where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::Visibility {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::Visibility where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::Visibility where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::Visibility where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::Visibility where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::Visibility where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::Visibility {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


##### `enum WherePredicate`

```rust
pub enum WherePredicate {
    BoundPredicate { type_: rustdoc_types::Type, bounds: alloc::vec::Vec<rustdoc_types::GenericBound>, generic_params: alloc::vec::Vec<rustdoc_types::GenericParamDef> },
    LifetimePredicate { lifetime: alloc::string::String, outlives: alloc::vec::Vec<alloc::string::String> },
    EqPredicate { lhs: rustdoc_types::Type, rhs: rustdoc_types::Term },
}
```

One `where` clause.
```rust
fn default<T>() -> T where T: Default { T::default() }
//                         ^^^^^^^^^^
```

###### Variants

####### `BoundPredicate { type_: rustdoc_types::Type, bounds: alloc::vec::Vec<rustdoc_types::GenericBound>, generic_params: alloc::vec::Vec<rustdoc_types::GenericParamDef> }`

A type is expected to comply with a set of bounds

####### `LifetimePredicate { lifetime: alloc::string::String, outlives: alloc::vec::Vec<alloc::string::String> }`

A lifetime is expected to outlive other lifetimes.

####### `EqPredicate { lhs: rustdoc_types::Type, rhs: rustdoc_types::Term }`

A type must exactly equal another type.

###### Trait Implementations for `WherePredicate`

- `Freeze`
- `Send`
- `StructuralPartialEq`
- `Sync`
- `Unpin`
- `clone::Clone`
- `cmp::Eq`
- `cmp::PartialEq`
- `fmt::Debug`
- `hash::Hash`
- `panic::unwind_safe::RefUnwindSafe`
- `panic::unwind_safe::UnwindSafe`
- `serde::ser::Serialize`

- `borrow::Borrow<T>`

    ```rust
    impl<T> core::borrow::Borrow<T> for rustdoc_types::WherePredicate where T: ?core::marker::Sized {
        pub fn borrow(self: &Self) -> &T { ... }
    }
    ```

- `borrow::BorrowMut<T>`

    ```rust
    impl<T> core::borrow::BorrowMut<T> for rustdoc_types::WherePredicate where T: ?core::marker::Sized {
        pub fn borrow_mut(self: &mut Self) -> &mut T { ... }
    }
    ```

- `clone::CloneToUninit`

    ```rust
    impl<T> core::clone::CloneToUninit for rustdoc_types::WherePredicate where T: core::clone::Clone {
        pub unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { ... }
    }
    ```

- `convert::Into<U>`

    ```rust
    impl<T, U> core::convert::Into<U> for rustdoc_types::WherePredicate where U: core::convert::From<T> {
        pub fn into(self: Self) -> U { ... }
    }
    ```

- `convert::From<T>`

    ```rust
    impl<T> core::convert::From<T> for rustdoc_types::WherePredicate {
        pub fn from(t: T) -> T { ... }
    }
    ```

- `convert::TryInto<U>`

    ```rust
    impl<T, U> core::convert::TryInto<U> for rustdoc_types::WherePredicate where U: core::convert::TryFrom<T> {
        type Error = <U as core::convert::TryFrom<T>>::Error;
        pub fn try_into(self: Self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error> { ... }
    }
    ```

- `convert::TryFrom<U>`

    ```rust
    impl<T, U> core::convert::TryFrom<U> for rustdoc_types::WherePredicate where U: core::convert::Into<T> {
        type Error = core::convert::Infallible;
        pub fn try_from(value: U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error> { ... }
    }
    ```

- `any::Any`

    ```rust
    impl<T> core::any::Any for rustdoc_types::WherePredicate where T: 'static + ?core::marker::Sized {
        pub fn type_id(self: &Self) -> core::any::TypeId { ... }
    }
    ```

- `alloc::borrow::ToOwned`

    ```rust
    impl<T> alloc::borrow::ToOwned for rustdoc_types::WherePredicate where T: core::clone::Clone {
        type Owned = T;
        pub fn to_owned(self: &Self) -> T { ... }
        pub fn clone_into(self: &Self, target: &mut T) { ... }
    }
    ```

- `serde::de::DeserializeOwned`

    ```rust
    impl<T> serde::de::DeserializeOwned for rustdoc_types::WherePredicate where T: for<'de> serde::de::Deserialize<'de> {
    }
    ```

- `serde::de::Deserialize<'de>`

    ```rust
    impl<'de> serde::de::Deserialize<'de> for rustdoc_types::WherePredicate {
        pub fn deserialize<__D> where __D: serde::de::Deserializer<'de>(__deserializer: __D) -> core::result::Result<Self, <__D as serde::de::Deserializer>::Error> { ... }
    }
    ```


#### Constants

##### `const FORMAT_VERSION`

The version of JSON output that this crate represents.

This integer is incremented with every breaking change to the API,
and is returned along with the JSON blob as [`Crate::format_version`].
Consuming code should assert that this value matches the format version(s) that it supports.


## Other

### `0`

_Referenced by:_
- `rustdoc_types::Type::FunctionPointer` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Primitive` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::Type::Primitive` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Variant` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::TypeAlias` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::Type::Tuple` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Use` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::Type::ImplTrait` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::Type::DynTrait` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::GenericBound::Outlives` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Union` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::StructKind::Tuple` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::Type::Generic` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::PreciseCapturingArg::Lifetime` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::ProcMacro` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Module` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::AssocItemConstraintKind::Constraint` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Struct` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::GenericBound::Use` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::GenericArg::Const` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::GenericArg::Lifetime` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::Type::ResolvedPath` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::Term::Constant` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::TraitAlias` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::Term::Type` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Macro` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Enum` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::AssocItemConstraintKind::Equality` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::GenericArg::Type` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Static` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Trait` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::PreciseCapturingArg::Param` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::StructField` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::Type::Slice` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Function` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::Abi::Other` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::ItemEnum::Impl` (VariantField)


### `0`

_Referenced by:_
- `rustdoc_types::VariantKind::Tuple` (VariantField)


### `args`

The list of each argument on this type.
```text
<'a, 32, B: Copy, C = u32>
 ^^^^^^
```

_Referenced by:_
- `rustdoc_types::GenericArgs::AngleBracketed` (VariantField)


### `args`

The generic arguments provided to the associated type.

```ignore (incomplete expression)
<core::slice::IterMut<'static, u32> as BetterIterator>::Item<'static>
//                                                          ^^^^^^^^^
```

_Referenced by:_
- `rustdoc_types::Type::QualifiedPath` (VariantField)


### `bounds`

The set of bounds that constrain the type.

```rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                                 ^^^^^^^^
```

_Referenced by:_
- `rustdoc_types::WherePredicate::BoundPredicate` (VariantField)


### `bounds`

Bounds applied directly to the type. Note that the bounds from `where` clauses
that constrain this parameter won't appear here.

```rust
fn default2<T: Default>() -> [T; 2] where T: Clone { todo!() }
//             ^^^^^^^
```

_Referenced by:_
- `rustdoc_types::GenericParamDefKind::Type` (VariantField)


### `bounds`

The bounds for this associated type. e.g.
```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
//                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^
}
```

_Referenced by:_
- `rustdoc_types::ItemEnum::AssocType` (VariantField)


### `const_`

The declared constant itself.

_Referenced by:_
- `rustdoc_types::ItemEnum::Constant` (VariantField)


### `constraints`

Associated type or constant bindings (e.g. `Item=i32` or `Item: Clone`) for this type.

_Referenced by:_
- `rustdoc_types::GenericArgs::AngleBracketed` (VariantField)


### `default`

The stringified expression for the default value, if provided. It's not guaranteed that
it'll match the actual source code for the default value.

_Referenced by:_
- `rustdoc_types::GenericParamDefKind::Const` (VariantField)


### `default`

The default type for this parameter, if provided, e.g.

```rust
trait PartialEq<Rhs = Self> {}
//                    ^^^^
```

_Referenced by:_
- `rustdoc_types::GenericParamDefKind::Type` (VariantField)


### `fields`

The list of fields in the struct.

All of the corresponding [`Item`]s are of kind [`ItemEnum::StructField`].

_Referenced by:_
- `rustdoc_types::StructKind::Plain` (VariantField)


### `fields`

The list of variants in the enum.
All of the corresponding [`Item`]s are of kind [`ItemEnum::Variant`].

_Referenced by:_
- `rustdoc_types::VariantKind::Struct` (VariantField)


### `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)
```text
where F: for<'a, 'b> Fn(&'a u8, &'b u8)
         ^^^^^^^^^^^
         |
         this part
```

_Referenced by:_
- `rustdoc_types::GenericBound::TraitBound` (VariantField)


### `generic_params`

Used for Higher-Rank Trait Bounds (HRTBs)
```rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                  ^^^^^^^
```

_Referenced by:_
- `rustdoc_types::WherePredicate::BoundPredicate` (VariantField)


### `generics`

The generic parameters and where clauses on ahis associated type.

_Referenced by:_
- `rustdoc_types::ItemEnum::AssocType` (VariantField)


### `has_stripped_fields`

Whether any variants have been removed from the result, due to being private or hidden.

_Referenced by:_
- `rustdoc_types::VariantKind::Struct` (VariantField)


### `has_stripped_fields`

Whether any fields have been removed from the result, due to being private or hidden.

_Referenced by:_
- `rustdoc_types::StructKind::Plain` (VariantField)


### `inputs`

The input types, enclosed in parentheses.

_Referenced by:_
- `rustdoc_types::GenericArgs::Parenthesized` (VariantField)


### `is_mutable`

This is `true` for `*mut _` and `false` for `*const _`.

_Referenced by:_
- `rustdoc_types::Type::RawPointer` (VariantField)


### `is_mutable`

This is `true` for `&mut i32` and `false` for `&i32`

_Referenced by:_
- `rustdoc_types::Type::BorrowedRef` (VariantField)


### `is_synthetic`

This is normally `false`, which means that this generic parameter is
declared in the Rust source text.

If it is `true`, this generic parameter has been introduced by the
compiler behind the scenes.

#### Example

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

_Referenced by:_
- `rustdoc_types::GenericParamDefKind::Type` (VariantField)


### `len`

The stringified expression that is the length of the array.

Keep in mind that it's not guaranteed to match the actual source code of the expression.

_Referenced by:_
- `rustdoc_types::Type::Array` (VariantField)


### `lhs`

The left side of the equation.

_Referenced by:_
- `rustdoc_types::WherePredicate::EqPredicate` (VariantField)


### `lifetime`

The name of the lifetime of the reference, if provided.

_Referenced by:_
- `rustdoc_types::Type::BorrowedRef` (VariantField)


### `lifetime`

The name of the lifetime.

_Referenced by:_
- `rustdoc_types::WherePredicate::LifetimePredicate` (VariantField)


### `modifier`

The context for which a trait is supposed to be used, e.g. `const

_Referenced by:_
- `rustdoc_types::GenericBound::TraitBound` (VariantField)


### `name`

The name of the imported crate.

_Referenced by:_
- `rustdoc_types::ItemEnum::ExternCrate` (VariantField)


### `name`

The name of the associated type in the parent type.

```ignore (incomplete expression)
<core::array::IntoIter<u32, 42> as Iterator>::Item
//                                            ^^^^
```

_Referenced by:_
- `rustdoc_types::Type::QualifiedPath` (VariantField)


### `outlives`

Lifetimes that this lifetime parameter is required to outlive.

```rust
fn f<'a, 'b, 'resource: 'a + 'b>(a: &'a str, b: &'b str, res: &'resource str) {}
//                      ^^^^^^^
```

_Referenced by:_
- `rustdoc_types::GenericParamDefKind::Lifetime` (VariantField)


### `outlives`

The lifetimes that must be encompassed by the lifetime.

_Referenced by:_
- `rustdoc_types::WherePredicate::LifetimePredicate` (VariantField)


### `output`

The output type provided after the `->`, if present.

_Referenced by:_
- `rustdoc_types::GenericArgs::Parenthesized` (VariantField)


### `parent`

ID of the module to which this visibility restricts items.

_Referenced by:_
- `rustdoc_types::Visibility::Restricted` (VariantField)
- `{id:271}` (IntraDocLink)


### `path`

The path with which [`parent`] was referenced
(like `super::super` or `crate::foo::bar`).

[`parent`]: Visibility::Restricted::parent

_Referenced by:_
- `rustdoc_types::Visibility::Restricted` (VariantField)


### `rename`

If the crate is renamed, this is its name in the crate.

_Referenced by:_
- `rustdoc_types::ItemEnum::ExternCrate` (VariantField)


### `rhs`

The right side of the equation.

_Referenced by:_
- `rustdoc_types::WherePredicate::EqPredicate` (VariantField)


### `self_type`

The type with which this type is associated.

```ignore (incomplete expression)
<core::array::IntoIter<u32, 42> as Iterator>::Item
// ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

_Referenced by:_
- `rustdoc_types::Type::QualifiedPath` (VariantField)


### `trait_`

`None` iff this is an *inherent* associated type.

_Referenced by:_
- `rustdoc_types::Type::QualifiedPath` (VariantField)


### `trait_`

The full path to the trait.

_Referenced by:_
- `rustdoc_types::GenericBound::TraitBound` (VariantField)


### `type_`

The type of the constant as declared.

_Referenced by:_
- `rustdoc_types::GenericParamDefKind::Const` (VariantField)


### `type_`

The type of the constant.

_Referenced by:_
- `rustdoc_types::ItemEnum::Constant` (VariantField)


### `type_`

The type of the contained element.

_Referenced by:_
- `rustdoc_types::Type::Array` (VariantField)


### `type_`

The base type, e.g. the `u32` in `u32 is 1..`

_Referenced by:_
- `rustdoc_types::Type::Pat` (VariantField)


### `type_`

The type of the constant.

_Referenced by:_
- `rustdoc_types::ItemEnum::AssocConst` (VariantField)


### `type_`

The type of the pointee, e.g. the `i32` in `&'a mut i32`

_Referenced by:_
- `rustdoc_types::Type::BorrowedRef` (VariantField)


### `type_`

Inside a trait declaration, this is the default for the associated type, if provided.
Inside an impl block, this is the type assigned to the associated type, and will always
be present.

```rust
type X = usize;
//       ^^^^^
```

_Referenced by:_
- `rustdoc_types::ItemEnum::AssocType` (VariantField)


### `type_`

The type that's being constrained.

```rust
fn f<T>(x: T) where for<'a> &'a T: Iterator {}
//                              ^
```

_Referenced by:_
- `rustdoc_types::WherePredicate::BoundPredicate` (VariantField)


### `type_`

The type of the pointee.

_Referenced by:_
- `rustdoc_types::Type::RawPointer` (VariantField)


### `unwind`

_Referenced by:_
- `rustdoc_types::Abi::Cdecl` (VariantField)


### `unwind`

_Referenced by:_
- `rustdoc_types::Abi::SysV64` (VariantField)


### `unwind`

_Referenced by:_
- `rustdoc_types::Abi::C` (VariantField)


### `unwind`

_Referenced by:_
- `rustdoc_types::Abi::System` (VariantField)


### `unwind`

_Referenced by:_
- `rustdoc_types::Abi::Win64` (VariantField)


### `unwind`

_Referenced by:_
- `rustdoc_types::Abi::Aapcs` (VariantField)


### `unwind`

_Referenced by:_
- `rustdoc_types::Abi::Stdcall` (VariantField)


### `unwind`

_Referenced by:_
- `rustdoc_types::Abi::Fastcall` (VariantField)


### `value`

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

_Referenced by:_
- `rustdoc_types::ItemEnum::AssocConst` (VariantField)

