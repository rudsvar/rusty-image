use log::*;
use std::fmt;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "An application for turning images into ASCII")]
struct Opt {
    /// Input image
    ///
    /// The image to convert to ASCII.
    #[structopt(parse(from_os_str))]
    image: PathBuf,

    /// Image scale
    ///
    /// The factor to multiply the image size with.
    /// If not specified, it will attempt to fit the terminal.
    #[structopt(short, long)]
    scale: Option<f64>,

    /// The characters to use to represent change in alpha.
    #[structopt(short, long)]
    chars: Option<Vec<char>>,

    /// A filler character to extend the image width
    ///
    /// By default this is the same as the neighboring characters.
    #[structopt(short, long)]
    fill: Option<char>,

    /// Silence all output
    #[structopt(short, long)]
    quiet: bool,

    /// Verbose mode (-v, -vv, -vvv, etc)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: usize,
}

fn main() -> fmt::Result {
    let opt = Opt::from_args();

    stderrlog::new()
        .quiet(opt.quiet)
        .verbosity(opt.verbose + 1)
        .init()
        .unwrap();

    // Open the provided image
    match image::open(&opt.image) {
        Ok(img) => {
            let img = img.to_rgba();
            let chars = opt.chars.unwrap_or(" .-%#@".chars().collect());

            // Find the step size based on:
            // 1. The provided scale if there is one
            // 2. The terminal size
            // 3. A default if none exist
            let step_size = if let Ok((tw, th)) = termion::terminal_size() {
                let (w, h) = img.dimensions();
                if let Some(scale) = opt.scale {
                    (1.0 / scale) as usize
                } else {
                    usize::max(
                        1,
                        usize::max(
                            ((w * 2) as f64 / (tw - 1) as f64).ceil() as usize,
                            (h as f64 / th as f64).ceil() as usize,
                        ),
                    )
                }
            } else {
                let default = 0.01;
                warn!("Could not find scale, defaulting to {}", default);
                (1.0 / default) as usize
            };

            let mut buf = String::new();
            rusty_image::image_to_ascii(&mut buf, img, step_size, &chars, opt.fill)?;
            println!("{}", buf);
        }
        Err(e) => error!("Could not open image: {}", e),
    };

    Ok(())
}
