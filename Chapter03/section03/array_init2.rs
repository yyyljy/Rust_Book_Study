fn main() {
    let points:[i32;5] = [10, 20, 40 ,50 ,60];
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