use log::error;
use std::{fs, path::PathBuf, process::exit};

pub fn read_input_or_crash(path: PathBuf) -> String {
    match fs::read_to_string(path.clone()) {
        Ok(result) => result,
        Err(e) => {
            let path_display = path.display();
            error!("Could not read {path_display}: {e}");
            exit(1)
        }
    }
}
