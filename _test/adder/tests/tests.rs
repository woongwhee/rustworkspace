use adder::*;
#[test]
fn greeting_contain_name(){
    let test_str="cargo";
    let result=greeting(test_str);
    assert!(result.contains(test_str));
}
#[test]
fn greeting_contain_name_print(){
    let result=greeting("cargo");
    assert!(
        result.contains("cargo"),
        "Greeting did not contain name, value was `{}`",
        result
    );
    //실패할시 출력할 문장을 지정 합니다.
}
#[test]
fn greeting_contain_name_print2(){
    let result=greeting("cargo");
    assert!(
        result.contains("cargo"),
        "Greeting did not contain name, value was `{}`",
        result
    );
    //에러를   이렇게 됬다.
}
#[test]
fn add_work() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
#[test]
fn exploration(){
    let result = add(2, 2);
    assert_eq!(result, 4);
}
#[test]
fn larger_can_hold_smaller(){
    let larger=figure::Rectangle::new(6,7);
    let smaller=figure::Rectangle::new(5,6);
    assert!(larger.can_hold(&smaller));
}
#[test]
fn smaller_cannot_hold_lager(){
    let larger=figure::Rectangle::new(6,7);
    let smaller=figure::Rectangle::new(5,6);
    assert!(!smaller.can_hold(&larger));
}
#[test]
fn it_adds_two(){
    //pub함수가 아니여도 접근할수 있따.
    assert_eq!(4,add_two(2));
}

#[test]
#[should_panic(expected ="less than or equal to 100")]
//특정 문자열을 포함하고있는 panic을 발생시키는지 확인
fn greater_than_100(){
    Guess::new(101);
}

#[test]
fn it_works()->Result<(),String>{
    if 1+1==2{
        Ok(())
    }else{
        Err(String::from("one plus one does not equal two"))
    }


}