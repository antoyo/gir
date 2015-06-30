use std::path::PathBuf;

use analysis;
use analysis::general::IsWidget;
use env::Env;
use file_saver::*;
use gobjects::*;
use nameutil::*;

pub fn generate(env: &Env) {
    let root_path = PathBuf::from(&env.config.target_path).join("src/widgets");

    for obj in env.config.objects.values() {
        if obj.status != GStatus::Generate || !obj.name.is_widget(&env.library){
            continue;
        }

        println!("Analyzing {:?}", obj.name);
        let class_analysis = analysis::widget::new(env, obj);
        if class_analysis.has_ignored_parents {
            println!("Skipping {:?}, it has ignored parents", obj.name);
            continue;
        }
        if class_analysis.all_constructors_deprecated {
            println!("Skipping {:?}, all his constructors deprecated", obj.name);
            continue;
        }

        let path = root_path.join(file_name(&class_analysis.full_name));
        println!("Generating file {:?}", path);

        save_to_file(path, &mut |w| super::widget::generate(w, env, &class_analysis));
    }
}
