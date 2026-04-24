/*struct User {
    name: String,
    age: i32,
    active: bool,
}
fn main() {
    let user1 = User {
        name: String::from("Ilshat"),
        age: 32,
        active: true,
    };
    println!("{}", user1.name)
}*/

struct FileInfo {
    name: String,
    size: u64,
    is_readonly: bool,
}

impl FileInfo {
    fn print_info(&self) {
        println!(
            "name: {}, size: {}, is_readonly: {}",
            self.name, self.size, self.is_readonly
        );
    }
    fn is_empty(&self) -> bool {
        if self.size == 0 {
            return true;
        } else {
            return false;
        }
    }
}

fn print_file_info(file: &FileInfo) {
    println!(
        "name: {}, size: {}, is_readonly: {}",
        file.name, file.size, file.is_readonly
    );
}

fn main() {
    let file1 = FileInfo {
        name: String::from("ZIP"),
        size: 10,
        is_readonly: true,
    };
    let file2 = FileInfo {
        name: String::from("RAR"),
        size: 0,
        is_readonly: true,
    };

    file1.print_info();
    file2.print_info();
    println!("1 - {}, 2 - {}", file1.is_empty(), file2.is_empty());
    print_file_info(&file1);
}
