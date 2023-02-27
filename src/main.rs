use bytes::Bytes;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};
use palette_extract::{
    get_palette_with_options, Color, MaxColors, PixelEncoding, PixelFilter, Quality,
};
use reqwest;

enum ImageType {
    Url,
    File,
}

fn get_image_by_url(url: String) -> Option<Bytes> {
    let req = reqwest::blocking::get(url).unwrap();
    match req.bytes() {
        Ok(res) => Some(res),
        Err(_) => None,
    }
}

fn main() {
    let image_type_input = std::env::args().nth(1).expect("no lang given");
    let image_location = std::env::args().nth(2).expect("no action given");

    let image_type = match image_type_input.as_str() {
        "url" => ImageType::Url,
        "file" => ImageType::File,
        _ => panic!("not supported"),
    };

    let img: DynamicImage;

    match image_type {
        ImageType::Url => {
            let url = image_location;
            if let Some(url_bytes) = get_image_by_url(url) {
                img = image::load_from_memory(&url_bytes).unwrap();
            } else {
                panic!();
            }
        }
        ImageType::File => {
            img = image::open(image_location).unwrap();
        }
    }

    let palette = get_palette_colors(&img);

    let persentage = 20;
    let (width, height) = img.dimensions();
    let extra_y_space = (persentage * height) / 100;
    let palette_x_stepper = width / (palette.len() as u32);

    let dimentions = (width, palette_x_stepper, extra_y_space);

    let multiple_images = get_palette_images(palette, dimentions, 5);

    let mut new_img = DynamicImage::new_rgb8(width, height + extra_y_space);

    match new_img.copy_from(&img, 0, 0) {
        Ok(_) => (),
        Err(_) => println!("error parsing header:"),
    }

    let mut pos = 0;
    for img in multiple_images {
        match new_img.copy_from(&img, pos, height) {
            Ok(_) => (),
            Err(_) => println!("error parsing header:"),
        };

        pos = pos + palette_x_stepper;
    }

    match new_img.save("output.png") {
        Ok(_) => println!("Success output.png"),
        Err(e) => println!("Error! {}", e),
    }
}

fn get_palette_colors(image: &DynamicImage) -> Vec<Color> {
    let pixels = image.as_bytes();
    get_palette_with_options(
        pixels,
        PixelEncoding::Rgb,
        Quality::new(1),
        MaxColors::new(11),
        PixelFilter::None,
    )
}

fn get_width_palette_diff(w_dimension: (u32, u32), index: usize, length: usize) -> u32 {
    let (total_w, color_w) = w_dimension;

    let mut actual_width = color_w;

    let diff = color_w * (length as u32);

    if index + 1 == length {
        if diff < total_w {
            let to_sum = total_w - diff;
            actual_width = color_w + to_sum;
        }
    }
    actual_width
}

fn get_pallete_square_color(
    color: &Color,
    border: u32,
    width: u32,
    height: u32,
    coordinates: (u32, u32),
) -> Rgba<u8> {
    let (x, y) = coordinates;
    let square_color = Rgba([color.r, color.g, color.b, 0]);
    let white: Rgba<u8> = Rgba([255, 255, 255, 0]);
    if x < border || x > width - border {
        white
    } else if y < border || y > height - border {
        white
    } else {
        square_color
    }
}

fn get_palette_images(
    palette: Vec<Color>,
    dimentions: (u32, u32, u32),
    border_width: u32,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let (total_width, color_width, color_height) = dimentions;

    palette
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let actual_width = get_width_palette_diff((total_width, color_width), i, palette.len());

            ImageBuffer::from_fn(actual_width, color_height, |x, y| {
                get_pallete_square_color(c, border_width, actual_width, color_height, (x, y))
            })
        })
        .collect()
}
