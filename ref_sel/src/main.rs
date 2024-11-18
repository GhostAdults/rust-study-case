use std::cell::RefCell;
fn main() {
    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send(String::from("Hello,world!"));
    mq.send(String::from("Message is pull"));
    println!("messagequeue is {:#?}",mq.msg_cache)
}
pub trait Messenger {
    fn send(&self,msg:String);
}

pub struct  LimitTracker<'a,T: 'a + Messenger> {
    messenger: & 'a T,
    value : usize,
    max : usize,
}

impl <'a,T> LimitTracker<'a,T> 
where T: Messenger,{
     pub fn new(messenger:&T,max:usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
     }
}

struct MsgQueue {
    msg_cache: RefCell<Vec<String>>,
}

impl Messenger for MsgQueue {p
    fn send(&self,msg:String) {
        self.msg_cache.borrow_mut().push(msg);
    }
}
