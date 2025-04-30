fn main() {
    let mut counter = 0;
    let resut = loop{
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {resut}");

    //let mut s: String = String::from("Hello World");
    //let hello: &str = &s[0..5];
    //let world: &str = &s[6..];
    //

    let s2:&str = "Hello World";
    

    let word : &str = first_word(s2);
    println!("The first word is : {}",word);
    //s.clear();
}

fn first_word(s: &str) -> &str {
    let bytes:&[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
