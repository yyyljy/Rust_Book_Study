fn main() {
    for i in 1392..1451 {
        print!("서력 {} 년 = ", i);
        if i >= 1419 {
            if i == 1419 { println!("세종 원년"); }
            else { println!("세종 {} 년", i-1419+1); }
        } else if i >= 1401 {
            if i == 1401 { println!("태종 원년"); }
            else { println!("태종 {} 년", i-1401+1); }
        } else if i >= 1399 {
            if i == 1399 { println!("정종 원년"); }
            else { println!("정종 {} 년", i-1399+1); }
        } else if i >= 1392 {
            if i == 1392 { println!("태조 원년"); }
            else { println!("태조 {} 년", i-1392+1); }
        }
    }
}