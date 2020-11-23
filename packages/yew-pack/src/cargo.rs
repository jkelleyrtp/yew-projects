//! Utilities for working with cargo and rust files
use crate::error::{Error, Result};
use std::{env, fs, path::PathBuf, process::Command, str};

/// How many parent folders are searched for a `Cargo.toml`
const MAX_ANCESTORS: u32 = 10;

/// Returns the root of the crate that the command is run from
///
/// If the command is run from the workspace root, this will return the top-level Cargo.toml
pub fn crate_root() -> Result<PathBuf> {
    /// Checks if the directory contains `Cargo.toml`
    fn contains_manifest(path: &PathBuf) -> bool {
        fs::read_dir(path)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .any(|ent| &ent.file_name() == "Cargo.toml")
            })
            .unwrap_or(false)
    }

    // From the current directory we work our way up, looking for `Cargo.toml`
    env::current_dir()
        .ok()
        .and_then(|mut wd| {
            for _ in 0..MAX_ANCESTORS {
                if contains_manifest(&wd) {
                    return Some(wd);
                }
                if !wd.pop() {
                    break;
                }
            }

            None
        })
        .ok_or_else(|| Error::CargoError("Faield to find the cargo directory".to_string()))
}

/// Returns the root of a workspace
pub fn workspace_root() -> Result<PathBuf> {
    let output = Command::new("cargo")
        .args(&["metadata"])
        .output()
        .map_err(|_| Error::CargoError("Manifset".to_string()))?;

    if !output.status.success() {
        let mut msg = str::from_utf8(&output.stderr).unwrap().trim();
        if msg.starts_with("error: ") {
            msg = &msg[7..];
        }

        return Err(Error::CargoError(msg.to_string()));
    }

    let stdout = str::from_utf8(&output.stdout).unwrap();
    for line in stdout.lines() {
        let meta = json::parse(line).map_err(|_| Error::CargoError("InvalidOutput".to_string()))?;
        let root = meta["workspace_root"]
            .as_str()
            .ok_or_else(|| Error::CargoError("InvalidOutput".to_string()))?;
        return Ok(root.into());
    }

    Err(Error::CargoError("InvalidOutput".to_string()))
}
