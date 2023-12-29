use std::path::{Path, PathBuf};

pub fn update_path(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    dbg!(&result);
    result.set_file_name(name);
    dbg!(&result);
    if let Some(ext) = path.extension() {
        result.set_extension(ext);
    }
    result
}
#[test]
fn test_update_path() {
    assert_eq!(1,1);
}