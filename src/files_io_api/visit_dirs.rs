/**
idea of the module is to provide functionality for recursive directory read and apply some function to files.
in test module need to provide example of anonymous function (e.g. just print path)
*/
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

// one possible implementation of walking a directory only visiting files
pub fn visit_dirs<P: AsRef<Path>>(dir_path: P, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir_path.as_ref().is_dir() {
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Path;
    use super::visit_dirs;
    use super::super::TEMPORARY_FOLDER_PATH;

    #[test]
    fn test_visit_dirs_print_hw() {
        visit_dirs(TEMPORARY_FOLDER_PATH, &|_|{println!("works!")});
    }

    #[test]
    fn test_visit_dirs_print_path() {
        visit_dirs(TEMPORARY_FOLDER_PATH, &|ref entry|{println!("{:?}", entry.path())});
    }
}