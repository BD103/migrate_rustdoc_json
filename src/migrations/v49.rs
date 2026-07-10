//! **v49 to v50 migration.**
//!
//! The `#[cold]` attribute in `Item::attrs` is now represented as `#[attr = Cold]`.
//!
//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0500---2025-06-23>

use rustdoc_types_49 as current;
use rustdoc_types_50 as up;

use crate::{
    declare_migrate_up, declare_serialize_deserialize, impl_unchanged_migrations,
    reporter::Reporter, traits::MigrateUp,
};

declare_migrate_up!(49, 50);
declare_serialize_deserialize!();

impl MigrateUp for current::Item {
    type Up = up::Item;

    fn migrate_up(self, reporter: &mut Reporter) -> Self::Up {
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
            id: id.migrate_up(reporter),
            crate_id: crate_id.migrate_up(reporter),
            name: name.migrate_up(reporter),
            span: span.migrate_up(reporter),
            visibility: visibility.migrate_up(reporter),
            docs: docs.migrate_up(reporter),
            links: links.migrate_up(reporter),
            attrs: attrs.migrate_up(reporter),
            deprecation: deprecation.migrate_up(reporter),
            inner: inner.migrate_up(reporter),
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
