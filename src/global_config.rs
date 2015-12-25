extern crate ini;
use ini::Ini;
use std::env;
use std::option::Option;
use std::path::PathBuf;

pub struct GlobalConfig {
    iniparser: Ini,
}

impl GlobalConfig {
    pub fn load_from_file(mut path: PathBuf) -> GlobalConfig {
        path.push("bard.conf");
        let parser = Ini::load_from_file(path.to_str().expect("Invalid config path"))
            .expect("Failed parsing global config file");
        return GlobalConfig {
            iniparser : parser
        };
    }

    //TODO:Figure out why this needs to be &String.
    fn get_setting(&self, section: &str, key: &str) -> Option<&String> {
        return self.iniparser.section(Some(section)).and_then(|x| x.get(key));
    }

    pub fn separator(&self) -> Result<&str, &str> {
        return match self.get_setting("display", "separator") {
            Some(x) => Ok(x),
            None    => Ok(" "),
        };
    }

    pub fn font(&self) -> Result<&str, &str> {
        return match self.get_setting("display", "font") {
            Some(x) => Ok(x),
            None    => Ok(" "),
        };
    }

    pub fn monitor(&self) -> Result<u32 , &str> {
        return match self.get_setting("display", "monitors") {
            Some(x) => match x.parse::<u32>() {
                Ok(x)  => Ok(x),
                Err(_) => Err("Failed parsing monitor count"),
            },
            None    => Ok(1),
        };
    }

    pub fn geometry(&self) -> Result<&str, &str> {
        return match self.get_setting("bar", "geometry") {
            Some(x) => Ok(x),
            None    => Ok(""),
        };
    }

    pub fn path(&self) -> Result<&str, &str> {
        return match self.get_setting("bar", "path") {
            Some(x) => Ok(x),
            None    => Ok("lemonbar"),
        };
    }

    pub fn background(&self) -> Result<&str, &str> {
        return match self.get_setting("bar", "background") {
            Some(x) => Ok(x),
            None    => Ok(""),
        };
    }

    pub fn foreground(&self) -> Result<&str, &str> {
        return match self.get_setting("bar", "foreground") {
            Some(x) => Ok(x),
            None    => Ok(""),
        };
    }
}
