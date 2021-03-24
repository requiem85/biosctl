use anyhow::*;
use biosctl::{
    cli::{Command, ProgramOptions},
    Attribute, AttributeType, AuthenticationRole, Device,
};
use env_logger::{Builder, Env};
use log::*;
use std::{
    ffi::OsStr,
    io::{stdout, Write},
};
use structopt::StructOpt;

fn main() -> Result<()> {
    let options = ProgramOptions::from_args();

    let mut b = Builder::from_env(Env::from("BIOSCTL_LOG"));
    b.format_timestamp(None);
    if let Some(level) = options.log_level_with_default(2) {
        b.filter_level(level);
    };
    b.try_init()?;

    std::process::exit(match run(options) {
        Ok(()) => 0,
        Err(e) => {
            println!("Error: {}", e);
            for cause in e.chain().skip(1) {
                info!("cause: {}", cause);
            }
            1
        }
    })
}

fn run(options: ProgramOptions) -> Result<()> {
    match options.cmd {
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
    }

    Ok(())
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
            AuthenticationRole::Unknown(a) => format!("Unkown role ({})", a),
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
