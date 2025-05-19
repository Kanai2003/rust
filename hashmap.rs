use std::collections::HashMap;

fn main () {
    // HashMap is a collection of key-value pairs
    // HashMap is dynamic size
    // HashMap is heap allocated
    // HashMap is resizable
    // HashMap is unordered
    // HashMap is not indexed
    // HashMap is not sorted
    // HashMap is not unique
    // HashMap is not thread safe

    // -----------------------
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    println!("scores : {:?}", scores);

    // -------------------------
    // creating a HashMap from two vectors
    let teams = vec!["Blue", "Yellow"];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores : {:?}", scores);

    // -------------------------
    // Accessing values in a HashMap
    let team_name = "Blue";
    let score = scores.get(&team_name);
    match score {
        Some(&score) => println!("{}: {}", team_name, score),
        None => println!("{} not found", team_name),
    }

    // ----------------------------
    // overwriting values in a HashMap
    scores.insert(&"Blue", &&25);
    println!("scores : {:?}", scores);


    // ----------------------------
    // if we dont want to overwrite the value, we can use entry() method
    scores.entry(&"Blue").or_insert(&&50);
    println!("scores : {:?}", scores);
    scores.entry(&"Red").or_insert(&&50);
    println!("scores : {:?}", scores);


    // ---------------------------
    // creating a hashmap from a string
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("word count : {:?}", map);
}