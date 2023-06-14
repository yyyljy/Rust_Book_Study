use std::fs;

fn main() {
    let afile = "./fizzbuzz_python_jw.txt";
    let bfile = "./fizzbuzz_python.txt";
    // let afile = "./fizzbuzz_rust.txt";
    // let bfile = "./fizzbuzz_rust_jw.txt";

    let astr = fs::read_to_string(afile).unwrap();
    let bstr = fs::read_to_string(bfile).unwrap();

    let astr = astr.trim().trim();
    let bstr = bstr.trim().trim();

    if astr == bstr {
        println!("OK");
    } else {
        println!("ng");
    }
}

// use std::fs;
// fn main() {
//     let afile = "./fizzbuzz_python_jw.txt";
//     let bfile = "./fizzbuzz_rust_jw.txt";

//     let astr = fs::read_to_string(afile).unwrap();
//     let bstr = fs::read_to_string(bfile).unwrap();
    
//     // let astr = astr.trim();
//     // let bstr = bstr.trim();
//     // println!("astr {}", astr);
//     // println!("bstr {}", bstr);


//     if astr == bstr {
//         println!("ok");
//     } else {
//         println!("ng");
//     }
// }

// use std::fs::File;
// use std::io::{BufRead, BufReader, Read};

// fn main() {
    // 파일 경로 설정
    // let file1_path = "./abc.txt";
    // let file2_path = "./abc2.txt";
    // let file1_path = "./fizzbuzz_file_result.txt";
//     // let file2_path = "./fizzbuzz_file2_result.txt";
//     let file1_path = "./fizzbuzz_python_jw.txt";
//     let file2_path = "./fizzbuzz_python.txt";
//     // let file1_path = "./fizzbuzz_rust.txt";
//     // let file2_path = "./fizzbuzz_rust_jw.txt";

//    // 파일 열기
//     let file1 = File::open(file1_path).expect("Failed to open file1");
//     let file2 = File::open(file2_path).expect("Failed to open file2");

//     // BufReader를 사용하여 파일 읽기 성능 향상
//     let reader1 = BufReader::new(file1);
//     let reader2 = BufReader::new(file2);

//     // 한 글자씩 비교
//     let mut bytes1 = reader1.bytes();
//     let mut bytes2 = reader2.bytes();

//     loop {
//         let result1 = bytes1.next();
//         let result2 = bytes2.next();
        
//         match (result1, result2) {
//             (Some(Ok(byte1)), Some(Ok(byte2))) => {
//                 println!("{}:::::{}",byte1,byte2);
//                 if byte1 != byte2 {
//                     println!("파일의 내용이 다릅니다.");
//                     break;
//                 }
//             }
//             (None, None) => {
//                 println!("파일의 내용이 같습니다.");
//                 break;
//             }
//             _ => {
//                 println!("파일의 길이가 다릅니다.");
//                 break;
//             }
//         }
//     }
// }