enum Currency {
    Currency100(isize),
    Currency500(isize),
    Currency1000(isize),
    Currency5000(isize),
    Currency10000(isize),
    Currency50000(isize),
}
impl Currency {
    fn calc_price(&self) -> isize {
        match *self {
            Currency::Currency100(v) => v * 100,
            Currency::Currency500(v) => v * 500,
            Currency::Currency1000(v) => v * 1000,
            Currency::Currency5000(v) => v * 5000,
            Currency::Currency10000(v) => v * 10000,
            Currency::Currency50000(v) => v * 50000,
        }
    }
}

fn main() {
    let wallet: Vec<Currency> = vec![
        Currency::Currency100(3),
        Currency::Currency500(2),
        Currency::Currency1000(1), 
        Currency::Currency5000(6),
        Currency::Currency10000(3),
        Currency::Currency50000(7),
    ];
    let total = wallet.iter().fold(0, |sum, v| sum + v.calc_price());
    println!("지갑 안 금액 {}원", total);
}