use super::*;

#[test]
fn parse_aml(){

    let filename = "example.aml";

    let data = load(filename.to_string());

    println!("{:?}", data);

    assert_eq!(data.get("key1").unwrap(), "data");
    assert_eq!(data.get("key2").unwrap(), "254235!fdjsfhksjd£$%");
    assert_eq!(data.get("key3").unwrap(), "Data with spaces");
}