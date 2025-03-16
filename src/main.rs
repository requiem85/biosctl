use anyhow::*;
use biosctl::{
    cli::{Command, ProgramOptions},
    Attribute, AttributeType, AuthenticationRole, Device,
};
use env_logger::{Builder, Env};
use log::*;
use std::{
    ffi::OsStr,
    fs,
    io::{stdout, Write},
    process::exit,
};
// Bring the StructOpt trait into scope so that ProgramOptions::clap() and ::from_clap() work.
use structopt::StructOpt;
// Import Arg from structopt's clap module (clap v2.33.3)
use structopt::clap::Arg;

const ADMIN_PASSWORD_PATH: &str = "/sys/class/firmware-attributes/dell-wmi-sysman/authentication/Admin/current_password";

type ReturnCode = i32;

/// Writes the given password to the sysfs file to unlock BIOS settings.
fn write_admin_password(password: &str) -> Result<()> {
    fs::write(ADMIN_PASSWORD_PATH, password)
        .with_context(|| format!("writing admin password to {}", ADMIN_PASSWORD_PATH))?;
    Ok(())
}

/// Clears the admin password from the sysfs file.
fn clear_admin_password() -> Result<()> {
    fs::write(ADMIN_PASSWORD_PATH, "")
        .with_context(|| format!("clearing admin password at {}", ADMIN_PASSWORD_PATH))?;
    Ok(())
}

fn main() -> Result<()> {
    // Extend the existing ProgramOptions clap app with a new --password flag.
    let mut app = ProgramOptions::clap();
    app = app.arg(
        Arg::with_name("password")
            .long("password")
            .help("BIOS admin password for authentication")
            .takes_value(true),
    );
    let options_matches = app.get_matches();
    let options = ProgramOptions::from_clap(&options_matches);

    if options.version {
        // HACK to disambiguate short/long invocations for the same cli option;
        // there has to be a better way of doing this...
        let i = options_matches
            .index_of("version")
            .ok_or_else(|| anyhow!("should never happen: version set yet no version flag"))?;
        if std::env::args().nth(i).unwrap_or_default() == "-V" {
            print_version(false);
        } else {
            print_version(true);
        }
        return Ok(());
    }

    let mut b = Builder::from_env(Env::from("BIOSCTL_LOG"));
    b.format_timestamp(None);
    if let Some(level) = options.log_level_with_default(2) {
        b.filter_level(level);
    };
    b.try_init()?;

    if options.cmd.is_none() {
        ProgramOptions::clap().print_help()?;
        exit(1);
    }

    // If a BIOS password was provided, unlock the BIOS by writing it to the sysfs node.
    let password = options_matches.value_of("password");
    if let Some(pwd) = password {
        if let Err(e) = write_admin_password(pwd) {
            eprintln!("Failed to unlock BIOS: {}", e);
            exit(1);
        }
        println!("BIOS unlocked for changes.");
    }

    let retcode = match run(options) {
        Ok(i) => i,
        Err(e) => {
            println!("Error: {}", e);
            for cause in e.chain().skip(1) {
                info!("cause: {}", cause);
            }
            // Attempt to clear the password even if an error occurred.
            if password.is_some() {
                if let Err(e) = clear_admin_password() {
                    eprintln!("Failed to clear BIOS password: {}", e);
                }
            }
            exit(1);
        }
    };

    // Clear the BIOS password if one was used.
    if password.is_some() {
        if let Err(e) = clear_admin_password() {
            eprintln!("Failed to clear BIOS password: {}", e);
            exit(1);
        }
        println!("BIOS password cleared.");
    }
    exit(retcode)
}

fn run(options: ProgramOptions) -> Result<ReturnCode> {
    let cmd = options.cmd.ok_or_else(|| anyhow!("should never happen: no command"))?;
    match cmd {
        Command::Print { attribute } => {
            print_device(&options.device_name, attribute.as_deref())?;
        }
        Command::List => {
            list_device(&options.device_name)?;
        }
        Command::Get {
            default,
            name,
            attribute,
        } => {
            print_attribute_value(&options.device_name, &attribute, default, name)?;
        }
        Command::Info => {
            device_info(&options.device_name)?;
        }
        Command::Set { attribute, value } => {
            let device = Device::from(&options.device_name);
            if let Some(mut attr) = device.attribute(&attribute)? {
                attr.set_value(&value)?;
            } else {
                bail!("no setting with name '{}'", attribute.to_string_lossy());
            }
        }
        Command::NeedsReboot => {
            let device = Device::from(&options.device_name);
            if device.modified()? {
                println!("true");
            } else {
                println!("false");
                return Ok(1);
            }
        }
    }

    Ok(0)
}

fn device_info(name: &OsStr) -> Result<()> {
    trace!("printing info for device {:?}", name);

    println!("Device: {}", name.to_string_lossy());
    let device = Device::from(name);

    let attributes = device.attributes()?;
    println!("    {} attributes", attributes.count());

    if device.modified()? {
        println!("\n    Reboot pending: configuration was modified!");
    }

    let mut auths = device.authentications()?.peekable();
    if auths.peek().is_some() {
        println!("\n    Authentication methods:");
    } else {
        warn!(
            "no authentications methods found for device '{}'",
            name.to_string_lossy()
        );
    }
    for a in auths {
        println!("        {}", a.name.to_string_lossy());
        let role = match a.role {
            AuthenticationRole::BiosAdmin => "Change BIOS Settings".to_string(),
            AuthenticationRole::PowerOn => "Power on computer".to_string(),
            AuthenticationRole::Unknown(a) => format!("Unknown role ({})", a),
        };
        println!("            Role: {}", role);

        let status = if a.is_enabled { "Enabled" } else { "Disabled" };
        println!("            Status: {}", status);
    }

    Ok(())
}

fn print_attribute_value(
    device_name: &OsStr,
    attribute: &OsStr,
    default: bool,
    name: bool,
) -> Result<(), Error> {
    trace!(
        "printing content of attribute {:?} (device={:?}, default={}, name={})",
        attribute,
        device_name,
        default,
        name
    );

    let device = Device::from(device_name);
    if let Some(a) = device.attribute(attribute)? {
        if default {
            if let Ok(d) = a.default_value {
                println!("{}", d);
            } else {
                println!("<Access Denied>");
            }
        } else if name {
            println!("{}", a.display_name);
        } else if let Ok(d) = a.current_value {
            println!("{}", d);
        } else {
            println!("<Access Denied>");
        }
        Ok(())
    } else {
        bail!("no attribute with name {}", attribute.to_string_lossy());
    }
}

fn list_device(name: &OsStr) -> Result<()> {
    trace!("listing attributes in device {:?}", name);

    let device = Device::from(name);
    let attributes = device.attributes()?;

    println!("Device: {}\n", name.to_string_lossy());
    for a in attributes {
        println!("{}: {}", a.name.to_string_lossy(), a.display_name);
    }

    Ok(())
}

fn print_device(name: &OsStr, attribute: Option<&OsStr>) -> Result<()> {
    trace!("printing device {:?}", name);
    let device = Device::from(name);
    let mut attributes = device.attributes()?;

    if let Some(attribute) = attribute {
        trace!("filtering by attribute: {:?}", attribute);
        if let Some(a) = attributes.find(|a| a.name == attribute) {
            println!("Device: {}\n", name.to_string_lossy());
            print_attribute(&a)?;
            return Ok(());
        } else {
            bail!("no attribute with name {}", attribute.to_string_lossy());
        }
    }

    println!("Device: {}\n", name.to_string_lossy());
    for a in attributes {
        print_attribute(&a)?;
    }
    Ok(())
}

fn print_attribute(a: &Attribute) -> Result<()> {
    let out = stdout();
    let mut f = out.lock();
    writeln!(f, "{}", a.name.to_string_lossy())?;
    writeln!(f, "    Name: {}", a.display_name)?;
    match a.tpe {
        AttributeType::Integer { min, max, step } => {
            writeln!(f, "    Type: Integer")?;
            writeln!(f, "        Min: {}", min)?;
            writeln!(f, "        Max: {}", max)?;
            writeln!(f, "        Step: {}", step)?;
        }
        AttributeType::String {
            min_length,
            max_length,
        } => {
            writeln!(f, "    Type: String")?;
            writeln!(f, "        Min: {}", min_length)?;
            writeln!(f, "        Max: {}", max_length)?;
        }
        AttributeType::Enumeration {
            ref possible_values,
        } => {
            writeln!(f, "    Type: Enumeration")?;
            writeln!(f, "        Possible Values:")?;
            for p in possible_values {
                writeln!(f, "            {}", p)?;
            }
        }
    }
    match &a.current_value {
        Ok(v) => {
            writeln!(f, "    Current value: {}", v)?;
        }
        Err(_) => {
            writeln!(f, "    Current value: <Access Denied>")?;
        }
    }
    match &a.default_value {
        Ok(v) => {
            writeln!(f, "    Default value: {}", v)?;
        }
        Err(_) => {
            writeln!(f, "    Default value: <Access Denied>")?;
        }
    }

    Ok(())
}

fn print_version(long: bool) {
    if long {
        println!(
            "{} {} ({})",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_ID").unwrap_or("unknown")
        );
        println!("rustc {} ({})", env!("BUILD_RUSTC"), env!("BUILD_INFO"));
    } else {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }
}
