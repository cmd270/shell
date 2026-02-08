use std::env;
use std::path::Path;

pub fn execute(args: Vec<&str>) {
    if args.is_empty() {
        let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"));
        if let Ok(home_path) = home {
            change_dir(home_path);
        }
        return;
    }

    let destination = args[0];
    change_dir(destination.to_string());
}

fn change_dir(target: String) {
    if let Err(e) = env::set_current_dir(Path::new(&target)) {
        eprintln!("Error <cd>:{}", e.to_string());
    }
}
