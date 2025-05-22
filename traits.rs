//  Traits are a way to define shared behavior in Rust.
// Traits can be used to define shared behavior for different types.
// This code defines a trait called `Summary` and implements it for two structs: `NewsArticle` and `Tweet`.
// The `Summary` trait requires a method `summarize` that returns a `String`.

pub trait Summary {
    // fn summarize (&self) -> String;
    // default implementation of the summarize method
    fn summarize(&self) -> String {
        String::from("Read more...")
    }

    fn summarize_author(&self) -> String {
        String::from("Author: Unknown")
    }
}

pub trait Display {
    fn display(&self) -> String;
}

pub trait Debug {
    fn debug(&self) -> String;
}

pub trait Clone {
    fn clone(&self) -> Self;
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

// using default implementation
// impl Summary for NewsArticle { }

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} tweeted: {}", self.username, self.content)
    }
}

// Trait bounds
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news!! {}", item.summarize());
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!(
        "Breaking news!!! ::  {} // {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify4<T: Summary, U: Summary>(item1: &T, item2: &U) {
    println!(
        "Breaking news!!!! ::  {} // {}",
        item1.summarize(),
        item2.summarize()
    );
}
// we can also specify multiple traits
pub fn notify5<T: Summary + Clone>(item: &T) {
    println!("Breaking news!!!!! ::  {}", item.summarize());
}

// building same functions with diff generic in two different way (using trait bounds)
// 1. using impl trait
pub fn some_function<T: Summary + Clone, U: Summary + Debug>(item1: &T, item2: &U) {
    println!(
        "Breaking news!!!!!! ::  {} // {}",
        item1.summarize(),
        item2.summarize()
    );
}
// 2. using where clause
pub fn some_function2<T, U>(item1: &T, item2: &U)
where
    T: Summary + Clone,
    U: Summary + Debug,
{
    println!(
        "Breaking news!!!!!!! ::  {} // {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Mr. K"),
        content: String::from("Rust is awesome"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let article = NewsArticle {
        author: String::from("Mr. K"),
        headline: String::from("Rust is awesome"),
        content: String::from("Rust language is pretty awesome and interesting."),
    };

    let tweet = Tweet {
        username: String::from("Mr. K"),
        content: String::from("Rust is awesome"),
        reply: false,
        retweet: false,
    };

    println!("News Article summary :: {}", article.summarize());
    println!("Tweet summary :: {}", tweet.summarize());

    // using default implementation
    println!("News Article author :: {}", article.summarize_author());
    println!("Tweet author :: {}", tweet.summarize_author());

    notify(&article);

    notify2(&tweet);

    notify3(&article, &tweet);

    notify4(&article, &tweet);

    println!(
        "Return summarizable :: {}",
        returns_summarizable().summarize()
    );
}
