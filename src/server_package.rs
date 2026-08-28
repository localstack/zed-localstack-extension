use std::path::PathBuf;
use zed_extension_api::{self as zed, Result};

const PACKAGE_NAME: &str = "@localstack/localstack-mcp-server";
const ENTRY_POINT: &str = "node_modules/@localstack/localstack-mcp-server/dist/cli.js";

/// Installs or updates the npm package in the extension work directory and
/// returns the absolute path of the server entry point.
pub fn ensure_installed() -> Result<PathBuf> {
    install_when_outdated()?;
    let work_dir =
        std::env::current_dir().map_err(|error| format!("cannot read work directory: {error}"))?;
    Ok(work_dir.join(ENTRY_POINT))
}

fn install_when_outdated() -> Result<()> {
    let installed = zed::npm_package_installed_version(PACKAGE_NAME)?;
    let latest = zed::npm_package_latest_version(PACKAGE_NAME);
    match (installed, latest) {
        (Some(current), Ok(target)) if current == target => Ok(()),
        (_, Ok(target)) => zed::npm_install_package(PACKAGE_NAME, &target),
        (Some(_), Err(_)) => Ok(()),
        (None, Err(error)) => Err(format!(
            "cannot resolve the latest {PACKAGE_NAME} version and no copy is installed: {error}"
        )),
    }
}
