use std::ffi::OsString;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Manage Dell BIOS/EFI settings")]
pub struct ProgramOptions {
    #[structopt(long, short = "D", default_value("dell-wmi-sysman".into()))]
    pub device_name: OsString,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Print {
        #[structopt(name = "SETTING")]
        attribute: Option<OsString>,
    },
    List,
    Get {
        #[structopt(long, short)]
        default: bool,

        #[structopt(long, short, conflicts_with("default"))]
        name: bool,

        #[structopt(name = "SETTING")]
        attribute: OsString,
    },
    Set {
        #[structopt(name = "SETTING")]
        attribute: OsString,

        #[structopt(name = "VALUE")]
        value: OsString,
    },
    Info,
}
