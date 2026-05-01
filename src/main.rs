fn read_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(path)
}

fn print_hex_line(offset: usize, bytes: &[u8]) {
    print!("{:08X}: ", offset);

    for byte in bytes {
        print!("{:02X} ", byte);
    }

    for _ in bytes.len()..16 {
        print!("   ");
    }

    print!(" |");

    for byte in bytes {
        if byte.is_ascii_graphic() || *byte == b' ' {
            print!("{}", *byte as char);
        } else {
            print!(".");
        }
    }

    println!("|");
}

fn analyze_bytes(path: &str) {
    match read_bytes(path) {
        Ok(bytes) => {
            println!("file: {}", path);
            println!("bytes: {}", bytes.len());

            print_hex_dump_limited(&bytes, 8);
            let pattern = b"Rust";
            let offsets = find_all_patterns(&bytes, pattern);

            if offsets.is_empty() {
                println!("pattern not found");
            } else {
                println!("pattern found at offsets: {:?}", offsets);
            }
        }
        Err(err) => {
            println!("Ошибка: {}", err);
        }
    }
}

fn print_hex_dump_limited(bytes: &[u8], max_lines: usize) {
    for (i, chunks) in bytes.chunks(16).take(max_lines).enumerate() {
        let offset = i * 16;
        print_hex_line(offset, chunks);
    }
}

fn find_pattern(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    if pattern.is_empty() {
        return None;
    }
    for (i, window) in bytes.windows(pattern.len()).enumerate() {
        if window == pattern {
            return Some(i);
        }
    }
    None
}

fn find_all_patterns(bytes: &[u8], pattern: &[u8]) -> Vec<usize> {
    let mut offsets = Vec::new();
    for (i, window) in bytes.windows(pattern.len()).enumerate() {
        if window == pattern {
            offsets.push(i);
        }
    }
    return offsets;
}

fn main() {
    analyze_bytes("test.txt");
    analyze_bytes("missing.txt");
}
