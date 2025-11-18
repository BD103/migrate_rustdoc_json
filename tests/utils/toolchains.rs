use std::{
    collections::HashMap,
    ops::ControlFlow,
    process::Command,
    sync::{LazyLock, Mutex},
};

use anyhow::Context;

/// A map of Rustdoc format versions to their associated nightly toolchain.
static TOOLCHAINS: LazyLock<HashMap<u32, &'static str>> = LazyLock::new(|| {
    // A list of Rustdoc format versions and their associated nightly toolchains. The links are the
    // rollup PRs that merged the format version bump. The nightly date is the date the rollup PR
    // was merged into `main` + 1. You can extend this list by trawling through the Git blame
    // history at <https://github.com/rust-lang/rust/blame/main/src/rustdoc-json-types/lib.rs>.
    let toolchains = [
        // <https://github.com/rust-lang/rust/pull/138548>
        (42, "nightly-2025-03-17"),
        // <https://github.com/rust-lang/rust/pull/138841>
        (43, "nightly-2025-03-24"),
        // <https://github.com/rust-lang/rust/pull/139393>
        (44, "nightly-2025-04-19"),
        // <https://github.com/rust-lang/rust/pull/140040>
        (45, "nightly-2025-04-20"),
        // <https://github.com/rust-lang/rust/pull/141437>
        (46, "nightly-2025-05-24"),
        // <https://github.com/rust-lang/rust/pull/138165>
        (48, "nightly-2025-06-19"),
        // <https://github.com/rust-lang/rust/pull/142770>
        (49, "nightly-2025-06-21"), // TODO: Collides, see which one is which
        // <https://github.com/rust-lang/rust/pull/142491>
        (50, "nightly-2025-06-21"),
        // <https://github.com/rust-lang/rust/pull/142817>
        (51, "nightly-2025-06-22"),
        // <https://github.com/rust-lang/rust/pull/142878>
        (52, "nightly-2025-06-23"),
        // <https://github.com/rust-lang/rust/pull/142901>
        (53, "nightly-2025-06-24"),
        // <https://github.com/rust-lang/rust/pull/144028>
        (54, "nightly-2025-07-17"),
        // <https://github.com/rust-lang/rust/pull/144700>
        (55, "nightly-2025-08-02"),
        // <https://github.com/rust-lang/rust/pull/142472>
        (56, "nightly-2025-08-29"),
    ];

    HashMap::from(toolchains)
});

/// A lock that prevents multiple tests from running `rustup` concurrently, as doing so would cause
/// `rustup` to break. <https://github.com/rust-lang/rustup/issues/988>
static RUSTUP_LOCK: Mutex<()> = Mutex::new(());

pub fn needs_toolchain(format_version: u32) -> ControlFlow<()> {
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

pub(super) fn get_toolchain(format_version: u32) -> &'static str {
    TOOLCHAINS
        .get(&format_version)
        .with_context(|| format!("could not get toolchain for format version {format_version}"))
        .unwrap()
}

fn install_toolchain_for_version(format_version: u32) {
    let lock = RUSTUP_LOCK.lock().unwrap();

    let status = Command::new("rustup")
        .arg("toolchain")
        .arg("install")
        .arg("--profile=minimal")
        .arg(get_toolchain(format_version))
        .status()
        .unwrap();

    drop(lock);

    assert!(
        status.success(),
        "installing toolchain {} for format version {format_version} failed",
        get_toolchain(format_version),
    );
}

fn uninstall_toolchain_for_version(format_version: u32) {
    let lock = RUSTUP_LOCK.lock().unwrap();

    let status = Command::new("rustup")
        .arg("toolchain")
        .arg("uninstall")
        .arg(get_toolchain(format_version))
        .status()
        .unwrap();

    drop(lock);

    if !status.success() {
        eprintln!(
            "Uninstalled toolchain {} failed, skipping...",
            get_toolchain(format_version)
        );
    }
}
