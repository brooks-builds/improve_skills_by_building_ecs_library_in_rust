#[test]
fn options() {
    let _name = MyName {
        name: String::from("Brooks"),
    };
    dbg!(hello(None));
}

fn hello(name: Option<MyName>) -> String {
    let unwrapped_name = name.unwrap_or_else(|| MyName {
        name: "stranger".to_owned(),
    });
    format!("hello {}", unwrapped_name.name)
}

struct MyName {
    name: String,
}
