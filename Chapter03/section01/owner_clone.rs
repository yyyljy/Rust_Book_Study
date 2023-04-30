fn main() {
    let g1 = String::from("온화한 마음은 몸에 좋다.");
    let g2 = g1.clone(); // 소유권 이동 x
    println!("{}", g2);
    println!("{}", g1); // 정상
}