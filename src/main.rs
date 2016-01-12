extern crate ini;
mod global_config;

use std::path::PathBuf;
use ini::Ini;
use std::env;
use std::fmt::{self, Display};
use std::ascii::AsciiExt;
use global_config::GlobalConfig;

struct ParserError {
    file: String,
    section: &'static str,
    key: &'static str,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Error reading {} in {}:{}", self.file, self.section, self.key);
    }
}

struct UnitLoader;

impl UnitLoader {
    fn read<'a>(parser: &'a Ini, section: &str, key: &str) -> Option<&'a String> {
        return match parser.section(Some(section)) {
            Some(x) => match x.get(key) {
                Some(x) => Some(x),
                None    => None,
            },
            None    => None,
        }
    }

    fn parse_type(val: &str) -> ExecutionType {
        if val.eq_ignore_ascii_case("running") {
            return ExecutionType::Running;
        }
        return ExecutionType::Poll;
    }

    fn parse_exec(path: &str, parser: &Ini) -> Result<ExecuteDef, ParserError> {
        return Ok(ExecuteDef::new(
            match UnitLoader::read(&parser, "exec", "interval") {
                Some(x) => x.parse::<u32>().unwrap(),
                None    => return Err(ParserError { file: path.to_owned(), section: "exec", key: "interval" }),
            },
            match UnitLoader::read(&parser, "exec", "command") {
                Some(x) => x.clone(),
                None    => return Err(ParserError { file: path.to_owned(), section: "exec", key: "command" }),
            },
            match UnitLoader::read(&parser, "exec", "type") {
                Some(x) => UnitLoader::parse_type(x),
                None    => return Err(ParserError { file: path.to_owned(), section: "exec", key: "type" }),
            }
        ));
    }

    pub fn load(path: PathBuf) -> Result<UnitDef, ParserError> {
        let path_str = path.to_str().unwrap();
        let parser = Ini::load_from_file(path_str)
                    .expect("Failed starting ini parser for file");
        return Ok(UnitDef::new(
            match UnitLoader::read(&parser, "unit", "name") {
                Some(x) => x.clone(),
                None    => return Err(ParserError { file: path_str.to_owned(), section: "unit", key: "name" }),
            },
            match UnitLoader::parse_exec(path_str, &parser) {
                Ok(x)  => x,
                Err(x) => return Err(x),
            }
        ))
    }
}

enum ExecutionType {
    Poll,
    Running,
}

struct ExecuteDef {
    interval: u32,
    command: String,
    exec_type: ExecutionType,
}

impl ExecuteDef {
    pub fn new(interval: u32, command: String, exec_type: ExecutionType) -> ExecuteDef {
        return ExecuteDef {
            interval: interval,
            command: command,
            exec_type: exec_type,
        };
    }
}

struct UnitDef {
    name: String,
    execute: ExecuteDef,
}

impl UnitDef {
    pub fn new(name: String, exec: ExecuteDef) -> UnitDef {
        return UnitDef {
            name: name,
            execute: exec,
        }
    }
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
