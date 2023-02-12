// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

mod tests;
mod paths;

use paths::normalize_paths;

use std::path::PathBuf;

pub fn create_tree(paths: &Vec<PathBuf>) -> String {
    let paths = normalize_paths(paths);
    
    let mut tree = String::new();

    for path in paths {
        let components: Vec<std::path::Component> = path.components().collect();

        if !components.is_empty() {
            if components.len() > 2 {
                for _ in 0..(components.len() - 2) {
                    tree.push_str("│  ");
                }
            }

            if components.len() > 1 {
                tree.push_str("├─ ");
            }

            tree.push_str(
                &components[components.len() - 1]
                    .as_os_str()
                    .to_string_lossy()
            );

            if path.to_string_lossy().ends_with('/') {
                tree.push('/');
            }

            tree.push('\n');
        }
    }

    tree.trim().to_string()
}
