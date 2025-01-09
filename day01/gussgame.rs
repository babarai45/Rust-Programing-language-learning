use std::io;

fn fullname() {
    let mut name = String::new();
    println!("GEST");
    io::stdin()
    .read_line(&mut name)
    .expect(" failed");
println!("{}", name.trim());
}
fn fullage(){
    let age = 18;
    println!("Age: {}", age);
}
fn main() {
    fullname();
    fullage();
}