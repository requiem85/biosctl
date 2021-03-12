pub mod cli;

use anyhow::*;
use std::{
    ffi::OsString,
    fs::DirEntry,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Attribute {
    pub name: OsString,
    pub tpe: AttributeType,
    pub current_value: Result<String>,
    pub default_value: Result<String>,
    pub display_name: String,
    pub display_name_lang: String,
}
#[derive(Debug)]
pub enum AttributeType {
    Integer { min: i64, max: i64, step: u64 },
    String { min_length: u64, max_length: u64 },
    Enumeration { possible_values: Vec<String> },
}

pub fn list_attributes(interface: &Path) -> Result<Vec<Attribute>> {
    fn read_value(entry: &DirEntry, name: &str) -> Result<String> {
        let mut p = entry.path();
        p.push(name);
        let mut v = std::fs::read_to_string(p)?;
        v = v.trim_end().to_string();

        Ok(v)
    }

    let mut attributes_path = PathBuf::from(interface);
    attributes_path.push("attributes");

    let mut attributes = Vec::new();

    for d in attributes_path.read_dir()? {
        if let Ok(d) = d {
            if d.file_type()?.is_dir() {
                let name = d.file_name();
                let current_value = read_value(&d, "current_value");
                let default_value = read_value(&d, "default_value");

                let display_name = read_value(&d, "display_name")?;
                let display_name_lang = read_value(&d, "display_name_language_code")?;

                let tpe_name = read_value(&d, "type")?;
                let tpe = match tpe_name.as_ref() {
                    "enumeration" => {
                        let p_value_string = read_value(&d, "possible_values")?;
                        let mut p_values = Vec::new();
                        for v in p_value_string.split(';') {
                            p_values.push(v.to_string());
                        }
                        AttributeType::Enumeration {
                            possible_values: p_values,
                        }
                    }
                    "integer" => {
                        let min: i64 = read_value(&d, "min_value")?.parse()?;
                        let max: i64 = read_value(&d, "max_value")?.parse()?;
                        let step: u64 = read_value(&d, "scalar_increment")?.parse()?;

                        AttributeType::Integer { min, max, step }
                    }
                    "string" => {
                        let min_length: u64 = read_value(&d, "min_length")?.parse()?;
                        let max_length: u64 = read_value(&d, "min_length")?.parse()?;

                        AttributeType::String {
                            min_length,
                            max_length,
                        }
                    }
                    a => {
                        bail!("Unknown attribute type: '{}'", a)
                    }
                };

                attributes.push(Attribute {
                    name,
                    tpe,
                    current_value,
                    default_value,
                    display_name,
                    display_name_lang,
                });
            }
        }
    }

    Ok(attributes)
}
