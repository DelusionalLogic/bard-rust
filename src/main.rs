extern crate ini;
mod global_config;

use std::path::PathBuf;
use ini::Ini;
use std::env;
use global_config::GlobalConfig;

enum Type {
    Poll,
    Running,
}

struct Unit {
    name: String,
    exec_type: Type,
    command: Option<String>,
    regex: Option<String>,
    format: Option<String>,
    interval: i32,
}

impl Unit {
    pub fn new() -> Unit {
        return Unit {
            name: "".to_owned(),
            exec_type: Type::Poll,
            command: None,
            regex: None,
            format: None,
            interval: 0,
        }
    }

    pub fn load(path: &PathBuf) -> Result<Unit, String> {
        let parser = Ini::load_from_file(path.to_str().expect("Invalid config path"))
            .expect("Failed parsing unit config file");
        return Ok(Unit {
            name:
                match parser.section(Some("unit")) {
                    Some(x) => x.get("name").expect("No unit name found").clone(),
                    None    => return Err(format!("Unit section not found in config file: {}", path.to_str().unwrap()))
                },
            exec_type: 
                match parser.section(Some("unit")) {
                    Some(x) => x.get("type"),
                    None    => Type::Poll,
                },
            command: None,
            regex: None,
            format: None,
            interval: 0,
        });
    }
}

fn load_unit(path : &PathBuf) -> Unit {
    let unit = Unit::new();
    return unit;
}

fn default_conf_dir<'a>() -> PathBuf {
    let mut config_fallback_path = env::home_dir()
        .expect("Failed to retrieve use home dir");
    config_fallback_path.push(".config/bard/");
    return config_fallback_path;
}

fn main() {
    let mut args = env::args();
    args.next(); //Discard the first (it just has our cmd line)
    let conf_path = match args.next() {
        Some(x) => PathBuf::from(x),
        None    => default_conf_dir(),
    };
    println!("Conf dir {}", conf_path.to_str().unwrap());

    let gconf = GlobalConfig::load_from_file(conf_path.clone());
    println!("Sep: {}", gconf.background().unwrap());
    println!("Hello World");
}
