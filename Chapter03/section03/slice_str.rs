fn main() {
    let s = String::from("beeping!@#$%^");
    let ss = &s[0..3];
    println!("{}", ss);
    println!("{}", s);
    let sss = &s[2..4];
    println!("{}", sss);
}