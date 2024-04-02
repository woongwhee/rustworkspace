mod lib;

use std::fmt::format;

fn lagest(list: &[i32]) -> &i32{
    let mut largst: &i32 =&list[0];
    for number in list{
        if number>largst{
            largst=number;
        }
    } 
    largst
}
struct Point<T,U>{
    x :T,
    y :U,
}
impl <U,T> Point<U,T> {//제네릭의 시그니처도 변경할수잇따.

    fn x(&self) ->&U{
        &self.x
    }
}

//타입지정도가능
impl Point<f32,f32>{
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2)+self.y.powi(2)).sqrt()
    }
}
impl <X1,Y1> Point<X1,Y1>{
    fn mixup<X2,Y2> (self,other:Point<X2,Y2>)->Point<X1,Y2>{
        Point {
             x: self.x,
             y:  other.y,
        }
    }
}
enum OptionI32{
    Some(i32),
    None,
}
enum OptionF64{
    Some(f64),
    None,
}
use generics_and_trait::{Tweet,NewsArticle};
use generics_and_trait::Summarze::Summary;
use std::fmt::Display;
pub fn notify(item:&impl Summary){
    println!("Breaking news {}",item.summarze());
}
//트레이트 바운스
pub fn notify2<T:Summary>(item:&T){
    println!("Breaking news {}",item.summarze());
}
//같은타입의 트레잇을 구현한 매게변수를 강제하고 싶다면 유용하다.
//아래경우 다른 타입의 구현체가 와도 강제되지않기때문
//pub fn notify3(item1: &impl Summary, item2: &impl Summary) {

pub fn notify3<T:Summary>(item1:&T,item2:&T){
    println!("Breaking news {}",item1.summarze());
    println!("Breaking news {}",item2.summarze());
}
//이런식으로 여러개의 트레잇을 구현한 구현체를 특정해서 받을수 있다.
pub fn notify4<T:Summary+Display>(item1:&T,item2:&T){
    println!("Breaking news {}",item1.summarze());
    println!("Breaking news {}",item2.summarze());
}
pub fn some_function<T,U>(t:&T,u:&U)->i32
where
    T:Display+Clone,
    U:Clone
{
    1
}
fn return_summarizable()->impl Summary{
    Tweet{
        username:String::from("horse_ebooks"),
        content: String::from("of course,as you probably already know, people"),
        reply: false,
        retweet:false,
    }
}
fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()>y.len(){
        x
    }else{
        y
    }
}
fn main() {
    let number_list=vec![32,50,25,100,65];
    let result=lagest(&number_list);
    println!("lagest!{result}");
    let number_list= vec![102,34,6000,89,54,2,43,8];
    let result=lagest(&number_list);
    println!("the lagest number : {result}");
    let integer: Point<u32,u32> =Point{x:5,y:10};
    let float=Point{x:1.0,y:4.0};
    let integer_and_float =Point{x:5,y:4.0};
    let p3=integer.mixup(float);
    let iOption=OptionI32::Some(5);
    let fOption=OptionF64::Some(5.0);

    let tweet=Tweet{
        username:String::from("horse_ebooks"),
        content: String::from("of course,as you probably already know, people"),
        reply: false,
        retweet:false,
    };
    println!("1 new tweet:{}",tweet.summarze());
    let article=NewsArticle::new("hi","hi","hi","hi");
    println!("1 new article: {}",article.summarze());
    println!("1 new article: {}",article.summarze_author());
    let tweet=Tweet::from(&article);
    println!("1 new twee: {}",tweet.summarze());
    println!("1 new twee: {}",tweet.summarze_author());
    notify(&tweet);
    notify(&article);
    notify2(&article);
    // let r;
    // {
    //     let x=5;
    //     // r=&x;
    // }
    // println!("r:{}",r);
    //이경우 해제된 변수를 참조 하게 된다.
    let long=longest("hi","hello");
    let string1 = String::from("long string is long");
    let result: &str ;
    
        let string2 = String::from("xyz");
        result=longest(string1.as_str(),string2.as_str());
    
    println!("{long}");
    println!("{result}");
    let v=vec![2,3,5];


}
