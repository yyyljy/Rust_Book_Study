fn echo(s: &'static str) {
    println!("{}", s);
}

fn main() {
    echo("웅변은 은이요");
    echo("침묵은 금이다");

    // let s = String::from("테스트");
    // let s2: &'static str = &s;
    // let s2: &'static str = "테스트";
    // let s2 = "테스트";
    echo(&s2);
}