// enum IpAddressKind {
//     V4,
//     V6,
// }

enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x:u32, y:u32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_function (){
        println!("Implementation block for enum Message");
    }
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

fn main() {
    // let ip_four: IpAddressKind = IpAddressKind::V4;
    // let ip_six: IpAddressKind = IpAddressKind::V6;


    let localhost : IpAddress = IpAddress {
        kind: IpAddressKind::V6(String::from("127.0.0.1")),
        address: String::from("127.0.0.1")
    };

   
    let localhost2 : IpAddressKind = IpAddressKind::V6(String::from("127.0.0.1"));
   
    let localhost3: IpAddressKind = IpAddressKind::V4(127,0,0,1);
    
}
