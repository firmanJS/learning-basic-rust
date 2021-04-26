// bool : The boolean type.
// char : A character type.
// i8 : The 8-bit signed integer type.
// i16 : The 16-bit signed integer type.
// i32 : The 32-bit signed integer type.
// i64 : The 64-bit signed integer type.
// isize : The pointer-sized signed integer type.
// u8 : The 8-bit unsigned integer type.
// u16 : The 16-bit unsigned integer type.
// u32 : The 32-bit unsigned integer type.
// u64 : The 64-bit unsigned integer type.
// usize : The pointer-sized unsigned integer type.
// f32 : The 32-bit floating point type.
// f64 : The 64-bit floating point type.
// array : A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
// slice : A dynamically-sized view into a contiguous sequence, [T].
// str : String slices.
// tuple : A finite heterogeneous sequence, (T, U, ..).
// example 1 Use the let keyword to declare a variable
// fn main() {
//     let company_string = "Tutorials"; // string type
//     let rating_float = 4.5; // float type
//     let is_growing_boolean = true; // boolean type
//     let icon_char = '♥'; //unicode character type

//     println!("company name is:{}", company_string);
//     println!("company rating on 5 is:{}", rating_float);
//     println!("company is growing :{}", is_growing_boolean);
//     println!("company icon is:{}", icon_char);
// }

// example 2 using let and define data type
// fn main() {
//     let result = 10;    // i32 by default
//     let age:u32 = 20;
//     let sum:i32 = 5-15;
//     let mark:isize = 10;
//     let count:usize = 30;
//     println!("result value is {}",result);
//     println!("sum is {} and age is {}",sum,age);
//     println!("mark is {} and count is {}",mark,count);
//  }

//example 3 integer overflow
// fn main() {
//    let age:u8 = 255;

//    // 0 to 255 only allowed for u8
//    let weight:u8 = 256;   //overflow value is 0
//    let height:u8 = 257;   //overflow value is 1
//    let score:u8 = 258;    //overflow value is 2

//    println!("age is {} ",age);
//    println!("weight is {}",weight);
//    println!("height is {}",height);
//    println!("score is {}",score);
// }

//example 4 float
// fn main() {
//     let result = 10.00; //f64 by default
//     let interest: f32 = 8.35;
//     let cost: f64 = 15000.600; //double precision

//     println!("result value is {}", result);
//     println!("interest is {}", interest);
//     println!("cost is {}", cost);
// }

// example 5 number seperator
// fn main() {
//     let float_with_separator = 11_000.555_001;
//     println!("float value {}",float_with_separator);
    
//     let int_with_separator = 50_000;
//     println!("int value {}",int_with_separator);
// }

//example 6 boolean
// fn main() {
//     let isfun:bool = true;
//     println!("Is Rust Programming Fun ? {}",isfun);
// }

//example 7 charset
fn main() {
    let special_character = '@'; //default
    let alphabet:char = 'A';
    let emoji:char = '😁';
    
    println!("special character is {}",special_character);
    println!("alphabet is {}",alphabet);
    println!("emoji is {}",emoji);
 }