use std::fs;
use std::io;
use std::path::Path; // Add this import statement

// list all files in a directory
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
