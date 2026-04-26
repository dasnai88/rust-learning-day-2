fn main() {
    let words = vec![
        String::from("banana"),
        String::from("Apple"),
        String::from("Lox"),
    ];

    for word in &words {
        if word.chars().count() > 4 {
            println!("{}", word);
        }
    }
}
