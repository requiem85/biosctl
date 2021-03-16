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
    },
    Get {
        #[structopt(long, short = "D")]
        device_name: Option<OsString>,

        #[structopt(long, short)]
        default: bool,

        #[structopt(long, short, conflicts_with("default"))]
        name: bool,

        #[structopt(name = "ATTRIBUTE")]
        attribute: OsString,
    },
    Info {
        device_name: Option<OsString>,
    },
}
