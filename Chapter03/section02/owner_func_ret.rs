fn main() {
    let mut g1 = String::from("실수할 줄 아는 사람이 아름답다");
    g1 = show_msg(g1); // 소유권 이동 후 반환
    println!("{}",g1); // 정상
}

fn show_msg(msg: String) -> String {
    println!("{}", msg);
    msg // 소유권 반환
}