/*
 *  Copy file or directory to Dropbox directory
 * */

use std::{
    fs::{self, File},
    io::{self, copy},
    path::Path,
};

use indicatif::{ProgressBar, ProgressStyle};

use crate::util;

pub fn cp(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path)?;

    if metadata.file_type().is_file() {
        return cp_file(path, metadata);
    } else if metadata.file_type().is_dir() {
        return recur_dir(path);
    } // TODO: symlinks

    Ok(())
}

fn cp_file(path: &Path, metadata: fs::Metadata) -> io::Result<()> {
    let progress_bar = ProgressBar::new(metadata.len());
    progress_bar.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .progress_chars("#>-"));

    let mut source = File::open(path)?;

    let mut file_path = util::get_dpbx_path()?;
    file_path.push(path.file_name().unwrap());

    let dest = File::create(file_path.as_path())?;

    println!("Copying {:?} to {:?}...", path, file_path);
    let _ = copy(&mut source, &mut progress_bar.wrap_write(dest))?;

    progress_bar.finish();

    Ok(())
}

fn recur_dir(path: &Path) -> io::Result<()> {
    todo!()
}
