//! **v52 to v53 migrations.**
//! 
//! The format version was bumped due to a mistake, no schema changes were made.
//! 
//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0530---2025-06-23>

use rustdoc_types_52 as current;
use rustdoc_types_53 as up;

use crate::{
    declare_migrate_up, declare_serialize_deserialize, impl_unchanged_migrations,
};

declare_migrate_up!(52, 53);
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
