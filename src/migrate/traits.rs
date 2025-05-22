pub trait MigrateUp {
    type Up;

    fn migrate_up(self) -> Self::Up;
}

// pub trait MigrateDown {
//     type Down;

//     fn migrate_down(self) -> Self::Down;
// }
