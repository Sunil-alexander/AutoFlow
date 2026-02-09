use std::fs;
use std::path::Path;

/// Initialize a new project
pub fn init_project(project_type: &str) {
    // Create a folder for the project
    let project_name = format!("new_{}", project_type);
    let path = Path::new(&project_name);

    if path.exists() {
        println!("Project '{}' already exists!", project_name);
        return;
    }

    if let Err(err) = fs::create_dir(&path) {
        eprintln!("Failed to create project folder: {}", err);
        return;
    }

    println!("Project '{}' initialized successfully!", project_name);

    // Optional: create Rust project structure
    if project_type.to_lowercase() == "rust" {
        let src_path = path.join("src");
        if let Err(err) = fs::create_dir(&src_path) {
            eprintln!("Failed to create src folder: {}", err);
            return;
        }

        // Create a main.rs file
        let main_file = src_path.join("main.rs");
        if let Err(err) = fs::write(
            &main_file,
            "fn main() {\n    println!(\"Hello, world!\");\n}\n",
        ) {
            eprintln!("Failed to create main.rs: {}", err);
            return;
        }

        println!("Rust project structure created!");
    }
}

/// Organize files in a folder by extension
pub fn organize_folder(folder: &str) {
    let folder_path = Path::new(folder);

    if !folder_path.exists() {
        eprintln!("Folder does not exist: {}", folder);
        return;
    }

    let entries = match fs::read_dir(folder_path) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Failed to read folder: {}", err);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                // FIXED: join directly with &OsStr
                let ext_folder = folder_path.join(ext);

                // Create folder for this extension if it doesn't exist
                if !ext_folder.exists() {
                    if let Err(err) = fs::create_dir(&ext_folder) {
                        eprintln!("Failed to create folder: {}", err);
                        continue;
                    }
                }

                let file_name = path.file_name().unwrap();
                let dest = ext_folder.join(file_name);
                if let Err(err) = fs::rename(&path, &dest) {
                    eprintln!("Failed to move file: {}", err);
                }
            }
        }
    }

    println!("Files in '{}' organized by extension!", folder);
}
