use std::io;
use std::path::PathBuf;
use std::fs;

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


// Open dropbox info.json and parses its contents into DpbxInfo struct
// Change hardcoded path to path based on cfg!(os_type)
// Errors if file doesn't exist or json cannot be parsed to struct (only personal account for now)
pub fn get_dpbx_path() -> io::Result<PathBuf> {
    let info_str = fs::read_to_string("/Users/avinashpai/.dropbox/info.json")?;
    let dpbx_info: DpbxInfo = serde_json::from_str(&info_str)?;
    Ok(dpbx_info.personal.path)
}
