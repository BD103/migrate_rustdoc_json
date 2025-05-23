use rustdoc_types_44 as current;
use rustdoc_types_45 as up;

use crate::{declare_migration_fns, migrate::MigrateUp};

declare_migration_fns!(44, 45);

/// The column's index used to start at zero, now it starts at one.
impl MigrateUp for current::Span {
    type Up = up::Span;

    fn migrate_up(self) -> Self::Up {
        let Self {
            filename,
            begin: (begin_line, begin_col),
            end: (end_line, end_col),
        } = self;

        up::Span {
            filename,
            begin: (begin_line, begin_col + 1),
            end: (end_line, end_col + 1),
        }
    }
}

crate::impl_migrations! {
    AssocItemConstraint,
    Constant,
    Crate,
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
    Static,
    Struct,
    Target,
    TargetFeature,
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
