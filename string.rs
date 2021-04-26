fn main() {
    /* 
        The String data type in Rust can be classified into the following 
        −String Literal(&str)
        -tring Object(String)
    */

    //example 1
    // let company:&str="JDS";
    // let location:&str = "Bandung";
    // println!("company is : {} location :{}",company,location);

    //example 2
    // let company:&'static str = "JDS";
    // let location:&'static str = "Bandung";
    // println!("company is : {} location :{}",company,location);

    //example 3
    // let empty_string = String::new();
    // println!("length is {}",empty_string.len());

    // let content_string = String::from("JDS");
    // println!("length is {}",content_string.len());

    // new method in string
    // let mut z = String::new();
    // z.push_str("hello");
    // println!("{}",z);

    // to string
    // let name1 = "Hello JDS , 
    // Hello!".to_string();
    // println!("{}",name1);
    
    // replace
    // let name1 = "Hello JDS , 
    // Hello!".to_string();         //String object
    // let name2 = name1.replace("Hello","Howdy");    //find and replace
    // println!("{}",name2);

    //literal
    // fn print_literal(data:&str ){
    //     println!("displaying string literal {}",data);
    // }

    // let example_string = String::from("example_string");
    // print_literal(example_string.as_str());

    //push
    // let mut company = "Tutorial".to_string();
    // company.push('s');
    // println!("{}",company);

    //push str
    // let mut company = "JDS".to_string();
    // company.push_str(" Ngabret");
    // println!("{}",company);
    
    //len
    // let fullname = " JDS";
    // println!("length is {}",fullname.len());

    //trim
    // let fullname = " JDS \r\n";
    // println!("Before trim ");
    // println!("length is {}",fullname.len());
    // println!();
    // println!("After trim ");
    // println!("length is {}",fullname.trim().len());

    //split white space
    // let msg = "JDS has good IT".to_string();
    // let mut i = 1;
    
    // for token in msg.split_whitespace(){
    //    println!("token {} {}",i,token);
    //    i+=1;
    // }


    //split string\
    // let fullname = "Firman,JS,JDS";

    // for token in fullname.split(","){
    //     println!("token is {}",token);
    // }

    // //store in a Vector
    // println!("\n");
    // let tokens:Vec<&str>= fullname.split(",").collect();
    // println!("firstName is {}",tokens[0]);
    // println!("lastname is {}",tokens[1]);
    // println!("company is {}",tokens[2]);

    //chars
    // let n1 = "JDS".to_string();

    // for n in n1.chars(){
    //     println!("{}",n);
    // }

    //Concatenation
    // let n1 = "JDS".to_string();
    // let n2 = "NGabrets".to_string();
 
    // let n3 = n1 + &n2; // n2 reference is passed
    // println!("{}",n3);

    //casting
    // let number = 2020;
    // let number_as_string = number.to_string(); 
    
    // // convert number to string
    // println!("{}",number_as_string);
    // println!("{}",number_as_string=="2020");

    //!format
    let n1 = "JDS".to_string();
    let n2 = "Jabar".to_string();
    let n3 = format!("{} {}",n1,n2);
    println!("{}",n3);
}