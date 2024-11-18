
fn  longest<'a>(x: &'a str,y: &'a str) -> & 'a str {
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("short");
    let result;
    {
        let string2 = "long string is long";
        result = longest(&string1,&string2);
    }
    println!("long string is {}",result)
 
}
