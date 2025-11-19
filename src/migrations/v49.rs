//! **v49 to v50 migration.**
//!
//! The `#[cold]` attribute in `Item::attrs` is now represented as `#[attr = Cold]`.
//!
//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0500---2025-06-23>

use rustdoc_types_49 as current;
use rustdoc_types_50 as up;

use crate::{
    declare_migrate_up, declare_serialize_deserialize, impl_unchanged_migrations, traits::MigrateUp,
};

declare_migrate_up!(49, 50);
declare_serialize_deserialize!();

impl MigrateUp for current::Item {
    type Up = up::Item;

    fn migrate_up(self) -> Self::Up {
        let Self {
            id,
            crate_id,
            name,
            span,
            visibility,
            docs,
            links,
            mut attrs,
            deprecation,
            inner,
        } = self;

        for attr in &mut attrs {
            if attr == "#[cold]" {
                *attr = String::from("#[attr = Cold]");
            }
        }

        up::Item {
            id: id.migrate_up(),
            crate_id: crate_id.migrate_up(),
            name: name.migrate_up(),
            span: span.migrate_up(),
            visibility: visibility.migrate_up(),
            docs: docs.migrate_up(),
            links: links.migrate_up(),
            attrs: attrs.migrate_up(),
            deprecation: deprecation.migrate_up(),
            inner: inner.migrate_up(),
        }
    }
}

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
