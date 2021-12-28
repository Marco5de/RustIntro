/**
Following GitHub Tutorial Rust for C++ Developers
https://github.com/nrc/r4cppp
*/
mod TutorialDatatypes;
mod Game;

use std::rc::Rc;
use std::cell::RefCell;
use TutorialDatatypes::{TutorialPoint, TutorialTupleStruct, TutorialEnum, TutorialEnumExpression, TutorialNode};
use Game::{Field, Game2048Interface};


fn get_hello_world(_x: &'static str) -> &'static str {
    let ret = "Hello world";
    return ret;
}

fn get_higher_lower_str(x: f64, thresh: f64) -> &'static str {
    if x < thresh {
        return "less than threshold";
    } else if x == thresh {
        return "equal to thresh (but float comparison)";
    } else {
        return "greater than threshold";
    }
}

fn print_vector(vec: Vec<i32>) -> () {
    for v in vec.iter() {
        println!("{}", v);
    }

    for i in 0..vec.len() {
        println!("{}", vec[i]);
    }

    for (i, val) in vec.iter().enumerate() {
        println!("Pos= {} Value= {}", i, val);
    }
}

fn unique_ptr_sample01() -> Box<i32> {
    // allocates mem on heap for value 75 Box<i32> in this case
    // x is aptr to heap location containing value 75
    // mem of unique ptr will be cleaned once variable goes out of scope
    let _x = Box::new(75);
    let _y: Box<i32> = Box::new(42);

    // dereferencation same as c++
    println!("Value of ptr= {}", *_x);

    // default ptrs are immuatable
    let mut x = Box::new(1337);
    *x = 42;
    // returns ptr, no mem leak, will eventually be cleaned up when it goes out of scope
    return x;
}

fn some_func_unique_ptr(x: Box<i32>) -> () {
    println!("I'm doing something in here and your owning ptr is gone :D");
}

fn some_func_borrowed_ptr(x: &mut i32, y: &i32) -> () {
    // x can be changed, y not (const reference)
    *x = 1337;
    println!("y= {}", *y);
}

fn int_to_str(x: i32) -> &'static str{
    // no fall through as is the case in C++ and C
    let ret = match x {
        0 =>  "x is 0",
        1 =>  "x is 1",
        2 =>  "x is 2",
        y if ((y < 20) && (y > 10)) => "x is between 10 and 20", // using this conditions, ranges etc. can be checked
        _ => "Not implemented"
    };
    return ret;
}


fn tuple_example() -> (i32, i32, &'static str) {
    let x = 32;
    let y = 42;
    let z = "Hello World";
    return (x, y, z);
}


fn tutorial_main() -> () {
    let world_str = "world";
    let number = 1337;
    println!("Hello {} - {}", world_str, number);
    println!("{}", get_hello_world(""));

    println!("{}", get_higher_lower_str(10.0, 10.0));

    let first_num = true;
    let x = if first_num {
        10 // no semi colon as this value is "returned"
    } else {
        5
    };
    println!("Value= {}", x);


    let mut count = 0;
    while count < 5 {
        println!("Count= {}", count);
        count += 1;
    }

    let mut vec = Vec::new();
    vec.push(0);
    vec.push(1);
    vec.push(2);
    print_vector(vec);

    println!("{}", int_to_str(15));

    // primitive types in Rust
    let _x0: bool = true;
    let _x1 = 42; // infers type to i32 on this system
    let _x2: i8 = 42;
    let _x3: i16 = 42;
    let _x4: i32 = 42; // and so on
    let _x5: u8 = 42; // unsigned
    let _x6 = 10 as f32; // rust uses less coecerion -> as to cast

    // unique pointers stuff
    let mut ptr = unique_ptr_sample01();
    // only one owning pointer to any memory piece with unique ptr
    let y = ptr;
    // let z = *z; // moved into y --> x cannot be accessed anymore
    println!("Value= {}", *y);
    // if an owning ptr is given to a function it can no longer be accessed
    some_func_unique_ptr(y);
    // let z = *y; // results in error: use of moved value
    // owning ptr very similar to unique_ptr in C++,
    // however most of the checks are statically at compile time
    let x: Box<i32> = Box::new(42);
    let z = Box::new(x); // copies value to z

    // Borrowed pointers (references)
    let  x = &3; // create reference
    let y = *x;
    let z = &17;

    let mut u = 5;
    let x = &mut u; // reference itself may be mutable or not!
    // & does not allocate any memory -> only applicable on existing variables
    // with this multiple references to the same value are allowed
    some_func_borrowed_ptr(x, z);
    println!("x= {}", *x);

    // Reference counted and raw pointers
    // used much less and more or less everything can be achieved with the previous ptrs

    // rc ptrs are part of the std:lib use std::rc::Rc;
    // rc ptrs are always immutable for this use Cell / RefCell
    let x = Rc::new(17);
    let y = x.clone(); // increments the ref count

    //Ref Cell
    let mut x = RefCell::new(17);
    x = RefCell::new(1337); // kp ob das sinnvoll ist

    // Raw pointers
    unsafe {
        let mut x = 17;
        let x_p: *mut i32 = &mut x;

        if ! x_p.is_null() {
            println!("Ptr not null - value= {}", *x_p);
        }
    }

    // Datatypes
    // Structs
    let mut s = TutorialPoint{ x: 0.0, y: 0.0, z: 1.0};
    s.x = 10.0;

    // tuples
    let (_x, _y, _str) = tuple_example();
    let _tuple = tuple_example();

    let _tuple_struct = TutorialTupleStruct(17, 42, 1337);
    let enum_val = TutorialEnum::BLUE;
    match enum_val {
        TutorialEnum::DEFAULT => println!("Default value"),
        TutorialEnum::BLUE => println!("blue"),
        TutorialEnum::GREEN => println!("green"),
        TutorialEnum::RED => println!("red"),
        _ => println!("Invalid value")
    }

    let expr = TutorialEnumExpression::OR(true, false);

    let node = TutorialNode{parent: None, value: 42};
    let is_root = match node.parent {
        Some(_) => false,
        None => true
    };
    println!("Is root= {}", is_root);
}


fn main() {
    tutorial_main();


    let mut s = Field::new();
    s.game_loop();


}
