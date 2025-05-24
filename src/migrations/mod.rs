mod _41;
mod _42;
mod _43;
mod _44;
mod _45;

type MigrateUpFn = unsafe fn(*mut ()) -> *mut ();
type DeserializeFn = fn(&str) -> *mut ();
type SerializeFn = unsafe fn(*mut ()) -> String;

/// A function lookup table for migrating `rustdoc` JSON from one version to another.
///
/// `MIGRATIONS[0]` migrates v1 to v2, `MIGRATIONS[1]` migrates v2 to v3, etc.
static MIGRATIONS: [(MigrateUpFn, DeserializeFn, SerializeFn); 45] = [
    (
        unimplemented_migrate_up::<1>,
        unimplemented_deserialize::<1>,
        unimplemented_serialize::<1>,
    ),
    (
        unimplemented_migrate_up::<2>,
        unimplemented_deserialize::<2>,
        unimplemented_serialize::<2>,
    ),
    (
        unimplemented_migrate_up::<3>,
        unimplemented_deserialize::<3>,
        unimplemented_serialize::<3>,
    ),
    (
        unimplemented_migrate_up::<4>,
        unimplemented_deserialize::<4>,
        unimplemented_serialize::<4>,
    ),
    (
        unimplemented_migrate_up::<5>,
        unimplemented_deserialize::<5>,
        unimplemented_serialize::<5>,
    ),
    (
        unimplemented_migrate_up::<6>,
        unimplemented_deserialize::<6>,
        unimplemented_serialize::<6>,
    ),
    (
        unimplemented_migrate_up::<7>,
        unimplemented_deserialize::<7>,
        unimplemented_serialize::<7>,
    ),
    (
        unimplemented_migrate_up::<8>,
        unimplemented_deserialize::<8>,
        unimplemented_serialize::<8>,
    ),
    (
        unimplemented_migrate_up::<9>,
        unimplemented_deserialize::<9>,
        unimplemented_serialize::<9>,
    ),
    (
        unimplemented_migrate_up::<10>,
        unimplemented_deserialize::<10>,
        unimplemented_serialize::<10>,
    ),
    (
        unimplemented_migrate_up::<11>,
        unimplemented_deserialize::<11>,
        unimplemented_serialize::<11>,
    ),
    (
        unimplemented_migrate_up::<12>,
        unimplemented_deserialize::<12>,
        unimplemented_serialize::<12>,
    ),
    (
        unimplemented_migrate_up::<13>,
        unimplemented_deserialize::<13>,
        unimplemented_serialize::<13>,
    ),
    (
        unimplemented_migrate_up::<14>,
        unimplemented_deserialize::<14>,
        unimplemented_serialize::<14>,
    ),
    (
        unimplemented_migrate_up::<15>,
        unimplemented_deserialize::<15>,
        unimplemented_serialize::<15>,
    ),
    (
        unimplemented_migrate_up::<16>,
        unimplemented_deserialize::<16>,
        unimplemented_serialize::<16>,
    ),
    (
        unimplemented_migrate_up::<17>,
        unimplemented_deserialize::<17>,
        unimplemented_serialize::<17>,
    ),
    (
        unimplemented_migrate_up::<18>,
        unimplemented_deserialize::<18>,
        unimplemented_serialize::<18>,
    ),
    (
        unimplemented_migrate_up::<19>,
        unimplemented_deserialize::<19>,
        unimplemented_serialize::<19>,
    ),
    (
        unimplemented_migrate_up::<20>,
        unimplemented_deserialize::<20>,
        unimplemented_serialize::<20>,
    ),
    (
        unimplemented_migrate_up::<21>,
        unimplemented_deserialize::<21>,
        unimplemented_serialize::<21>,
    ),
    (
        unimplemented_migrate_up::<22>,
        unimplemented_deserialize::<22>,
        unimplemented_serialize::<22>,
    ),
    (
        unimplemented_migrate_up::<23>,
        unimplemented_deserialize::<23>,
        unimplemented_serialize::<23>,
    ),
    (
        unimplemented_migrate_up::<24>,
        unimplemented_deserialize::<24>,
        unimplemented_serialize::<24>,
    ),
    (
        unimplemented_migrate_up::<25>,
        unimplemented_deserialize::<25>,
        unimplemented_serialize::<25>,
    ),
    (
        unimplemented_migrate_up::<26>,
        unimplemented_deserialize::<26>,
        unimplemented_serialize::<26>,
    ),
    (
        unimplemented_migrate_up::<27>,
        unimplemented_deserialize::<27>,
        unimplemented_serialize::<27>,
    ),
    (
        unimplemented_migrate_up::<28>,
        unimplemented_deserialize::<28>,
        unimplemented_serialize::<28>,
    ),
    (
        unimplemented_migrate_up::<29>,
        unimplemented_deserialize::<29>,
        unimplemented_serialize::<29>,
    ),
    (
        unimplemented_migrate_up::<30>,
        unimplemented_deserialize::<30>,
        unimplemented_serialize::<30>,
    ),
    (
        unimplemented_migrate_up::<31>,
        unimplemented_deserialize::<31>,
        unimplemented_serialize::<31>,
    ),
    (
        unimplemented_migrate_up::<32>,
        unimplemented_deserialize::<32>,
        unimplemented_serialize::<32>,
    ),
    (
        unimplemented_migrate_up::<33>,
        unimplemented_deserialize::<33>,
        unimplemented_serialize::<33>,
    ),
    (
        unimplemented_migrate_up::<34>,
        unimplemented_deserialize::<34>,
        unimplemented_serialize::<34>,
    ),
    (
        unimplemented_migrate_up::<35>,
        unimplemented_deserialize::<35>,
        unimplemented_serialize::<35>,
    ),
    (
        unimplemented_migrate_up::<36>,
        unimplemented_deserialize::<36>,
        unimplemented_serialize::<36>,
    ),
    (
        unimplemented_migrate_up::<37>,
        unimplemented_deserialize::<37>,
        unimplemented_serialize::<37>,
    ),
    (
        unimplemented_migrate_up::<38>,
        unimplemented_deserialize::<38>,
        unimplemented_serialize::<38>,
    ),
    (
        unimplemented_migrate_up::<39>,
        unimplemented_deserialize::<39>,
        unimplemented_serialize::<39>,
    ),
    (
        unimplemented_migrate_up::<40>,
        unimplemented_deserialize::<40>,
        unimplemented_serialize::<40>,
    ),
    (_41::migrate_up, _41::deserialize, _41::serialize),
    (_42::migrate_up, _42::deserialize, _42::serialize),
    (_43::migrate_up, _43::deserialize, _43::serialize),
    (_44::migrate_up, _44::deserialize, _44::serialize),
    (
        unimplemented_migrate_up::<45>,
        _45::deserialize,
        _45::serialize,
    ),
];

pub fn migrate_up(current: &str, to_version: u32) -> anyhow::Result<String> {
    let current_version = crate::version::detect_version(current)?;

    if current_version > to_version {
        return Err(anyhow::anyhow!(
            "`--input` format version {current_version} is greater than `--to-version` {to_version}"
        )
        .context("downgrading to an older format version is not supported"));
    }

    let deserialize = MIGRATIONS[current_version as usize - 1].1;

    let mut crate_ = (deserialize)(current);

    for i in current_version..to_version {
        let migrate_up = MIGRATIONS[i as usize - 1].0;
        crate_ = unsafe { (migrate_up)(crate_) };
    }

    let serialize = MIGRATIONS[to_version as usize - 1].2;

    Ok(unsafe { (serialize)(crate_) })
}

/// Panics when called, displaying an error that the given migration isn't yet supported.
fn unimplemented_migrate_up<const N: usize>(_: *mut ()) -> *mut () {
    unimplemented_inner(N);
}

/// Panics when called, displaying an error that deserialization isn't yet supported.
fn unimplemented_deserialize<const N: usize>(_: &str) -> *mut () {
    unimplemented_inner(N);
}

/// Panics when called, displaying an error that serialization isn't yet supported.
fn unimplemented_serialize<const N: usize>(_: *mut ()) -> String {
    unimplemented_inner(N);
}

fn unimplemented_inner(n: usize) -> ! {
    unimplemented!("format v{n} is not yet supported");
}
