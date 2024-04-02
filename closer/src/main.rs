use std::thread;
use std::time::Duration;
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> 
    where T: Fn(u32) -> u32    
{
    fn new(calculation: T)->Cacher<T>{
        Cacher { calculation, value:None }
    }
    fn value(&mut self,arg:u32)->u32{
        match self.value{
            Some(v) => v,
            None =>{
                let v = (self.calculation)(arg);
                self.value=Some(v);
                v
            }
        }
    }
}
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculationg slowly...");
    thread::sleep(Duration::from_secs_f64(2.0));
    intensity
}
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    // let expensive_closure = |num| {
    //     println!("calculationg slowly...");
    //     thread::sleep(Duration::from_secs_f64(2.0));
    //     num
    // };
    let  mut expensive_cacher=Cacher::new(|num| {
        println!("calculationg slowly...");
        thread::sleep(Duration::from_secs_f64(2.0));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_cacher.value(intensity));
        println!("Next, do {} situps!", expensive_cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated")
        } else {
            println!("Today, run for {}minutes", expensive_cacher.value(intensity));
        }
    }
}
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
    let list =vec![1,2,3];
    // println!("borows_defining closure:{:?}",list);
    // let mut borrow_mutably=||list.push(7);
    // let only_borrows=|| println!("{:?}",list);
    // borrow_mutably();
    // only_borrows();
    // println!("after calling closure:{:?}",list);
    //다른스레드로 이동시 더이상 이 스레드에 이 객체를 남겨두지 않는것이 좋다.
    thread::spawn(move || println!("From thread: {:?} ",list)).join().unwrap();

    

}
