use std::path::PathBuf;

use analysis;
use env::Env;
use file_saver::*;
use gobjects::*;
use nameutil::*;

pub fn generate(env: &Env) {
    let root_path = PathBuf::from(&env.config.target_path).join("src/widgets");
    let objs = &env.config.objects;
    for obj in objs {
        if obj.status != GStatus::Generate || obj.gtype != GType::Widget {
            continue;
        }

        println!("Analyzing {:?}", obj.name);
        let class_analysis = analysis::widget::new(env, obj);
        if class_analysis.has_ignored_parents {
            println!("Skipping {:?}, it has ignored parents", obj.name);
            continue;
        }

        let path = root_path.join(file_name(&class_analysis.full_name));
        println!("Generating file {:?}", path);

        save_to_file(path, &mut |w| super::widget::generate(w, env, &class_analysis));
    }
}
