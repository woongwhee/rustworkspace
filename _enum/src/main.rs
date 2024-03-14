// enum IpAddrKind{
//     V4,
//     V6
// }
// struct IpAddr{
//     kind: IpAddrKind,
//     address: String,
// }
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
//이런식으로 활용도 가능하다.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChageColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {}
}
#[derive(Debug)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]

enum UsState {
    Alaska,
    Alabama,
    California,
    Texas,
    Ohio,
    Georgia,
    Pennsylvania
}

impl Coin {
    fn value_in_cent(&self) -> u32 {
        match self {
            Coin::Penny => {
                println!("im Penny");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // println!("state quarter from: {:?}!",state);
                25
            },
        }
    }
}
fn plus_one(x:Option<i32>)->Option<i32>{
    match x {
        None=>None,
        Some(i)=>Some(i+1)
    }
}
fn main() {
    // let home: IpAddr =IpAddr{
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    // let loopback: IpAddr =IpAddr{
    //     kind:IpAddrKind::V4,
    //     address:String::from("::1"),
    // };
    // println!(loopback.address==home.address);
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V4(127, 0, 0, 1);
    let m = Message::Move { x: (32), y: (43) };
    m.call();
    let o1 = Some(m);
    let m2: Option<Message> = None;
    println!("{}", o1.is_some());
    let x = 5;
    let y: Option<i32> = Some(6);
    let z = y.is_some_and(|x| x > 3);
    println!("{}", z);
    let penny=Coin::Penny;
    let texas_quarter=Coin::Quarter(UsState::Texas);
    let i=penny.value_in_cent();
    let j=texas_quarter.value_in_cent();
    let alabama_quarter: Coin=Coin::Quarter(UsState::Alabama);
    alabama_quarter.value_in_cent();
    match alabama_quarter{
        Coin::Quarter(state)=>println!("2"),
        _=>println!("1"),//defualt
    }
    if let Coin::Penny =  penny{
        println!("is penny!");
    }else{
        println!("not penny!");
    }
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn route(ipkind: IpAddr) {}
