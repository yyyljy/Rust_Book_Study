fn main() {
    let year = 2023;
    let month = 12;
    let day = 1;
    println!("KR:{0}/{1}/{2}", year, month, day);
    println!("US:{1}/{2}/{0}", year, month, day);
    println!("UK:{2}/{1}/{0}", year, month, day);
    println!("{yy}년 {mm}월 {dd}일", yy=year, mm=month, dd=day);
}