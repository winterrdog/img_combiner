// declare modules 4 usage.
mod args;
mod errors;
mod float_img;

use args::ImgArgs;
use errors::ImgDataErrors;
use float_img::FloatingImage;
use image::{
    self, imageops::CatmullRom, io::Reader as ImgReader, ColorType, DynamicImage, GenericImageView,
    ImageFormat,
};

fn find_img_from_path(path: &str) -> (DynamicImage, ImageFormat) {
    let img_reader = ImgReader::open(path).unwrap();
    let img_format = img_reader.format().unwrap();
    let img = img_reader.decode().unwrap();

    (img, img_format)
}

fn get_smaller_img(dim_a: (u32, u32), dim_b: (u32, u32)) -> (u32, u32) {
    let pix_a = dim_a.0 * dim_a.1;
    let pix_b = dim_b.0 * dim_b.1;

    if pix_a < pix_b {
        dim_a
    } else {
        dim_b
    }
}

fn standardize_size(img_a: DynamicImage, img_b: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smaller_img(img_a.dimensions(), img_b.dimensions());
    println!(
        "Smaller image's dimensions.\nwidth: {} pixels, height: {} pixels\n",
        width, height
    );

    // Make the bigger image fit the smaller image
    if img_a.dimensions() == (width, height) {
        (img_a, img_b.resize_exact(width, height, CatmullRom))
    } else {
        (img_a.resize_exact(width, height, CatmullRom), img_b)
    }
}

fn combine_imgs(img_a: DynamicImage, img_b: DynamicImage) -> Vec<u8> {
    let vec_a = img_a.to_rgba8().into_vec();
    let vec_b = img_b.to_rgba8().into_vec();

    alternate_pixels(vec_a, vec_b)
}

fn alternate_pixels(vec_a: Vec<u8>, vec_b: Vec<u8>) -> Vec<u8> {
    let mut combined_data = vec![0u8; vec_a.len()];

    let mut start_index = 0usize;
    while start_index < vec_a.len() {
        let end_index = start_index + 3;

        if start_index % 8 == 0 {
            combined_data.splice(
                start_index..=end_index,
                set_rgba(&vec_a, start_index, end_index),
            );
        } else {
            combined_data.splice(
                start_index..=end_index,
                set_rgba(&vec_b, start_index, end_index),
            );
        }

        start_index += 4;
    }

    combined_data
}

fn set_rgba(vec: &[u8], start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::<u8>::new();

    for i in start..=end {
        let val = match vec.get(i) {
            Some(expr) => *expr,
            None => panic!("[-] index out of bounds."),
        };

        rgba.push(val);
    }

    rgba
}

fn main() -> Result<(), ImgDataErrors> {
    let cmdline_args = ImgArgs::new();

    let (img_1, img_1_fmt) = find_img_from_path(&cmdline_args.img_1);
    let (img_2, img_2_fmt) = find_img_from_path(&cmdline_args.img_2);

    // make sure img formats are similar
    if img_1_fmt != img_2_fmt {
        return Err(ImgDataErrors::DifferentImgFormats);
    }

    // make both images have the same dimensions
    let (img_1, img_2) = standardize_size(img_1, img_2);

    // setup the output image
    let mut out_img = FloatingImage::new(img_1.width(), img_1.height(), cmdline_args.output);
    let combined_data = combine_imgs(img_1, img_2);
    out_img.set_data(combined_data)?;

    image::save_buffer_with_format(
        out_img.name,
        out_img.data.as_slice(),
        out_img.width,
        out_img.height,
        ColorType::Rgba8,
        img_1_fmt,
    )
    .unwrap();

    Ok(())
}
