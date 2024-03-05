fn main() {
    let s1:String=String::from("Hello, world!");
    println!("{}",s1);
    // take_owner_ship(s1);
    // println!("print {} again",s1);//에러를 이르킨다. 소유권이 이미넘어갔음으로
    let s1=takes_and_gives_back(s1);
    let i1=10;
    makes_copy(i1);
    println!("print {} again",i1);//에러를 이르키지않는다. 원시값은 그값이 복사되어 넘어감으로.

    println!("print {} again",s1);//에러를 이르키지않는다. 소유권을 돌려받았음으로
    let (s1,return_value)=calculate_length(s1);//반환값을 함께 튜플로 돌려 받을 수 있다.
    let length=calculate_length2(&s1);//s1의 참조만 전달했기 때문에 소유권을 돌려받지않아도 된다.
    //하지만 참조만 전달한경우 값에 대한 수정을 할수없다.
    println!("print {0} again,result{1}",s1,length);
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}",s);
    let r1 = &mut s;
    // let r2 = &mut s;//error
    // let mut r3=s;
    println!("{}ref",r1);
    // println!("{}ref2",r2);
    // println!("{}ref3",r3);


}
fn change(s :&mut String){
    s.push_str("hh");
    // let r2= &mut s;
}
fn calculate_length2(s: &String) -> usize {
    // s.push_str("hi");
    s.len()
}
fn give_owner_ship()->String{
    let some_string=String::from("hihi world");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    a_string
}
fn calculate_length(a_string: String) -> (String,usize) { // a_string comes into
    let length=a_string.len();
    (a_string,length)
}

fn take_owner_ship(some_string:String){
    println!("{}",some_string);
}
fn makes_copy(some_integer:i32){
    println!("{0}",some_integer);
}