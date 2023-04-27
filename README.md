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

