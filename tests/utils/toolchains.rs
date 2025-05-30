use std::{
    collections::BTreeMap,
    ops::ControlFlow,
    process::Command,
    sync::{LazyLock, Mutex},
};

use anyhow::Context;

static TOOLCHAINS: LazyLock<BTreeMap<u32, &'static str>> = LazyLock::new(|| {
    let toolchains = [
        (43, "nightly-2025-04-18"),
        (44, "nightly-2025-04-19"),
        (45, "nightly-2025-04-20"),
    ];

    BTreeMap::from(toolchains)
});

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
