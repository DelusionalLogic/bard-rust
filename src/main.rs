extern crate toml;
extern crate rustc_serialize;

use std::path::PathBuf;
use std::env;
use std::fmt::{self, Display};
use std::error;
use std::ascii::AsciiExt;

#[derive(Debug)]
enum ExecutionType {
    Poll,
    Running,
}

impl Display for ExecutionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ExecutionType::Poll => return write!(f, "Poll"),
            &ExecutionType::Running => return write!(f, "Running"),
        };
    }
}

impl rustc_serialize::Decodable for ExecutionType {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<ExecutionType, D::Error> {
        match try!(d.read_str()).as_str() {
            "Poll" => return Ok(ExecutionType::Poll),
            "Running" => return Ok(ExecutionType::Running),
            _ => return Err(d.error("String not a valid execution type")),
        };
    }
}

#[derive(RustcDecodable)]
#[derive(Debug)]
struct RunDef {
    mode: ExecutionType,
    script: String,
}

impl Display for RunDef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Mode = ({}), Script = ({})", self.mode, self.script);
    }
}

#[derive(RustcDecodable)]
#[derive(Debug)]
struct MatchDef {
   regex : String,
}

#[derive(RustcDecodable)]
#[derive(Debug)]
struct FormatDef {
    string: Option<String>,
    script: Option<String>,
}

#[derive(RustcDecodable)]
#[derive(Debug)]
struct Unit {
    name: String,
    run: RunDef,
    format: FormatDef,
    exp: MatchDef,
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

    //let gconf = GlobalConfig::load_from_file(conf_path.clone());
    //println!("Sep: {}", gconf.background().unwrap());
    let mut parser = toml::Parser::new(r##"
    name="hello"
    [run]
    mode="Poll"
    script="/home/delusional/aaa2"
    [exp]
    regex="(asd*)"
    [format]
    string="$1asd$1"
    "##);
    let toml = match parser.parse() {
        Some(toml) => toml,
        None       => {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                         "aa", loline, locol, hiline, hicol, err.desc);
            }
            panic!();
        }
    };
    let unit: Unit = toml::decode(toml::Value::Table(toml)).unwrap();
    println!("{:?}", unit);
    
    println!("Hello World");
}
