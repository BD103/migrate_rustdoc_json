//! **v48 to v49 migration.**
//!
//! The format for the `#[optimize]` family of attributes in `Item::attrs` has changed:
//!
//! |Before|After|
//! |-|-|
//! |`#[optimize(none)]`|`#[attr = Optimize(DoNotOptimize)]`|
//! |`#[optimize(speed)]`|`#[attr = Optimize(Speed)]`|
//! |`#[optimize(size)]`|`#[attr = Optimize(Size)]`|
//!
//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0490---2025-06-23>

use rustdoc_types_48 as current;
use rustdoc_types_49 as up;

use crate::{
    declare_migrate_up, declare_serialize_deserialize, impl_unchanged_migrations,
    reporter::Reporter, traits::MigrateUp,
};

declare_migrate_up!(48, 49);
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
            if let Some(optimize) = attr
                .strip_prefix("#[optimize(")
                .and_then(|attr| attr.strip_suffix(")]"))
            {
                match optimize {
                    "none" => *attr = String::from("#[attr = Optimize(DoNotOptimize)]"),
                    "speed" => *attr = String::from("#[attr = Optimize(Speed)]"),
                    "size" => *attr = String::from("#[attr = Optimize(Size)]"),
                    _ => unreachable!(
                        "attribute {attr} does not match expected `#[optimize(none|speed|size)]`",
                    ),
                }
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
