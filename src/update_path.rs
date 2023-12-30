use std::path::{Path, PathBuf};

pub fn update_path(path: impl AsRef<Path>, name: &str, extension: &Option<String>) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    dbg!(extension);
    result.set_file_name(name);
    match extension {
        Some(a) => _ = result.set_extension(a),
        None => {
            if let Some(ext) = path.extension() {
                result.set_extension(ext);
            } else {
                false;
            }

        }
    }
    result
}
#[test]
fn test_update_path_1(){
    let path = String::from("./testDir/text.txt");
    let name = "test1";
    let exp: PathBuf = PathBuf::from("./testDir/test1.txt");
    let ans: PathBuf = update_path(path, name, &None);
    dbg!(&exp,&ans);
    assert_eq!(exp, ans);
}
#[test]
fn test_update_path_2(){
    let path = String::from("./testDir/text.txt");
    let extension = Some(String::from("py"));
    let name = "test1";
    let exp: PathBuf = PathBuf::from("./testDir/test1.py");
    let ans: PathBuf = update_path(path, name, &extension);
    dbg!(&exp,&ans);
    assert_eq!(exp, ans);
}