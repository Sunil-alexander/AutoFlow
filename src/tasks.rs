use std::fs;

pub fn init_project(project_type: &str) {
    let folder = format!("{}_project", project_type);
    fs::create_dir_all(&folder).expect("Failed to create folder");
    println!("Created project folder: {}", folder);
}

pub fn clean_system() {
    println!("Simulating system cleanup...");
}

pub fn organize_folder(folder: &str) {
    println!("Organizing folder: {}", folder);
}
