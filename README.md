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