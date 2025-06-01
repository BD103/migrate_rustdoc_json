//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0390---2025-03-24>

use rustdoc_types_42 as current;
use rustdoc_types_43 as up;

use crate::{declare_migrate_up, declare_serialize_deserialize, traits::MigrateUp};

declare_migrate_up!(42, 43);
declare_serialize_deserialize!();

impl MigrateUp for current::Item {
    type Up = up::Item;

    fn migrate_up(self) -> Self::Up {
        let current::Item {
            id,
            crate_id,
            name,
            span,
            visibility,
            docs,
            links,
            attrs,
            deprecation,
            inner,
        } = self;

        let attrs = attrs.into_iter().map(pretty_print_attr).collect();

        up::Item {
            id: id.migrate_up(),
            crate_id,
            name,
            span: span.migrate_up(),
            visibility: visibility.migrate_up(),
            docs,
            links: links.migrate_up(),
            attrs,
            deprecation: deprecation.migrate_up(),
            inner: inner.migrate_up(),
        }
    }
}

fn pretty_print_attr(debug_printed: String) -> String {
    let Some(reprs) = debug_printed.strip_prefix("#[attr = Repr([") else {
        return debug_printed;
    };

    let Some(reprs) = reprs.strip_suffix("])]\n") else {
        return debug_printed;
    };

    reprs.to_string()
}

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
