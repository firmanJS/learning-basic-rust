/* 
    let variable_name = value;            // no type specified
    let variable_name:dataType = value;   //type specified
*/

// example 1 Rules for Naming a Variable
// fn main() {
//    let fees = 25_000;
//    let salary:f64 = 35_000.00;
//    println!("fees is {} and salary is {}",fees,salary);
// }

// example 2 immutable
// fn main() {
//     let fees = 25_000;
//     println!("fees is {} ",fees);
//     fees = 35_000;
//     println!("fees changed is {}",fees);
// }

//example 3 muttable
// let mut variable_name = value;
// let mut variable_name:dataType = value;
// Let us understand this with an example

fn main() {
   let mut fees:i32 = 25_000;
   println!("fees is {} ",fees);
   fees = 35_000;
   println!("fees changed is {}",fees);
}