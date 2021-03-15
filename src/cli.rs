use std::ffi::OsString;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Manage firmware configuration")]
pub struct ProgramOptions {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Print {
        #[structopt(long, short = "D")]
        device_name: Option<OsString>,

        #[structopt(name = "ATTRIBUTE")]
        attribute: Option<OsString>,
    },
    List {
        #[structopt(long, short = "D")]
        device_name: Option<OsString>,
    }
}
