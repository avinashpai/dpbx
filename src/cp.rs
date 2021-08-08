/*
 *  Copy file or directory to Dropbox directory
 * */

use std::{
    fs::{self, File},
    io::{self, copy},
    path::{Path, PathBuf},
};

use indicatif::{ProgressBar, ProgressStyle};

use crate::util;

pub struct Copier {
    cwd: PathBuf,
}

impl Copier {
    pub fn new() -> Self {
        Copier {
            cwd: util::get_dpbx_path().unwrap(),
        }
    }

    pub fn cp(&mut self, path: &Path) -> io::Result<()> {
        if path.is_dir() {
            return self.recur_dir(path);
        } else {
            return self.cp_file(path);
        }
    }

    fn cp_file(&self, file: &Path) -> io::Result<()> {
        let metadata = fs::metadata(file)?;
        let progress_bar = ProgressBar::new(metadata.len());
        progress_bar.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .progress_chars("#>-"));

        let mut source = File::open(file)?;

        let mut path = self.cwd.clone();
        path.push(file.file_name().unwrap());

        let dest = File::create(path.as_path())?;

        println!("Copying {:?} to {:?}...", file, path);
        let _ = copy(&mut source, &mut progress_bar.wrap_write(dest))?;

        progress_bar.finish();

        Ok(())
    }

    fn recur_dir(&mut self, dir: &Path) -> io::Result<()> {
        self.cwd.push(dir.file_name().unwrap());
        fs::create_dir(&self.cwd)?;
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.recur_dir(&path)?;
                self.cwd.pop();
            } else {
                self.cp_file(&path)?;
            }
        }
        Ok(())
    }
}
