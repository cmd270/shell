use std::env;

pub fn execute() {
    let current_path = env::current_dir().unwrap_or_default();
    println!("{}", current_path.display());
}
