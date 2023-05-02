fn main() {
    let banana = ("바나나", 300);
    let apple = ("사과", 200);

    let total = banana.1 + apple.1;
    print_tuple(&banana);
    print_tuple(&apple);
    println!("합계 : {}원", total);
}

fn print_tuple(item: &(&str, i64)) {
    println!("{}를 {}원에 구입", item.0, item.1);
}