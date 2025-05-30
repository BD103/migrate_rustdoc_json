use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    process::Command,
};

use jsonpath_rust::query::QueryRef;
use serde_json::Value;

pub fn generate_json(source: &Path, format_version: u32) -> PathBuf {
    assert_eq!(
        source.extension(),
        Some(OsStr::new("rs")),
        "can only generate `rustdoc` JSON for `.rs` files",
    );

    let json_path = json_path(source);
    let json_dir = json_path.parent().unwrap();

    let status = Command::new("rustup")
        .arg("run")
        .arg(super::get_toolchain(format_version))
        .arg("rustdoc")
        .arg(format!("--out-dir={}", json_dir.display()))
        .arg("-Zunstable-options")
        .arg("--output-format=json")
        .arg(source)
        .status()
        .unwrap();

    assert!(
        status.success(),
        "failed to generate `rustdoc` JSON, run with `INSTALL_TOOLCHAINS=1` to auto-install toolchains",
    );
    assert!(json_path.is_file());

    json_path
}

pub fn migrate_json(json: &Path, format_version: u32) -> PathBuf {
    let program = Path::new(env!("CARGO_BIN_EXE_migrate_rustdoc_types"));

    assert!(program.is_file(), "`migrate_rustdoc_types` cannot be found");

    let migrated_path = json.with_extension("migrated.json");
    let migrated_file = File::create(&migrated_path).unwrap();

    let status = Command::new(program)
        .arg("--input")
        .arg(json)
        .arg("--to-version")
        .arg(format_version.to_string())
        .stdout(migrated_file)
        .status()
        .unwrap();

    assert!(
        status.success(),
        "migrating {} to format version {format_version} failed",
        json.display(),
    );

    migrated_path
}

fn json_path(source: &Path) -> PathBuf {
    let source = source.canonicalize().unwrap();
    let test_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let temp_dir = Path::new(env!("CARGO_TARGET_TMPDIR"));

    temp_dir
        .join(source.strip_prefix(test_root).unwrap())
        .with_extension("json")
}

pub fn read_json(json: &Path) -> Value {
    let file = File::open(json).unwrap();
    let buffer = BufReader::new(file);

    serde_json::from_reader(buffer).unwrap()
}

pub fn path_val_map(queries: Vec<QueryRef<'_, Value>>) -> BTreeMap<String, Value> {
    let mut map = BTreeMap::new();

    for query in queries {
        // These clones are needed, unfortunately. :(
        let path = query.clone().path();
        let val = query.val().clone();

        map.insert(path, val);
    }

    map
}
