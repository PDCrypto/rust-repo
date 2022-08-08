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

    // String
    let str: &str = "hello world";
    let mut string: String = String::from("Hello World");

    let slice = &string[.. 6];
    println!("{}", slice.len());

    string.push('1');
    string.push_str("! Bob");
    string = string.replace("Hello", "Bye");
    println!("{}", string);
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