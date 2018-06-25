
/**idea of module is to provide ability to read/write files in tmp directory in project folder*/


use std::fs::{File, remove_dir_all, create_dir_all};
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::io::Result;

pub mod visit_dirs;

pub fn make_AsRefPath_printable<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().as_os_str().to_os_string().into_string().unwrap()
}

/**
use code from
https://doc.rust-lang.org/beta/rust-by-example/std_misc/file/open.html
*/
pub fn read_from_path<P: AsRef<Path>>(file_path: P) -> Result<Vec<u8>>
{
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file: File = match File::open(&file_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => return Err(why),
            Ok(file) => file,
    };

    // Read the file contents into a Vec<u8>, returns `io::Result<usize>`
    let mut v: Vec<u8> = Vec::new();
    match file.read_to_end(&mut v) {
        Err(why) => return Err(why),
        Ok(_) => return Ok(v)
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

pub fn read_from_file_in_folder<P1: AsRef<Path>, P2: AsRef<Path>>(file_name: P1, folder_path: P2) -> Result<Vec<u8>>
{
    // Create a path to the desired file
    let file_path = folder_path.as_ref().as_os_str().to_os_string().into_string().unwrap() + &file_name.as_ref().as_os_str().to_os_string().into_string().unwrap();//todo get rid of extra variable
//    let path = Path::new(&file_path);
    read_from_path(Path::new(&file_path))
}

pub fn make_path_from_file_name_and_folder_path<P1: AsRef<Path>, P2: AsRef<Path>>(file_name: P1, folder_path: P2) -> PathBuf {
    let mut path_buf = PathBuf::new();
    path_buf.push(folder_path);
    path_buf.push(file_name);
    path_buf
//    &Path::new(&(folder_path.as_ref().as_os_str().to_os_string().into_string().unwrap() + &file_name.as_ref().as_os_str().to_os_string().into_string().unwrap()))
}

/**
use code
https://doc.rust-lang.org/beta/rust-by-example/std_misc/file/create.html
*/
//pub fn write_to_file<P1: AsRef<Path>, P2: AsRef<Path>, B: AsRef<[u8]>>(file_name: P1, file_content: B, folder_path: P2) -> Result<()>
pub fn write_to_file<P: AsRef<Path>, B: AsRef<[u8]>>(file_path: P, file_content: B) -> Result<()>
{
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&file_path) {
        Err(why) => return Err(why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(file_content.as_ref()) {
        Err(why) => return Err(why),
        Ok(_) => Ok(()),
    }
}

/**
removes tmp folder and recreates it
todo for some reason sometimes remove and create overlaps and folder is not created. Investigate?
*/
pub fn init<P: AsRef<Path>>(folder_path: P) -> Result<()>
    where P: Copy
{
    remove_dir_all(folder_path)?;
    create_dir_all(folder_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use config::TEMPORARY_FOLDER_PATH;
    use super::*;

    #[test]
    fn test_init() {
        init(TEMPORARY_FOLDER_PATH);
    }

    #[test]
    fn test_write() {
        write_to_file(make_path_from_file_name_and_folder_path("qwerty.txt", TEMPORARY_FOLDER_PATH), "qwerty");
    }


    #[test]
    fn test_read() {
        init(TEMPORARY_FOLDER_PATH);
        write_to_file(make_path_from_file_name_and_folder_path("qwerty.txt", TEMPORARY_FOLDER_PATH), "qwerty");
        assert_eq!("qwerty".as_bytes().to_vec(), read_from_file_in_folder("qwerty.txt", TEMPORARY_FOLDER_PATH).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_read_negative() {
        init(TEMPORARY_FOLDER_PATH);
        assert_eq!("qwerty".as_bytes().to_vec(), read_from_file_in_folder("qwerty2.txt", TEMPORARY_FOLDER_PATH).unwrap());
    }
}