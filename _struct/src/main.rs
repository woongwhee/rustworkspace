use std::cmp::Ordering;
struct User{
    username:String,
    email:String,
    sign_in_count:u64,
    active:bool
}
// struct Man{ 구조체는 소유권을 가지고 있어야된다. 라이프타임이 명시되지 않기때문에 문제를 야기시킬수 있기 떄문
//     name:&str
// }
fn build_user(email:String,username:String)->User{
    User {
        email,
        username,//매게변수와 구조체의 필드의 이름이 같다면 축약 가능하다.
        sign_in_count:0,
        active:true
    }
}
struct Rectangle{
    length:u32,
    width:u32
}
impl Rectangle{
    fn rectangle(length:u32,width:u32)->Rectangle{
        Rectangle{length,width}
    }
    fn area(&self)->u32{
        self.width*self.length
    }
    fn compare(&self,other:&Rectangle)->i32{
        let self_size:u32=self.area();
        let other_size:u32=other.area();
         match self_size.cmp(&other_size){
             Ordering::Greater=>1,
             Ordering::Equal=>0,
             Ordering::Less=>-1
         }
    }
}
struct Board{
    title:String,
    content:String,
    count:i32
}
impl Board{
    fn build_board(title:&str,content:&str)->Board{
        Board {
            title:String::from(title),
            content:String::from(title),
            count:0
        }
    }
    fn increase_count(&mut self){
        self.count+=1;
    }
}
//튜플구조체 필드명을 명시할 필요없는 구조체 이다.
struct RGB(i32, i32,i32);
struct Pair(User,User);

fn main() {
    let user=User{
        username: String::from( "jin") ,
        email: String::from("hihi@gmail.com"),
        sign_in_count: 0,
        active: false,
    };
    println!("{}",user.username);
    //구조체 변경
    let user2=build_user(String::from("gig@gmail.com"),String::from("kim"));
    println!("{}",user2.email);
    // user2.active=true; error
    //맴버 하나 단위로 mut 키워드가 붙는건 불가능하다
    //때문에 구조체 전체를 mut로 설정해야 변경이가능하다.
    let mut user5=build_user(String::from("gig@gmail.com"),String::from("kim"));
    println!("{}",user5.email);
    user5.active=false;
    //구조체 복사
    //새로 할당한 값을 제외한 나머지 필드를 입력된 객체의 값을 복사해 할당한다.
    //값을 복사하는 것이기 때문에 ownership도 함께 넘어간다.
    //때문에 구조체의 일정 값을 갱신하는 용도로 사용 될 수 있다.
    let user3=User{
        username: String::from( "jin") ,
        email: String::from("hihi@gmail.com"),
        ..user2
    };
    let user4=User{
        username:String::from("###"),
        ..user3
    };
    // let user3=User{
    //     active:false,
    //     ..user3
    // };//변경
    //println!("user3{}",user3.email);
    //                    ^^^^^^^^^^^ value borrowed here after move
    println!("user3{}",user4.email);
    // println!("{}",user3.email); 이런경우도 소유권이 넘어간다.
    let mut user_a=build_user(String::from("gig@gmail.com"),String::from("kim"));
    let user_b=build_user(String::from("gig@gmail.com"),String::from("kim"));
    let mut pair=Pair(user_a,user_b);
    // let mut pair=Pair(user_b,user_b);
    //같은 구조체를 전달했을때 ownership이 문제가 생기기때문에 에러가 일어난다.
    pair.1.sign_in_count=3;
    pair.0.sign_in_count=3;
    let strings:String=String::from("hihi");
    // strings.push_str("ByeBye");
    let mut strings=strings;
    strings.push_str("ByeBye");
    // println!("{:?}",user4);
    let r2=Rectangle::rectangle(1,5);
    let r1=Rectangle::rectangle(1,5);
    println!("{}",r1.compare(&r2));

    let mut b1:Board=Board::build_board("안녕하세요","반갑습니다\n잘부탁드려요");
    b1.increase_count();
    
    println!("{}",b1.count);
}
