// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

use std::path::PathBuf;

pub(crate) fn normalize_paths(paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    // Add all parent directories of paths to paths and sort them

    let mut new_paths: Vec<PathBuf> = Vec::new();

    for path in paths {
        let mut new_path = PathBuf::new();
        for component in path.components() {
            new_path.push(component);
            if !paths.contains(&new_path) 
            && !new_paths.contains(&new_path) 
            && new_path.components().count() > 0 {
                new_paths.push(new_path.clone());
            }
        }
    }

    for path in paths {
        if !new_paths.contains(path) 
        && path.components().count() > 0 {
            new_paths.push(path.clone());
        }
    }

    new_paths.sort();

    new_paths
}
