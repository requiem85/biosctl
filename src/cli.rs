use log::LevelFilter;
use std::ffi::OsString;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Manage Dell BIOS/EFI settings",
    setting = structopt::clap::AppSettings::DisableVersion
)]
pub struct ProgramOptions {
    #[structopt(long, short = "D", default_value = "dell-wmi-sysman")]
    pub device_name: OsString,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,

    /// Prints version information.
    #[structopt(short = "V", long = "version")]
    pub version: bool,

    #[structopt(long, short, global = true, parse(from_occurrences))]
    verbose: i8,

    /// Pass many times for less log output
    #[structopt(
        long,
        short,
        global = true,
        parse(from_occurrences),
        conflicts_with = "verbose"
    )]
    quiet: i8,

    /// BIOS admin password for authentication
    #[structopt(long, global = true)]
    pub password: Option<String>,
}

impl ProgramOptions {
    pub fn log_level_with_default(&self, default: i8) -> Option<LevelFilter> {
        let level = default + self.verbose - self.quiet;
        let new_level = match level {
            i8::MIN..=0 => LevelFilter::Off,
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            5..=i8::MAX => LevelFilter::Trace,
        };

        if level != default {
            Some(new_level)
        } else {
            None
        }
    }
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

        #[structopt(long, short, conflicts_with = "default")]
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
    NeedsReboot,
}
