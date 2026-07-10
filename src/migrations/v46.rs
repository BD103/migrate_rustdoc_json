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
    declare_migrate_up, declare_serialize_deserialize, impl_unchanged_migrations,
    reporter::Reporter, traits::MigrateUp,
};

declare_migrate_up!(46, 48);
declare_serialize_deserialize!();

impl MigrateUp for current::Crate {
    type Up = up::Crate;

    fn migrate_up(self, reporter: &mut Reporter) -> Self::Up {
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
            root: root.migrate_up(reporter),
            crate_version: crate_version.migrate_up(reporter),
            includes_private: includes_private.migrate_up(reporter),
            index: index.migrate_up(reporter),
            paths: paths.migrate_up(reporter),
            external_crates: external_crates.migrate_up(reporter),
            target: target.migrate_up(reporter),
            // Bump the format version by 2, going from v46 to v48, since v47 does not exist.
            format_version: format_version + 2,
        }
    }
}

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
