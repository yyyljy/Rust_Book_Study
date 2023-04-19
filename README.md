# Rust_Book_Study

"만들면서 배우는 러스트 프로그래밍" 책으로 공부하는 Rust 언어

<img src="..\assets\book_img" alt="만들면서 배우는 러스트 프로그래밍 - 예스24" style="zoom: 25%;" />

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
