use std::{
    future::{Future},
    time::Duration,
};

use trpl::{Either, Html};

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
        // let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        // vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
        // trpl::join_all(futures).await;
        // trpl::race(tx_fut, tx1_fut).await;
        let slow = async {
            trpl::sleep(Duration::from_secs(4)).await;
            "I finished!"
        };
        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("succeded with '{message}'"),
            Err(duration) => println!("failed after {} seconds", duration.as_secs()),
        }
    });
    for i in 0..100 {
        do_something();
    }
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(message) => Ok(message),
        Either::Right(_) => Err(max_time),
    }
}
fn do_something() {}
