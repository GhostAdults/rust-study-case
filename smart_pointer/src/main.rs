use std::ops::Deref;

fn main() {
    let x = 3;
    let y = CustomSmartPointer::new(x);
    
    assert_eq!(x, *y);
} 

struct CustomSmartPointer<T> {
    data: T,
}
impl<T> CustomSmartPointer<T> {
    fn new(s: T) -> CustomSmartPointer<T> {
        CustomSmartPointer { data: s }
    }
}

impl<T> Deref for CustomSmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
