use semver::VersionReq;
use std::io::prelude::*;
use std::fs::File;

use docopt::Docopt;
use toml;

use gobjects;

static USAGE: &'static str = "
Usage: gir [-d <girs_dir>] [-o <target_path>] [<library>]

Options:
    -d PATH             Directory for girs
    -o PATH             Target root path
";

#[derive(Debug)]
pub struct Config {
    pub girs_dir: String,
    pub library_name: String,
    pub target_path: String,
    pub objects: gobjects::GObjects,
    pub allowed_deprecated_version: VersionReq,
}

impl Config {
    pub fn new() -> Config {
        let args = Docopt::new(USAGE).unwrap()
            .parse().unwrap_or_else(|e| e.exit());

        let toml = read_toml("Gir.toml");

        let girs_dir = match args.get_str("-d") {
            "" => toml.lookup("options.girs_dir")
                      .unwrap_or_else(|| panic!("No options.girs_dir in config"))
                      .as_str().unwrap(),
            a => a
        };

        let library_name = match args.get_str("<library>") {
            "" => toml.lookup("options.library")
                    .unwrap_or_else(|| panic!("No options.library in config"))
                    .as_str().unwrap(),
            a => a
        };

        let target_path = match args.get_str("-o") {
            "" => toml.lookup("options.target_path")
                    .unwrap_or_else(|| panic!("No options.target_path in config"))
                    .as_str().unwrap(),
            a => a
        };

        let allowed_deprecated_version = VersionReq::parse(
            toml.lookup("options.allowed_deprecated_version").map(|t| t.as_str().unwrap())
            .unwrap_or("*")
        ).unwrap();

        let objects = gobjects::parse_toml(toml.lookup("object").unwrap());

        Config {
            girs_dir: girs_dir.into(),
            library_name: library_name.into(),
            target_path: target_path.into(),
            objects: objects,
            allowed_deprecated_version: allowed_deprecated_version,
        }
    }
}

fn read_toml(filename: &str) -> toml::Value {
    let mut input = String::new();
    File::open(filename).and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();
    let mut parser = toml::Parser::new(&input);
    match parser.parse() {
        Some(toml) => toml::Value::Table(toml),
        None => {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                         filename, loline, locol, hiline, hicol, err.desc);
            }
            panic!("Errors in config")
        }
    }
}
