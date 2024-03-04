use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game!!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("숫자를 입력 하세요!!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("입력 실패");
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("당신의 입력 : {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("작습니다."),
            Ordering::Greater => println!("큽니다."),
            Ordering::Equal => {
                println!("승리!");
                break;
            }
        }
    }
}
