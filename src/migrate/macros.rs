#[macro_export]
macro_rules! impl_migrations {
    { AssocItemConstraint, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct AssocItemConstraint {
                name,
                args,
                binding,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Constant, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Constant {
                expr,
                value,
                is_literal,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Crate, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Crate {
                root,
                crate_version,
                includes_private,
                index,
                paths,
                external_crates,
                target,
                format_version,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Deprecation, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Deprecation {
                since,
                note,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Discriminant, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Discriminant {
                expr,
                value,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { DynTrait, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct DynTrait {
                traits,
                lifetime,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Enum, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Enum {
                generics,
                has_stripped_variants,
                variants,
                impls,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { ExternalCrate, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct ExternalCrate {
                name,
                html_root_url,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Function, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Function {
                sig,
                generics,
                header,
                has_body,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { FunctionHeader, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct FunctionHeader {
                is_const,
                is_unsafe,
                is_async,
                abi,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { FunctionPointer, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct FunctionPointer {
                sig,
                generic_params,
                header,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { FunctionSignature, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct FunctionSignature {
                inputs,
                output,
                is_c_variadic,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { GenericParamDef, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct GenericParamDef {
                name,
                kind,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Generics, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Generics {
                params,
                where_predicates,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Id, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Id(id);
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Impl, $($tt:tt)* } => {
        $crate::impl_single_migration! {
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

        $crate::impl_migrations! { $($tt)* }
    };
    { Item, $($tt:tt)* } => {
        $crate::impl_single_migration! {
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

        $crate::impl_migrations! { $($tt)* }
    };
    { ItemSummary, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct ItemSummary {
                crate_id,
                path,
                kind,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Module, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Module {
                is_crate,
                items,
                is_stripped,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Path, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Path {
                path,
                id,
                args,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { PolyTrait, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct PolyTrait {
                trait_,
                generic_params,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Primitive, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Primitive {
                name,
                impls,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { ProcMacro, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct ProcMacro {
                kind,
                helpers,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Span, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Span {
                filename,
                begin,
                end,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Static, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Static {
                type_,
                is_mutable,
                expr,
                is_unsafe,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Struct, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Struct {
                kind,
                generics,
                impls,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Target, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Target {
                triple,
                target_features,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { TargetFeature, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct TargetFeature {
                name,
                implies_features,
                unstable_feature_gate,
                globally_enabled,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Trait, $($tt:tt)* } => {
        $crate::impl_single_migration! {
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

        $crate::impl_migrations! { $($tt)* }
    };
    { TraitAlias, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct TraitAlias {
                generics,
                params,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { TypeAlias, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct TypeAlias {
                type_,
                generics,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Union, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Union {
                generics,
                has_stripped_fields,
                fields,
                impls,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Use, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Use {
                source,
                name,
                id,
                is_glob,
            }

        }

        $crate::impl_migrations! { $($tt)* }
    };
    { Variant, $($tt:tt)* } => {
        $crate::impl_single_migration! {
            struct Variant {
                kind,
                discriminant,
            }
        }

        $crate::impl_migrations! { $($tt)* }
    };
    {} => {};
}

#[macro_export]
macro_rules! impl_single_migration {
    {
        struct $struct:ident {
            $($field:ident),* $(,)?
        }
    } => {
        impl $crate::migrate::MigrateUp for current::$struct {
            type Up = up::$struct;

            fn migrate_up(self) -> Self::Up {
                let current::$struct {
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
        impl $crate::migrate::MigrateUp for current::$struct {
            type Up = up::$struct;

            fn migrate_up(self) -> Self::Up {
                let current::$struct($($field),*) = self;

                up::$struct (
                    $($field.migrate_up()),*
                )
            }
        }
    };
}
