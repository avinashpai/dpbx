/*
 *  Copy file or directory to Dropbox directory
 * */

use std::{fs,
    io::{self, copy, Read},
    path::Path,
};

use std::thread;

use indicatif::{ProgressBar, ProgressStyle};

struct CopyProgress<T> {
    inner: T,
    progress_bar: ProgressBar,
}

impl<T: Read> Read for CopyProgress<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.progress_bar.inc(n as u64);
            n
        })
    }
}

pub fn cp(path: &Path) -> io::Result<u64> {
    let bar = ProgressBar::new(1000);
    bar.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
    .progress_chars("##-"));
    for _ in 0..1000 {
        bar.inc(1);
        thread::sleep(std::time::Duration::from_millis(10));
    }
    bar.finish();
    Ok(0)
}
