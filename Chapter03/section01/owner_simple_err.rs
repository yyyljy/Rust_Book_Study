fn main() {
    let g1 = String::from("온화한 마음은 몸에 좋다.");
    let g2 = g1;
    println!("{}", g2);
    println!("{}", g1); // 소유권이 없어서 오류 발생
}