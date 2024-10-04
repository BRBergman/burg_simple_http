use std::fs::File;
use std::io::Error;
use std::path::{Path, PathBuf};

impl ToResultPath for Path {
    /// checks if the Path is a File, 
    /// a Directory, 
    /// or Neither
    /// ```
    /// use std::fs::File;
    /// use std::path::{Path, PathBuf};
    /// 
    /// let path_full = Path::new("/some/path/");
    /// match path_full.to_result_path() {
    ///     ResultPath::File(file) => {
    ///         println!("{:?}",file);
    ///     }
    ///     ResultPath::Directory(path) => {
    ///         println!("{:?}",path);
    ///     }
    ///     ResultPath::Err(err) => {
    ///         panic!("not file or directory");
    ///     }
    /// };
    /// ```
    fn to_result_path(&self) -> ResultPath {
        ResultPath::from_path(self.to_path_buf())
    }
}
pub(crate) trait ToResultPath {
    fn to_result_path(&self) -> ResultPath;
}
#[allow(dead_code)]
#[derive(Debug)]
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
            match File::open(path_buf.clone()) {
                Ok(file) => {
                    return ResultPath::File(file);
                }
                Err(_) => (),
            };
        } else if path_buf.is_dir() {
            return ResultPath::Directory(path_buf);
        }
        return ResultPath::Err(Error::new(
            std::io::ErrorKind::NotFound,
            path_buf.to_str().unwrap(),
        ));
    }
}
