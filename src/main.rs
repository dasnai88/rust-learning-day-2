use std::env;

fn main(){
    let mut args = env::args();
    args.next();

    let name = args.next();
    let age = args.next();
    match (name,age) {
        (Some(name), Some(age)) => {
            println!("Hello {}, you {} old", name,age);
        }
        (Some(_), None) => println!("Введите возраст"),
        _ => println!("Введите имя и возраст"),
    }
}