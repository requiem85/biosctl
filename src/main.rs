use anyhow::*;
use firmconfig::cli::ProgramOptions;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() -> Result<()> {
    let options = ProgramOptions::from_args();
    match options.cmd {
        firmconfig::cli::Command::Print { interface_name } => {
            let mut path = PathBuf::from("/sys/class/firmware-attributes");
            path.push(interface_name);

            let attributes = firmconfig::list_attributes(&path)?;

            for a in attributes {
                println!("{:?}", a);
            }
        }
    }

    Ok(())
}
