use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn quick_sort(arr: &mut [u32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = arr[arr.len() / 2];
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        while arr[left] < pivot {
            left += 1;
        }
        while arr[right] > pivot {
            right -= 1;
        }
        if left <= right {
            arr.swap(left, right);
            left += 1;
            right = right.saturating_sub(1);
        }
    }

    if right > 0 {
        quick_sort(&mut arr[..=right]);
    }
    if left < arr.len() {
        quick_sort(&mut arr[left..]);
    }
}

fn main() {
    println!("猜数字游戏!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let mut guesses = Vec::new(); // 存储猜测历史
    
    loop {
        println!("请输入你猜的数字:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取输入失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字!");
                continue;
            }
        };

        guesses.push(guess); // 将猜测添加到历史记录中

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("你赢了!");
                println!("你总共猜了 {} 次", guesses.len());
                
                // 使用自定义的快速排序函数
                // quick_sort(&mut guesses);
                
                println!("你的猜测历史 (已排序): {:?}", guesses);
                break;
            }
        }
    }
}
