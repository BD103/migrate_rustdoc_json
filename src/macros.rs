//! The macros used by this crate.

/// Implements [`MigrateUp`](crate::traits::MigrateUp) for `rustdoc_types` types that do not
/// change in this version.
///
/// This macro requires that you import the current version of `rustdoc_types` as the name
/// `current` and the newer version as the name `up`.
///
/// Each supported type has a separate branch in this macro, meaning you can view a list of all
/// supported types in Rustdoc's auto-generated reference. Note that multiple versions of the
/// same type may be supported, such as `Crate` and `Crate@v44`. The "unversioned" branch is for
/// the oldest supported version of `rustdoc_types`, while the "versioned" branch is for the
/// specified version (v44) and later.
///
/// # Example
///
/// ```
/// use rustdoc_types_41 as current;
/// use rustdoc_types_42 as up;
///
/// use migrate_rustdoc_json::impl_unchanged_migrations;
///
/// impl_unchanged_migrations! {
///     Crate,
///     Constant,
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! impl_unchanged_migrations {
    // A custom `Crate` implementation that updates `format_version`.
    { Crate, $($tt:tt)* } => {
        impl $crate::traits::MigrateUp for current::Crate {
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
                    // Bump the format version.
                    format_version: format_version + 1,
                }
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Crate@v44, $($tt:tt)* } => {
        impl $crate::traits::MigrateUp for current::Crate {
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
                    // Bump the format version.
                    format_version: format_version + 1,
                }
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };

    // All structs, sorted alphabetically
    { AssocItemConstraint, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct AssocItemConstraint {
                name,
                args,
                binding,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Constant, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Constant {
                expr,
                value,
                is_literal,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Deprecation, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Deprecation {
                since,
                note,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Discriminant, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Discriminant {
                expr,
                value,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { DynTrait, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct DynTrait {
                traits,
                lifetime,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Enum, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Enum {
                generics,
                has_stripped_variants,
                variants,
                impls,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { ExternalCrate, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct ExternalCrate {
                name,
                html_root_url,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Function, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Function {
                sig,
                generics,
                header,
                has_body,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { FunctionHeader, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct FunctionHeader {
                is_const,
                is_unsafe,
                is_async,
                abi,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { FunctionPointer, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct FunctionPointer {
                sig,
                generic_params,
                header,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { FunctionSignature, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct FunctionSignature {
                inputs,
                output,
                is_c_variadic,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { GenericParamDef, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct GenericParamDef {
                name,
                kind,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Generics, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Generics {
                params,
                where_predicates,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Id, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Id(id);
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Impl, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Impl {
                is_unsafe,
                generics,
                provided_trait_methods,
                trait_,
                for_,
                items,
                is_negative,
                is_synthetic,
                blanket_impl,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Item, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Item {
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
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { ItemSummary, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct ItemSummary {
                crate_id,
                path,
                kind,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Module, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Module {
                is_crate,
                items,
                is_stripped,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Path, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Path {
                path,
                id,
                args,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { PolyTrait, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct PolyTrait {
                trait_,
                generic_params,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Primitive, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Primitive {
                name,
                impls,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { ProcMacro, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct ProcMacro {
                kind,
                helpers,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Span, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Span {
                filename,
                begin,
                end,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Static, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Static {
                type_,
                is_mutable,
                expr,
                is_unsafe,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Struct, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Struct {
                kind,
                generics,
                impls,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Target@v44, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Target {
                triple,
                target_features,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { TargetFeature@v44, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct TargetFeature {
                name,
                implies_features,
                unstable_feature_gate,
                globally_enabled,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Trait, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Trait {
                is_auto,
                is_unsafe,
                is_dyn_compatible,
                items,
                generics,
                bounds,
                implementations,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { TraitAlias, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct TraitAlias {
                generics,
                params,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { TypeAlias, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct TypeAlias {
                type_,
                generics,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Union, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Union {
                generics,
                has_stripped_fields,
                fields,
                impls,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Use, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Use {
                source,
                name,
                id,
                is_glob,
            }

        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Variant, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            struct Variant {
                kind,
                discriminant,
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };

    // All enums, sorted alphabetically
    { Abi, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum Abi {
                "struct" {
                    C { unwind },
                    Cdecl { unwind },
                    Stdcall { unwind },
                    Fastcall { unwind },
                    Aapcs { unwind },
                    Win64 { unwind },
                    SysV64 { unwind },
                    System { unwind },
                },
                "tuple" {
                    Other(string),
                },
                "unit" {
                    Rust,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { AssocItemConstraintKind, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum AssocItemConstraintKind {
                "tuple" {
                    Equality(term),
                    Constraint(generic_bounds),
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { GenericArg, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum GenericArg {
                "tuple" {
                    Lifetime(string),
                    Type(type_),
                    Const(constant),
                },
                "unit" {
                    Infer,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { GenericArgs, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum GenericArgs {
                "struct" {
                    AngleBracketed {
                        args,
                        constraints,
                    },
                    Parenthesized {
                        inputs,
                        output,
                    },
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { GenericArgs@v42, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum GenericArgs {
                "struct" {
                    AngleBracketed {
                        args,
                        constraints,
                    },
                    Parenthesized {
                        inputs,
                        output,
                    },
                },
                "unit" {
                    ReturnTypeNotation,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { GenericBound, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum GenericBound {
                "struct" {
                    TraitBound {
                        trait_,
                        generic_params,
                        modifier,
                    },
                },
                "tuple" {
                    Outlives(string),
                    Use(precise_capturing_args),
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { GenericParamDefKind, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum GenericParamDefKind {
                "struct" {
                    Lifetime {
                        outlives,
                    },
                    Type {
                        bounds,
                        default,
                        is_synthetic,
                    },
                    Const {
                        type_,
                        default,
                    },
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { ItemEnum, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum ItemEnum {
                "struct" {
                    ExternCrate {
                        name,
                        rename,
                    },
                    Constant {
                        type_,
                        const_,
                    },
                    AssocConst {
                        type_,
                        value,
                    },
                    AssocType {
                        generics,
                        bounds,
                        type_,
                    },
                },
                "tuple" {
                    Module(module),
                    Use(use_),
                    Union(union_),
                    Struct(struct_),
                    StructField(type_),
                    Enum(enum_),
                    Variant(variant),
                    Function(function),
                    Trait(trait_),
                    TraitAlias(trait_alias),
                    Impl(impl_),
                    TypeAlias(type_alias),
                    Static(static_),
                    Macro(string),
                    ProcMacro(proc_macro),
                    Primitive(primitive),
                },
                "unit" {
                    ExternType,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { ItemKind, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum ItemKind {
                "unit" {
                    Module,
                    ExternCrate,
                    Use,
                    Struct,
                    StructField,
                    Union,
                    Enum,
                    Variant,
                    Function,
                    TypeAlias,
                    Constant,
                    Trait,
                    TraitAlias,
                    Impl,
                    Static,
                    ExternType,
                    Macro,
                    ProcAttribute,
                    ProcDerive,
                    AssocConst,
                    AssocType,
                    Primitive,
                    Keyword,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { MacroKind, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum MacroKind {
                "unit" {
                    Bang,
                    Attr,
                    Derive,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { PreciseCapturingArg, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum PreciseCapturingArg {
                "tuple" {
                    Lifetime(string),
                    Param(string),
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { StructKind, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum StructKind {
                "struct" {
                    Plain {
                        fields,
                        has_stripped_fields,
                    },
                },
                "tuple" {
                    Tuple(ids),
                },
                "unit" {
                    Unit,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Term, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum Term {
                "tuple" {
                    Type(type_),
                    Constant(constant),
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { TraitBoundModifier, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum TraitBoundModifier {
                "unit" {
                    None,
                    Maybe,
                    MaybeConst,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Type, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum Type {
                "struct" {
                    Array {
                        type_,
                        len,
                    },
                    Pat {
                        type_,
                        // HACK: This could break at any time, so be warned.
                        __pat_unstable_do_not_use,
                    },
                    RawPointer {
                        is_mutable,
                        type_,
                    },
                    BorrowedRef {
                        lifetime,
                        is_mutable,
                        type_,
                    },
                    QualifiedPath {
                        name,
                        args,
                        self_type,
                        trait_,
                    },
                },
                "tuple" {
                    ResolvedPath(path),
                    DynTrait(dyn_trait),
                    Generic(string),
                    Primitive(string),
                    FunctionPointer(function_pointer),
                    Tuple(types),
                    Slice(type_),
                    ImplTrait(generic_bounds),
                },
                "unit" {
                    Infer,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { VariantKind, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum VariantKind {
                "struct" {
                    Struct {
                        fields,
                        has_stripped_fields,
                    },
                },
                "tuple" {
                    Tuple(ids),
                },
                "unit" {
                    Plain,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { Visibility, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum Visibility {
                "struct" {
                    Restricted {
                        parent,
                        path,
                    },
                },
                "unit" {
                    Public,
                    Default,
                    Crate,
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };
    { WherePredicate, $($tt:tt)* } => {
        $crate::impl_single_unchanged_migration! {
            enum WherePredicate {
                "struct" {
                    BoundPredicate {
                        type_,
                        bounds,
                        generic_params,
                    },
                    LifetimePredicate {
                        lifetime,
                        outlives,
                    },
                    EqPredicate {
                        lhs,
                        rhs,
                    },
                },
            }
        }

        $crate::impl_unchanged_migrations! { $($tt)* }
    };

    {} => {};
}

/// Implements [`MigrateUp`](crate::traits::MigrateUp) for a single type that does not change in
/// this version.
///
/// You likely want to use [`impl_unchanged_migrations!`] instead of this macro.
///
/// This macro uses a pseudo-Rust DSL that specifies the type kind, field names, and variant names
/// of a type.
///
/// # Examples
///
/// ```
/// use rustdoc_types_41 as current;
/// use rustdoc_types_42 as up;
///
/// use migrate_rustdoc_json::impl_single_unchanged_migration;
///
/// impl_single_unchanged_migration! {
///     struct NamedStruct {
///         field_a,
///         field_b,
///     }
/// }
/// ```
///
/// ```
/// use rustdoc_types_41 as current;
/// use rustdoc_types_42 as up;
///
/// use migrate_rustdoc_json::impl_single_unchanged_migration;
///
/// impl_single_unchanged_migration! {
///     struct NamedTuple(field_a, field_b);
/// }
/// ```
///
/// ```
/// use rustdoc_types_41 as current;
/// use rustdoc_types_42 as up;
///
/// use migrate_rustdoc_json::impl_single_unchanged_migration;
///
/// impl_single_unchanged_migration! {
///     enum Enum {
///         // Note that order matters here.
///         "struct" {
///             StructVariant {
///                 field_a,
///                 field_b,
///             },
///         },
///         "tuple" {
///             TupleVariantA(field_c, field_d),
///             TupleVariantB(field_e, field_f),
///         },
///         "unit" {
///             UnitVariant,
///         },
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_single_unchanged_migration {
    {
        struct $struct:ident {
            $($field:ident),* $(,)?
        }
    } => {
        impl $crate::traits::MigrateUp for current::$struct {
            type Up = up::$struct;

            fn migrate_up(self) -> Self::Up {
                let Self {
                    $($field),*
                } = self;

                up::$struct {
                    $($field: $field.migrate_up()),*
                }
            }
        }
    };
    {
        struct $struct:ident($($field:ident),*);
    } => {
        impl $crate::traits::MigrateUp for current::$struct {
            type Up = up::$struct;

            fn migrate_up(self) -> Self::Up {
                let Self($($field),*) = self;

                up::$struct (
                    $($field.migrate_up()),*
                )
            }
        }
    };
    {
        enum $enum:ident {
            $(
                "struct" {
                    $(
                        $struct_variant:ident {
                            $($struct_field:ident),* $(,)?
                        }
                    ),* $(,)?
                },
            )?
            $(
                "tuple" {
                    $(
                        $tuple_variant:ident($($tuple_field:ident),*)
                    ),* $(,)?
                },
            )?
            $(
                "unit" {
                    $(
                        $unit_variant:ident
                    ),* $(,)?
                },
            )?
        }
    } => {
        impl $crate::traits::MigrateUp for current::$enum {
            type Up = up::$enum;

            fn migrate_up(self) -> Self::Up {
                match self {
                    $($(
                        Self::$struct_variant {
                            $($struct_field),*
                        } => up::$enum::$struct_variant {
                            $($struct_field: $struct_field.migrate_up()),*
                        },
                    )*)?

                    $($(
                        Self::$tuple_variant($($tuple_field),*) => up::$enum::$tuple_variant(
                            $($tuple_field.migrate_up()),*
                        ),
                    )*)?

                    $($(
                        Self::$unit_variant => up::$enum::$unit_variant,
                    )*)?
                }
            }
        }
    };
}

/// Declares the `migrate_up()` function for a given migration.
///
/// This macro accepts two parameter: the format version of the current `rustdoc_types` and the
/// format version of the migrated `rustdoc_types`.
///
/// # Example
///
/// ```
/// use rustdoc_types_41 as current;
/// use rustdoc_types_42 as up;
///
/// use migrate_rustdoc_json::declare_migrate_up;
///
/// declare_migrate_up!(41, 42);
/// ```
#[macro_export]
macro_rules! declare_migrate_up {
    ($current:literal, $up:literal) => {
        #[doc = concat!("Migrates a v", $current, " `Crate` to a ", $up, " `Crate`.")]
        ///
        /// # Safety
        ///
        #[doc = concat!("`current_crate` must be a [`rustdoc_types_", $current, "::Crate`] put in a [`Box`] then converted to a raw")]
        /// pointer with [`Box::into_raw()`].
        ///
        #[doc = concat!("The returned raw pointer is a [`rustdoc_types_", $up, "::Crate`] put in a [`Box`] then converted to a")]
        /// raw pointer with [`Box::into_raw()`].
        pub fn migrate_up(current_crate: ::std::boxed::Box<dyn ::std::any::Any>) -> ::anyhow::Result<::std::boxed::Box<dyn ::std::any::Any>> {
            use $crate::traits::MigrateUp;

            let current_crate = current_crate.downcast::<current::Crate>().unwrap();

            let up_crate = (*current_crate).migrate_up();

            Ok(::std::boxed::Box::new(up_crate))
        }
    };
}

/// Declares the `serialize()` and `deserialize()` functions for a given migration.
///
/// # Example
///
/// ```
/// use rustdoc_types_41 as current;
/// use rustdoc_types_42 as up;
///
/// use migrate_rustdoc_json::declare_serialize_deserialize;
///
/// declare_serialize_deserialize!();
/// ```
#[macro_export]
macro_rules! declare_serialize_deserialize {
    () => {
        pub fn deserialize(
            current_crate: &str,
        ) -> ::anyhow::Result<::std::boxed::Box<dyn ::std::any::Any>> {
            use ::anyhow::Context;

            let current_crate: current::Crate = ::serde_json::from_str(current_crate)
                .context("failed to deserialize `Crate` from JSON")?;

            Ok(::std::boxed::Box::new(current_crate))
        }

        pub fn serialize(
            current_crate: ::std::boxed::Box<dyn ::std::any::Any>,
        ) -> ::anyhow::Result<String> {
            use ::anyhow::Context;

            let current_crate = current_crate.downcast::<current::Crate>().unwrap();

            ::serde_json::to_string(current_crate.as_ref())
                .context("failed to serialize `Crate` to JSON")
        }
    };
}
