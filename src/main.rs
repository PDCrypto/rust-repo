use std::collections::HashMap;

fn main() {
    // let unsigned: u8 = 10;
    // // let signed: i8 = -10;

    // let emoji = "\u{1F600}";
    // println!("Hello, world, {} {}!", unsigned, emoji);

    // // arrays
    // let arr: [u8; 3] = [1, 2, 3];
    // let other_arr: [u8; 5] = [100; 5];

    // println!("index {}, length: {}", arr[0], other_arr.len());

    // println!("{:?}", other_arr);

    // // functions
    // println!("{}", is_even(1));

    // // mutable
    // let mut num = 5;
    // num = 3;
    // println!("{}", num);

    // // slice
    // let arr = [0, 1, 2, 3];
    // let slice = &arr[1 .. 3]; // sub-array, don't know the length
    // borrowing_slice(arr, slice);

    // // String
    // let str: &str = "hello world";
    // let mut string: String = String::from("Hello World");

    // let slice = &string[.. 6];
    // println!("{}", slice.len());

    // string.push('1');
    // string.push_str("! Bob");
    // string = string.replace("Hello", "Bye");
    // println!("{}", string);

    // // match statement
    // let i = 4;
    // match i {
    //     0 => println!("0"),
    //     1 | 2 => println!("1,2"),
    //     3..=5 => println!("3,4,5"),
    //     _ => println!("default")
    // }

    // // Struct
    // let name = String::from("Farfetch");
    // let bird = Bird {name: name, attack: 5};
    // bird.print_name();
    // println!("{} {}", bird.can_fly(), bird.is_animal());

    // // maps
    // let mut map = HashMap::new();

    // map.insert(0, "Hi");
    // map.insert(1, "Hi2");
    // println!("{:?}", map);

    // match map.get(&2) {
    //     Some(str) => println!("{}", str),
    //     _ => println!("Doesn't Exist.")
    // }

    // map.remove(&0);
    // println!("{:?}", map);

    // // Options
    // let divide1: Option<i32> = divide(4, 2);
    // let divide2: Option<i32> = divide(2, 3);

    // println!("{:?} unwraps to {}", divide1, divide1.unwrap());
    // println!("{:?} unwraps to {}", divide2, divide2.unwrap());

    // // Result
    // let divide = divide(4, 2);
    // // let res = divide.expect("Crashed");

    // if divide.is_ok() {
    //     println!("{}", divide.unwrap());
    // }
    // // println!("{}", divide.unwrap_or(100));
    // // println!("{}", res);
}

// pub fn is_even(num: u8) -> bool {
//     let digit: u8 = num % 2;
//     digit == 0 // return bool
// }

// fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
//     println!("{:?}", arr);
//     println!("{:?}", slice);
//     println!("length: {}", slice.len());
//     println!("{} {}", slice[0], slice[1]);
// }

// // struct
// struct Bird {
//     name: String,
//     attack: u64
// }

// impl Bird {
//     fn print_name(&self) {
//         println!("{} {}", self.name, self.attack);
//     }
// }

// impl Animal for Bird {
//     fn can_fly(&self) -> bool {
//         true
//     }
//     fn is_animal(&self) -> bool {
//         false
//     }
// }

// trait Animal {
//     fn can_fly(&self) -> bool;
//     fn is_animal(&self) -> bool {
//         true
//     }
// }

// // Options
// fn divide(dividend: i32, divisor: i32) -> Option<i32> {
//     if dividend % divisor != 0 {
//         None
//     } else {
//         Some (dividend / divisor)
//     }
// }

// // Results
// #[derive(Debug)]
// enum MyError {
//     Error1
// }

// fn divide(dividend: i32, divisor: i32) -> Result<i32,MyError> {
//     if dividend % divisor != 0 {
//         Err(MyError::Error1)
//     } else {
//         Ok(dividend / divisor)
//     }
// }