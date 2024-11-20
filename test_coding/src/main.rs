use std::future::Future;

use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str,Option<String>) {
    let repsonse_text = trpl::get(url).await.text().await;
    let title = Html::parse(&repsonse_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
        (url,title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
        // let url = &args[1];

        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);
        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => {left},
            Either::Right(right) => {right},
        };
        match maybe_title {
            Some(title) => println!("title is {}", title),
            None => println!("{url} had no title"),
        }
    })
}
