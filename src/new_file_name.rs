pub fn new_file_name(file_name: &str, add: &String) -> String {
    add.to_owned() + file_name
}
#[test]
fn test_nfn_1(){
    let file_name = String::from("test.txt");
    let add = String::from("1");
    assert_eq!("1test.txt", new_file_name(&file_name, &add));
}
#[test]
fn test_nfn_2(){
    let file_name = String::from("test.txt");
    let add = String::from("");
    assert_eq!(file_name, new_file_name(&file_name, &add));
}