# rustdoc_types

Rustdoc's JSON output interface

These types are the public API exposed through the `--output-format json` flag.
The [`Crate`] struct is the root of the JSON blob and all other items are
contained within.

We expose a `rustc-hash` feature that is disabled by default. This feature
switches the [`std::collections::HashMap`] for [`rustc_hash::FxHashMap`] to
improve the performance of said `HashMap` in specific situations.

`cargo-semver-checks` for example, saw a [-3% improvement][1] when benchmarking
using the `aws_sdk_ec2` JSON output (~500MB of JSON). As always, we recommend
measuring the impact before turning this feature on, as [`FxHashMap`][2] only
concerns itself with hash speed, and may increase the number of collisions.

[1]:
    https://rust-lang.zulipchat.com/#narrow/channel/266220-t-rustdoc/topic/rustc-hash.20and.20performance.20of.20rustdoc-types/near/474855731
[2]: https://crates.io/crates/rustc-hash

## Modules

### [`rustdoc_types`](rustdoc_types.md)

_1 constant, 19 enums, 35 structs_

---

**rustdoc_types**

# Module: rustdoc_types

## Contents

**Structs**

- [`AssocItemConstraint`](#associtemconstraint) - Describes a bound applied to
  an associated type/constant.
- [`AttributeRepr`](#attributerepr) - The contents of a `#[repr(...)]`
  attribute.
- [`Constant`](#constant) - A constant.
- [`Crate`](#crate) - The root of the emitted JSON blob.
- [`Deprecation`](#deprecation) - Information about the deprecation of an
  [`Item`].
- [`Discriminant`](#discriminant) - The value that distinguishes a variant in an
  [`Enum`] from other variants.
- [`DynTrait`](#dyntrait) - Dynamic trait object type (`dyn Trait`).
- [`Enum`](#enum) - An `enum`.
- [`ExternalCrate`](#externalcrate) - Metadata of a crate, either the same crate
  on which `rustdoc` was invoked, or its dependency.
- [`Function`](#function) - A function declaration (including methods and other
  associated functions).
- [`FunctionHeader`](#functionheader) - A set of fundamental properties of a
  function.
- [`FunctionPointer`](#functionpointer) - A type that is a function pointer.
- [`FunctionSignature`](#functionsignature) - The signature of a function.
- [`GenericParamDef`](#genericparamdef) - One generic parameter accepted by an
  item.
- [`Generics`](#generics) - Generic parameters accepted by an item and `where`
  clauses imposed on it and the parameters.
- [`Id`](#id) - An opaque identifier for an item.
- [`Impl`](#impl) - An `impl` block.
- [`Item`](#item) - Anything that can hold documentation - modules, structs,
  enums, functions, traits, etc.
- [`ItemSummary`](#itemsummary) - Information about an external (not defined in
  the local crate) [`Item`].
- [`Module`](#module) - A module declaration, e.g. `mod foo;` or `mod foo {}`.
- [`Path`](#path) - A type that has a simple path to it. This is the kind of
  type of structs, unions, enums, etc.
- [`PolyTrait`](#polytrait) - A trait and potential HRTBs
- [`Primitive`](#primitive) - A primitive type declaration. Declarations of this
  kind can only come from the core library.
- [`ProcMacro`](#procmacro) - A procedural macro.
- [`Span`](#span) - A range of source code.
- [`Static`](#static) - A `static` declaration.
- [`Struct`](#struct) - A `struct`.
- [`Target`](#target) - Information about a target
- [`TargetFeature`](#targetfeature) - Information about a target feature.
- [`Trait`](#trait) - A `trait` declaration.
- [`TraitAlias`](#traitalias) - A trait alias declaration, e.g.
  `trait Int = Add + Sub + Mul + Div;`
- [`TypeAlias`](#typealias) - A type alias declaration, e.g.
  `type Pig = std::borrow::Cow<'static, str>;`
- [`Union`](#union) - A `union`.
- [`Use`](#use) - A `use` statement.
- [`Variant`](#variant) - A variant of an enum.

**Enums**

- [`Abi`](#abi) - The ABI (Application Binary Interface) used by a function.
- [`AssocItemConstraintKind`](#associtemconstraintkind) - The way in which an
  associate type/constant is bound.
- [`Attribute`](#attribute) - An attribute, e.g. `#[repr(C)]`
- [`GenericArg`](#genericarg) - One argument in a list of generic arguments to a
  path segment.
- [`GenericArgs`](#genericargs) - A set of generic arguments provided to a path
  segment, e.g.
- [`GenericBound`](#genericbound) - Either a trait bound or a lifetime bound.
- [`GenericParamDefKind`](#genericparamdefkind) - The kind of a
  [`GenericParamDef`].
- [`ItemEnum`](#itemenum) - Specific fields of an item.
- [`ItemKind`](#itemkind) - The fundamental kind of an item. Unlike
  [`ItemEnum`], this does not carry any additional info.
- [`MacroKind`](#macrokind) - The way a [`ProcMacro`] is declared to be used.
- [`PreciseCapturingArg`](#precisecapturingarg) - One precise capturing
  argument. See
  [the rust reference](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing).
- [`ReprKind`](#reprkind) - The kind of `#[repr]`.
- [`StructKind`](#structkind) - The kind of a [`Struct`] and the data specific
  to it, i.e. fields.
- [`Term`](#term) - Either a type or a constant, usually stored as the
  right-hand side of an equation in places like
- [`TraitBoundModifier`](#traitboundmodifier) - A set of modifiers applied to a
  trait.
- [`Type`](#type) - A type.
- [`VariantKind`](#variantkind) - The kind of an [`Enum`] [`Variant`] and the
  data specific to it, i.e. fields.
- [`Visibility`](#visibility) - Visibility of an [`Item`].
- [`WherePredicate`](#wherepredicate) - One `where` clause.

**Constants**

- [`FORMAT_VERSION`](#format_version) - The version of JSON output that this
  crate represents.

---

## rustdoc_types::Abi

_Enum_

The ABI (Application Binary Interface) used by a function.

If a variant has an `unwind` field, this means the ABI that it represents can be
specified in 2 ways: `extern "_"` and `extern "_-unwind"`, and a value of `true`
for that field signifies the latter variant.

See the
[Rustonomicon section](https://doc.rust-lang.org/nightly/nomicon/ffi.html#ffi-and-unwinding)
on unwinding for more info.

**Variants:**

- `Rust` - The default ABI, but that can also be written explicitly with
  `extern "Rust"`.
- `C{ unwind: bool }` - Can be specified as `extern "C"` or, as a shorthand,
  just `extern`.
- `Cdecl{ unwind: bool }` - Can be specified as `extern "cdecl"`.
- `Stdcall{ unwind: bool }` - Can be specified as `extern "stdcall"`.
- `Fastcall{ unwind: bool }` - Can be specified as `extern "fastcall"`.
- `Aapcs{ unwind: bool }` - Can be specified as `extern "aapcs"`.
- `Win64{ unwind: bool }` - Can be specified as `extern "win64"`.
- `SysV64{ unwind: bool }` - Can be specified as `extern "sysv64"`.
- `System{ unwind: bool }` - Can be specified as `extern "system"`.
- `Other(String)` - Any other ABI, including unstable ones.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Abi) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Abi`

## rustdoc_types::AssocItemConstraint

_Struct_

Describes a bound applied to an associated type/constant.

Example:

```text
IntoIterator<Item = u32, IntoIter: Clone>
             ^^^^^^^^^^  ^^^^^^^^^^^^^^^
```

**Fields:**

- `name: String` - The name of the associated type/constant.
- `args: Option<Box<GenericArgs>>` - Arguments provided to the associated
  type/constant.
- `binding: AssocItemConstraintKind` - The kind of bound applied to the
  associated type/constant.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &AssocItemConstraint) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> AssocItemConstraint`

## rustdoc_types::AssocItemConstraintKind

_Enum_

The way in which an associate type/constant is bound.

**Variants:**

- `Equality(Term)` - The required value/type is specified exactly. e.g.
- `Constraint(Vec<GenericBound>)` - The type is required to satisfy a set of
  bounds.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &AssocItemConstraintKind) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> AssocItemConstraintKind`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::Attribute

_Enum_

An attribute, e.g. `#[repr(C)]`

This doesn't include:

- `#[doc = "Doc Comment"]` or `/// Doc comment`. These are in [`Item::docs`]
  instead.
- `#[deprecated]`. These are in [`Item::deprecation`] instead.

**Variants:**

- `NonExhaustive` - `#[non_exhaustive]`
- `MustUse{ reason: Option<String> }` - `#[must_use]`
- `MacroExport` - `#[macro_export]`
- `ExportName(String)` - `#[export_name = "name"]`
- `LinkSection(String)` - `#[link_section = "name"]`
- `AutomaticallyDerived` - `#[automatically_derived]`
- `Repr(AttributeRepr)` - `#[repr]`
- `NoMangle` - `#[no_mangle]`
- `TargetFeature{ enable: Vec<String> }` - #[target_feature(enable = "feature1",
  enable = "feature2")]
- `Other(String)` - Something else.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Attribute) -> bool`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Clone**
    - `fn clone(self: &Self) -> Attribute`

## rustdoc_types::AttributeRepr

_Struct_

The contents of a `#[repr(...)]` attribute.

Used in [`Attribute::Repr`].

**Fields:**

- `kind: ReprKind` - The representation, e.g. `#[repr(C)]`,
  `#[repr(transparent)]`
- `align: Option<u64>` - Alignment in bytes, if explicitly specified by
  `#[repr(align(...)]`.
- `packed: Option<u64>` - Alignment in bytes, if explicitly specified by
  `#[repr(packed(...)]]`.
- `int: Option<String>` - The integer type for an enum descriminant, if
  explicitly specified.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &AttributeRepr) -> bool`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Clone**
    - `fn clone(self: &Self) -> AttributeRepr`

## rustdoc_types::Constant

_Struct_

A constant.

**Fields:**

- `expr: String` - The stringified expression of this constant. Note that its
  mapping to the original
- `value: Option<String>` - The value of the evaluated expression for this
  constant, which is only computed for numeric
- `is_literal: bool` - Whether this constant is a bool, numeric, string, or char
  literal.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Constant`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Constant) -> bool`

## rustdoc_types::Crate

_Struct_

The root of the emitted JSON blob.

It contains all type/documentation information about the language items in the
local crate, as well as info about external items to allow tools to find or link
to them.

**Fields:**

- `root: Id` - The id of the root [`Module`] item of the local crate.
- `crate_version: Option<String>` - The version string given to
  `--crate-version`, if any.
- `includes_private: bool` - Whether or not the output includes private items.
- `index: std::collections::HashMap<Id, Item>` - A collection of all items in
  the local crate as well as some external traits and their
- `paths: std::collections::HashMap<Id, ItemSummary>` - Maps IDs to fully
  qualified paths and other info helpful for generating links.
- `external_crates: std::collections::HashMap<u32, ExternalCrate>` - Maps
  `crate_id` of items to a crate name and html_root_url if it exists.
- `target: Target` - Information about the target for which this documentation
  was generated
- `format_version: u32` - A single version number to be used in the future when
  making backwards incompatible changes

**Traits:** Eq

**Trait Implementations:**

- **Clone**
    - `fn clone(self: &Self) -> Crate`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **PartialEq**
    - `fn eq(self: &Self, other: &Crate) -> bool`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

## rustdoc_types::Deprecation

_Struct_

Information about the deprecation of an [`Item`].

**Fields:**

- `since: Option<String>` - Usually a version number when this [`Item`] first
  became deprecated.
- `note: Option<String>` - The reason for deprecation and/or what alternatives
  to use.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Deprecation`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Deprecation) -> bool`

## rustdoc_types::Discriminant

_Struct_

The value that distinguishes a variant in an [`Enum`] from other variants.

**Fields:**

- `expr: String` - The expression that produced the discriminant.
- `value: String` - The numerical value of the discriminant. Stored as a string
  due to

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Discriminant) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Discriminant`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::DynTrait

_Struct_

Dynamic trait object type (`dyn Trait`).

**Fields:**

- `traits: Vec<PolyTrait>` - All the traits implemented. One of them is the
  vtable, and the rest must be auto traits.
- `lifetime: Option<String>` - The lifetime of the whole dyn object

**Traits:** Eq

**Trait Implementations:**

- **Clone**
    - `fn clone(self: &Self) -> DynTrait`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &DynTrait) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`

## rustdoc_types::Enum

_Struct_

An `enum`.

**Fields:**

- `generics: Generics` - Information about the type parameters and `where`
  clauses of the enum.
- `has_stripped_variants: bool` - Whether any variants have been removed from
  the result, due to being private or hidden.
- `variants: Vec<Id>` - The list of variants in the enum.
- `impls: Vec<Id>` - `impl`s for the enum.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Enum`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Enum) -> bool`

## rustdoc_types::ExternalCrate

_Struct_

Metadata of a crate, either the same crate on which `rustdoc` was invoked, or
its dependency.

**Fields:**

- `name: String` - The name of the crate.
- `html_root_url: Option<String>` - The root URL at which the crate's
  documentation lives.
- `path: std::path::PathBuf` - A path from where this crate was loaded.

**Traits:** Eq

**Trait Implementations:**

- **Clone**
    - `fn clone(self: &Self) -> ExternalCrate`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &ExternalCrate) -> bool`

## rustdoc_types::FORMAT_VERSION

_Constant_: `u32`

The version of JSON output that this crate represents.

This integer is incremented with every breaking change to the API, and is
returned along with the JSON blob as [`Crate::format_version`]. Consuming code
should assert that this value matches the format version(s) that it supports.

## rustdoc_types::Function

_Struct_

A function declaration (including methods and other associated functions).

**Fields:**

- `sig: FunctionSignature` - Information about the function signature, or
  declaration.
- `generics: Generics` - Information about the function’s type parameters and
  `where` clauses.
- `header: FunctionHeader` - Information about core properties of the function,
  e.g. whether it's `const`, its ABI, etc.
- `has_body: bool` - Whether the function has a body, i.e. an implementation.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Function) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Function`

## rustdoc_types::FunctionHeader

_Struct_

A set of fundamental properties of a function.

**Fields:**

- `is_const: bool` - Is this function marked as `const`?
- `is_unsafe: bool` - Is this function unsafe?
- `is_async: bool` - Is this function async?
- `abi: Abi` - The ABI used by the function.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &FunctionHeader) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> FunctionHeader`

## rustdoc_types::FunctionPointer

_Struct_

A type that is a function pointer.

**Fields:**

- `sig: FunctionSignature` - The signature of the function.
- `generic_params: Vec<GenericParamDef>` - Used for Higher-Rank Trait Bounds
  (HRTBs)
- `header: FunctionHeader` - The core properties of the function, such as the
  ABI it conforms to, whether it's unsafe, etc.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &FunctionPointer) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> FunctionPointer`

## rustdoc_types::FunctionSignature

_Struct_

The signature of a function.

**Fields:**

- `inputs: Vec<(String, Type)>` - List of argument names and their type.
- `output: Option<Type>` - The output type, if specified.
- `is_c_variadic: bool` - Whether the function accepts an arbitrary amount of
  trailing arguments the C way.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
    - `fn eq(self: &Self, other: &FunctionSignature) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> FunctionSignature`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`

## rustdoc_types::GenericArg

_Enum_

One argument in a list of generic arguments to a path segment.

Part of [`GenericArgs`].

**Variants:**

- `Lifetime(String)` - A lifetime argument.
- `Type(Type)` - A type argument.
- `Const(Constant)` - A constant as a generic argument.
- `Infer` - A generic argument that's explicitly set to be inferred.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &GenericArg) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> GenericArg`

## rustdoc_types::GenericArgs

_Enum_

A set of generic arguments provided to a path segment, e.g.

```text
std::option::Option<u32>
                   ^^^^^
```

**Variants:**

- `AngleBracketed{ args: Vec<GenericArg>, constraints: Vec<AssocItemConstraint> }` -
  `<'a, 32, B: Copy, C = u32>`
- `Parenthesized{ inputs: Vec<Type>, output: Option<Type> }` - `Fn(A, B) -> C`
- `ReturnTypeNotation` - `T::method(..)`

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &GenericArgs) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> GenericArgs`

## rustdoc_types::GenericBound

_Enum_

Either a trait bound or a lifetime bound.

**Variants:**

- `TraitBound{ trait_: Path, generic_params: Vec<GenericParamDef>, modifier: TraitBoundModifier }` -
  A trait bound.
- `Outlives(String)` - A lifetime bound, e.g.
- `Use(Vec<PreciseCapturingArg>)` - `use<'a, T>` precise-capturing bound syntax

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &GenericBound) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> GenericBound`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::GenericParamDef

_Struct_

One generic parameter accepted by an item.

**Fields:**

- `name: String` - Name of the parameter.
- `kind: GenericParamDefKind` - The kind of the parameter and data specific to a
  particular parameter kind, e.g. type

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &GenericParamDef) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> GenericParamDef`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::GenericParamDefKind

_Enum_

The kind of a [`GenericParamDef`].

**Variants:**

- `Lifetime{ outlives: Vec<String> }` - Denotes a lifetime parameter.
- `Type{ bounds: Vec<GenericBound>, default: Option<Type>, is_synthetic: bool }` -
  Denotes a type parameter.
- `Const{ type_: Type, default: Option<String> }` - Denotes a constant
  parameter.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &GenericParamDefKind) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> GenericParamDefKind`

## rustdoc_types::Generics

_Struct_

Generic parameters accepted by an item and `where` clauses imposed on it and the
parameters.

**Fields:**

- `params: Vec<GenericParamDef>` - A list of generic parameter definitions (e.g.
  `<T: Clone + Hash, U: Copy>`).
- `where_predicates: Vec<WherePredicate>` - A list of where predicates (e.g.
  `where T: Iterator, T::Item: Copy`).

**Traits:** Eq

**Trait Implementations:**

- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Generics`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Generics) -> bool`

## rustdoc_types::Id

_Struct_

An opaque identifier for an item.

It can be used to lookup in [`Crate::index`] or [`Crate::paths`] to resolve it
to an [`Item`].

Id's are only valid within a single JSON blob. They cannot be used to resolve
references between the JSON output's for different crates.

Rustdoc makes no guarantees about the inner value of Id's. Applications should
treat them as opaque keys to lookup items, and avoid attempting to parse them,
or otherwise depend on any implementation details.

**Tuple Struct**: `(u32)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Id) -> bool`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Clone**
    - `fn clone(self: &Self) -> Id`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **PartialOrd**
    - `fn partial_cmp(self: &Self, other: &Id) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Ord**
    - `fn cmp(self: &Self, other: &Id) -> $crate::cmp::Ordering`

## rustdoc_types::Impl

_Struct_

An `impl` block.

**Fields:**

- `is_unsafe: bool` - Whether this impl is for an unsafe trait.
- `generics: Generics` - Information about the impl’s type parameters and
  `where` clauses.
- `provided_trait_methods: Vec<String>` - The list of the names of all the trait
  methods that weren't mentioned in this impl but
- `trait_: Option<Path>` - The trait being implemented or `None` if the impl is
  inherent, which means
- `for_: Type` - The type that the impl block is for.
- `items: Vec<Id>` - The list of associated items contained in this impl block.
- `is_negative: bool` - Whether this is a negative impl (e.g. `!Sized` or
  `!Send`).
- `is_synthetic: bool` - Whether this is an impl that’s implied by the compiler
- `blanket_impl: Option<Type>`

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
    - `fn eq(self: &Self, other: &Impl) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Impl`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`

## rustdoc_types::Item

_Struct_

Anything that can hold documentation - modules, structs, enums, functions,
traits, etc.

The `Item` data type holds fields that can apply to any of these, and leaves
kind-specific details (like function args or enum variants) to the `inner`
field.

**Fields:**

- `id: Id` - The unique identifier of this item. Can be used to find this item
  in various mappings.
- `crate_id: u32` - This can be used as a key to the `external_crates` map of
  [`Crate`] to see which crate
- `name: Option<String>` - Some items such as impls don't have names.
- `span: Option<Span>` - The source location of this item (absent if it came
  from a macro expansion or inline
- `visibility: Visibility` - By default all documented items are public, but you
  can tell rustdoc to output private items
- `docs: Option<String>` - The full markdown docstring of this item. Absent if
  there is no documentation at all,
- `links: std::collections::HashMap<String, Id>` - This mapping resolves
  [intra-doc links](https://github.com/rust-lang/rfcs/blob/master/text/1946-intra-rustdoc-links.md)
  from the docstring to their IDs
- `attrs: Vec<Attribute>` - Attributes on this item.
- `deprecation: Option<Deprecation>` - Information about the item’s deprecation,
  if present.
- `inner: ItemEnum` - The type-specific fields describing this item.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Clone**
    - `fn clone(self: &Self) -> Item`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Item) -> bool`

## rustdoc_types::ItemEnum

_Enum_

Specific fields of an item.

Part of [`Item`].

**Variants:**

- `Module(Module)` - A module declaration, e.g. `mod foo;` or `mod foo {}`
- `ExternCrate{ name: String, rename: Option<String> }` - A crate imported via
  the `extern crate` syntax.
- `Use(Use)` - An import of 1 or more items into scope, using the `use` keyword.
- `Union(Union)` - A `union` declaration.
- `Struct(Struct)` - A `struct` declaration.
- `StructField(Type)` - A field of a struct.
- `Enum(Enum)` - An `enum` declaration.
- `Variant(Variant)` - A variant of a enum.
- `Function(Function)` - A function declaration (including methods and other
  associated functions)
- `Trait(Trait)` - A `trait` declaration.
- `TraitAlias(TraitAlias)` - A trait alias declaration, e.g.
  `trait Int = Add + Sub + Mul + Div;`
- `Impl(Impl)` - An `impl` block.
- `TypeAlias(TypeAlias)` - A type alias declaration, e.g.
  `type Pig = std::borrow::Cow<'static, str>;`
- `Constant{ type_: Type, const_: Constant }` - The declaration of a constant,
  e.g. `const GREETING: &str = "Hi :3";`
- `Static(Static)` - A declaration of a `static`.
- `ExternType` - `type`s from an `extern` block.
- `Macro(String)` - A macro_rules! declarative macro. Contains a single string
  with the source
- `ProcMacro(ProcMacro)` - A procedural macro.
- `Primitive(Primitive)` - A primitive type, e.g. `u32`.
- `AssocConst{ type_: Type, value: Option<String> }` - An associated constant of
  a trait or a type.
- `AssocType{ generics: Generics, bounds: Vec<GenericBound>, type_: Option<Type> }` -
  An associated type of a trait or a type.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &ItemEnum) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> ItemEnum`

## rustdoc_types::ItemKind

_Enum_

The fundamental kind of an item. Unlike [`ItemEnum`], this does not carry any
additional info.

Part of [`ItemSummary`].

**Variants:**

- `Module` - A module declaration, e.g. `mod foo;` or `mod foo {}`
- `ExternCrate` - A crate imported via the `extern crate` syntax.
- `Use` - An import of 1 or more items into scope, using the `use` keyword.
- `Struct` - A `struct` declaration.
- `StructField` - A field of a struct.
- `Union` - A `union` declaration.
- `Enum` - An `enum` declaration.
- `Variant` - A variant of a enum.
- `Function` - A function declaration, e.g. `fn f() {}`
- `TypeAlias` - A type alias declaration, e.g.
  `type Pig = std::borrow::Cow<'static, str>;`
- `Constant` - The declaration of a constant, e.g.
  `const GREETING: &str = "Hi :3";`
- `Trait` - A `trait` declaration.
- `TraitAlias` - A trait alias declaration, e.g.
  `trait Int = Add + Sub + Mul + Div;`
- `Impl` - An `impl` block.
- `Static` - A `static` declaration.
- `ExternType` - `type`s from an `extern` block.
- `Macro` - A macro declaration.
- `ProcAttribute` - A procedural macro attribute.
- `ProcDerive` - A procedural macro usable in the `#[derive()]` attribute.
- `AssocConst` - An associated constant of a trait or a type.
- `AssocType` - An associated type of a trait or a type.
- `Primitive` - A primitive type, e.g. `u32`.
- `Keyword` - A keyword declaration.
- `Attribute` - An attribute declaration.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Clone**
    - `fn clone(self: &Self) -> ItemKind`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &ItemKind) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::ItemSummary

_Struct_

Information about an external (not defined in the local crate) [`Item`].

For external items, you don't get the same level of information. This struct
should contain enough to generate a link/reference to the item in question, or
can be used by a tool that takes the json output of multiple crates to find the
actual item definition with all the relevant info.

**Fields:**

- `crate_id: u32` - Can be used to look up the name and html_root_url of the
  crate this item came from in the
- `path: Vec<String>` - The list of path components for the fully qualified path
  of this item (e.g.
- `kind: ItemKind` - Whether this item is a struct, trait, macro, etc.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &ItemSummary) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> ItemSummary`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::MacroKind

_Enum_

The way a [`ProcMacro`] is declared to be used.

**Variants:**

- `Bang` - A bang macro `foo!()`.
- `Attr` - An attribute macro `#[foo]`.
- `Derive` - A derive macro
  `#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Clone**
    - `fn clone(self: &Self) -> MacroKind`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &MacroKind) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`

## rustdoc_types::Module

_Struct_

A module declaration, e.g. `mod foo;` or `mod foo {}`.

**Fields:**

- `is_crate: bool` - Whether this is the root item of a crate.
- `items: Vec<Id>` - [`Item`]s declared inside this module.
- `is_stripped: bool` - If `true`, this module is not part of the public API,
  but it contains

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Module) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Module`

## rustdoc_types::Path

_Struct_

A type that has a simple path to it. This is the kind of type of structs,
unions, enums, etc.

**Fields:**

- `path: String` - The path of the type.
- `id: Id` - The ID of the type.
- `args: Option<Box<GenericArgs>>` - Generic arguments to the type.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Path) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Path`

## rustdoc_types::PolyTrait

_Struct_

A trait and potential HRTBs

**Fields:**

- `trait_: Path` - The path to the trait.
- `generic_params: Vec<GenericParamDef>` - Used for Higher-Rank Trait Bounds
  (HRTBs)

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &PolyTrait) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> PolyTrait`

## rustdoc_types::PreciseCapturingArg

_Enum_

One precise capturing argument. See
[the rust reference](https://doc.rust-lang.org/reference/types/impl-trait.html#precise-capturing).

**Variants:**

- `Lifetime(String)` - A lifetime.
- `Param(String)` - A type or constant parameter.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &PreciseCapturingArg) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> PreciseCapturingArg`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::Primitive

_Struct_

A primitive type declaration. Declarations of this kind can only come from the
core library.

**Fields:**

- `name: String` - The name of the type.
- `impls: Vec<Id>` - The implementations, inherent and of traits, on the
  primitive type.

**Traits:** Eq

**Trait Implementations:**

- **PartialEq**
    - `fn eq(self: &Self, other: &Primitive) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Primitive`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`

## rustdoc_types::ProcMacro

_Struct_

A procedural macro.

**Fields:**

- `kind: MacroKind` - How this macro is supposed to be called: `foo!()`,
  `#[foo]` or `#[derive(foo)]`
- `helpers: Vec<String>` - Helper attributes defined by a macro to be used
  inside it.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &ProcMacro) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> ProcMacro`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::ReprKind

_Enum_

The kind of `#[repr]`.

See [AttributeRepr::kind]`.

**Variants:**

- `Rust` - `#[repr(Rust)]`
- `C` - `#[repr(C)]`
- `Transparent` - `#[repr(transparent)]
- `Simd` - `#[repr(simd)]`

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &ReprKind) -> bool`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Clone**
    - `fn clone(self: &Self) -> ReprKind`

## rustdoc_types::Span

_Struct_

A range of source code.

**Fields:**

- `filename: std::path::PathBuf` - The path to the source file for this span
  relative to the path `rustdoc` was invoked with.
- `begin: (usize, usize)` - One indexed Line and Column of the first character
  of the `Span`.
- `end: (usize, usize)` - One indexed Line and Column of the last character of
  the `Span`.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Span) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Span`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`

## rustdoc_types::Static

_Struct_

A `static` declaration.

**Fields:**

- `type_: Type` - The type of the static.
- `is_mutable: bool` - This is `true` for mutable statics, declared as
  `static mut X: T = f();`
- `expr: String` - The stringified expression for the initial value.
- `is_unsafe: bool` - Is the static `unsafe`?

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Static) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Static`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::Struct

_Struct_

A `struct`.

**Fields:**

- `kind: StructKind` - The kind of the struct (e.g. unit, tuple-like or
  struct-like) and the data specific to it,
- `generics: Generics` - The generic parameters and where clauses on this
  struct.
- `impls: Vec<Id>` - All impls (both of traits and inherent) for this struct.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Struct`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Struct) -> bool`

## rustdoc_types::StructKind

_Enum_

The kind of a [`Struct`] and the data specific to it, i.e. fields.

**Variants:**

- `Unit` - A struct with no fields and no parentheses.
- `Tuple(Vec<Option<Id>>)` - A struct with unnamed fields.
- `Plain{ fields: Vec<Id>, has_stripped_fields: bool }` - A struct with named
  fields.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &StructKind) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> StructKind`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::Target

_Struct_

Information about a target

**Fields:**

- `triple: String` - The target triple for which this documentation was
  generated
- `target_features: Vec<TargetFeature>` - A list of features valid for use in
  `#[target_feature]` attributes

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Target) -> bool`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Clone**
    - `fn clone(self: &Self) -> Target`

## rustdoc_types::TargetFeature

_Struct_

Information about a target feature.

Rust target features are used to influence code generation, especially around
selecting instructions which are not universally supported by the target
architecture.

Target features are commonly enabled by the [`#[target_feature]` attribute][1]
to influence code generation for a particular function, and less commonly
enabled by compiler options like `-Ctarget-feature` or `-Ctarget-cpu`. Targets
themselves automatically enable certain target features by default, for example
because the target's ABI specification requires saving specific registers which
only exist in an architectural extension.

Target features can imply other target features: for example, x86-64 `avx2`
implies `avx`, and aarch64 `sve2` implies `sve`, since both of these
architectural extensions depend on their predecessors.

Target features can be probed at compile time by [`#[cfg(target_feature)]`][2]
or `cfg!(…)` conditional compilation to determine whether a target feature is
enabled in a particular context.

[1]:
    https://doc.rust-lang.org/stable/reference/attributes/codegen.html#the-target_feature-attribute
[2]:
    https://doc.rust-lang.org/reference/conditional-compilation.html#target_feature

**Fields:**

- `name: String` - The name of this target feature.
- `implies_features: Vec<String>` - Other target features which are implied by
  this target feature, if any.
- `unstable_feature_gate: Option<String>` - If this target feature is unstable,
  the name of the associated language feature gate.
- `globally_enabled: bool` - Whether this feature is globally enabled for this
  compilation session.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &TargetFeature) -> bool`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Clone**
    - `fn clone(self: &Self) -> TargetFeature`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::Term

_Enum_

Either a type or a constant, usually stored as the right-hand side of an
equation in places like [`AssocItemConstraint`]

**Variants:**

- `Type(Type)` - A type.
- `Constant(Constant)` - A constant.

**Traits:** Eq

**Trait Implementations:**

- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Term`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Term) -> bool`

## rustdoc_types::Trait

_Struct_

A `trait` declaration.

**Fields:**

- `is_auto: bool` - Whether the trait is marked `auto` and is thus implemented
  automatically
- `is_unsafe: bool` - Whether the trait is marked as `unsafe`.
- `is_dyn_compatible: bool` - Whether the trait is
  [dyn compatible](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)[^1].
- `items: Vec<Id>` - Associated [`Item`]s that can/must be implemented by the
  `impl` blocks.
- `generics: Generics` - Information about the type parameters and `where`
  clauses of the trait.
- `bounds: Vec<GenericBound>` - Constraints that must be met by the implementor
  of the trait.
- `implementations: Vec<Id>` - The implementations of the trait.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Trait) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Trait`

## rustdoc_types::TraitAlias

_Struct_

A trait alias declaration, e.g. `trait Int = Add + Sub + Mul + Div;`

See [the tracking issue](https://github.com/rust-lang/rust/issues/41517)

**Fields:**

- `generics: Generics` - Information about the type parameters and `where`
  clauses of the alias.
- `params: Vec<GenericBound>` - The bounds that are associated with the alias.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &TraitAlias) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> TraitAlias`

## rustdoc_types::TraitBoundModifier

_Enum_

A set of modifiers applied to a trait.

**Variants:**

- `None` - Marks the absence of a modifier.
- `Maybe` - Indicates that the trait bound relaxes a trait bound applied to a
  parameter by default,
- `MaybeConst` - Indicates that the trait bound must be applicable in both a
  run-time and a compile-time

**Traits:** Eq, Copy

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Clone**
    - `fn clone(self: &Self) -> TraitBoundModifier`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &TraitBoundModifier) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`

## rustdoc_types::Type

_Enum_

A type.

**Variants:**

- `ResolvedPath(Path)` - Structs, enums, unions and type aliases, e.g.
  `std::option::Option<u32>`
- `DynTrait(DynTrait)` - Dynamic trait object type (`dyn Trait`).
- `Generic(String)` - Parameterized types. The contained string is the name of
  the parameter.
- `Primitive(String)` - Built-in numeric types (e.g. `u32`, `f32`), `bool`,
  `char`.
- `FunctionPointer(Box<FunctionPointer>)` - A function pointer type, e.g.
  `fn(u32) -> u32`, `extern "C" fn() -> *const u8`
- `Tuple(Vec<Type>)` - A tuple type, e.g. `(String, u32, Box<usize>)`
- `Slice(Box<Type>)` - An unsized slice type, e.g. `[u32]`.
- `Array{ type_: Box<Type>, len: String }` - An array type, e.g. `[u32; 15]`
- `Pat{ type_: Box<Type> }` - A pattern type, e.g. `u32 is 1..`
- `ImplTrait(Vec<GenericBound>)` - An opaque type that satisfies a set of
  bounds, `impl TraitA + TraitB + ...`
- `Infer` - A type that's left to be inferred, `_`
- `RawPointer{ is_mutable: bool, type_: Box<Type> }` - A raw pointer type, e.g.
  `*mut u32`, `*const u8`, etc.
- `BorrowedRef{ lifetime: Option<String>, is_mutable: bool, type_: Box<Type> }` -
  `&'a mut String`, `&str`, etc.
- `QualifiedPath{ name: String, args: Option<Box<GenericArgs>>, self_type: Box<Type>, trait_: Option<Path> }` -
  Associated types like `<Type as Trait>::Name` and `T::Item` where

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Type) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Type`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::TypeAlias

_Struct_

A type alias declaration, e.g. `type Pig = std::borrow::Cow<'static, str>;`

**Fields:**

- `type_: Type` - The type referred to by this alias.
- `generics: Generics` - Information about the type parameters and `where`
  clauses of the alias.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &TypeAlias) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> TypeAlias`

## rustdoc_types::Union

_Struct_

A `union`.

**Fields:**

- `generics: Generics` - The generic parameters and where clauses on this union.
- `has_stripped_fields: bool` - Whether any fields have been removed from the
  result, due to being private or hidden.
- `fields: Vec<Id>` - The list of fields in the union.
- `impls: Vec<Id>` - All impls (both of traits and inherent) for this union.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Union) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Union`

## rustdoc_types::Use

_Struct_

A `use` statement.

**Fields:**

- `source: String` - The full path being imported.
- `name: String` - May be different from the last segment of `source` when
  renaming imports:
- `id: Option<Id>` - The ID of the item being imported. Will be `None` in case
  of re-exports of primitives:
- `is_glob: bool` - Whether this statement is a wildcard `use`, e.g.
  `use source::*;`

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Use) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Use`

## rustdoc_types::Variant

_Struct_

A variant of an enum.

**Fields:**

- `kind: VariantKind` - Whether the variant is plain, a tuple-like, or
  struct-like. Contains the fields.
- `discriminant: Option<Discriminant>` - The discriminant, if explicitly
  specified.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Variant) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Variant`

## rustdoc_types::VariantKind

_Enum_

The kind of an [`Enum`] [`Variant`] and the data specific to it, i.e. fields.

**Variants:**

- `Plain` - A variant with no parentheses
- `Tuple(Vec<Option<Id>>)` - A variant with unnamed fields.
- `Struct{ fields: Vec<Id>, has_stripped_fields: bool }` - A variant with named
  fields.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &VariantKind) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> VariantKind`

## rustdoc_types::Visibility

_Enum_

Visibility of an [`Item`].

**Variants:**

- `Public` - Explicitly public visibility set with `pub`.
- `Default` - For the most part items are private by default. The exceptions are
  associated items of
- `Crate` - Explicitly crate-wide visibility set with `pub(crate)`
- `Restricted{ parent: Id, path: String }` - For `pub(in path)` visibility.

**Traits:** Eq

**Trait Implementations:**

- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &Visibility) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> Visibility`
- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`

## rustdoc_types::WherePredicate

_Enum_

One `where` clause.

```rust
fn default<T>() -> T where T: Default { T::default() }
//                         ^^^^^^^^^^
```

**Variants:**

- `BoundPredicate{ type_: Type, bounds: Vec<GenericBound>, generic_params: Vec<GenericParamDef> }` -
  A type is expected to comply with a set of bounds
- `LifetimePredicate{ lifetime: String, outlives: Vec<String> }` - A lifetime is
  expected to outlive other lifetimes.
- `EqPredicate{ lhs: Type, rhs: Term }` - A type must exactly equal another
  type.

**Traits:** Eq

**Trait Implementations:**

- **Deserialize**
    - `fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, <__D as >::Error>`
- **Serialize**
    - `fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private228::Result<<__S as >::Ok, <__S as >::Error>`
- **Debug**
    - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
    - `fn eq(self: &Self, other: &WherePredicate) -> bool`
- **Hash**
    - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
    - `fn clone(self: &Self) -> WherePredicate`
