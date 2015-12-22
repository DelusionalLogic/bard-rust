extern crate ini;
mod global_config;

use std::path::PathBuf;
use std::env;
use global_config::GlobalConfig;

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
