//! **v43 to v44 migration.**
//!
//! The `Crate::target` field was added. This migration currently leaves the target tuple and
//! target features empty, however in the future it may fill in these values with [defaults
//! specified in #139393](https://github.com/rust-lang/rust/pull/139393).
//!
//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0400---2025-04-19>

use rustdoc_types_43 as current;
use rustdoc_types_44 as up;

use crate::{declare_migrate_up, declare_serialize_deserialize, traits::MigrateUp};

declare_migrate_up!(43, 44);
declare_serialize_deserialize!();

/// The `Crate::target` field was added, and defaults to be blank.
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
            format_version,
        } = self;

        up::Crate {
            root: root.migrate_up(),
            crate_version: crate_version.migrate_up(),
            includes_private: includes_private.migrate_up(),
            index: index.migrate_up(),
            paths: paths.migrate_up(),
            external_crates: external_crates.migrate_up(),
            target: up::Target {
                // Currently we leave the target triple empty, as we can only guess if the JSON was
                // built on the same device migrating it.
                triple: String::new(),
                target_features: Vec::new(),
            },
            // Bump the format version.
            format_version: format_version + 1,
        }
    }
}

crate::impl_unchanged_migrations! {
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
