extern crate image;

use image::RgbImage;
use image::Rgb;

use std::env::args;
use std::process::exit;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!("Not enough arguments, please provide a file and scale.");
        exit(1);
    }

    let scale = match args[2].parse() {
        Ok(s) => s,
        Err(_) => {
            let default = 0.01;
            println!("Invalid scale, defaulting to {}", default);
            default
        },
    };

    match image::open(&args[1]) {
        Ok(img) => {
            let img = img.to_rgb();
            let chars = ".;+%#".chars().collect();
            let ascii = to_ascii(img, scale, &chars);
            println!("{}", ascii);
        },
        Err(e) => println!("Could not open image: {}", e),
    };
}

fn to_ascii(img: RgbImage, scale: f64, chars: &Vec<char>) -> String {
    let mut res = String::new();
    res.push_str("\x1b[7l");

    let (w, h) = img.dimensions();
    let step_size = (1.0 / scale) as u32;

    for y in range(0, h, step_size) {
        for x in range(0, w, step_size) {
            let px = img.get_pixel(x,y);
            let (r, g, b) = rgb(&px);
            let avg = ((r+g+b)/3) as f64;
            let c = num_to_char(avg, chars);
            res.push_str(&format!("\x1b[38;2;{};{};{}m,{}", r, g, b, c));

            let percent = (y*w + x) as f32 / (w*h) as f32;
            print!("{:.1}%\r", percent*100.0);
        }
        res.push('\n');
    }

    res.push_str("\x1b[?7h");
    return res;
}

fn range(start: u32, end: u32, sz: u32) -> Vec<u32> {
    (start .. end)
        .filter(|x| x % sz == 0)
        .collect()
}

fn rgb(px: &Rgb<u8>) -> (i32, i32, i32) {
    (px[0] as i32, px[1] as i32, px[2] as i32)
}

fn num_to_char(avg: f64, chars: &Vec<char>) -> char {
    let ratio = avg/256.0;
    let len = chars.len() as f64;
    let index = ratio * len;
    chars[index as usize]
}
