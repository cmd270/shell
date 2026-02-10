use std::env;
use std::fs::DirEntry;
use std::fs::ReadDir;
use std::path::Path;
use std::time::UNIX_EPOCH;

pub fn execute(path: Vec<&str>) {
    if path.is_empty() {
        let current_path = env::current_dir().unwrap_or_default();
        match std::fs::read_dir(current_path.as_path()) {
            Ok(result) => {
                traverse_dir(result);
            }
            Err(e) => {
                eprintln!("Error: {}", e.to_string());
            }
        }
    } else {
        let path_str = path.join(" ");
        let path = Path::new(path_str.as_str());
        if path.is_file() {}
        if path.is_dir() {
            match std::fs::read_dir(path) {
                Ok(result) => {
                    traverse_dir(result);
                }
                Err(e) => {
                    eprintln!("Error: {}", e.to_string());
                }
            }
        }
    }
}

fn traverse_dir(dir: ReadDir) {
    for dir_entry in dir {
        pretty_print(&dir_entry.unwrap());
    }
}

fn pretty_print(entry: &DirEntry) {
    let metadata = entry.metadata().unwrap();
    let created = metadata
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    println!(
        "{} {:>20} {:>8}",
        entry.file_name().display(),
        metadata.len(),
        created.as_secs()
    );
}
