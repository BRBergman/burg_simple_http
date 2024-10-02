use std::fs::File;
use std::io::Error;
use std::path::{Path, PathBuf};

impl ToResultPath for Path {
    ///checks if the Path is a File, a Directory, or Neither/ Does Not Exist
    /// ```
    ///let path_full = Path::new("/some/path");
    ///match path_full.to_result_path() {
    ///ResultPath::File(file) => {
    ///     //do something with file
    ///}
    ///ResultPath::Directory(path) => {
    ///    //do something with the path 
    ///}
    ///ResultPath::Err(err) => {
    ///panic!() 
    /// }};
    ///```
    fn to_result_path(&self) -> ResultPath {
        ResultPath::from_path(self.to_path_buf())
    }
}

impl ToResultPath for PathBuf {
    ///checks if the Path is a File, a Directory, or Neither / Does Not Exist
    /// ```
    ///let path_full = Path::new("/some/path");
    ///match path_full.to_result_path() {
    ///ResultPath::File(file) => {
    ///     //do something with file
    ///}
    ///ResultPath::Directory(path) => {
    ///    //do something with the path 
    ///}
    ///ResultPath::Err(err) => {
    ///panic!() 
    /// }};
    ///```
    fn to_result_path(&self) -> ResultPath {
        ResultPath::from_path(self.to_path_buf())
    }
}
pub(crate) trait ToResultPath {
    fn to_result_path(&self) -> ResultPath;
}
pub enum ResultPath {
    /// File of type:
    /// ```
    /// std::fs::File
    /// ```
    File(File),
    /// Directory of type:
    /// ```
    /// std::path::PathBuf
    /// ```
    Directory(PathBuf),
    /// Error
    Err(Error),
}

impl ResultPath {
    fn from_path(path_buf: PathBuf) -> ResultPath {
        if path_buf.is_file() {
            return ResultPath::File(File::open(path_buf).unwrap());
        } else if path_buf.is_dir() {
            return ResultPath::Directory(path_buf);
        }
        return ResultPath::Err(Error::new(
            std::io::ErrorKind::NotFound,
            path_buf.to_str().unwrap(),
        ));
    }
}
