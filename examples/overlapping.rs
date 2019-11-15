use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::env;
use std::fs::File;
use std::path::Path;
use std::str::from_utf8;

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

fn get_attribute<'a, 'b>(attributes: &'b [Attribute<'a>], key: &str) -> &'b str {
    let v = attributes
        .into_iter()
        .find(|a| a.key == key.as_bytes())
        .map(|a| &a.value)
        .unwrap();
    from_utf8(v).unwrap()
}

fn get_attribute_or<'a, 'b>(
    attributes: &'b [Attribute<'a>],
    key: &str,
    default: &'b str,
) -> &'b str {
    let v = attributes
        .into_iter()
        .find(|a| a.key == key.as_bytes())
        .map_or(default.as_bytes(), |a| &a.value);
    from_utf8(v).unwrap()
}

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let mut reader = Reader::from_file(&file).unwrap();
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Empty(ref c)) => match c.name() {
                b"overlapping" => {
                    let attributes = c.attributes().map(|a| a.unwrap()).collect::<Vec<_>>();
                    let name = &get_attribute(&attributes, "name");
                    let pattern_size = get_attribute_or(&attributes, "N", "3")
                        .parse::<usize>()
                        .unwrap();
                    let periodic_output =
                        get_attribute_or(&attributes, "periodic", "False") == "True";
                    let periodic_input =
                        get_attribute_or(&attributes, "periodicInput", "True") == "True";
                    let ground = get_attribute_or(&attributes, "ground", "0")
                        .parse::<i32>()
                        .unwrap()
                        != 0;
                    let symmetry = get_attribute_or(&attributes, "symmetry", "8")
                        .parse::<usize>()
                        .unwrap();
                    let screenshots = get_attribute_or(&attributes, "screenshots", "2")
                        .parse::<usize>()
                        .unwrap();
                    let out_width = get_attribute_or(&attributes, "width", "48")
                        .parse::<usize>()
                        .unwrap();
                    let out_height = get_attribute_or(&attributes, "height", "48")
                        .parse::<usize>()
                        .unwrap();

                    let options = OverlappingWFCOptions {
                        periodic_input,
                        periodic_output,
                        out_height,
                        out_width,
                        symmetry,
                        pattern_size,
                        ground,
                    };

                    run_example(name, options, screenshots);
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            _ => println!("other"),
        }
    }
}

fn run_example(filename: &str, options: OverlappingWFCOptions, screenshots: usize) {
    println!("{} started!", filename);
    let image = read_image(&(String::from("samples/") + filename + ".png"));
    let image = image_to_vec2d(&image);

    let mut i = 0;
    let mut wfc = OverlappingWFC::new(image.clone(), options, [i; 16]);

    for _ in 0..screenshots {
        let mut result_image = None;
        for _ in 0..10 {
            i += 1;
            wfc.restart([i; 16]);
            result_image = wfc.run();
            if result_image.is_some() {
                break;
            }
            println!("failed!");
        }
        println!("{} finished!", filename);
        if let Some(image) = result_image {
            let image = vec2d_to_image(&image);
            write_to_file(&(String::from("results/") + filename), image);
        }
    }
}
