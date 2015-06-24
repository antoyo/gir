extern crate case;
extern crate docopt;
extern crate xml;
extern crate toml;

use env::Env;
use library::*;

mod codegen;
mod config;
mod env;
mod file_saver;
mod gobjects;
mod library;
mod nameutil;
mod parser;

fn main() {
    let cfg = config::Config::new();

    let mut library = Library::new();
    library.read_file(&cfg.girs_dir, &cfg.library_name);
    library.check_resolved();
    show(&library);

    let env = Env{ library: library, config: cfg };
    codegen::generate(&env);
}

#[allow(dead_code)]
fn show(library: &Library) {
    for ns in &library.namespaces {
        println!("Namespace: {}", ns.name);
        print!("\tNames: ");
        for name in ns.index.keys() {
            print!("{}, ", name);
        }
        println!("");

        for typ in &ns.types {
            match *typ {
                Some(Type::Class(ref x)) => println!("\tclass {}", x.name),
                Some(Type::Record(ref x)) => println!("\trecord {}", x.name),
                Some(Type::Union(ref x)) => println!("\tunion {}", x.name),
                Some(Type::Interface(ref x)) => println!("\tinterface {}", x.name),
                Some(Type::Callback(ref x)) => println!("\tcallback {}", x.name),
                Some(Type::Bitfield(ref x)) => println!("\tbitfield {}", x.name),
                Some(Type::Enumeration(ref x)) => println!("\tenumeration {}", x.name),
                _ => (),
            }
        }
        for c in &ns.constants {
            println!("\tconst {} = {}", c.name, c.value);
        }
        for f in &ns.functions {
            println!("\tfunction {}", f.name);
        }
    }
}