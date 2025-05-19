// Strings are stored as a collection of UTF-8 bytes

// String is UTF-8 encoded
// String is mutable
// String is heap allocated
// String is resizable
// String is dynamic size

fn main(){

    let s1: String  = String::new();
    let s2: &str = "Hello";  // string slice
    let s3: String = "Hello".to_string();
    let mut s4: String  = String::from("Hello");

    println!(" s1 : {:?} \n s2 : {:?} \n s3 : {:?} \n s4 : {:?}", s1, s2, s3, s4);

    // Format macros 
    let formatStr = format!("{} {} {}", s2, s3, s4);
    println!("formatStr : {:?}", formatStr);

    // String concatenation
    s4.push_str(" World");
    println!("s4 : {:?}", s4);
    s4.push('!');
    println!("s4 : {:?}", s4);

    // String length
    let len = s4.len();
    println!("s4 length : {:?}", len);

    // String capacity
    let capacity = s4.capacity();
    println!("s4 capacity : {:?}", capacity);



    // characters in a string
    for c in s4.chars() {
        println!("char : {:?}", c);
    }
    println!("4th char of s4 : {:?}", s4.chars().nth(4));


    let s5 = String::from("नमस्ते");
    println!("s5 : {:?}", s5);
    println!("s5 length : {:?}", s5.len());
    println!("s5 capacity : {:?}", s5.capacity());


    

}