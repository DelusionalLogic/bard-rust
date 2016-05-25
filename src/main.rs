extern crate toml;
extern crate rustc_serialize;
extern crate getopts;

use std::path::PathBuf;
use std::env;
use std::fmt::{self, Display};
use getopts::Options;
use toml::Decoder;
use rustc_serialize::Decodable;

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
struct StringFormat {
    string: String,
}

#[derive(RustcDecodable)]
#[derive(Debug)]
struct ScriptFormat {
    script: String,
}

#[derive(RustcDecodable)]
#[derive(Debug)]
enum FormatDef {
    StringFormat(StringFormat),
    ScriptFormat(ScriptFormat),
}

#[derive(RustcDecodable)]
#[derive(Debug)]
struct Unit {
    name: String,
    run: RunDef,
    format: FormatDef,
    matching: MatchDef,
}

fn default_conf_dir<'a>() -> PathBuf {
    let mut config_fallback_path = env::home_dir()
        .expect("Failed to retrieve use home dir");
    config_fallback_path.push(".config/bard/");
    return config_fallback_path;
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    //Parse the arguments
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optopt("c", "config", "Set configuration directory", "PATH");
    opts.optflag("h", "help", "Print this help menu");
    let matches =  match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    //End of argument parsing

    let conf_path = match matches.opt_str("c") {
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
    [matching]
    regex="(asd*)"
    [format]
    script="as"
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
    let mut decoder = Decoder::new(toml::Value::Table(toml));
    let unit: Unit = match Decodable::decode(&mut decoder) {
        Ok(x) => x,
        Err(err) => panic!("Error reading config: {}", err),
    };
    println!("{:?}", unit);
    
    println!("Hello World");
}
