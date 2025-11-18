//! **v46 to v48 migration.**
//!
//! The format for the `#[inline]` family of attributes in `Item::attrs` has changed:
//!
//! |Before|After|
//! |-|-|
//! |`#[inline]`|`#[attr = Inline(Hint)]`|
//! |`#[inline(always)]`|`#[attr = Inline(Always)]`|
//! |`#[inline(never)]`|`#[attr = Inline(Never)]`|
//!
//! Note that this migration skips v47, as it doesn't exist.
//!
//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0480---2025-06-19>

use rustdoc_types_46 as current;
use rustdoc_types_48 as up;

use crate::{
    declare_migrate_up, declare_serialize_deserialize, impl_unchanged_migrations, traits::MigrateUp,
};

declare_migrate_up!(46, 48);
declare_serialize_deserialize!();

impl MigrateUp for current::Crate {
    type Up = up::Crate;

    fn migrate_up(self) -> Self::Up {
        let Self {
            root,
            crate_version,
            includes_private,
            index,
            paths,
            external_crates,
            target,
            format_version,
        } = self;

        up::Crate {
            root: root.migrate_up(),
            crate_version: crate_version.migrate_up(),
            includes_private: includes_private.migrate_up(),
            index: index.migrate_up(),
            paths: paths.migrate_up(),
            external_crates: external_crates.migrate_up(),
            target: target.migrate_up(),
            // Bump the format version by 2, going from v46 to v48, since v47 does not exist.
            format_version: format_version + 2,
        }
    }
}

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

        for attr in attrs.iter_mut() {
            // If the attribute is some form of `#[inline]`, replace it with its structured form
            // instead.
            match attr.as_str() {
                "#[inline]" => *attr = String::from("#[attr = Inline(Hint)]"),
                "#[inline(always)]" => *attr = String::from("#[attr = Inline(Always)]"),
                "#[inline(never)]" => *attr = String::from("#[attr = Inline(Never)]"),
                _ => {}
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
