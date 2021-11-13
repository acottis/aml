This is a very basic library to read my AML (Adam Markup Language) files. The syntax is basic, but very strict, for simplicity.
This is a way to use "secret" variable files in Rust.

The library reads a .aml file and returns a dictionary of key/value pairs. This library can also create an .aml from a HashMap

Here is a file example.aml for reference
```aml
key1===data
key2===254235!fdjsfhksjd£$%
key3===Data with spaces
```

Syntax is:

`key===value`

No additional whitespace is needed. Any whitespace used will be considered as part of the key/value.

# Usage Examples

- Load File
```rust
    use aml::load;    

    let filename = "example.aml";
    let data = load(filename.to_string());

    println!("{:?}", data);
```
The above returns `{"key2": "254235!fdjsfhksjd£$%", "key3": "Data with spaces", "key1": "data"}`

- Save file
```rust
    use aml::save;

    let filename = "test_save.aml";
    let data = std::collections::HashMap::from([
       ("key1".to_string(), "data".to_string()),
       ("key2".to_string(), "254235!fdjsfhksjd£$%".to_string()),
       ("key3".to_string(), "Data with spaces".to_string()),
    ]);
    save(filename.to_string(), data).ok();
```
Creates a file called "test_save.aml" with contents
```aml
key1===data
key2===254235!fdjsfhksjd£$%
key3===Data with spaces
```

# Limitations
- Value or key cannot contain three consecutive equal signs `===`
- Key cannot end with 2 consecutive equal signs `==`