mod cp;
mod util;

use std::path::PathBuf;

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
    println!("{:#?}", opt);

    let dpbx_path = util::get_dpbx_path();
    println!("{:?}", dpbx_path);

    // create execute module that parses and executes commands
    match opt.cmd.as_str() {
        "cp" => cp::Copier::new().cp(&opt.path).unwrap(),
        _ => println!("invalid command"),
    };
}
