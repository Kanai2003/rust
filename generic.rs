fn main () {

    let number_list = vec![34, 50, 25, 100, 65];
    let result = get_largest(number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(char_list);
    println!("The largest char is {}", result);


    // -------------------
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);

    println!("p2.x() = {}, p2.y() = {}", p2.x(), p2.y());

    let p3 = Point2 {x: 5, y: 10.0};
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    println!("p3.x() = {}, p3.y() = {}", p3.x(), p3.y());

    let p4 = Point2 {x: 51, y: 101.0};
    println!("p4.x = {}, p4.y = {} :: mixup : x: {} , y: {}", p4.x.clone(), p4.y.clone(), p4.clone().mixup(p3.clone()).x(), p4.clone().mixup(p3.clone()).y());

    let mixup = p4.mixup(p3);
    println!("p4 mixup with p3 :: x: {} , y: {}", mixup.x(), mixup.y());

}

// generic type function
// fn get_largest<T> (list: Vec<T>) -> T {}
//  :PartialOrd + Copy  is a trait bound -> to ensure that T implements the PartialOrd and Copy traits means comparable and can be copied
fn get_largest<T:PartialOrd + Copy> (list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// ======================
// generic struct
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug, Clone)]
struct Point2<T, U> {
    x: T,
    y: U,   
}

// implementing methods for generic struct
impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

// implementing methods for generic struct with different types
impl <T, U> Point2<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U{
        &self.y
    }

    fn mixup<V, W> (self, other: Point2<V,W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}