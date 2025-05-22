#[macro_export]
macro_rules! impl_migrations {
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
                format_version
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
    {} => {};
}

#[macro_export]
macro_rules! impl_single_migration {
    {
        struct $struct:ident {
            $($field:ident),*
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
