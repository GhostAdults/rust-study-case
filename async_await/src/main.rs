use std::pin::Pin;
use std::thread;
use std::{
    env::consts::FAMILY,
    future::{self, Future},
    time::Duration,
};

use trpl::{Either, Html};
use trpl::{ReceiverStream, Stream, StreamExt};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let repsonse_text = trpl::get(url).await.text().await;
    let title = Html::parse(&repsonse_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {
    // let args: Vec<String> = std::env::args().collect();
    // trpl::run(async {
    //     // let url = &args[1];
    //     let title_fut_1 = page_title(&args[1]);
    //     let title_fut_2 = page_title(&args[2]);
    //     let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };
    //     match maybe_title {
    //         Some(title) => println!("title is {}", title),
    //         None => println!("{url} had no title"),
    //     }
    // })
    // trpl::run(async {
    //     let (tx, mut rx) = trpl::channel(); // mpsc 异步版本

    //     let tx1 = tx.clone();
    //     let tx1_fut = async move {
    //         let vals = vec![
    //             String::from("hi"),
    //             String::from("from"),
    //             String::from("the"),
    //             String::from("future"),
    //         ];

    //         for val in vals {
    //             tx1.send(val).unwrap();
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };

    //     let tx_fut = async move {
    //         let vals = vec![
    //             String::from("more"),
    //             String::from("messages"),
    //             String::from("for"),
    //             String::from("you"),
    //         ];

    //         for val in vals {
    //             tx.send(val).unwrap();
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };
    //     let rx_fut = async {
    //         while let Some(value) = rx.recv().await {
    //             println!("got {}", value)
    //         }
    //     };
    // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
    // vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
    // trpl::join_all(futures).await;
    // trpl::race(tx_fut, tx1_fut).await;
    // });
    // let slow = async {
    //     trpl::sleep(Duration::from_secs(4)).await;
    //     "I finished!"
    // };
    // trpl::run(async {
    //     match timeout(slow, Duration::from_secs(2)).await {
    //         Ok(message) => println!("succeded with '{message}'"),
    //         Err(duration) => println!("failed after {} seconds", duration.as_secs()),
    //     }
    // });
    trpl::run(async {
        let mut messages = get_messages();
       
        while let Some(message) = messages.next().await {
            println!("{message}");
        }
        println!("start")
    });

}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
     println!("geting");
    let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
    for message in messages {
        tx.send(format!("Message: '{message}'")).unwrap();
    }
    

    ReceiverStream::new(rx)
}