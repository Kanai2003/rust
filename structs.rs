struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    created_at: String,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
        created_at: chrono::Local::now().format("%Y-%m-%d").to_string(),
    }
}
// -----------------------------------------------------------------

// fn rect_area (dimensions: (u32, u32) ) -> u32 {
//     dimensions.0 * dimensions.1
// }

#[derive(Debug)]  // This will allow us to print the struct
struct Rectangle {
    width: u32,
    height: u32,
}

// fn rect_are(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

// impl is implementation -> helps to implement methods for the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// associated functions
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}




fn main() {
    // let user1 = build_user("username1".to_string(), "user1email@gmail.com".to_string());
    // println!("User {}", user1.username);


    let rect = Rectangle {
        width: 15,
        height: 10,
    };

    println!("For the rectangle {:#?}, Area  is {}, Perimeter is {}",rect,  rect.area(), rect.perimeter()); // {:?} is used to print the struct, // {:#?} is used to print the struct in a pretty way

    println!("Associated fn is called {:#?}", Rectangle::square(50))
    

}
