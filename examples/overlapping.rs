use std::env;
use std::fs::File;
use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

use fast_wfc::overlapping_wfc::*;
use fast_wfc::utils::vec2d::*;

fn read_image(filepath: &str) -> DynamicImage {
    image::open(&Path::new(&filepath)).unwrap()
}

fn image_to_vec2d(image: &DynamicImage) -> Vec2D<Rgb<u8>> {
    let mut image_vec2d = Vec2D::new(
        image.height() as usize,
        image.width() as usize,
        &Rgb { data: [0, 0, 0] },
    );

    for (x, y, pixel) in image.pixels() {
        image_vec2d[y as usize][x as usize] = Rgb {
            data: [pixel[0], pixel[1], pixel[2]],
        };
    }

    image_vec2d
}

fn vec2d_to_image(image: &Vec2D<Rgb<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(image.width() as u32, image.height() as u32, |x, y| {
        image[y as usize][x as usize]
    })
}

fn write_to_file(file: &str, image: ImageBuffer<Rgb<u8>, Vec<u8>>) {
    let image = DynamicImage::ImageRgb8(image);
    let fout = &mut File::create(&Path::new(file)).unwrap();
    image.write_to(fout, image::PNG).unwrap();
}

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };
;
    let image = read_image(&file);
    let image = image_to_vec2d(&image);

    let options = OverlappingWFCOptions {
        periodic_input: true,
        periodic_output: true,
        out_height: 42,
        out_width: 42,
        symmetry: 2,
        pattern_size: 3,
        ground: true,
    };

    let mut i = 0;
    let mut result_image = None;
    let mut wfc = OverlappingWFC::new(image.clone(), options, [i; 16]);
    while result_image.is_none() {
        i += 1;
        wfc.restart([i; 16]);
        result_image = wfc.run();
    }
    let image = result_image.unwrap();

    let image = vec2d_to_image(&image);
    write_to_file("result.png", image);
}
