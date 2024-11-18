fn main() {

    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop
    }

    let v = Status::Value(3);


    let s:Vec<Status> = (1u32..2).map(Status::Value).collect();

    println!("{:#?}",s);

    fn return_closure(r : i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(|r| r+ 1)
    }
    
}
