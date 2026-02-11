use std::env;
use std::path::PathBuf;
use std::fs;
use tabular::{Table, Row};

pub fn execute(path: Vec<&str>) {
    let dir_path: PathBuf = if path.is_empty() {
        env::current_dir().unwrap_or_default()
    } else {
        PathBuf::from(path.join(" ")) 
    };

    let mut table = Table::new("{:>}  {:<} {:<}  {:<}");

    if let Ok(entries) = fs::read_dir(&dir_path) {
        for entry_result in entries {
            if let Ok(entry) = entry_result {
                if let Ok(metadata) = entry.metadata() {
                    let size = metadata.len();
                    let readonly = if metadata.permissions().readonly() { "r" } else { "-" };
                    let is_dir = if metadata.is_dir() { "d" } else { "f" };
                    let name = entry.file_name().display().to_string();

                    table.add_row(Row::new()
                        .with_cell(size)
                        .with_cell(readonly)
                        .with_cell(is_dir)
                        .with_cell(name));
                }
            }
        }
    } else {
        eprintln!("Error: cant read directory {:?}", dir_path);
        return;
    }

    print!("{}", table);
}