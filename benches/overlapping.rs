#![feature(test)]

use std::env;
use std::path::Path;

use image::{DynamicImage, GenericImageView, Rgb};

use fast_wfc::overlapping_wfc::*;
use fast_wfc::utils::vec2d::*;

extern crate test;
use test::test::Bencher;

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

fn bench_overlapping(bencher: &mut Bencher, file: &str, options: OverlappingWFCOptions) {
    let image = read_image(file);
    let image = image_to_vec2d(&image);

    bencher.iter(|| loop {
        let image = image.clone();
        let options = options.clone();
        let mut wfc = OverlappingWFC::new(image, options, [1; 16]);
        let image = wfc.run();
        if image.is_some() {
            break;
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

    bench_overlapping(bencher, "images/Flowers.png", options);
}
