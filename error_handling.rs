use std::fs::File;
use std::io::ErrorKind;


fn main(){
    // panic!("This is a panic message");

    // -------------------------------
    // a();
    // -------------------------------

    // 
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // ----------------------------------

    // let f = File::open("Hello.txt");

    // let f = match f {
    //     Ok(file) => file,
        // Err(error) => {
        //     panic!("There was a problem opening the file Hello.txt: {:?}", error);
        // }
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound =>  {
    //             match File::create("Hello.txt"){
    //                 Ok(fc) => fc,
    //                 Err(e) => {
    //                     panic!("There was a problem creating the file Hello.txt: {:?}", e);
    //                 }
    //             }
    //         }
    //         other_error => {
    //             panic!("There was a problem opening the file Hello.txt: {:?}", other_error);
    //         }
    //     }
    // };

    // .unwrap() also does the same thing, like if the file not found then panic
    // let f = File::open("Hello.txt").unwrap();
    // .expect() also does the same thing, like if the file not found then panic
    // let f = File::open("Hello.txt").expect("There was a problem opening the file Hello.txt");


    // `?` operator is used to propagate errors
    // let mut f = File::open("Hello.txt")?;
    // let s = String::new();
    // f.read_to_string(&mut s)?;

    let f = read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("Hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


// ------------------------------------------------
// run `RUST_BACKTRACE=1 cargo run` to see the backtrace
// run `RUST_BACKTRACE=0 cargo run` to see the panic message
// run `RUST_BACKTRACE=full cargo run` to see the full backtrace

// fn a(){
//     b();
// }

// fn b(){
//     c();
// }
// fn c(){
//     panic!("This is a panic message");
// }
// ------------------------------------------------
