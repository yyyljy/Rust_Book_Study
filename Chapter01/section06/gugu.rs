fn main() {
    for i in 1..10 {
        for j in 1..10 {
            print!("{:5}", j * i);
        }
        println!("");
    }
}