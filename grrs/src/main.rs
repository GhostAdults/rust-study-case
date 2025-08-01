use core::time;
use std::{fs, thread};
use structopt::StructOpt;

fn do_hard_work(query: &String, content: &String) {
    thread::sleep(time::Duration::from_secs(5));
    for line in content.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}

fn main() {
    #[derive(StructOpt)]
    #[structopt(about = "-h")]
    struct Cli {
        #[structopt(default_value = "", long)]
        query: String,
        #[structopt(long)]
        filename: String,
    }
    // 获取参数
    let clis = Cli::from_args();

    let query = &clis.query;
    let filename = &clis.filename;
    let result = fs::read_to_string(&filename);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("无法读取文件：{}", error)
        }
    };
    let pb = indicatif::ProgressBar::new(1);
        do_hard_work(&query, &content);
        pb.set_message(format!("[+] finished"));
        pb.inc(1);
        
    pb.finish_with_message("done");
}
