// Copyright (c) 2023 Zoe <zoe@zyoh.ca>

#![cfg(test)]

use super::create_tree;

use std::path::PathBuf;

const TEST_ARRAY: [&str; 15] = [
    "my-app/private/",
    "my-app/node_modules/",
    "my-app/public/",
    "my-app/public/favicon.ico",
    "my-app/public/index.html",
    "my-app/public/robots.txt",
    "my-app/src/",
    "my-app/src/index.css",
    "my-app/src/index.js",
    "my-app/.gitignore",
    "my-app/package.json",
    "my-app/README.md",
    "",
    "your-app",
    "your-app/private/",
];

const TEST_ARRAY_GOAL: &str = r#"my-app
├─ .gitignore
├─ README.md
├─ node_modules/
├─ package.json
├─ private/
├─ public/
│  ├─ favicon.ico
│  ├─ index.html
│  ├─ robots.txt
├─ src/
│  ├─ index.css
│  ├─ index.js
your-app
├─ private/"#;

#[test]
fn test_text_file_tree() {
    let mut paths: Vec<PathBuf> = Vec::new();
    for line in TEST_ARRAY {
        paths.push(PathBuf::from(line));
    }

    let tree = create_tree(&paths);

    assert_eq!(tree, TEST_ARRAY_GOAL);
}
