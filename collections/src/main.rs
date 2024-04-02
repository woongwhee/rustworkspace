fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7); 
    let third: &i32 = &v[2];
    let a = third + 3;
    println!("{}", a);
    let b = v[2];
    let third: &i32 = &v[2];
    //? 이건 왜 ownership을 안가저가?
    //값의 복사가 자동으로 이뤄지기 때문
    let third: &i32 = &v[2];        
    println!("{}", third);
    let mut v3: Vec<ab> = Vec::new();
    v3.push(ab { value: 3 });
    v3.push(ab { value: 4 });
    v3.push(ab { value: 5 });
    let thrid = v3.get(3);
    //get을 하면 Option
    if let Some(abs) = thrid {
        print!("{}", abs.value)
    };
    // let c=v3[2];
    //이건 복사다.
    let third = &v3[2];
    println!("{}", third.value);
    // let does_not_exit=&v[100];
    //위의 경우 패닉을 이르키지만 아래경우 패닉을 이르키지않는다.
    let does_not_exit = v.get(100);
    let mut v = vec![1, 2, 3, 4, 5];
    //이경우 에러를 이르킨다 불변참조기떄문
    let first = &v[0];
    v.push(6);
    // println!("{first}",first);
    {
        let first = &v[0];
        println!("first:{}", first);
        //이경우 first가 사용한 참조의 범위가 명확하기떄문에
        //반환된다.
        //혹은 명시적으로 frist를 재사용하는 방법이있다.
    }
    v.push(6);
    let second = v.get(2);
    if let Some(int) = second {
        println!("second{}", int);
    }
    v.push(5);
    let second = v3.get(2);
    v3.push(ab { value: 33 });
    // if let Some(ab1)=second{println!("second{}",ab1.value);}
    let mut index = 0;
    print!("[");
    for i in &mut v {
        print!("{i},");
        *i += 10;
        index += 1;
    }
    print!("]\n 변경후\n [");
    for i in &v {
        print!("{i},");
    }
    println!("]");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.0),
        SpreadsheetCell::Text(String::from("320")),
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.0),
        SpreadsheetCell::Text(String::from("320")),
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.0),
        SpreadsheetCell::Text(String::from("320")),
    ];
    //enum도 반복자로 넣을수있어서 장점이다.
    for i in &row {
        match i {
            SpreadsheetCell::Int(int) => {
                println!("{int}");
            }
            SpreadsheetCell::Float(float) => {
                println!("{float}")
            }
            _ => {
                println!("Hi")
            }
        }
    }
    let data = "hi";
    let s = data.to_string();
    let s1: String = "hello".to_string(); //리터럴에도 그대로 적용됨 String::from() 과 같은 의미
    let s2 = String::from("wolrd");
    let s3 = s1 + &s2;

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    //let s = &hello[0..1];//error
    //
    for c in "Зд".chars() {
        println!("{c}");
    }
    for c in "Зд".bytes() {
        println!("{c}"); //바이트단위기떄문
        println!("hihi");
        // s1.push_str("hello");안됨
    }
    //hash map
    hash_map();



}
use core::fmt;
use std::collections::HashMap;
fn hash_map()->(){

    let mut map: HashMap<String,RGB>=HashMap::new();
    map.insert(String::from("Blue"), RGB(0,0,255));
    map.insert(String::from("Red"), RGB(255,0,0));
    map.insert(String::from("Green"), RGB(0,0,255));
    map.insert(String::from("Yellow"), RGB(255,255,30));
    let blue=String::from("Blue");
    let color=map.get(&blue).copied().unwrap_or(RGB(0,0,0));
    println!("{color}");
    let red=String::from("Red");
    // let mut color=map.get(&blue).unwrap_or(&RGB(0,0,0));
    // let &color.0=200 이렇게 수정할순 없음
    let color=map.get(&red).copied().unwrap_or(RGB(0,0,0));
    println!("before change {color}");
    if let Some(color)=map.get_mut(&red){
        color.0=200;
    }
    let color=map.get(&red).copied().unwrap_or(RGB(0,0,0));
    
    println!("after change {color}");
    println!("iter map");
    // for (key,value) in map{
    //     println!("{key}:{value}");
    // }
    for (key,value) in &map{
        println!("{key}:{value}");
    }
    //콜랙션의 iterator
    let mut v1=vec![3,265,34,134,66,1243,6,3];
    let v2=vec![3,265,34,134,66,1243,6,3];
    v1.extend(v2);
    for i in v1.iter_mut(){
        print!("{i} ");
        *i += 1;
    }

}
struct  RGB(u8,u8,u8);
impl Clone for RGB {
    fn clone(&self) -> Self {
        RGB(self.0, self.1, self.2)
    }
}
impl Copy for RGB {}
use std::fmt::{Display,Formatter};
impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0, self.1, self.2는 각각 RGB 구조체의 필드에 접근합니다.
        write!(f, "RGB({}, {}, {})", self.0, self.1, self.2)
    }

}
enum SpreadsheetCell {
    Int(i32),
    Float(f32),
    Text(String),
}
struct ab {
    value: i32,
}
