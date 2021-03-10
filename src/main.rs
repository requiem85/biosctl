use anyhow::*;
use firmconfig::{cli::ProgramOptions, Attribute, AttributeType};
use std::{
    ffi::OsStr,
    io::{stdout, Write},
    path::{Path, PathBuf},
};
use structopt::StructOpt;

fn main() -> Result<()> {
    let options = ProgramOptions::from_args();
    match options.cmd {
        firmconfig::cli::Command::Print { device_name } => {
            if let Some(name) = device_name {
                print_device(&name)?;
            } else {
                let path = Path::new("/sys/class/firmware-attributes");
                for d in path.read_dir()? {
                    if let Ok(d) = d {
                        print_device(&d.file_name())?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn print_device(name: &OsStr) -> Result<()> {
    let mut path = PathBuf::from("/sys/class/firmware-attributes");
    path.push(name);

    let attributes = firmconfig::list_attributes(&path)?;

    println!("Device: {}\n", name.to_string_lossy());
    for a in attributes {
        print_attribute(&a)?;
    }
    Ok(())
}

fn print_attribute(a: &Attribute) -> Result<()> {
    let out = stdout();
    let mut f = out.lock();
    writeln!(f, "{}", a.name)?;
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
