use std::io;
use std::io::BufRead;

#[derive(PartialEq, Eq)]
pub enum Operation{
    LEFT,
    RIGHT,
    UP,
    DOWN,
    INVALID
}



pub struct Field{
    field: Vec<i32>,
    game_over: bool
}

pub fn read_input() -> Operation {
    let mut op: Operation;
    let mut buffer = String::new();
    loop {
        println!("Enter input (wasd):");
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        handle.read_line(&mut buffer);

        let mut str = buffer.to_string();
        str.pop();

        op = match str.as_str() {
            "w" => Operation::UP,
            "a" => Operation::LEFT,
            "s" => Operation::DOWN,
            "d" => Operation::RIGHT,
            _ => Operation::INVALID
        };

        match op {
            Operation::UP => println!("Input operation UP"),
            Operation::DOWN => println!("Input operation DOWN"),
            Operation::LEFT => println!("Input operation LEFT"),
            Operation::RIGHT => println!("Input operation RIGHT"),
            Operation::INVALID => println!("Input operation INVALID")
        }

        if ! (Operation::INVALID == op) {
            break;
        }
        // clear abuffer
        buffer.clear();
    }
    return op;
}