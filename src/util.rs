extern crate dirs;

use std::{fs, io, path::PathBuf};

use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct Personal {
    path: PathBuf,
    host: u64,
    is_team: bool,
    subscription_type: String,
}

#[derive(Debug, Deserialize)]
struct DpbxInfo {
    personal: Personal,
}

// UNIX only
const INFO_PATH: &str = ".dropbox/info.json";

// Open dropbox info.json and parses its contents into DpbxInfo struct
// Panic if home directory cannot be found
// Errors if file doesn't exist or json cannot be parsed to struct (only personal account for now)
pub fn get_dpbx_path() -> io::Result<PathBuf> {
    let mut full_info_path = dirs::home_dir().expect("Home directory not found");
    full_info_path.push(INFO_PATH);

    let info_str = fs::read_to_string(full_info_path)?;
    let dpbx_info: DpbxInfo = serde_json::from_str(&info_str)?;

    Ok(dpbx_info.personal.path)
}
