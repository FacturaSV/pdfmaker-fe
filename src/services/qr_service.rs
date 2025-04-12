use image::{DynamicImage, GrayImage, ImageError, Luma, RgbImage};
use printpdf::{Image, ImageXObject, ImageTransform, Mm, Px, ColorBits, ColorSpace};
use std::io::Cursor;
use image::codecs::png::PngEncoder;
use image::ImageEncoder;
use std::borrow::Cow;
use image::imageops::FilterType;
use qrcodegen::{QrCode, QrCodeEcc};

/// Genera un código QR y lo escala al tamaño deseado
pub fn generate_qr_code_image(url: &str, module_size: u32) -> DynamicImage {
    let qr = QrCode::encode_text(url, QrCodeEcc::Medium).unwrap();
    let size = qr.size() as u32;

    let img_size = size * module_size; // Asegurar que los módulos sean visibles
    let mut img = GrayImage::from_pixel(img_size, img_size, Luma([255])); // Fondo blanco

    for y in 0..size {
        for x in 0..size {
            let color = if qr.get_module(x as i32, y as i32) { 0 } else { 255 };
            for dy in 0..module_size {
                for dx in 0..module_size {
                    img.put_pixel(x * module_size + dx, y * module_size + dy, Luma([color]));
                }
            }
        }
    }

    DynamicImage::ImageLuma8(img)
}

pub fn generate_qr_code_image2(url: &str, target_size: u32) -> DynamicImage {
    let qr = QrCode::encode_text(url, QrCodeEcc::Medium).unwrap();
    let size = qr.size() as u32;  // Tamaño del QR original

    let img_size = size + 2; // Agregar margen
    let mut img = GrayImage::from_pixel(img_size, img_size, Luma([255])); // Fondo blanco

    for y in 0..size {
        for x in 0..size {
            let color = if qr.get_module(x as i32, y as i32) { 0 } else { 255 };
            img.put_pixel(x + 2, y + 2, Luma([color]));
        }
    }

    DynamicImage::ImageLuma8(img)
}

pub fn convert_image_for_pdf(image: &DynamicImage) -> Result<Image, ImageError> {
    let gray_image = image.to_luma8();

    let (width, height) = gray_image.dimensions();

    let mut buffer = Vec::new();
    for pixel in gray_image.pixels() {
        buffer.push(pixel[0]);
    }

    // Crear `ImageXObject`
    let image_xobj = ImageXObject {
        width: printpdf::Px(width as usize),
        height: printpdf::Px(height as usize),
        color_space: ColorSpace::Greyscale,
        bits_per_component: ColorBits::Bit8,
        interpolate: false,
        image_data: buffer,
        image_filter: None,
        smask: None,
        clipping_bbox: None,
    };

    Ok(Image::from(image_xobj))
}