use std::fs;
use std::io;
use std::path::PathBuf;

const IGNORE_DIRS: [&str; 4] = [".git", ".github", "layout", "site-build"];

// Copy the files and folders from the source folder to the destination folder
pub fn copy_files(src: &PathBuf, out: &PathBuf) -> io::Result<()> {
    let ignore = IGNORE_DIRS
        .iter()
        .map(|dir| dir.to_string())
        .chain(std::iter::once(
            out.file_name().unwrap().to_str().unwrap().to_string(),
        ))
        .collect::<Vec<String>>();

    dirs_walker(src)?
        .iter()
        .chain(std::iter::once(src))
        .filter(|path| !ignore.contains(&path.file_name().unwrap().to_str().unwrap().to_string()))
        .map(|path| {
            let dest = out.join(&path.strip_prefix(src).unwrap());
            if !dest.exists() {
                fs::create_dir_all(&dest).unwrap();
            }
            path
        })
        .filter_map(|dirs| files_walker(dirs).ok())
        .flatten()
        .map(|file| {
            let path = file.strip_prefix(src).unwrap().to_path_buf();
            (file, out.join(path))
        })
        .filter_map(|(src, out)| fs::copy(src, out).ok())
        .for_each(|_| ());

    Ok(())
}

// return the list of directories contained in the source folder
// and recursively in its subfolders
pub fn dirs_walker(source_folder: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut walker = fs::read_dir(source_folder)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|entry| {
            entry.is_dir() && !IGNORE_DIRS.contains(&entry.file_name().unwrap().to_str().unwrap())
        })
        .filter_map(|dir| dir.canonicalize().ok())
        .collect::<Vec<PathBuf>>();

    // recursively add subfolders
    walker.extend(
        walker
            .clone()
            .iter()
            .filter_map(|path| dirs_walker(&path).ok())
            .flatten(),
    );
    Ok(walker)
}

// return the list of files contained in the source folder
// and recursively its subfolders
pub fn files_walker(source_folder: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let walker = fs::read_dir(source_folder)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter_map(|entry| entry.path().canonicalize().ok())
        .collect::<Vec<PathBuf>>();
    Ok(walker)
}
