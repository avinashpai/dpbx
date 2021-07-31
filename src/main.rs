mod util;
mod cp;

use std::path::PathBuf;
use std::thread;

use indicatif::{ProgressBar, ProgressStyle};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    cmd: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,

    #[structopt(short, long)]
    verbose: bool,
}

// Parse cmdline args and call appropriate subroutine
fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let dpbx_path = util::get_dpbx_path();
    println!("{:?}", dpbx_path);

    // Testing progress bar
       // create execute module that parses and executes commands

    let mut children = vec![];
    if opt.cmd.eq("cp") {
        children.push(thread::spawn(move || cp::cp(opt.path.as_path()).unwrap()))
    }

    for child in children {
        let _ = child.join();
    }
}
