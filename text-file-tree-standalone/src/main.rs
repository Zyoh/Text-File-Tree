// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use std::{path::PathBuf, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();

    let root = std::env::args().nth(1).ok_or("No root path provided")?;
    let root_path = PathBuf::from(&root);

    std::env::set_current_dir(&root_path)?;

    for entry in glob::glob("**/*")? {
        if let Ok(path) = entry {
            if path.is_file() {
                files.push(path);
            }
        }
    }

    let root_name = root_path
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or("tree".into());

    let output_name = format!("{}-tree.txt", root_name);
    let output_path = root_path.join(output_name);

    if output_path.exists() {
        return Err("Output file already exists".into());
    } else {
        let tree = text_file_tree::create_tree(&files);
        std::fs::write(output_path, tree)?;
    }

    Ok(())
}