fn main() {
    let g1 = String::from("실수할 줄 아는 사람이 아름답다");
    show_msg(g1); // 소유권 이동
    println!("{}",g1); // 오류 발생
}

fn show_msg(msg: String) {
    println!("{}", msg);
}