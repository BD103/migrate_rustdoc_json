//! **v45 to v46 migration.**
//!
//! Rustdoc used to only include `#[repr(transparent)]` in its output if there was at least one
//! public ZST in a type's signature. While this makes sense for the HTML output (don't include
//! `#[repr(transparent)]` if the user can't do anything with the information), it doesn't for the
//! JSON output. In v46, Rustdoc now always includes `#[repr(transparent)]` for types that use it,
//! no matter what visibility their fields are.
//!
//! Unfortunately, this migration is unable to add back `#[repr(transparent)]` if it was excluded
//! from the output, as `migrate_rustdoc_types` won't know it had the attribute in the first place!
//! Because of this fact, this migration does nothing beyond change the format version.
//!
//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0460---2025-06-03>

use rustdoc_types_45 as current;
use rustdoc_types_46 as up;

use crate::{declare_migrate_up, declare_serialize_deserialize, impl_unchanged_migrations};

declare_migrate_up!(45, 46);
declare_serialize_deserialize!();

impl_unchanged_migrations! {
    Crate@v44,
    AssocItemConstraint,
    Constant,
    Deprecation,
    Discriminant,
    DynTrait,
    Enum,
    ExternalCrate,
    Function,
    FunctionHeader,
    FunctionPointer,
    FunctionSignature,
    GenericParamDef,
    Generics,
    Id,
    Impl,
    Item,
    ItemSummary,
    Module,
    Path,
    PolyTrait,
    Primitive,
    ProcMacro,
    Span,
    Static,
    Struct,
    Target@v44,
    TargetFeature@v44,
    Trait,
    TraitAlias,
    TypeAlias,
    Union,
    Use,
    Variant,
    Abi,
    AssocItemConstraintKind,
    GenericArg,
    GenericArgs@v42,
    GenericBound,
    GenericParamDefKind,
    ItemEnum,
    ItemKind,
    MacroKind,
    PreciseCapturingArg,
    StructKind,
    Term,
    TraitBoundModifier,
    Type,
    VariantKind,
    Visibility,
    WherePredicate,
}
