#![allow(unused_parens)]

use std::path::PathBuf;

use ab_glyph::FontVec;
use clap::Parser;
use image::{DynamicImage, GenericImage, Rgb};
use imageproc::{drawing::text_size, rect::Rect};

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[arg(help = "The image to meme")]
    image: PathBuf,
    #[arg(help = "where to put the final image")]
    output: PathBuf,
    #[arg(short, long, help = "The text to put bellow the image")]
    text: Vec<String>,
    #[arg(short, long, help = "Override default font")]
    font: Option<PathBuf>,
}

const PADDING: u32 = 50;
const TEXT_MARGIN: u32 = 30;
const TEXT_SIZE: u32 = 60;
const BORDER_THICKNESS: u32 = 5;
const WHITE: Rgb<u8> = Rgb([0xff, 0xff, 0xff]);
fn main() {
    let args = Args::parse();
    let mut image = image::open(args.image)
        .unwrap_or_else(|e| {
            eprintln!("Failed to open original image file: {e}");
            std::process::exit(1);
        });
    let texts = if args.text.is_empty() { vec!["WHAT?".to_string()] } else { args.text };
    for text in texts {
        image = frame_image(&image, &text, args.font.clone());
    }
    image.save(args.output).unwrap_or_else(|e| {
        eprintln!("Failed to save output image: {e}");
        std::process::exit(1);
    });
}


pub fn frame_image(original_image: &DynamicImage, text: &str, font_path: Option<PathBuf>) -> DynamicImage {
    let mut output_image = image::RgbImage::new(
        PADDING + original_image.width() + PADDING,
        PADDING + original_image.height() + TEXT_MARGIN + TEXT_SIZE + TEXT_MARGIN,
    );
    for i in 0..BORDER_THICKNESS {
        imageproc::drawing::draw_hollow_rect_mut(
            &mut output_image,
            Rect::at((PADDING - i) as i32, (PADDING - i) as i32).of_size(
                original_image.width() + i * 2,
                original_image.height() + i * 2,
            ),
            WHITE,
        );
    }
    output_image
        .copy_from(&original_image.to_rgb8(), PADDING, PADDING)
        .expect("original image must fit the final image");
    let font_bytes = font_path.map(|path| {
            std::fs::read(path).unwrap_or_else(|e| {
                eprintln!("Failed to open font file: {e}");
                std::process::exit(1);
            })
        })
        .unwrap_or_else(|| include_bytes!("../FiraCode-Medium.ttf").to_vec());
    let font = FontVec::try_from_vec(font_bytes).unwrap_or_else(|e| {
        eprintln!("Failed to parse font file: {e}");
        std::process::exit(1);
    });
    let text_x = output_image.width() / 2 - text_size(TEXT_SIZE as f32, &font, text).0 / 2;
    imageproc::drawing::draw_text_mut(
        &mut output_image,
        WHITE,
        text_x as i32,
        (PADDING + original_image.height() + TEXT_MARGIN) as i32,
        TEXT_SIZE as f32,
        &font,
        text,
    );
    output_image.into()
}