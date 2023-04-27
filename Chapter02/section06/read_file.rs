use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("읽어올 파일을 지정해주세요.");
        return;
    }

    let filename = &args[1];
    let text = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(e) => e.to_string()
    };
    println!("{}",text);
}