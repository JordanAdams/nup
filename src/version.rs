use std::{fs, str::FromStr};

use crate::package;

pub type VersionReq = semver::VersionReq;

fn current_from_nvmrc() -> Option<VersionReq> {
    let mut file_path = std::env::current_dir().unwrap();
    file_path.push(".nvmrc");

    if !file_path.exists() {
        return None;
    }

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            log::debug!("Failed to read .nvmrc: {}", err);
            return None;
        }
    };

    match VersionReq::from_str(&contents) {
        Ok(version) => Some(version),
        Err(err) => {
            log::debug!("Unable to parse .nvmrc {}", err);
            return None;
        }
    }
}

pub fn current_node_version() -> Option<VersionReq> {
    current_from_nvmrc()
    // todo - fetch from package.json
}
