#![feature(test)]

use std::path::Path;

use image::{DynamicImage, GenericImageView, Rgb};

use fast_wfc::overlapping_wfc::*;
use fast_wfc::utils::vec2d::*;

extern crate test;
use test::Bencher;

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
    let mut i = 0;
    let mut wfc = OverlappingWFC::new(image, options, [i; 16]);
    bencher.iter(|| {
        loop {
            wfc.restart([i; 16]);
            let image = wfc.run();
            if image.is_some() {
                break;
            }
            i += 1;
        }
    });
}

#[bench]
fn bench_flowers_small(bencher: &mut Bencher) {
    let options = OverlappingWFCOptions {
        periodic_input: true,
        periodic_output: true,
        out_height: 21,
        out_width: 21,
        symmetry: 2,
        pattern_size: 3,
        ground: true,
    };

    bench_overlapping(bencher, "images/Flowers.png", options);
}

#[bench]
fn bench_flowers_medium(bencher: &mut Bencher) {
    let options = OverlappingWFCOptions {
        periodic_input: true,
        periodic_output: true,
        out_height: 42,
        out_width: 42,
        symmetry: 2,
        pattern_size: 3,
        ground: true,
    };

    bench_overlapping(bencher, "images/Flowers.png", options);
}

#[bench]
fn bench_flowers_big(bencher: &mut Bencher) {
    let options = OverlappingWFCOptions {
        periodic_input: true,
        periodic_output: true,
        out_height: 63,
        out_width: 63,
        symmetry: 2,
        pattern_size: 3,
        ground: true,
    };

    bench_overlapping(bencher, "images/Flowers.png", options);
}


fn bench_restart(bencher: &mut Bencher, file: &str, options: OverlappingWFCOptions) {
    let image = read_image(file);
    let image = image_to_vec2d(&image);
    let mut wfc = OverlappingWFC::new(image, options, [0; 16]);
    bencher.iter(|| {
        wfc.restart([0; 16]);
    });
}

#[bench]
fn bench_flowers_restart_small(bencher: &mut Bencher) {
    let options = OverlappingWFCOptions {
        periodic_input: true,
        periodic_output: true,
        out_height: 21,
        out_width: 21,
        symmetry: 2,
        pattern_size: 3,
        ground: true,
    };

    bench_restart(bencher, "images/Flowers.png", options);
}

#[bench]
fn bench_flowers_restart_medium(bencher: &mut Bencher) {
    let options = OverlappingWFCOptions {
        periodic_input: true,
        periodic_output: true,
        out_height: 42,
        out_width: 42,
        symmetry: 2,
        pattern_size: 3,
        ground: true,
    };

    bench_restart(bencher, "images/Flowers.png", options);
}

#[bench]
fn bench_flowers_restart_big(bencher: &mut Bencher) {
    let options = OverlappingWFCOptions {
        periodic_input: true,
        periodic_output: true,
        out_height: 63,
        out_width: 63,
        symmetry: 2,
        pattern_size: 3,
        ground: true,
    };

    bench_restart(bencher, "images/Flowers.png", options);
}

