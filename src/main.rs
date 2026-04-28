fn check_file_size(size: i32) -> Result<i32, String> {
    if size < 0 {
        return Err(String::from("размер не может быть отрицательным"));
    } else if size == 0 {
        return Err(String::from("файл пустой"));
    } else {
        return Ok(size);
    }
}

fn parse_file_size(text: &str) -> Result<i32, String> {
    match text.parse::<i32>() {
        Err(_) => return Err(String::from("размер должен быть числом")),
        Ok(size) => {
            return check_file_size(size);
        }
    }
}

fn main() {
    let size1 = "150";
    let size2 = "0";
    let size3 = "-20";
    let size4 = "abc";
    match parse_file_size(size1) {
        Ok(value) => println!("значение {}", value),
        Err(err) => println!("{}", err),
    }
    match parse_file_size(size2) {
        Ok(value) => println!("значение {}", value),
        Err(err) => println!("{}", err),
    }
    match parse_file_size(size3) {
        Ok(value) => println!("значение {}", value),
        Err(err) => println!("{}", err),
    }
    match parse_file_size(size4) {
        Ok(value) => println!("значение {}", value),
        Err(err) => println!("{}", err),
    }
}
