use std::{ops::ControlFlow, path::Path};

use jsonpath_rust::JsonPath;
use serde_json::{Number, Value};
use utils::{generate_json, migrate_json, need, path_val_map, read_json};

#[test]
fn v44() {
    let ControlFlow::Continue(()) = need(44) else {
        return;
    };

    let original = generate_json(Path::new("tests/v44/v44.rs"), 44);
    let migrated = migrate_json(&original, 45);

    let original_json = read_json(&original);
    let migrated_json = read_json(&migrated);

    let query = "$.index[*].span['begin', 'end']";

    let original_spans = path_val_map(original_json.query_with_path(query).unwrap());

    for migrated_span in migrated_json.query_with_path(query).unwrap() {
        let path = migrated_span.clone().path();

        let mut original_span = original_spans.get(&path).unwrap().clone();

        if let Value::Number(ref mut col) = original_span[1] {
            *col = Number::from(col.as_u64().unwrap() + 1);
        }

        let migrated_span = migrated_span.val();

        assert_eq!(&original_span, migrated_span);
    }
}

mod utils {
    use std::{
        collections::BTreeMap,
        ffi::OsStr,
        fs::File,
        io::BufReader,
        ops::ControlFlow,
        path::{Path, PathBuf},
        process::Command,
        sync::LazyLock,
    };

    use anyhow::Context;
    use jsonpath_rust::query::QueryRef;
    use serde_json::Value;

    static TOOLCHAINS: LazyLock<BTreeMap<u32, &'static str>> = LazyLock::new(|| {
        let toolchains = [
            (40, "nightly-2025-02-24"),
            (41, "nightly-2025-03-10"),
            (42, "nightly-2025-03-15"),
            (43, "nightly-2025-03-22"),
            (44, "nightly-2025-04-19"),
            (45, "nightly-2025-04-18"),
        ];

        BTreeMap::from(toolchains)
    });

    pub fn need(format_version: u32) -> ControlFlow<()> {
        let install_toolchains = option_env!("INSTALL_TOOLCHAINS");

        match install_toolchains {
            Some("1") => {
                install_toolchain_for_version(format_version);
                ControlFlow::Continue(())
            }
            Some("-1") => {
                uninstall_toolchain_for_version(format_version);
                ControlFlow::Break(())
            }
            _ => ControlFlow::Continue(()),
        }
    }

    fn get_toolchain(format_version: u32) -> &'static str {
        TOOLCHAINS
            .get(&format_version)
            .with_context(|| format!("could not get toolchain for format version {format_version}"))
            .unwrap()
    }

    fn install_toolchain_for_version(format_version: u32) {
        let status = Command::new("rustup")
            .arg("toolchain")
            .arg("install")
            .arg("--profile=minimal")
            .arg(get_toolchain(format_version))
            .status()
            .unwrap();

        assert!(
            status.success(),
            "installing toolchain {} for format version {format_version} failed",
            get_toolchain(format_version),
        );
    }

    fn uninstall_toolchain_for_version(format_version: u32) {
        let status = Command::new("rustup")
            .arg("toolchain")
            .arg("uninstall")
            .arg(get_toolchain(format_version))
            .status()
            .unwrap();

        if !status.success() {
            eprintln!(
                "Uninstalled toolchain {} failed, skipping...",
                get_toolchain(format_version)
            );
        }
    }

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
            .arg(get_toolchain(format_version))
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
}
