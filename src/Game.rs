use std::io;
use std::io::BufRead;
use rand::Rng;
use std::os::linux::raw::stat;

fn add(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

pub trait Game2048Interface {
    fn game_loop(&mut self) -> ();
}

trait Game2048 {
    fn read_input(&self) -> Operation;
    fn update_field(&mut self, op: Operation) -> ();
    fn print_field(&self) -> ();
    fn is_game_over(&self) -> bool;

    fn shift_up(&mut self) -> ();
    fn shift_down(&mut self) -> ();
    fn shift_horizontal(&mut self, offset: i32) -> ();

    fn add_new(&mut self) -> ();
}


#[derive(PartialEq, Eq)]
pub enum Operation {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    INVALID,
}


pub struct Field {
    pub(crate) field: Vec<i32>,
    pub(crate) game_over: bool,
    field_size: usize,
    init_prob: f32,
    new_prob: f32,
}

impl Field {
    pub(crate) fn new() -> Field {
        let mut field = Field {
            field: Vec::with_capacity(4 * 4),
            game_over: false,
            field_size: 4,
            init_prob: 0.85,
            new_prob: 0.9,
        };

        let mut rng = rand::thread_rng();

        for idx in 0..field.field_size.pow(2) {
            let rand = rng.gen::<f32>();
            if rand > field.init_prob {
                field.field.push(2);
            } else {
                field.field.push(0);
            }
        }

        return field;
    }
}

impl Game2048 for Field {
    fn read_input(&self) -> Operation {
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

            if !(Operation::INVALID == op) {
                break;
            }
            // clear abuffer
            buffer.clear();
        }
        return op;
    }

    fn print_field(&self) -> () {
        println!("Current playing field: ");
        for (idx, value) in self.field.iter().enumerate() {
            if idx % self.field_size == 0 && idx != 0 {
                print!("\n{}\t\t", value);
            } else {
                print!("{}\t\t", value);
            }
        }
        println!("");
    }

    fn update_field(&mut self, op: Operation) -> () {
        // todo: not implemented
        println!("Update field is not yet implemented");
        match op {
            Operation::UP => self.shift_up(),
            Operation::DOWN => self.shift_down(),
            Operation::LEFT => self.shift_horizontal(1),
            Operation::RIGHT => self.shift_horizontal(-1),
            Operation::INVALID => eprintln!("Invalid operation in update field")
        }

        self.add_new();
    }

    fn shift_up(&mut self) -> () {
        let mut static_field = false;

        while !static_field {
            static_field = true;
            for idx in 0..self.field_size.pow(2) {
                // ignore upper / lower row
                if idx >= (self.field_size.pow(2) - self.field_size)
                    || self.field[idx + self.field_size] == 0 {
                    continue;
                }

                if self.field[idx] == self.field[idx + self.field_size] || self.field[idx] == 0 {
                    self.field[idx] += self.field[idx + self.field_size];
                    self.field[idx + self.field_size] = 0;
                    static_field = false;
                }
            }
        }
    }

    fn shift_down(&mut self) -> () {
        // todo: shift up and shift down are more or less duplicate code
        let mut static_field = false;

        while !static_field {
            static_field = true;
            for idx in 0..self.field_size.pow(2) {
                // ignore upper / lower row
                if idx < self.field_size
                    || self.field[idx - self.field_size] == 0 {
                    continue;
                }

                if self.field[idx] == self.field[idx - self.field_size] || self.field[idx] == 0 {
                    self.field[idx] += self.field[idx - self.field_size];
                    self.field[idx - self.field_size] = 0;
                    static_field = false;
                }
            }
        }
    }

    fn shift_horizontal(&mut self, offset: i32) -> () {
        assert!(offset == 1 || offset == -1);
        let side_bound = if offset == 1 { 1 } else { 0 };

        let mut static_field = false;
        while !static_field {
            static_field = true;
            for idx in 0..self.field_size.pow(2) {
                // always works due to assertion above! todo: find a better solution for this!
                let offset_idx = if idx != 0 { add(idx, offset).unwrap() } else { 1 };
                if (idx + side_bound) % self.field_size == 0 || self.field[offset_idx] == 0 {
                    continue;
                }
                // if fields have same value, add them
                if self.field[idx] == self.field[offset_idx] || self.field[idx] == 0{
                    self.field[idx] += self.field[offset_idx];
                    self.field[offset_idx] = 0;
                    static_field = false;
                }
            }
        }
    }

    fn add_new(&mut self) {
        let mut rng = rand::thread_rng();

        for v in self.field.iter_mut() {
            if *v == 0 && rng.gen::<f32>() > self.new_prob {
                *v = 2;
            }
        }
    }

    fn is_game_over(&self) -> bool {
        // todo: not implemented
        return false;
    }
}

impl Game2048Interface for Field {
    fn game_loop(&mut self) -> () {
        println!("\n\nWelcome to the 2048 game!");
        loop {
            self.print_field();
            let op = self.read_input();
            self.update_field(op);
        }
    }
}