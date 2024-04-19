use std::fs;
use std::io;

use serde::{Deserialize, Serialize};

// list all files in a directory
#[allow(dead_code)]
pub fn list_files(dir: &str) -> Result<Vec<String>, io::Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path.display().to_string());
        }
    }
    Ok(files)
}

// list all source code files (go, rust, c, c++, java, python) in a directory
#[allow(dead_code)]
pub fn list_source_files(dir: &str) -> Result<Vec<String>, io::Error> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let ext = path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            if ext == "go"
                || ext == "rs"
                || ext == "c"
                || ext == "cpp"
                || ext == "java"
                || ext == "py"
            {
                files.push(path.display().to_string());
            }
        }
    }
    Ok(files)
}

/// FileContent is a struct to store file name and associated content
#[derive(Serialize, Deserialize)]
struct FileContent {
    name: String,
    content: String,
}

#[allow(dead_code)]
// generate a list of file name and content JSON result
// { "file1": "content1", "file2": "content2", "file3": "content3" }
pub fn list_files_content(dir: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut files_content = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let content = fs::read(&path)?;
            let content = String::from_utf8_lossy(&content);
            files_content.push(FileContent {
                name: path.to_string_lossy().into_owned(),
                content: content.into_owned(),
            });
        }
    }
    let json = serde_json::to_string(&files_content)?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_list_files() {
        let dir = "."; // test with the current directory
        match list_files(dir) {
            Ok(files) => {
                for file in files {
                    println!("files -> {}", file);
                    assert!(Path::new(&file).is_file());
                }
            }
            Err(e) => panic!("Failed to list files: {}", e),
        }
    }

    #[test]
    fn test_list_source_files() {
        let dir = "."; // test with the current directory
        match list_source_files(dir) {
            Ok(files) => {
                for file in files {
                    println!("source codes -> {}", file);
                    assert!(Path::new(&file).is_file());
                }
            }
            Err(e) => panic!("Failed to list source files: {}", e),
        }
    }

    #[test]
    fn test_list_files_content() {
        let dir = "test_dir";
        let file1 = "test_dir/file1.txt";
        let file2 = "test_dir/file2.txt";
        let content1 = "Hello, world!";
        let content2 = "Hello, Rust!";

        // Setup: create a test directory with two files
        fs::create_dir_all(dir).unwrap();
        let mut f1 = File::create(file1).unwrap();
        let mut f2 = File::create(file2).unwrap();
        f1.write_all(content1.as_bytes()).unwrap();
        f2.write_all(content2.as_bytes()).unwrap();

        // Test the function
        let result = list_files_content(dir).unwrap();
        let files_content: Vec<FileContent> = serde_json::from_str(&result).unwrap();

        // Check if the function correctly listed the files and their content
        assert_eq!(files_content.len(), 2);
        let file1_canonical = fs::canonicalize(file1).unwrap();
        let file2_canonical = fs::canonicalize(file2).unwrap();
        for file_content in files_content {
            let file_content_canonical = fs::canonicalize(file_content.name.clone()).unwrap();
            if file_content_canonical == file1_canonical {
                assert_eq!(file_content.content, content1);
            } else if file_content_canonical == file2_canonical {
                assert_eq!(file_content.content, content2);
            } else {
                panic!("Unexpected file: {}", file_content.name);
            }
        }

        // Cleanup: remove the test directory
        fs::remove_dir_all(dir).unwrap();
    }
}
