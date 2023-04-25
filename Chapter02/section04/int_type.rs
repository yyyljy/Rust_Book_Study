use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let a = 100u8;
    let b = 100i128;
    let c = 10_000;
    println!("타입 체크 : a={},({}), b={},({}), c={},({})", a, type_of(a), b, type_of(b), c, type_of(c));

}