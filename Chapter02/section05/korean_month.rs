use std::collections::HashMap;
fn main() {
    let months = ["해오름달", "시샘달", "꽃내음달", "잎새달", "푸른달", "누리달",
        "빗방울달", "타오름달", "거둠달", "온누리달", "눈마중달", "매듭달"];

    let mut months_map: HashMap<&str, usize> = HashMap::new();

    for (i,v) in months.iter().enumerate() {
        months_map.insert(v, i+1);
    }

    println!("누리달 = {}월", months_map["누리달"]);
    println!("온누리달 = {}월", months_map["온누리달"]);
    println!("매듭달 = {}월", months_map["매듭달"]);

    if months_map.get("초코달달") == None {
        println!("초코달달은 존재하지 않음")
    } else {
        println!{"초코달달={}", months_map["초코달달"]};
    }
}