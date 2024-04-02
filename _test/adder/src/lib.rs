pub fn add(left: usize, right: usize) -> usize {
    println!("hihih");

    left + right
}
pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub mod figure {
    pub struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        pub fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
    }
}
pub fn greeting(name: &str) -> String {
    format!("hello {}", name)
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}
