fn add_qoute(msg: &mut String) {
    msg.insert(0, '"');
    msg.push('"');
}

fn main() {
    let mut msg = String::from("건강한 신체에 건강한 정신이 깃든다.");
    println!("{}",msg);
    add_qoute(&mut msg);
    println!("{}",msg);
}