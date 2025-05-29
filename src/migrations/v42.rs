//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0390---2025-03-24>
//! 
//! Currently attributes are left as-is in their debug format. We may be able to reformat them into
//! their pretty-printed variants.

use rustdoc_types_42 as current;
use rustdoc_types_43 as up;

use crate::{declare_migrate_up, declare_serialize_deserialize};

declare_migrate_up!(42, 43);
declare_serialize_deserialize!();

crate::impl_unchanged_migrations! {
    Crate,
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
