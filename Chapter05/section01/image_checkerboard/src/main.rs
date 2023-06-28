// use image::{Rgb, ImageBuffer};
// fn main() {
//     let white = Rgb::<u8>([255, 255, 255]);
//     let red = Rgb::<u8>([255, 90, 90]);
//     let w = 64;
//     let draw = |x, y| {
//         let (xi, yi) = (x / w, y / w);
//         match (xi % 2, yi % 2) {
//             (0, 0) => white,
//             (1, 0) => red,
//             (0, 1) => red,
//             (1, 1) => white,
//             (_, _) => panic!("error"),
//         }
//     };
//     let img = ImageBuffer::from_fn(512, 512, draw);
//     img.save("checkerboard.png").unwrap();
// }

// *************************** //
// RGB CHECKER
// *************************** //

// fn main() {
//     let red = Rgb::<u8>([255, 90, 90]);
//     let green = Rgb::<u8>([90, 255, 90]);
//     let blue = Rgb::<u8>([90, 90, 255]);
//     let w = 64;
//     let draw = |x, y| {
//         let (xi, yi) = (x / w, y / w);
//         match (xi % 3, yi % 3) {
//             (0, 0) => red,
//             (1, 0) => green,
//             (2, 0) => blue,
//             (0, 1) => green,
//             (1, 1) => blue,
//             (2, 1) => red,
//             (0, 2) => blue,
//             (1, 2) => red,
//             (2, 2) => green,
//             (_, _) => panic!("error"),
//         }
//     };
//     let img = ImageBuffer::from_fn(512, 512, draw);
//     img.save("RGBcheckerboard.png").unwrap();
// }

// *************************** //
// IMAGE THUMB
// *************************** //

// use image::{self, imageops, GenericImageView};
// fn main() {
//     let size = 128;
//     let args: Vec<String> = std::env::args().collect();
//     if args.len() < 2 {
//         println!("[USAGE] image_thumb imagefile");
//         return;
//     }
//     let infile = String::from(&args[1]);
//     let file_name: Vec<&str> = infile.split(".").collect();
//     let outfile = format!("{}-thumb.png", file_name[0]);
//     println!("input: {}", infile);
//     println!("output: {}", outfile);

//     let mut img = image::open(infile)
//         .expect("파일을 읽어올 수 없습니다.");
//     let dim = img.dimensions();
//     let w = if dim.0 > dim.1 {dim.1} else {dim.0};
//     let mut img2 = imageops::crop(&mut img,
//         (dim.0-w)/2, (dim.1-w)/2, w, w).to_image();
//     let img3 = imageops::resize(&mut img2, size, size, 
//         imageops::Lanczos3);
//     img3.save(outfile).unwrap(); 
// }

// *************************** //
// IMAGE FILTER
// *************************** //

use image::{GenericImage, GenericImageView, Rgba};
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] image_filter imagefile");
        return;
    }
    let infile = args[1].clone();
    let file_name: Vec<&str> = infile.split(".").collect();
    // let outfile = format!("{}-out.jpg", file_name[0]);
    // let outfile = format!("{}-Rout.jpg", file_name[0]);
    // let outfile = format!("{}-Gout.jpg", file_name[0]);
    let outfile = format!("{}-Bout.jpg", file_name[0]);
    println!("infile={}", infile);
    println!("outfile={}", outfile);

    let mut img = image::open(infile).expect("파일 없음");
    let (w, h) = img.dimensions();
    for y in 0..h {
        for x in 0..w {
            let c: Rgba<u8> = img.get_pixel(x, y);
            let c = Rgba([
                // c[0],
                255 - c[0],
                // c[1],
                255 - c[1],
                c[2],
                // 255 - c[2],
                c[3],      
            ]);
            img.put_pixel(x, y, c);
        }
    }
    img.save(outfile).unwrap();
}