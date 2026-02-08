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
        if path.is_dir() {}
    }
}

fn traverse_dir(dir: ReadDir) {
    println!("Directory of {}", dir.display());
    for dir_entry in dir {
        let entry = dir.unwrap();
        pretty_print(&entry);
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
