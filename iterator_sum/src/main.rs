//! # iter_num说明
//! 迭代器的使用 最大次数 5

fn main() {
    iter_num();
}

///!  迭代器方法
pub fn iter_num() {
    let counter1:u32 =  
    Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a,b)| a * b)
    .filter(|x| x % 3  == 0).sum();
     assert_eq!(11,counter1);
    
}

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32; 
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 3 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum() {
        iter_num();
    }
}
