enum Option<T> {
    Some(T),
    None,
}

// -------------------------------------------------
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    // Quarter,
    Quarter(UsState),
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        // Coin::Penny => {
        //     println!("This is a penny");
        // }
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Coin::Quarter => 25,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}
// --------------------------------------
// pattern matching 
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i:i32) => Some(i+1),
        None => None,
        _ => (),
    }
}



fn main() {

    let some_integer = Option::Some(5);
    let some_float = Option::Some(5.0);
    let some_string = Option::Some("Hello");
    let some_none: Option<i32> = Option::None;


    let x: i8 = 5;
    let y: std::option::Option<i8> = std::option::Option::Some(5);

    // let sum:i8 = x+y; // This will not work because y is an Option
    // if we do not use unwrap_or, it will throw an error -> because y is an Option
    let sum:i8 = x + y.unwrap_or(0);



    value_in_cents(Coin::Quarter(UsState::Alabama));



    let five: Option<i32> = Option::Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

}
