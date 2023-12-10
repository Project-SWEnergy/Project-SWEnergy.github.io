use std::fs;
use std::io;
use std::path::PathBuf;

const IGNORE_DIRS: [&str; 4] = [".git", ".github", "layout", "site-build"];

// Copy the files and folders from the source folder to the destination folder
pub fn copy_files(source_folder: &PathBuf, destination_folder: &PathBuf) -> io::Result<()> {
    // Create the destination folder if it doesn't exist
    if !destination_folder.exists() {
        fs::create_dir_all(destination_folder)?;
    }

    // Iterate through the entries in the source folder
    for entry in fs::read_dir(source_folder)? {
        let entry = entry?;
        let source_path = entry.path();
        let file_name = source_path.file_name().unwrap_or_default();

        if IGNORE_DIRS.contains(&file_name.to_str().unwrap()) {
            continue;
        }

        // Construct the destination path by joining the destination folder with the file/directory name
        let destination_path = destination_folder.join(&file_name);

        if source_path.is_file() {
            // If the entry is a file, copy it to the destination folder
            fs::copy(&source_path, &destination_path)?;
        } else if source_path.is_dir() {
            // If the entry is a directory, recursively copy its contents
            copy_files(&source_path, &destination_path)?;
        }
    }

    Ok(())
}

// return the list of directories contained in the source folder
// and recursively in its subfolders
pub fn dirs_walker(source_folder: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut walker = fs::read_dir(source_folder)?
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path())
        .filter(|entry| {
            entry.is_dir() && !IGNORE_DIRS.contains(&entry.file_name().unwrap().to_str().unwrap())
        })
        .map(|dir| dir.canonicalize().unwrap())
        .collect::<Vec<PathBuf>>();

    // recursively add subfolders
    walker.extend(
        walker
            .clone()
            .iter()
            .flat_map(|path| dirs_walker(&path).unwrap()),
    );
    Ok(walker)
}

// return the list of files contained in the source folder
// and recursively its subfolders
pub fn files_walker(source_folder: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let walker = fs::read_dir(source_folder)?
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_file())
        .map(|path| path.canonicalize().unwrap())
        .collect::<Vec<PathBuf>>();
    Ok(walker)
}
