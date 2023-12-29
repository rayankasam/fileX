pub fn new_file_name(file_name: &str, add: &String, suffix: bool) -> String {
    let without_ext: String = file_name.chars().take_while(|elem| elem.ne(&'.')).collect();
    dbg!(&without_ext);
    if suffix {
        without_ext + add
    }
    else {
        add.to_owned() + &without_ext
    }
}
#[test]
fn test_nfn_1(){
    let file_name = String::from("test.txt");
    let add = String::from("1");
    assert_eq!("1test", new_file_name(&file_name, &add, false));
}
#[test]
fn test_nfn_2(){
    let file_name = String::from("test.txt");
    let add = String::from("");
    assert_eq!("test", new_file_name(&file_name, &add, false));
}
#[test]
fn test_nfn_3(){
    let file_name = String::from("test.txt");
    let add = String::from("123");
    assert_eq!("test123", new_file_name(&file_name, &add, true));
}