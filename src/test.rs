use super::*;

#[test]
fn parse_aml(){

    let filename = "example.aml";

    let data = load(filename.to_string());

    assert_eq!(data.get("key1").unwrap(), "data");
    assert_eq!(data.get("key2").unwrap(), "254235!fdjsfhksjd£$%");
    assert_eq!(data.get("key3").unwrap(), "Data with spaces");
}

#[test]
fn save_and_load_aml(){

    let filename = "test_save.aml";

    let data = HashMap::from([
        ("key1".to_string(), "data".to_string()),
        ("key2".to_string(), "254235!fdjsfhksjd£$%".to_string()),
        ("key3".to_string(), "Data with spaces".to_string()),
    ]);

    save(filename.to_string(), data).unwrap();

    let data = load(filename.to_string());

    assert_eq!(data.len(), 3);
    assert_eq!(data.get("key1").unwrap(), "data");
    assert_eq!(data.get("key2").unwrap(), "254235!fdjsfhksjd£$%");
    assert_eq!(data.get("key3").unwrap(), "Data with spaces");
}