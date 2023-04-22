fn main() {
    let n = 5;
    let check_even_odd = if n % 2 == 0 {"짝수"} else {"홀수"};
    // let check_even_odd2 = (n % 2 == 0) ? "짝수" : "홀수";
    println!("{}", check_even_odd);
}