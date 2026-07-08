//! **v50 to v51 migration.**
//!
//! `AssocItemConstraint::args` is now boxed and optional. `Type::QualifiedPath::args` is now
//! optional (it was already boxed).
//!
//! <https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md#v0510---2025-06-23>

use rustdoc_types_50 as current;
use rustdoc_types_51 as up;

use crate::{
    declare_migrate_up, declare_serialize_deserialize, impl_unchanged_migrations, reporter::Reporter, traits::MigrateUp,
};

declare_migrate_up!(50, 51);
declare_serialize_deserialize!();

impl MigrateUp for current::AssocItemConstraint {
    type Up = up::AssocItemConstraint;

    fn migrate_up(self, reporter: &mut Reporter) -> Self::Up {
        let Self { name, args, binding } = self;

        up::AssocItemConstraint {
            name: name.migrate_up(reporter),
            args: Some(Box::new(args.migrate_up(reporter))),
            binding: binding.migrate_up(reporter),
        }
    }
}

impl MigrateUp for current::Type {
    type Up = up::Type;

    fn migrate_up(self, reporter: &mut Reporter) -> Self::Up {
        match self {
            Self::Array { type_, len } => up::Type::Array {
                type_: type_.migrate_up(reporter),
                len: len.migrate_up(reporter),
            },
            Self::Pat {
                type_,
                __pat_unstable_do_not_use,
            } => up::Type::Pat {
                type_: type_.migrate_up(reporter),
                __pat_unstable_do_not_use: __pat_unstable_do_not_use.migrate_up(reporter),
            },
            Self::RawPointer { is_mutable, type_ } => up::Type::RawPointer {
                is_mutable: is_mutable.migrate_up(reporter),
                type_: type_.migrate_up(reporter),
            },
            Self::BorrowedRef {
                lifetime,
                is_mutable,
                type_,
            } => up::Type::BorrowedRef {
                lifetime: lifetime.migrate_up(reporter),
                is_mutable: is_mutable.migrate_up(reporter),
                type_: type_.migrate_up(reporter),
            },
            Self::QualifiedPath {
                name,
                args,
                self_type,
                trait_,
            } => up::Type::QualifiedPath {
                name: name.migrate_up(reporter),
                args: Some(args.migrate_up(reporter)),
                self_type: self_type.migrate_up(reporter),
                trait_: trait_.migrate_up(reporter),
            },
            Self::ResolvedPath(path) => up::Type::ResolvedPath(path.migrate_up(reporter)),
            Self::DynTrait(dyn_trait) => up::Type::DynTrait(dyn_trait.migrate_up(reporter)),
            Self::Generic(string) => up::Type::Generic(string.migrate_up(reporter)),
            Self::Primitive(string) => up::Type::Primitive(string.migrate_up(reporter)),
            Self::FunctionPointer(function_pointer) => {
                up::Type::FunctionPointer(function_pointer.migrate_up(reporter))
            }
            Self::Tuple(types) => up::Type::Tuple(types.migrate_up(reporter)),
            Self::Slice(type_) => up::Type::Slice(type_.migrate_up(reporter)),
            Self::ImplTrait(generic_bounds) => up::Type::ImplTrait(generic_bounds.migrate_up(reporter)),
            Self::Infer => up::Type::Infer,
        }
    }
}

impl_unchanged_migrations! {
    Crate@v44,
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
    VariantKind,
    Visibility,
    WherePredicate,
}
