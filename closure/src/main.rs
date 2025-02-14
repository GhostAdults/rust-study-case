use std::thread;
use std::time::Duration;

fn main() {
    let x = 1;
    // 闭包
    let mut exp_closure = Cacher::new(|num| {
        thread::sleep(Duration::from_secs(2));
        num + x
    });
    println!("calculation:{}", exp_closure.get_value(1));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn get_value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                println!("this way");
                self.value = Some(v);
                v
            }
        }
    }
}
