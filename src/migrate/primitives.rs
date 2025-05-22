macro_rules! impl_primitive_migrations {
    {
        $($primitive:path),*
    } => {
        $(
            impl $crate::migrate::MigrateUp for $primitive {
                type Up = Self;

                fn migrate_up(self) -> Self::Up {
                    self
                }
            }
        )*
    };
}

impl_primitive_migrations! {
    u32
}
