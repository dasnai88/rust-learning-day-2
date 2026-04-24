/*enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

fn calculate(operation: Operation, num1: i32, num2: i32) -> i32 {
    match operation {
        Operation::Add => num1 + num2,
        Operation::Sub => num1 - num2,
        Operation::Mul => num1 * num2,
        Operation::Div => num1 / num2,
    }
}

fn main() {
    let add = calculate(Operation::Add, 10, 12);
    let sub = calculate(Operation::Sub, 10, 12);
    let div = calculate(Operation::Div, 10, 12);
    let mul = calculate(Operation::Mul, 10, 12);

    println!("Add -{}, Sub -{}, Div - {}, Mul - {}", add, sub, div, mul)
}
*/
enum Message {
    Text(String),
    Number(i32),
    Empty,
}
fn print_message(message: Message) {
    match message {
        Message::Text(value) => println!("{}", value),
        Message::Number(value) => println!("{}", value),
        Message::Empty => println!("сообщение пустое"),
    }
}

fn main() {
    let text = String::from("Hello");
    let number: i32 = 12;
    print_message(Message::Text(text));
    print_message(Message::Number(number));
    print_message(Message::Empty);
}
