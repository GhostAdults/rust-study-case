fn main() {
    let name = "mY NAME IS ANDY".to_string();

    let mut flag = true;
    for _i in 1..10 {
        if flag == true {
            flag = false;
            let tmp = name;
           println!("Hello, {}!",tmp);
        }
    }
    
}
