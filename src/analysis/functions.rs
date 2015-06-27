use std::vec::Vec;

use env::Env;
use library;

pub struct Info {
    pub name: String,
    pub glib_name: String,
    pub kind: library::FunctionKind,
    pub comented: bool,
    pub deprecated: bool,
    //TODO: parameters, return value
}

pub fn analyze(env: &Env, type_: &library::Class, class_tid: library::TypeId) -> Vec<Info> {
    let mut funcs = Vec::new();

    for func in &type_.functions {
        let info = analyze_function(env, func, class_tid);
        funcs.push(info);
    }

    funcs
}

fn analyze_function(env: &Env, type_: &library::Function, class_tid: library::TypeId) -> Info {
    //TODO: temp
    let _ = class_tid;

    let deprecated = match type_.deprecated_version {
        Some(ref ver) => !env.config.allowed_deprecated_version.matches(ver),
        None => false,
    };

    Info {
        name: type_.name.clone(),
        glib_name: type_.c_identifier.clone(),
        kind: type_.kind,
        comented: false,
        deprecated: deprecated,
    }
}
