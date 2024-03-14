pub fn add(left: usize, right: usize) -> usize {
    left + right
}
mod cilent;
mod network;
mod foo;
mod front_of_house;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn eat_at_restaurant(){
    //절대경로
    crate::front_of_house::hosting::add_to_waitlist();
    //상대 경로
     front_of_house::hosting::add_to_waitlist();



}
fn deliver_order(){}
mod back_of_house{
    fn fix_incrrect_order(){
        cook_order();
        super::deliver_order();
    }
    fn cook_order(){}
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String
    }
    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}
//스코프네에 use키워드를 이용해 해당 스코프의 모듈을 가저올수 있다.
//관례상 함수의 부모모듈까지만 가저온다.
pub use crate::front_of_house::hosting;
// pub use crate::front_of_house::serving;
//pub을 붙이면 이제 외부에서 serving을접근할때 경로를 단축해서 사용할수 있게된다.
pub fn eat_at_restaurant2(){
    let mut meal =back_of_house::Breakfast::summer("Rye");
    //필드변경
    meal.toast=String::from("Wheat");
    println!("I'd like {} toast please",meal.toast);
    //변경불가능한 필드
    // meal.seasonal_fruit=String::from("peach");
    hosting::add_to_waitlist();
}

use std::fmt::Result;
//같은 스코프내에 동일한 이름의 타입을 가저오면 에러를 이르킨다 이떄 as키워드를 이용해서 방지할수 있다.
use std::io::{Result as IoResult,Error};
//같은 모듈안에 있는 다른 아이템을 가저올떄 중첩경로를 사용해 생략할수 있다.
// use std::{cmp::Ordering,io::Write};
//use std::io::{self,Write};
//io자체도 같이 참조하고 싶다면 self키워드를 사용하면 된다.
//use std::collections::*';
// fn fn1()->Result{}
// fn fn2()->IoResult<()>{}
