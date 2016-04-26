use config;
use env::Env;
use library;
use nameutil::signal_to_snake;
use parser::is_empty_c_type;
use super::bounds::{Bounds, BoundType};
use super::conversion_type::ConversionType;
use super::ffi_type::used_ffi_type;
use super::parameter;
use super::ref_mode::RefMode;
use super::rust_type::{bounds_rust_type, rust_type, used_rust_type};
use traits::IntoString;
use version::Version;

#[derive(Debug)]
pub struct Trampoline {
    pub name: String,
    pub parameters: Vec<parameter::Parameter>,
    pub ret: library::Parameter,
    pub bounds: Bounds,
    pub version: Option<Version>,
    pub deprecated_version: Option<Version>,
}

pub type Trampolines = Vec<Trampoline>;

pub fn analyze(env: &Env, signal: &library::Signal, type_tid: library::TypeId, in_trait: bool,
               trampolines: &mut Trampolines, used_types: &mut Vec<String>,
               version: Option<Version>, deprecated_version: Option<Version>)
               -> Result<String, Vec<String>> {
    let errors = closure_errors(env, signal);
    if !errors.is_empty() {
        warn!("Can't generate {} trampoline for signal '{}'", type_tid.full_name(&env.library),
              signal.name);
        return Err(errors);
    }

    let name = format!("{}_trampoline", signal_to_snake(&signal.name));

    let owner = env.type_(type_tid);

    let c_type = format!("{}*", owner.get_glib_name().unwrap());

    //Fake
    let configured_functions: Vec<&config::functions::Function> = Vec::new();

    let mut bounds: Bounds = Default::default();

    let mut parameters: Vec<parameter::Parameter> = Vec::with_capacity(signal.parameters.len() + 1);

    let this = parameter::Parameter {
        name: "this".to_owned(),
        typ: type_tid,
        c_type: c_type,
        instance_parameter: false, //true,
        direction: library::ParameterDirection::In,
        transfer: library::Transfer::None,
        caller_allocates: false,
        nullable: library::Nullable(false),
        allow_none: false,
        is_error: false,
        ref_mode: RefMode::ByRef,
        to_glib_extra: String::new(),
    };
    parameters.push(this);

    if let Some(s) = used_ffi_type(env, type_tid) {
        used_types.push(s);
    }

    if in_trait {
        let type_name = bounds_rust_type(env, type_tid);
        bounds.add_parameter("this", &type_name.into_string(), BoundType::IsA);
    }

    for par in &signal.parameters {
        let analysis = parameter::analyze(env, par, &configured_functions);

        if let Ok(s) = used_rust_type(env, par.typ) {
            used_types.push(s);
        }
        if let Some(s) = used_ffi_type(env, par.typ) {
            used_types.push(s);
        }

        parameters.push(analysis);
    }

    if signal.ret.typ != Default::default() {
        if let Ok(s) = used_rust_type(env, signal.ret.typ) {
            used_types.push(s);
        }
        if let Some(s) = used_ffi_type(env, signal.ret.typ) {
            used_types.push(s);
        }
    }

    let trampoline = Trampoline {
        name: name.clone(),
        parameters: parameters,
        ret: signal.ret.clone(),
        bounds: bounds,
        version: version,
        deprecated_version: deprecated_version,
    };
    trampolines.push(trampoline);
    Ok(name)
}

fn closure_errors(env: &Env, signal: &library::Signal) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();
    for par in &signal.parameters {
        if let Some(error) = type_error(env, par) {
            errors.push(format!("{} {}: {}", error, par.name,
                                par.typ.full_name(&env.library)));
        }
    }
    if signal.ret.typ != Default::default() {
        if let Some(error) = type_error(env, &signal.ret) {
            errors.push(format!("{} return value {}", error,
                                signal.ret.typ.full_name(&env.library)));
        }
    }
    errors
}

fn type_error(env: &Env, par: &library::Parameter) -> Option<&'static str> {
    use super::rust_type::TypeError::*;
    if par.direction == library::ParameterDirection::Out {
        Some("Out")
    } else if par.direction == library::ParameterDirection::InOut {
        Some("InOut")
    } else if is_empty_c_type(&par.c_type) {
        Some("Empty ctype")
    } else if ConversionType::of(&env.library, par.typ) == ConversionType::Unknown {
        Some("Unknown conversion")
    } else {
        match rust_type(env, par.typ) {
            Err(Ignored(_)) => Some("Ignored"),
            Err(Mismatch(_)) => Some("Mismatch"),
            Err(Unimplemented(_)) => Some("Unimplemented"),
            Ok(_) => None,
        }
    }
}
