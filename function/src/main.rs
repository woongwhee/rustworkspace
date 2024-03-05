

fn main() {
    let block = {
        let y=0;
        let z= y+1;
        println!("Hello, world!{}{}",z,y);
        z+100
    };
    another_function(block);
    // let panic_value=panic_lapper(false);
    // println!("{}",panic_value);
    // let panic_value=panic_lapper(true);
    // println!("{}",panic_value);
    fun_if(10);

}

fn another_function(x:i32)->i32{
    println!("{x}");

    return 0;
}
fn panic_lapper(is_panic:bool)->i32{
        panic_function(is_panic);
    return 1;
}
fn panic_function(is_panic:bool)->!{
        panic!("is panic");
}


fn fun_if(x:i32){
    if(x>10){
        println!("big");
    }else if(x==10){
        println!("equal");
    }else {
        println!("mall");
    }
    let result:&str= if(x%2==0){"even"}else{"odd"};
    println!("{}",result);


}
