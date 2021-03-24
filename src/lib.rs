pub mod cli;

use anyhow::*;
use log::*;
use std::{
    ffi::{OsStr, OsString},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};

#[derive(Debug)]
pub struct Device {
    pub name: OsString,
    path: PathBuf,
}

impl Device {
    pub fn from(name: &OsStr) -> Device {
        let mut path = PathBuf::from("/sys/class/firmware-attributes");
        path.push(name);

        Device {
            name: name.to_os_string(),
            path,
        }
    }

    pub fn authentications(&self) -> Result<impl Iterator<Item = Authentication>> {
        let mut auth_path = PathBuf::from(&self.path);
        auth_path.push("authentication");

        debug!("reading device authentication path {:?}", auth_path);

        Ok(auth_path
            .read_dir()
            .with_context(|| {
                format!(
                    "failed to read authentications for device at path '{}'",
                    self.path.to_string_lossy()
                )
            })?
            .filter_map(|d| {
                make_authentication(d).map_or_else(
                    |e| {
                        warn!("skipping authentication with error: {}", e);
                        for cause in e.chain().skip(1) {
                            info!("cause: {}", cause);
                        }
                        None
                    },
                    Some,
                )
            }))
    }

    pub fn attributes(&self) -> Result<impl Iterator<Item = Attribute>> {
        let mut attributes_path = PathBuf::from(&self.path);
        attributes_path.push("attributes");

        debug!("reading device attribute path {:?}", attributes_path);

        Ok(attributes_path
            .read_dir()
            .with_context(|| {
                format!(
                    "failed to read attributes for device at path '{}'",
                    self.path.to_string_lossy()
                )
            })?
            .filter_map(move |d| {
                self.make_attribute(d).map_or_else(
                    |e| {
                        warn!("skipping attribute with error: {}", e);
                        for cause in e.chain().skip(1) {
                            info!("cause: {}", cause);
                        }
                        None
                    },
                    Some,
                )
            }))
    }

    pub fn attribute(&self, name: &OsStr) -> Result<Option<Attribute>> {
        let mut attributes = self.attributes()?;

        Ok(attributes.find(|a| a.name == name))
    }

    fn make_attribute(&self, d: Result<std::fs::DirEntry, std::io::Error>) -> Result<Attribute> {
        match d {
            Ok(d) => {
                if d.file_type()?.is_dir() {
                    let name = d.file_name();
                    let current_value = read_value(d.path(), OsStr::new("current_value"));
                    let default_value = read_value(d.path(), OsStr::new("default_value"));

                    let display_name = read_value(d.path(), OsStr::new("display_name"))?;
                    let display_name_lang =
                        read_value(d.path(), OsStr::new("display_name_language_code"))?;

                    let tpe_name = read_value(d.path(), OsStr::new("type"))?;
                    let tpe = match tpe_name.as_ref() {
                        "enumeration" => {
                            let p_value_string =
                                read_value(d.path(), OsStr::new("possible_values"))?;
                            let mut p_values = Vec::new();
                            for v in p_value_string.split(';') {
                                p_values.push(v.to_string());
                            }
                            AttributeType::Enumeration {
                                possible_values: p_values,
                            }
                        }
                        "integer" => {
                            let min: i64 =
                                read_value(d.path(), OsStr::new("min_value"))?.parse()?;
                            let max: i64 =
                                read_value(d.path(), OsStr::new("max_value"))?.parse()?;
                            let step: u64 =
                                read_value(d.path(), OsStr::new("scalar_increment"))?.parse()?;

                            AttributeType::Integer { min, max, step }
                        }
                        "string" => {
                            let min_length: u64 =
                                read_value(d.path(), OsStr::new("min_length"))?.parse()?;
                            let max_length: u64 =
                                read_value(d.path(), OsStr::new("min_length"))?.parse()?;

                            AttributeType::String {
                                min_length,
                                max_length,
                            }
                        }
                        a => {
                            bail!("Unknown attribute type: '{}'", a)
                        }
                    };

                    Ok(Attribute {
                        device: self,
                        name,
                        tpe,
                        current_value,
                        default_value,
                        display_name,
                        display_name_lang,
                    })
                } else {
                    Err(anyhow!("attribute isn't a directory"))
                }
            }
            Err(e) => Err(e).context("error iterating attributes"),
        }
    }

    pub fn modified(&self) -> Result<bool> {
        let mut attributes_path = PathBuf::from(&self.path);
        attributes_path.push("attributes");

        debug!("reading device attribute path {:?}", attributes_path);

        let v: u8 = read_value(attributes_path, OsStr::new("pending_reboot"))?.parse()?;

        match v {
            1 => Ok(true),
            _ => Ok(false),
        }
    }
}

fn make_authentication(d: Result<std::fs::DirEntry, std::io::Error>) -> Result<Authentication> {
    match d {
        Ok(d) => {
            if d.file_type()?.is_dir() {
                let name = d.file_name();
                let is_enabled = !matches!(
                    read_value(d.path(), OsStr::new("is_enabled"))?.as_ref(),
                    "0"
                );
                let min_password_length =
                    read_value(d.path(), OsStr::new("min_password_length"))?.parse()?;
                let max_password_length =
                    read_value(d.path(), OsStr::new("max_password_length"))?.parse()?;
                let role = match read_value(d.path(), OsStr::new("role"))?.as_ref() {
                    "bios-admin" => AuthenticationRole::BiosAdmin,
                    "power-on" => AuthenticationRole::PowerOn,
                    a => AuthenticationRole::Unknown(a.to_string()),
                };

                Ok(Authentication {
                    name,
                    is_enabled,
                    min_password_length,
                    max_password_length,
                    role,
                })
            } else {
                Err(anyhow!("authentication isn't a directory"))
            }
        }
        Err(e) => Err(e).context("error iterating authentications"),
    }
}

#[derive(Debug)]
pub struct Attribute<'a> {
    device: &'a Device,
    pub name: OsString,
    pub tpe: AttributeType,
    pub current_value: Result<String>,
    pub default_value: Result<String>,
    pub display_name: String,
    pub display_name_lang: String,
}

impl<'a> Attribute<'a> {
    pub fn set_value(&mut self, value: &OsStr) -> Result<()> {
        let mut p = PathBuf::from(&self.device.path);
        p.push("attributes");
        p.push(&self.name);
        p.push("current_value");

        debug!("writing value {:?} to attribute {:?}", value, p);

        std::fs::write(&p, value.as_bytes())?;

        self.current_value = read_value(self.device.path.to_path_buf(), &self.name);

        Ok(())
    }
}

#[derive(Debug)]
pub enum AttributeType {
    Integer { min: i64, max: i64, step: u64 },
    String { min_length: u64, max_length: u64 },
    Enumeration { possible_values: Vec<String> },
}

#[derive(Debug)]
pub struct Authentication {
    pub name: OsString,
    pub is_enabled: bool,
    pub min_password_length: u64,
    pub max_password_length: u64,
    pub role: AuthenticationRole,
}

#[derive(Debug)]
pub enum AuthenticationRole {
    BiosAdmin,
    PowerOn,
    Unknown(String),
}

fn read_value(path: PathBuf, name: &OsStr) -> Result<String> {
    let mut p = path;
    p.push(name);
    let mut v = std::fs::read_to_string(p)
        .with_context(|| format!("failed to read value '{}'", name.to_string_lossy()))?;
    v = v.trim_end().to_string();

    Ok(v)
}
