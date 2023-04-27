use std::{env, fs};

fn main() {
    let args = env::args();
    let mut total:f64 = 0.0;

    if args.len() < 2 {
        println!("읽어올 파일을 지정해주세요.");
        return;
    }

    for (i, fname) in args.enumerate() {
        if i == 0 { continue; }

        let text = match fs::read_to_string(fname) {
            Ok(v) => v,
            Err(e) => e.to_string()
        };
                
        let lines = text.split("\r\n");

        for line in lines {
            let n:f64 = match line.parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };
            total += n;
        }
    }
    println!("{}",total);
}