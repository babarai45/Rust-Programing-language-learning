use std::io;

fn main(){

    println!("Hello World");

    let  name ="Babar";
    println!("Well come to {}" ,name);

    let _name = "Umair";
    println!("Well come to {}" ,_name);

//  take name from user
    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Well come to {}",name);

}