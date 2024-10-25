fn main() {
    let pattern = std::env::args().nth(1).expect("PAttern error message");
    let path = std::env::args().nth(2).expect("Path error message");

    println!("pattern: {:?}, path: {:?}", pattern, path)
}
