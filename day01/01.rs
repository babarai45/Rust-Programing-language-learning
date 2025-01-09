// we are excited to launch Rust Course

// fn main(){
//     println!("____________Well Come to Rust learning______________ ");

//     Code Explanation
//     fn is a keyword that is used to declare a function in Rust.  fn stands for function. 
//     main is the name of the function. It is the entry point of the program. and it is mandatory in Rust.
// but it can be written in lowercase or uppercase letters.and it is a function that takes no arguments and returns nothing. if we pass arguments to the function, we have to mention the data type of the arguments.like fn main(a: i32, b: i32) -> i32
//     println! is a macro that is used to print the text to the console. It is used to print the text to the console. macro means it is a function that is used to generate code at compile time rather than runtime. println! is used to print the text to the console and it adds a new line at the end of the text.! is used to call the macro. if we don't want to add a new line at the end of the text, we can use print! macro. like below
//     print!("Hello, World");

//     Code Execution
//     To execute the code, we have to run the code using the rustc command. rustc is a compiler that is used to compile the Rust code. rustc command takes the name of the file as an argument and compiles the code. rustc command generates an executable file with the same name as the source file. To run the executable file, we have to use the ./ command followed by the name of the executable file. like below
//     rustc 01.rs 
//     ./01
//     Output
//     Hello, World
//     The output of the code is Hello, World. The code prints the text Hello, World to the console. The text is printed to the console using the println! macro. The println! macro adds a new line at the end of the text. If we don't want to add a new line at the end of the text, we can use the print! macro. print! macro is used to print the text to the console without adding a new line at the end of the text.



// creating a new function
    // fn hello(){
    //     let _fname = "Muhammad";
    //     let _lname = "Babar";
    //     println!("Hello, {} {}", _lname, _lname);
    // }
    // hello();
   
//     Code Explanation
//     fn is a keyword that is used to declare a function in Rust. fn stands for function.
//     hello is the name of the function. It is the name of the function that we have created. We can give any name to the function. The name of the function should be meaningful so that we can understand the purpose of the function by looking at the name of the function.
//     {} is a placeholder that is used to print the value of the variable. {} is used to print the value of the variable
//     _fname is a variable that is used to store the first name. _fname is a variable that is used to store the first name of the person. We have used the let keyword to declare the variable. let keyword is used to declare the variable in Rust. The variable name is _fname. The variable name should be meaningful so that we can understand the purpose of the variable by looking at the name of the variable. The variable name should start with an underscore (_) or an alphabet. The variable name can contain alphabets, numbers, and underscores. The variable name should not contain special characters like @, #, $, %, etc. The variable name should not contain spaces. The variable name should not start with a number. The variable name should not be a keyword. The variable name should not be a reserved word. The variable name should not be a data type. The variable name should not be a function name. The variable name should not be a macro name. The variable name should not be a module name. The variable name should not be a crate name. The variable name should not be a struct name. The variable name should not be an enum name. The variable name should not be a trait name. The variable name should not be a type name. The variable name should not be a lifetime name. The variable name should not be a constant name. The variable name should not be a static name. The variable name should not be a function parameter name. The variable name should not be a function return type name. The variable name should not be a module name. The variable name should not be a crate name. The variable name should not be a struct name. The variable name should not be an enum name. The variable name should not be a trait name. The variable name should not be a type name. The variable name should not be a lifetime name. The variable name should not be a constant name. The variable name should not be a static name. The variable name should not be a function parameter name. The variable name should not be a function return type name. The variable name should not be a module name. The variable name should not be a crate name. The variable name should not be a struct name. The variable name should not be an enum name. The variable name should not be a trait name. The variable name should not be a type name. The variable name should not be a lifetime name. The variable name
//    _lname is a variable that is used to store the last name. _lname is a variable that is used to store the last name of the person. We have used the let keyword to declare the variable. let keyword is used to declare the variable in Rust. The variable name is _lname. The variable name should be meaningful so that we can understand the purpose of the variable by looking at the name of the variable. The variable name should start with an underscore (_) or an alphabet. The variable name can contain alphabets, numbers, and underscores. The variable name should not contain special characters like @, #, $, %, etc. The variable name should not contain spaces. The variable name should not start with a number. The variable name should not be a keyword. The variable name should not be a reserved word. The variable name should not be a data type. The variable name should not be a function name. The variable name should not be a macro name. The variable name should not be a module name. The variable name should not be a crate name. The variable name should not be a struct name. The variable name should not be an enum name. The variable name should not be a trait name. The variable name should not be a type name. The variable name should not be a lifetime name. The variable name should not be a constant name. The variable name should not be a static name. The variable name should not be a function parameter name. The variable name should not be a function return type name. The variable name should not be a module name. The variable name should not be a crate name. The variable name should not be a struct name. The variable name should not be an enum name. The variable name should not be a trait name. The variable name should not be a type name. The variable name should not be a lifetime name. The variable name should not be a constant name. The variable name should not be a static name. The variable name should not be a function parameter name. The variable name should not be a function return type name. The variable name should not be a module name. The variable name should not be a crate name. The variable name should not be a struct name. The variable name should not be an enum name. The variable name should not be a trait name. The variable name should not be a type name. The variable name should not be a lifetime name. The variable name
//     println! is a macro that is used to print the text to the console. It is used to print the text to the console. macro means it is a function that is used to generate code at compile time rather than runtime. println! is used to print the text to the console and it adds a new line at the end of the text.! is used to call the macro. if we don't want to add a new line at the end of the text, we can use print! macro. like below
//     print!("Hello, {} {}", _lname, _lname);  {} is a placeholder that is used to print the value of the variable. {} is used to print the value of the variable
//     hello is a function that is used to print the text Hello, World to the console. The function is called using the hello() function. The function is called using the function name followed by the parentheses. The function name is hello. The function name should be meaningful so that we can understand the purpose of the function by looking at the name of the function. The function name should start with an alphabet. The function name can contain alphabets, numbers, and underscores. The function name should not contain special characters like @, #, $, %, etc. The function name should not contain spaces. The function name should not start with a number. The function name should not be a keyword. The function name should not be a reserved word. The function name should not be a data type. The function name should not be a variable name. The function name should not be a macro name. The function name should not be a module name. The function name should not be a crate name. The function name should not be a struct name. The function name should not be an enum name. The function name should not be a trait name. The function name should not be a type name. The function name should not be a lifetime name. The function name should not be a constant name. The function name should not be a static name. The function name should not be a function parameter name. The function name should not be a function return type name. The function name should not be a module name. The function name should not be a crate name. The function name should not be a struct name. The function name should not be an enum name. The function name should not be a trait name. The function name should not be a type name. The function name should not be a lifetime name. The function name should not be a constant name. The function name should not be a static name. The function name should not be a function parameter name. The function name should not be a function return type name. The function name should not be a module name. The function name should not be a crate name. The function name should not be a struct name. The function name should not be an enum name. The function name should not be a trait name. The function name should not be a type name. The function name should not be a lifetime name. The function name should not be a constant name. The function name should not be a static name.



// }

//  Best Practice for Above Code

fn hello() -> String {
    let _fname = "Muhammad";
    let _lname = "Babar";
    format!("Hello, {} {}", _fname, _lname)  // format! is a macro that is used to format the text. like f string in python 
}
fn babar () -> i32 {
    
let a = 10;
let b = 5;
let c = a + b;
println!("The value of c is: {}", c);
c   // here we are returning the value of c  to the function babar  
}
fn main() {
    let result1 = hello();
    let result2 = babar();
    println!("{}\n{}", result1, result2);
}

// Code Explanation
// fn is a keyword that is used to declare a function in Rust. fn stands for function.
// hello is the name of the function. It is the name of the function that we have created. We can give any name to the function. The name of the function should be meaningful so that we can understand the purpose of the function by looking at the name of the function.
// -> String is a return type that is used to specify the return type of the function. -> is a separator that is used to separate the function signature from the return type
// let _fname is a variable that is used to store the first name. _fname is a variable that is used to store the first name of the person. We have used the let keyword to declare the variable. let keyword is used to declare the variable in Rust. The variable name is _fname. The variable name should be meaningful so that we can understand the purpose of the variable by looking at the name of the variable. 
// The variable name should start with an underscore (_) or an alphabet. The variable name can contain alphabets, numbers, and underscores. The variable name should not contain special characters like @, #, $, %, etc. The variable name should not contain spaces. The variable name should not start with a number. The variable name should not be a keyword. The variable name should not be a reserved word. 
// The variable name should not be a data type. The variable name should not be a function name. The variable name should not be a macro name. The variable name should not be a module name. The variable name should not be a crate name. The variable name should not be a struct name. The variable name should not be an enum name. The variable name should not be a trait name. The variable name should not be a type name.
//  The variable name should not be a lifetime name. The variable name should not be a constant name. The variable name should not be a static name. The variable name should not be a function parameter name. The variable name should not be a function return type name. The variable name should not be a module name. The variable name should not be a crate name. The variable name should not be a struct name. 
// The variable name should not be an enum name. The variable name should not be a trait name. The variable name should not be a type name. The variable name should not be a lifetime name. The variable name should not be a constant name. The variable name should not be a static name. The variable name should not be a function parameter name. The variable name should not be a function return type name.
//  The variable name should not be a module name. The variable name should not be a crate name. The variable name should not be a struct name. The variable name should not be an enum name. The variable name should not be a trait name. The variable name should not be a type name. The variable name should not be a lifetime name.

// for next line print we can use \n in the string like below 
// println!("{} \n{}", result1, result2);
