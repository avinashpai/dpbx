use std::{
    fs,
    io,
    env,
    path::Path,
};

use crate::util;

pub struct Mover;

impl Mover {
    pub fn new() -> Self {
        Mover{}
    }

    pub fn mv(&mut self, path: &Path) -> io::Result<()> {
        let mut to_dpbx_path = util::get_dpbx_path()?;
        to_dpbx_path.push(path.file_name().unwrap());
        println!("Moving {:?} to {:?}...", path, to_dpbx_path);
        fs::rename(path, to_dpbx_path)?;
        println!("Done!");
        Ok(())
    }

    // mOvE hErE.. moves the file or directory to the current directory you're in
    pub fn mvhr(&mut self, path: &Path) -> io::Result<()> {
        let mut from_dpbx_path = util::get_dpbx_path()?;
        let filename = path.file_name().unwrap();
        from_dpbx_path.push(filename);
        let mut to_path = env::current_dir()?;
        to_path.push(filename);
        println!("Moving {:?} to {:?}...", from_dpbx_path, to_path);
        fs::rename(from_dpbx_path, to_path)?;
        println!("Done!");
        Ok(())
    }
}
