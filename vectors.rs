fn main () {

    // normal array
    // array is fixed size
    // array is stack allocated
    // array is not resizable/dynamic
    let a :[i32; 3] = [1, 2, 3];
    
    println!(">>>>> a : {:?}", a);


    // vector
    // vector is dynamic size
    // vector is heap allocated
    // vector is resizable
    
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!(">>>>Vector : v : {:?}", v);

    // --------------------------
    let v3 = vec![1, 2, 3, 4, 5];

    // Accessing values in a vector, using index
    let third_item = &v3[2];
    println!("The third item is {}", third_item);

    //  Accessing values in a vector, using get() method, return an option
    // If the index is out of bounds, it returns None
    match v3.get( 4 ){
        Some(value) => println!("Value is {}", value),
        None => println!("No value"),
    }

    for i in &v3 {
        print!("{} ", i);
    }

    // ----------------------------
    let mut v4 = vec![1, 2, 3, 4, 5];

    for i in &mut v4 {
        *i *= 10;
    }
    println!("\n>>>> v4 : {:?}", v4);



    // ----------------------
    // storing enum in a vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("Hello")),
    ];

    match row[0] {
        SpreadsheetCell::Int(i) => println!("Int: {}", i),
        SpreadsheetCell::Float(f) => println!("Float: {}", f),
        SpreadsheetCell::Text(ref s) => println!("Text: {}", s),
    }
}