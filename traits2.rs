// conditionally implement methods based on trait bounds

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is {}", self.x);
        } else if self.x < self.y {
            println!("The largest number is {}", self.y);
        } else {
            println!("The numbers are equal");
        }
    }
}

// =====================
// Blanket implementation -> implementing a trait for all types that implement another trait
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

fn main() {}
