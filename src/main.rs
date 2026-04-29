fn read_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(path)
}

fn print_hex_line(offset: usize, bytes: &[u8]) {
    print!("{:08X}: ", offset);

    for byte in bytes {
        print!("{:02X} ", byte);
    }

    println!();
}

fn analyze_bytes(path: &str) {
    match read_bytes(path) {
        Ok(bytes) => {
            println!("file: {}", path);
            println!("bytes: {}", bytes.len());

            print_hex_dump(&bytes);
        }
        Err(err) => {
            println!("Ошибка: {}", err);
        }
    }
}

fn print_hex_dump(bytes: &[u8]) {
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let offset = i * 16;
        print_hex_line(offset, chunk);
    }
}

fn main() {
    analyze_bytes("test.txt");
    analyze_bytes("missing.txt");
}
