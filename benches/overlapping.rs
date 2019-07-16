#![feature(test)]

use std::env;
use std::path::Path;

use image::{GenericImageView, Rgb, DynamicImage, ImageBuffer};

use fast_wfc::overlapping_wfc::*;
use fast_wfc::utils::vec2d::*;

extern crate test;
use test::test::Bencher;

fn read_image(filepath: &str) -> DynamicImage {
    image::open(&Path::new(&filepath)).unwrap()
}

fn image_to_vec2d(image: &DynamicImage) -> Vec2D<Rgb<u8>> {
    let mut image_vec2d = Vec2D::new(image.height() as usize, image.width() as usize, &Rgb{data: [0,0,0]});

    for (x, y, pixel) in image.pixels() {
        image_vec2d[y as usize][x as usize] = Rgb { data: [pixel[0], pixel[1], pixel[2]]};
    }

    image_vec2d
}

fn vec2d_to_image(image: &Vec2D<Rgb<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
    ImageBuffer::from_fn(image.width() as u32, image.height() as u32, |x, y| {
        image[y as usize][x as usize]
    })
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
        periodic_input: false,
        periodic_output: false,
        out_height: 42,
        out_width: 42,
        symmetry: 2,
        pattern_size: 3,
    };

    let mut wfc = OverlappingWFC::new(image, options, [1; 16]);
    let image = wfc.run().unwrap();
}

fn bench_overlapping(bencher: &mut Bencher, file: &str, options: OverlappingWFCOptions) {
    let image = read_image("benches/images/Flowers.png");
    let image = image_to_vec2d(&image);


    bencher.iter(|| {
        loop {
            let image = image.clone();
            let options = options.clone();
            let mut wfc = OverlappingWFC::new(image, options, [1; 16]);
            let image = wfc.run();
            if image.is_some() {
                break;
            }
        }
    });

}

#[bench]
fn bench_flowers(bencher: &mut Bencher) {
    let options = OverlappingWFCOptions {
        periodic_input: false,
        periodic_output: false,
        out_height: 21,
        out_width: 21,
        symmetry: 2,
        pattern_size: 3,
    };

    bench_overlapping(bencher, "benches/images/Flowers.png", options);
}
