use std::{collections::BTreeMap, ops::ControlFlow, process::Command, sync::LazyLock};

use anyhow::Context;

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

pub(super) fn get_toolchain(format_version: u32) -> &'static str {
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
