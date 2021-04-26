fn main(){
    //decision single
    // let num:i32 = 5;
    // if num > 0 {
    //     println!("number is positive") ;
    // }

    //if else condition
    // let num = 12;
    // if num % 2==0 {
    //     println!("Even");
    // } else {
    //     println!("Odd");
    // }

    //nested if
    // let num = 2 ;
    // if num > 0 {
    //     println!("{} is positive",num);
    // } else if num < 0 {
    //     println!("{} is negative",num);
    // } else {
    //     println!("{} is neither positive nor negative",num) ;
    // }

    //Match statement
    let state_code = "MH";
    let state = match state_code {
        "MH" => {println!("Found match for MH"); "Maharashtra"},
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "GA" => "Goa",
        _ => "Unknown"
    };
    println!("State name is {}",state);
}