use std::io::{Result, Write};

use analysis;
use env::Env;
use super::{function, general};

//TODO: where go `function`s: impl or trait?
pub fn generate<W: Write>(w: &mut W, env: &Env, analysis: &analysis::widget::Info) -> Result<()>{
    let type_ = analysis.type_(&env.library);

    let mut generate_impl = false;
    let mut generate_trait = false;

    generate_impl |= analysis.has_constructors && !analysis.all_constructors_deprecated;

    if analysis.has_children { generate_trait |= true } else { generate_impl |= true };

    try!(general::start_comments(w));
    //TODO: uses
    try!(general::objects_child_type(w, &analysis.name, &type_.glib_type_name));
    try!(general::impl_parents(w, &analysis.name, &analysis.parents));
    //TODO: impl interfaces

    if generate_impl {
        try!(writeln!(w, ""));
        try!(writeln!(w, "impl {} {{", analysis.name));
        for func_analysis in &analysis.constructors() {
            try!(function::generate(w, func_analysis, false, false, 1));
        }

        if !analysis.has_children {
            for func_analysis in &analysis.methods() {
                try!(function::generate(w, func_analysis, false, false, 1));
            }
        }
        try!(writeln!(w, "}}"));
    }
    try!(general::impl_static_type(w, &analysis.name, &type_.glib_get_type));

    if generate_trait {
        try!(writeln!(w, ""));
        try!(writeln!(w, "pub trait {}Ext {{", analysis.name));
        for func_analysis in &analysis.methods() {
            try!(function::generate(w, func_analysis, true, true, 1));
        }
        try!(writeln!(w, "}}"));

        try!(writeln!(w, ""));
        try!(writeln!(w, "impl<O: Upcast<{}>> {}Ext for O {{", analysis.name, analysis.name));
        for func_analysis in &analysis.methods() {
            try!(function::generate(w, func_analysis, true, false, 1));
        }
        try!(writeln!(w, "}}"));
    }

    Ok(())
}
