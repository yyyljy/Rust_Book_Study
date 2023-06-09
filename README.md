# Rust_Book_Study

"만들면서 배우는 러스트 프로그래밍" 책으로 공부하는 Rust 언어

<img src="..\main\assets\book_img" alt="만들면서 배우는 러스트 프로그래밍 - 예스24" style="zoom: 25%;" width="300px" />

이미지 출처 : YES24

## Chapter 01

hello.rs

- 화면 출력, println! 매크로 사용

```bash
rustc hello.rs
./hello
```

```rust
println!("hello, world");
println("hello, world"); // 오류 발생
```

!가 붙는 것은 러스트 매크로를 뜻한다.



show_price.rs

- 변수를 문자열에 대입해서 출력

```rust
let banana = 300; // 값을 변수에 바인딩한다.
println!("바나나가격{}", banana);
```



moon.rs

- 사칙연산

```rust
// let moon = 384400.0;
// let car = 80.0;
// let btrain = 300.0;
// println!("달까지 자동차로 {}일", moon / car / 24.0); // 달까지 자동차로 200.20833333333334일
// println!("달까지 KTX로 {}일", moon / btrain / 24.0); // 달까지 KTX로 53.388888888888886일
let moon = 384400;
let car = 80;
let btrain = 300;
println!("달까지 자동차로 {}일", moon / car / 24); // 달까지 자동차로 200일
println!("달까지 KTX로 {}일", moon / btrain / 24); // 달까지 KTX로 53일
```

.0까지 입력하면 float로 계산



fizzbuzz.rs

- for문, 반복문

```rust
for i in 1..101 {
    if i % 3 == 0 && i % 5 == 0 {
        println!("FizzBuzz");
    } else if i % 3 == 0 {
        println!("Fizz");
    } else if i % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", i);
    }
}
```



함수에서 값을 반환할 때 해당 문장 끝에는 세미콜론을 붙이지 않는다.

조건문에 불필요한 ()를 붙이지 않는다.



gugu.rs

```rust
print!("{:5}", j * i);
```

공백 5칸을 기준으로 우측 정렬하여 출력

<img src="..\main\assets\gugu.png" alt="gugu.rs" style="zoom:50%;" />



gugu2.rs

```rust
for i in 1..10 {
	let s = (1..10)
	    .map(|j| format!("{:3}", j * i))
    	.collect::<Vec<String>>().join(",");
    println!("{}", s);
}
```

csv 형식으로 구구단 출력. 맨 끝에는 , 가 붙지 않게



fibo.rs

- 변수 선언

```rust
let mut a = 1; // 변수는 기본적으로 불변(immutable)으로 선언되며 mut를 붙여야 가변(mutable)
for _ in 0..30 { // 인덱스 없이 반복 _
    println!("{}", a + b);
}
```



coin_type.rs

- let 변수명: 타입 = 값;

```rust
let count500: i64 = 10;
```



caesar_enc.rs

```rust
fn encrypt(text: &str, shift: i16) -> String {
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;

    let mut result = String::new();

    for ch in text.chars() {
        let mut code = ch as i16;
        if code_a <= code && code <= code_z {
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
        result.push((code as u8) as char);
    }
    return result;
}

fn main() {
    let enc = encrypt("I LOVE RUST", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}
```

- 매개변수 타입 지정
- 함수 반환 타입 지정
- 자료형변환
- 문자열 변수 선언
- string -> char 배열
- 문자형 변수 매개변수 사

prime100.rs

```rust
fn get_primes(primes: &mut[usize; 100]) { // 100개의 usize 타입 배열 가변 매개변수
    let mut i = 2;
    let mut count = 0;
    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    let mut primes = [0; 100]; // 초기값이 0이고, 100개의 요소를 가진 배열
    get_primes(&mut primes); // 가변 배열 변수를 매개변수로 전달
    println!("{:?}", primes); // 배열형 변수 출력
    // println!("{:#?}", primes); // 줄바꿈 출력
}
```



sum1to10v2.rs

```rust
for i in 1..=10 { // 1,2,..10 까지 실행되는 반복문
	total += i;
}
```



sum1to10vec.rs

```rust
let nums = vec![1,2,3,4,5,6,7,8,9,10]; // Vector. 배열 요소 수 변경 가능한 가변 크기 배열
```



if_value.rs

```rust
let check_even_odd = if n % 2 == 0 {"짝수"} else {"홀수"};
// let check_even_odd2 = (n % 2 == 0) ? "짝수" : "홀수";
```

- RUST에는 삼항연산자가 존재하지 않는 대신 변수 선언문에서 if else를 사용할 수 있다.



## Chapter 02

"hello"

```bash
cargo new hello
cargo build
cargo run
...
cargo --help
```

- cargo new hello
  - hello 라는 이름으로 프로젝트 생성
- cargo run
  - build 후에 run



"pow1234_5678"

Cargo.toml - dependencies에 필요한 라이브러리(크레이트 Crate)를 입력

```rust
use num_bigint::BigInt;

fn main() {
    let v = BigInt::from(1234);
    println!("{}", v.pow(5678));
}
```



"dice"

Cargo.toml

```
[dependencies]
rand = "0.8.0"
```

```rust
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let dice = rng.gen_range(1..=6); // 1 ~ 6 사이 랜덤 숫자 생성
        println!("{}", dice);
    }
}
```

```rust
let r = 3..15;
println!("{}..{}", r.start, r.end);
// 3..15
```



"maze"

```rust
const MAP_N: usize = 25;
let mut maze = [[0; MAP_N]; MAP_N]; // 이차원 배열 선언, 초기값 0

match r { // r 값에 따른 분기 처리 방법
     0 => maze[y-1][x] = 1,
     1 => maze[y+1][x] = 1,
     2 => maze[y][x-1] = 1,
     3 => maze[y][x+1] = 1,
     _ => {},
}
```



"bingocard"

```rust
use rand::seq::SliceRandom;
let mut nums = [0; 75];
for i in 1..=75 { nums[i-1] = i; }

let mut rng = rand::thread_rng();
nums.shuffle(&mut rng); // 배열 섞기
```



"vec_nomacro.rs"

```rust
let mut nums = Vec::new(); // vec![1,2,3] 처럼 vec! 매크로를 쓰지 않고 벡터 선언하는 방법
nums.push(1);
```



"vec_u32_str.rs"

```rust
let a_vec: Vec<u32> = vec![100,200,300];
for i in a_vec {
	println!("{}",i);
}

let s_vec: Vec<&str> = vec!["개","고양이","닭"];
for i in s_vec {
	println!("{}",i);
}
```



"bmi.rs"

```rust
let bmi = weight / height.powf(2.0); // 실수의 제곱
println!("BMI={:.1}", bmi); // 소수점 첫째자리까지 표현

fn input(prompt: &str) -> f64 { // 키보드 입력 받는 함수
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("입력 에러"); // 에러 발생시 메세지 출력
    return s.trim().parse().expect("숫자가 아닙니다."); // 타입추론으로 parse()가 f64를 반환
}
```



"str_parse.rs"

```rust
let num = s.parse::<f64>().expect("변환 실패"); // 형변환
```



Result 타입 처리

```rust
let s = "3.1415a";
let num = s.parse::<f64>();
match num {
	Ok(result) => println!("{:2}", result),
	Err(e) => println!("에러가 발생했습니다. 이유 : '{:?}'", e)
}
```



"input_height.rs"

```rust
loop {
    println!("키(cm) : ");
    height = input_f(0.0);
    if height > 0.0 { break; }
    println!("숫자만 입력 가능");
}
```

```rust
let i: i32 = match s.unwrap(); // 오류 발생 시 에러처리 생략 (프로그램 종료)
```



"int_type.rs"

```rust
fn main() {
    let a = 100u8;
    let b = 100i128;
    let c = 10_000; // 10000과 똑같은 값. 보기 편하게
}
```



```rust
let a = 'a';
let b = b'a'; // ASCII 코드 97u8
let c = '\x61'; // 16진수로 문자 'a'
println!("{}, {:2x}, {}",a,b,c);

let d = '곰';
let e = '곰' as u32;
let f = '\u{acf0}'; // 16진수
println!("{}, {:4x}, {}",d, e, f);

let v1 = 0xFF; // 16진수
let v2 = 0o655; // 8진수
let v3 = 0b1101_0101; // 2진수
println!("{}, {}, {}", v1, v2, v3);

let f1 = 10.5; // 부동 소수점 숫자 리터럴
let f2 = 10.5f32; // f32 부동 소수점
let f3 = 10.5e+8; // 1050000000
println!("{}, {}, {}", f1, f2, f3);
```



"counter.rs"

```rust
use std::collections::HashMap;
let mut c_map = HashMap::new(); // HashMap 생성
c_map.insert("A", 0); // HashMap 데이터 삽입

for k in ["A","B","C"] {
    println!("{}: {:>2}", k, c_map[k]); // 2칸 기준 오른쪽 정렬
}
```



"korean_month.rs"

```rust
let mut months_map: HashMap<&str, usize> = HashMap::new(); // HashMap 타입지정 선언
for (i,v) in months.iter().enumerate() { // for 문에서 index와 value`
	months_map.insert(v, i+1);
}
if months_map.get("초코달달") == None { // Map에 존재하는 키인지 확인 (존재하면 Some 반환)
    println!("초코달달은 존재하지 않음")
} else {
    println!{"초코달달={}", months_map["초코달달"]};
}
```



"hashmap_match.rs"

```rust
match map.get("C") { // match를 이용해 Option 타입 분기로 map에 키값 존재 유무 컨트롤
	Some(v) => println!("C={}", v);
	None => println!("C는 존재하지 않음");
}
```



"sum_args.rs"

```rust
let args = std::env::args(); // 명령줄에 인수 받기

for (i, s) in args.enumerate() {
    if i == 0 { continue; }

    let num: f64 = match s.parse() {
        Ok(v) => v,
        Err(_) => 0.0,
    };
    total += num;
}
```



"print_args.rs"

```rust
use std::env;

fn main() {
    let args = env::args();
    for arg in args {
        println!("{}", arg);
    }
}
```

"print_args_vec.rs"

```rust
fn main() {
    let args:Vec<String> = std::env::args().collect();
    println!("{:?}", args);
}
```



"read_file.rs"

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("읽어올 파일을 지정해주세요.");
        return;
    }

    let filename = &args[1];
    let text = match fs::read_to_string(filename) { // 파일 읽기
        Ok(v) => v,
        Err(e) => e.to_string()
    };
    println!("{}",text);
}
```



- 실행 결과 파일 출력

```shell
./fizzbuzz > ./fizzbuzz_rust.txt
```



"dictionary.rs"

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

let dicfile = "dict.txt";
let fp = File::open(dicfile).unwrap(); // 파일 열기
let reader = BufReader::new(fp); // 파일 읽기
```



"fizzbuzz_filename.rs"

```rust
use std::fs::{self, File};
use std::io::{Write, BufWriter};

fn main() {
    let filename = "fizzbuzz_file_result.txt";
    {
        let fp = File::create(filename).unwrap();
        let mut writer = BufWriter::new(fp);

        for i in 1..=100 {
            let mut line = format!{"{}\n", i};
            if (i % 3 == 0) && (i % 5 == 0) {
                line = String::from("FizzBuzz\n");
            } else if i % 3 == 0 {
                line = String::from("Fizz\n");
            } else if i % 5 == 0 {
                line = String::from("Buzz\n");
            }

            let bytes = line.as_bytes();
            writer.write(bytes).unwrap(); // 한 줄씩 파일에 쓰기
        }
    }
}
```



"fizzbuzz_filename2.rs"

```rust
use std::fs::File;
use std::io::Write;

fn main() {
    let filename = "fizzbuzz_file2_result.txt";
    let data = get_fizzbuzz(100);
    
    let mut fp = File::create(filename).unwrap();

    let bytes = data.as_bytes();
    fp.write_all(bytes).unwrap(); // 한 번에 파일 쓰기
}

fn get_fizzbuzz(max: u32) -> String {
    let mut result = String::new();

    for i in 1..=max {
        if (i % 3 == 0) && (i % 5 == 0) {
            result += "FizzBuzz\n";
        } else if i % 3 == 0 {
            result += "Fizz\n";
        } else if i % 5 == 0 {
            result += "Buzz\n";
        } else {
            result += &format!("{}\n", i);
        }
    }
    result
}
```



파일 에러 처리 방법

```rust
let text = fs::read_to_string("something.txt").unwrap(); // 에러 시 강제종료

let text1 = fs::read_to_string("something.txt").unwrap_or("실패 값".to_string());

let text2 = match fs::read_to_string("something.txt") {
    Ok(text) => {
        text // 정상 값 반환
    },
    Err(e) => {
        // 에러 처리
    }
}

if let Ok(s) = fs::read_to_string("somefile.txt") {
    // 성공시 처리
} else {
    // 에러시 처리
}
```



재귀함수

```rust
fn sum(n:i32) -> i32 {
    if n <= 0 {return 0;}
    return sum(n-1) + n ;
}

fn main() {
    println!("{}", sum(10));
}
```



재귀함수 사용 파일 탐색 "findfile.rs"

```rust
fn findfile(target: &path::PathBuf, keyword: &str) {
    let files = target.read_dir().expect("존재하지 않는 경로");
    for dir_entry in files {
        let path = dir_entry.unwrap().path();
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }

        let fname = path.file_name().unwrap().to_string_lossy();
        if None == fname.find(keyword) { continue; }
        println!("{}", path.to_string_lossy());
    }
}
```



unwrap_or

```rust
let value = 매서드().unwrap();
let value = 메서드().unwrap_or(인수 값);
let value = 메서드().unwrap_or_else(클로저);
```



tree

```rust
fn tree(target: &path::PathBuf, level: isize) {
    let files = target.read_dir().expect("존재하지 않는 경로입니다.");
    for ent in files {
        let path = ent.unwrap().path();
        for _ in 1..=level {
            print!("|   ");
        }
        let fname = path.file_name().unwrap().to_string_lossy();
        if path.is_dir() {
            println!("|-- <{}>", fname);
            tree(&path, level+1);
            continue;
        }
        println!("|-- {}", fname);
    }
}
```



## Chapter03

소유권

```rust
fn main() {
    let g1 = String::from("온화한 마음은 몸에 좋다.");
    let g2 = g1; // 값 소유권 이동
    println!("{}", g2);
    println!("{}", g1); // 소유권이 없어서 오류 발생
}
```

```rust
fn main() {
    let g1 = 30;
    let g2 = g1; // 정수, 부동 소수점 등의 숫자 타입, Boolean 타입 등은 소유권 이동 X
    // 힙 : 임의의 순서로 메모리 확보, 해제 가능
    // 스택 : 순차적(마지막 메모리부터) 메모리 확보 및 해제 가능
    // 기본 타입 데이터는 크기가 정해져 있어서 스택 영역에 저장되어 있음.
    println!("{}", g1); // 정상
    println!("{}", g2); // 정상
}
```

```rust
let g1 = String::from("온화한 마음은 몸에 좋다.");
let g2 = g1.clone(); // .clone() 소유권 이동 x
```

```rust
fn main() {
    let g1 = String::from("실수할 줄 아는 사람이 아름답다");
    show_msg(g1); // 함수 호출에 따른 소유권 이동
    println!("{}",g1); // 오류 발생
}

fn show_msg(msg: String) {
    println!("{}", msg);
}
```

```rust
fn main() {
    let mut g1 = String::from("실수할 줄 아는 사람이 아름답다");
    g1 = show_msg(g1); // 함수 호출에 따른 소유권 이동 후 반환
    println!("{}",g1); // 정상
}

fn show_msg(msg: String) -> String {
    println!("{}", msg);
    msg // 소유권 반환
}
```

```rust
fn main() {
    let g1 = String::from("실수할 줄 아는 사람이 아름답다");
    show_msg(&g1); // 소유권 빌리(borrow)
    println!("{}",g1); // 정상
}

fn show_msg(msg: &String) { // 소유권 빌림 (borrow)
    println!("{}", msg);
}
```

```rust
fn add_qoute(msg: &mut String) { // 가변 참조자를 인수로 사용
    msg.insert(0, '"');
    msg.push('"');
}

fn main() {
    let mut msg = String::from("건강한 신체에 건강한 정신이 깃든다.");
    println!("{}",msg);
    add_qoute(&mut msg); // 가변 참조자 전달
    println!("{}",msg);
}
```

```rust
fn p2(arg: &mut i32) {
    *arg = *arg + 2;
}

fn main() {
    let mut v = 16;
    p2(&mut v);
    println!("{}", v);
}
```

```rust
fn main() {
    let year = 2023;
    let month = 12;
    let day = 1;
    println!("KR:{0}/{1}/{2}", year, month, day);
    println!("US:{1}/{2}/{0}", year, month, day);
    println!("UK:{2}/{1}/{0}", year, month, day);
    println!("{yy}년 {mm}월 {dd}일", yy=year, mm=month, dd=day);
}
```

- 출력 서식 지정 : p205 참고



struct, tuple

```rust
struct Item(String, i64);

fn main() {
    let banana = Item("바나나".to_string(), 300);
    let apple = Item("사과".to_string(), 200);
    let mango = Item("망고".to_string(), 500);
    let items = vec![banana, apple, mango];

    let total = print_and_sum_items(&items);
    println!("합게 {}원", total);
}

fn print_tuple(item: &Item) {
    println!("{}를 {}원에 구입", item.0, item.1);
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for it in items {
        print_tuple(&it);
        total += it.1;
    }
    total
}
```



array

```rust
fn main() {
    let points:[i32;5] = [10, 20, 40 ,50 ,60]; // 스택 영역에 메모리 확보
    // 배열 선언 후 크기 변경 불가
    print_array(&points);
    array_get(1, &points);
}

fn print_array(e: &[i32;5]) {
    println!("{:?}",e);
    println!("len={}", e.len());
}
 
fn array_get(i: usize, arr:&[i32]) {
    println!("{} = {:?}", i, &arr[i]);
}
```



slice

```rust
fn main() {
    let s = String::from("beep");
    let ss = &s[0..3]; // 0번 index 부터 3번 index 전까지
    println!("{}", ss); // bee  [0,1,2]
    println!("{}", s); // 소유권 갖고 있기 때문에 출력 가능
    let sss = &s[2..4];
    println!("{}", sss); // ep  [2,3]
}
```



struct

```rust
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
    println!("car1: {}, {}cc, {:06x}", car1.model, car1.cc, car1.color);
    println!("{:06x}",get_car_color(&car1));
}

fn get_car_color(car: &CarSpec) -> i32 {
    car.color
}
```



rust 명명 규칙

카멜 방식						: CamelCase		: 구조체, 타입, 열거형, 타입 매개변수

스네이크 방식(소문자)	: snake_case		: 크레이트, 모듈, 함수, 메서드, 지역 변수

스네이크 방식(대문자)	: SNAKE_CASE	: 상수, 고정 변수



string

```rust
fn main() {
    let s = "안녕하세여";
    let ch = s.chars().nth(0).unwrap(); // 안
    println!("{}", ch);

    let ch = s.chars().nth(2).unwrap(); // 하
    println!("{}", ch);
    
    let s2 = "abcdefg";
    println!("{}", &s2[0..1]); // a

    let ch = &s[..3];
    println!("{}", ch); // 안

    let ch = &s[6..9];
    println!("{}", ch); // 하
}
```



string find

```rust
fn main() {
    let s = format!("{}{}", "There is more happiness in giving ",
                            "than there is in receiving.");

    let res = s.find(|c:char| c.to_ascii_uppercase() == 'S');
    match res {
        Some(i) => println!("S={}B", i), // 7B
        None => println!("None"),
    };

    let s = "제주도의 특산물 중 귤은 겨울에 많이 먹을 수 있다.";

    match s.find('귤') {
        Some(i) => println!("귤 = {}B", i), // 27B
        None => println!("'귤'이라는 단어는 없습니다."),
    };
}
```



string replace

```rust
let s = "내 자신에 대한 자신감을 잃으면 온 세상이 나의 적이 된다.";
let s2 = s.replace("잃으면", "가지면");
```



string split

```rust
let telno = "955-3658";

println!("--슬라이스--");
println!("국번: {}", &telno[..3]);
println!("사번: {}", &telno[4..]);

println!("--split_at--");
let (telno1, telno2) = telno.split_at(3);
let (telno2, telno3) = telno2.split_at(1);
println!("국번: {}", telno1);
println!("구분: {}", telno2); // -
println!("사번: {}", telno3);

println!("--split_off--");
let mut telno1 = String::from(telno);
let mut telno2 = telno1.split_off(3);
let telno3 = telno2.split_off(1);
println!("국번: {}", telno1);
println!("구분: {}", telno2); // -
println!("사번: {}", telno3);

println!("--split--");
let telno_a: Vec<&str> = telno.split("-").collect();
println!("국번: {}", telno_a[0]);
println!("사번: {}", telno_a[1]);
```



string file  encode

```rust
Cargo.toml
[dependencies]
encoding_rs = "0.8.28"

main.rs
use encoding_rs;

let (enc, _, _) = encoding_rs::EUC_KR.decode(&buf);
```



random number

```rust
use std::time::{SystemTime, UNIX_EPOCH};

static mut SEED: u32 = 0;

unsafe fn rand_global(start: u32, end: u32) -> u32 {
    if SEED == 0 {
        let epoc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        SEED = epoc.as_millis() as u32;
    }
    SEED ^= SEED << 15;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;
    return SEED % (end - start + 1) * start;
}

fn main() {
    for _ in 0..100 {
        unsafe {
            let v = rand_global(1, 6);
            println!("{}", v);
        }
    }
}
```

