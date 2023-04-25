fn main() {
    let a = 'a';
    let b = b'a';
    let c = '\x61';
    println!("{}, {:2x}, {}",a,b,c);

    let d = '곰';
    let e = '곰' as u32;
    let f = '\u{acf0}';
    println!("{}, {:4x}, {}",d, e, f);

    let v1 = 0xFF;
    let v2 = 0o655;
    let v3 = 0b1101_0101;
    println!("{}, {}, {}", v1, v2, v3);

    let f1 = 10.5;
    let f2 = 10.5f32;
    let f3 = 10.5e+8;
    println!("{}, {}, {}", f1, f2, f3);
}