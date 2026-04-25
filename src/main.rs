fn first_word(text: &str) -> Option<&str> {
    if text.is_empty() {
        return None;
    }
    for (i, ch) in text.char_indices() {
        if ch == ' ' {
            return Some(&text[..i]);
        }
    }
    return Some(text);
}

fn main() {
    let text1 = "Hello World";
    let text2 = "Rust";
    let text3 = "";
    match first_word(text1) {
        Some(value) => println!("{}", value),
        None => println!("Пусто"),
    }
    match first_word(text2) {
        Some(value) => println!("{}", value),
        None => println!("Пусто"),
    }
    match first_word(text3) {
        Some(value) => println!("{}", value),
        None => println!("Пусто"),
    }
}
