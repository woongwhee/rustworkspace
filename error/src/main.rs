

use std::fs::File;

use std::io:: {self, Read};
use std::path::PathBuf;
fn main() {
    println!("Hello, world!");
    // let v=vec![1,2,3];
    // // v[100];//panic을 반환하는 코드
    // //vec의 []는 무조건 반환된다는 가정하에 작동되는 코드기떄문에 패닉이 발생한다.
    // let greeting_file =File::open("hello.txt")
    //     .unwrap_or_else(|error| {
    //         if error.kind()==ErrorKind::NotFound{
    //             File::create("hello.txt").unwrap_or_else(|error| panic!("Problem creating the file"))
    //         }else{
    //             panic!("Problem opening the file: {:?}", error);
    //         }
    // });
    // let file_name=String::from("hello2.txt");
    // // let greeting_file2 =File::open(file_name)  
    //     // .expect("hello.txt should be included in this project");
    let path=String::from("path");
    let file_name=String::from("username.txt");
    let mut file_path=PathBuf::from(path);
    file_path.push(&file_name);
    let username=read_username_from_file(&file_path);
    let username=username.unwrap();
    println!("{}",username);
}
fn read_username_from_file(file_name:&PathBuf)->Result<String,io::Error>{
    let username_file_result=File::open(file_name);
    let mut username_file=match username_file_result{
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username=String::new();
    // match username_file.read_to_string(&mut username){
    //     Ok(_)=>Ok(username),
    //     Err(e)=>Err(e),
    // }
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
fn getFruit(name:&str)->Fruit{
//    Apple(name)일반적인 enum인 Fruit의 경우 타입을 명시하지 않으면 에러가 일어난다.
    Fruit::Apple(name.to_string())
}
enum Fruit{
    Apple(String),
    Pear(i32),
}