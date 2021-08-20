mod cp;
mod mv;
mod util;

use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

use cp::Copier;
use mv::Mover;

#[derive(Debug, StructOpt)]
struct Opt {
    cmd: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,

    #[structopt(short, long)]
    debug: bool,
}

// Parse cmdline args and call appropriate subroutine
fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    if opt.debug {
        println!("{:#?}", opt);

        let dpbx_path = util::get_dpbx_path();
        println!("{:?}", dpbx_path);
    }

    // create execute module that parses and executes commands
    match opt.cmd.as_str() {
        "cp" => Copier::new().cp(&opt.path)?,
        "mv" => Mover::new().mv(&opt.path)?,
        "mvhr" => Mover::new().mvhr(&opt.path)?,
        _ => println!("invalid command"),
    };

    Ok(())
}
