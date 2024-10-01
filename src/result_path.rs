use std::io::Error;
use std::path::{Path, PathBuf};

impl ToResultPath for Path {
    fn to_result_path(&self) -> ResultPath {
        ResultPath::from_path(self.to_path_buf())
    }
}
impl ToResultPath for PathBuf {
    fn to_result_path(&self) -> ResultPath {
        ResultPath::from_path(self.to_path_buf())
    }
}
pub(crate) trait ToResultPath {
    fn to_result_path(&self) -> ResultPath;
}
pub enum ResultPath {
    File(PathBuf),
    Directory(PathBuf),
    Err(Error),
}

impl ResultPath {
    fn from_path(path: PathBuf) -> ResultPath {
        if path.is_file() {
            return ResultPath::File(path);
        } else if path.is_dir() {
            return ResultPath::Directory(path);
        }
        return ResultPath::Err(Error::new(
            std::io::ErrorKind::NotFound,
            path.to_str().unwrap(),
        ));
    }
}
