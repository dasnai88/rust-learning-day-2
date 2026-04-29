fn read_file(path: &str) -> Result<String, std::io::Error> {
    return std::fs::read_to_string(path);
}
fn print_file_stats(content: &str) {
    println!(
        "bytes => {}, simvols => {}, stroki => {}, Words => {}",
        content.len(),
        content.chars().count(),
        content.lines().count(),
        content.split_whitespace().count()
    );
}
fn analyze_file(path: &str) {
    match read_file(path) {
        Ok(value) => {
            println!("{}", value);
            print_file_stats(&value);
        }
        Err(_) => println!("Ошибка"),
    }
}
fn main() {
    let path1 = "test.txt";
    let path2 = "missing.txt";
    analyze_file(path1);
    analyze_file(path2);
}
