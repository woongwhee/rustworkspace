use adder::*;
mod commons;
#[test]
fn add_test(){
    commons::setup();
    assert_eq!(6,add_two(4));
}