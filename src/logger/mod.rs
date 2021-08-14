pub fn ok(title: &str, info: &str) {
    println!("OK    {:>12} {}", title, info)
}

pub fn info(title: &str, info: &str) {
    println!("INFO  {:>12} {}", title, info)
}

pub fn err(title: &str, info: &str) {
    println!("ERROR {:>12} {}", title, info)
}