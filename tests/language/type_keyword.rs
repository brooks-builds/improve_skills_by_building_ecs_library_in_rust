type NameWithNumberOfExclamations = Option<(String, u8)>;

#[test]
fn type_keyword() {
    let name = Some(("Brooks".to_owned(), 3));
    say_hello(name);
}

fn say_hello(name: NameWithNumberOfExclamations) {
    dbg!(name);
}
