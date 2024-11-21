use std::{future::{self, Future}, time::Duration};

use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let repsonse_text = trpl::get(url).await.text().await;
    let title = Html::parse(&repsonse_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}

fn main() {
    let mut s = String::from("hello");
    println!("{s}");
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
    trpl::run(async {
        let (tx, mut rx) = trpl::channel(); // mpsc 异步版本

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("got {}", value)
            }
        };
        // let futures = vec![tx1_fut,rx_fut,tx_fut];
        // trpl::join_all(futures).await;
        trpl::join(tx_fut,tx1_fut).await
    });
}
