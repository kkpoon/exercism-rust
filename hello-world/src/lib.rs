pub fn hello(name: Option<&str>) -> String {
    match name {
        None => return "Hello, World!".to_string(),
        Some(n) => return format!("Hello, {}!", n).to_string()
    }
}
