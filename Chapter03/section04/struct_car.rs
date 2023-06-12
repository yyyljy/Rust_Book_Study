struct CarSpec {
    model: i32,
    cc: i32,
    color: i32,
}

fn main() {
    let car1 = CarSpec {
        model: 3333,
        cc: 1500,
        color: 0xFF0000,
    };

    let car2 = CarSpec {
        model: 101,
        cc:1000,
        color: 0x000000,
    };

    println!("car1: {}, {}cc, {:06x}", car1.model, car1.cc, car1.color);
    println!("car1: {}, {}cc, {:06x}", car2.model, car2.cc, car2.color);
    // println!("car1: {}, {}cc", car1.model, car1.cc);
    // println!("car1: {}, {}cc", car2.model, car2.cc);
    println!("{:06x}",get_car_color(&car1));
    println!("{:06x}",get_car_color(&car2));
}

fn get_car_color(car: &CarSpec) -> i32 {
    car.color
}