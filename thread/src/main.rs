use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let v = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for var in v {
            tx.send(var).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let v2 = vec![
            String::from("1,hi"),
            String::from("1,from"),
            String::from("1,the"),
            String::from("1,thread"),
        ];
        for var in v2 {
            tx1.send(var).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("got message: {:?}", received);
    }
}
