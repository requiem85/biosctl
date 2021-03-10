use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(
    about = "Manage firmware configuration",
)]
pub struct ProgramOptions {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    Print {
        #[structopt(long, short)]
        device_name: String,
    },
}
