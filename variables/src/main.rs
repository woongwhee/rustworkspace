
use std::io;
use std::mem::size_of;
fn main() {
    let mut x=5;
    println!("{x}");
    x=5;
    println!("{x}");
    const PI:f64=3.141592;
    println!("{PI}");
    let circle=(PI*(x as f64)*2.0)as u32;
    println!("{PI}");
    println!("{circle}");
    {
        let circle=10;
        println!("{circle}");
    }
    println!("{circle}");
    // const PI:i32=10;
    // let PI:i32=10;
    // Data Type
    //정수형
    let _un_signed:u32=113;//양수
    let _hex:i32=0xf8;//16진수
    let _octal:i32=0o77;
    let _binary=0b11111111;
    let _decimal=4_444_222;
    let ascii_to_num:u8=b'a';
    let _num_to_num:i32=(ascii_to_num as i32)+1;
    let is_hi :bool=true;
    if is_hi{
        println!("hi");
    }

    //overflow
    let y:u8=1;
    let (y,is_over_flow):(u8,bool)=y.overflowing_add(255);
    println!("{y}");//attempt to add with overflow
    println!("{is_over_flow}");//attempt to add with overflow
    let z:u8=4;
    let z:u8=z.saturating_mul(100);
    println!("{z}");//attempt to add with overflow
    println!("{is_over_flow}");//attempt to add with overflow


    // addition
    let sum = 5 + 10;
    println!("sum : {sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("sub: {difference}");
    // multiplication
    let product = 4 * 30;
    println!("mul:{product}");
    // division
    let quotient = 56.7 / 32.2;
    println!("div:{quotient}");
    let truncated = -5 / 3; // Results in -1
    println!("div2:{truncated}");
    // remainder
    let remainder = 43 % 5;
    println!("remainder:{remainder}");

    /*tuple*/
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{},{},{}",tup.0,tup.1,tup.2);
    let (x, y, z) = tup;
    println!("{x}{y}{z}");
    /*
    Array
    */
    let a=[1,2,3,4,5];
    let b:[i32;10]=[1,1,1,1,11,1,23,4,45,1];
    println!("{}",b[0]);
    let c=[10;c];
    println!("{:?}char size",size_of::<char>());
    println!("{:?}",c);//[3,3,3,3,3];
    let mut index=String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("fail to read line");
    let index: usize =index
        .trim()
        .parse()
        .expect("entered was not a number");
    println!("{}",b[index]);

}
