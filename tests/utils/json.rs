use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    process::Command,
};

use jsonpath_rust::JsonPath;
use serde_json::Value;

pub fn generate_and_migrate_to(
    source: impl AsRef<Path>,
    source_format_version: u32,
    to_version: u32,
) -> (Value, Value) {
    let source_path = generate_json(source.as_ref(), source_format_version);
    let migrated_path = migrate_json(&source_path, to_version);

    let source_json = read_json(&source_path);
    let migrated_json = read_json(&migrated_path);

    assert_eq!(source_json["format_version"], source_format_version);
    assert_eq!(migrated_json["format_version"], to_version);

    (source_json, migrated_json)
}

pub fn query_both<'a, 'b>(
    source_json: &'a Value,
    migrated_json: &'b Value,
    query: &str,
) -> BTreeMap<String, (Option<&'a Value>, Option<&'b Value>)> {
    let mut map = BTreeMap::new();

    let source_query = source_json.query_with_path(query).unwrap();
    let migrated_query = migrated_json.query_with_path(query).unwrap();

    for q in source_query {
        // We have to clone the `QueryRef` here because both methods take `self`, and not `&self`.
        let path = q.clone().path();
        let val = q.val();

        map.insert(path, (Some(val), None));
    }

    for q in migrated_query {
        // We have to clone the `QueryRef` here because both methods take `self`, and not `&self`.
        let path = q.clone().path();
        let val = q.val();

        map.entry(path)
            .and_modify(|(_source, migrated)| *migrated = Some(val))
            .or_insert((None, Some(val)));
    }

    map
}

fn generate_json(source: &Path, format_version: u32) -> PathBuf {
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

fn migrate_json(json: &Path, format_version: u32) -> PathBuf {
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

fn read_json(json: &Path) -> Value {
    let file = File::open(json).unwrap();
    let buffer = BufReader::new(file);

    serde_json::from_reader(buffer).unwrap()
}
