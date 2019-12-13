use image::RgbaImage;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::fmt;
use termion::color;

/// A function that converts an image to ASCII using
/// a provided scale and a set of characters to use.
///
/// # Arguments
///
/// * `img` - The image to convert
/// * `scale` - The scale of the image
/// * `chars` - The characters to use
///
pub fn image_to_ascii(
    target: &mut impl fmt::Write,
    img: RgbaImage,
    step_size: usize,
    chars: &Vec<char>,
    fill: Option<char>,
) -> fmt::Result {
    let (w, h) = img.dimensions();
    let bar = ProgressBar::new(100).with_style(ProgressStyle::default_bar().progress_chars("##-"));

    for y in (0..h).step_by(step_size) {
        for x in (0..w).step_by(step_size) {
            let px = img.get_pixel(x, y);
            let (r, g, b, a) = (px[0], px[1], px[2], px[3]);
            let c = num_to_char(a, chars);
            let fill = fill.unwrap_or(c);
            write!(target, "{}{}{}", color::Fg(color::Rgb(r, g, b)), fill, c)?;
        }
        bar.set_position((100.0 * (y as f64 / h as f64)) as u64);
        writeln!(target)?;
    }
    bar.finish_and_clear();

    let reset = termion::color::Reset;
    write!(target, "{}", reset.fg_str())
}

fn num_to_char(avg: u8, chars: &Vec<char>) -> char {
    let ratio = avg as f64 / 256.0;
    let len = chars.len();
    let index = ratio * len as f64;
    chars[index as usize]
}
