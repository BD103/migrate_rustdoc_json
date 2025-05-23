use rustdoc_types_43 as current;
use rustdoc_types_44 as up;

use crate::{declare_migrate_up, declare_serialize_deserialize, migrate::MigrateUp};

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
                triple: String::new(),
                target_features: Vec::new(),
            },
            // Bump the format version.
            format_version: format_version + 1,
        }
    }
}

crate::impl_migrations! {
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
    GenericArgs,
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
