use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use jsonpath_rust::JsonPath;
use serde_json::Value;

pub struct GeneratedAndMigrated {
    pub original_json: Value,
    pub new_json: Value,
    pub migrated_json: Value,
}

pub fn generate_and_migrate_to(
    source: impl AsRef<Path>,
    original_format_version: u32,
    migrated_format_version: u32,
) -> GeneratedAndMigrated {
    let original_path = generate_json(source.as_ref(), original_format_version, "original.json");
    let new_path = generate_json(source.as_ref(), migrated_format_version, "new.json");
    let migrated_path = migrate_json(&original_path, migrated_format_version);

    let original_json = read_json(&original_path);
    let new_json = read_json(&new_path);
    let migrated_json = read_json(&migrated_path);

    assert_eq!(
        original_json["format_version"],
        original_format_version,
        "toolchain {toolchain} failed to generate JSON with the expected format version v{original_format_version}",
        toolchain = super::get_toolchain(original_format_version),
    );

    assert_eq!(
        new_json["format_version"],
        migrated_format_version,
        "toolchain {toolchain} failed to generate JSON with the expected format version v{migrated_format_version}",
        toolchain = super::get_toolchain(migrated_format_version),
    );

    assert_eq!(
        migrated_json["format_version"], migrated_format_version,
        "`migrate_rustdoc_json` did not bump the format version to the expected v{migrated_format_version}",
    );

    GeneratedAndMigrated {
        original_json,
        new_json,
        migrated_json,
    }
}

pub fn query_both<'a, 'b>(
    original_json: &'a Value,
    migrated_json: &'b Value,
    query: &str,
) -> HashMap<String, (Option<&'a Value>, Option<&'b Value>)> {
    let mut map = HashMap::new();

    let original_query = original_json.query_with_path(query).unwrap();
    let migrated_query = migrated_json.query_with_path(query).unwrap();

    for q in original_query {
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
            .and_modify(|(_original, migrated)| *migrated = Some(val))
            .or_insert((None, Some(val)));
    }

    map
}

fn generate_json(source: &Path, format_version: u32, extension: &'static str) -> PathBuf {
    assert_eq!(
        source.extension(),
        Some(OsStr::new("rs")),
        "can only generate Rustdoc JSON for `.rs` files",
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
        "failed to generate Rustdoc JSON, run with `INSTALL_TOOLCHAINS=1` to auto-install toolchains",
    );
    assert!(json_path.is_file());

    let renamed_path = json_path.with_extension(extension);
    std::fs::rename(json_path, &renamed_path).unwrap();

    renamed_path
}

fn migrate_json(original_json: &Path, to_format_version: u32) -> PathBuf {
    let program = Path::new(env!("CARGO_BIN_EXE_migrate_rustdoc_json"));

    assert!(program.is_file(), "`migrate_rustdoc_json` cannot be found");

    // `with_extension()` only replaces the segment after the final ".". For the path
    // "vXX.original.json", we use `with_extension("")` to get "vXX.original" before replacing it
    // with "migrated.json".
    let migrated_path = original_json
        .with_extension("")
        .with_extension("migrated.json");
    let migrated_file = File::create(&migrated_path).unwrap();

    let output = Command::new(program)
        .arg("--input")
        .arg(original_json)
        .arg("--to-version")
        .arg(to_format_version.to_string())
        .stdout(migrated_file)
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "migrating {original_json} to format version {to_format_version} failed:\n{stderr}",
        original_json = original_json.display(),
        stderr = String::from_utf8_lossy(&output.stderr),
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
