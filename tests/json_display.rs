#[cfg(test)]
mod tests {
    use json_display::JsonDisplay;
    use serde::Serialize;

    #[derive(Serialize, JsonDisplay)]
    struct MyStruct {
        name: String,
        age: u32,

        #[serde(skip)]
        message: String,
    }

    #[test]
    fn it_works() {
        let my_struct = MyStruct {
            name: String::from("Alice"),
            age: 30,
            message: String::from("Hello, world!"),
        };
        println!("my_struct.message: {}", my_struct.message);
        assert_eq!(format!("{}", my_struct), r#"{"name":"Alice","age":30}"#);
    }
}
