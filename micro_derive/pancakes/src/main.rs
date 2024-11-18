use hello_mirco_derive::HelloMicros;
use hello_mirco::HelloMicros;

#[derive(HelloMicros)]
struct  Pancakes;

fn main() {
    Pancakes::hello_macro();
}
