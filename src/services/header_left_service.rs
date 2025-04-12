use std::fs::File;
use printpdf::*;
use printpdf::path::{PaintMode, WindingOrder};
use std::io::{BufReader, BufWriter, Cursor, Error as IoError, ErrorKind, Result};
use std::path::Path;
use ::image::io::Reader as ImageReader;
use ::image::{codecs::jpeg::JpegEncoder, DynamicImage,ColorType, GenericImageView, imageops};
use ::image::ExtendedColorType;
use std::error::Error;
use crate::routes::Pdf::{DteJson, Emisor};

pub fn create_shape_logo(
    layer: &PdfLayerReference,
    start_x: Mm,
    start_y: Mm,
    width: Mm,
    height: Mm,
    fill_color: &Color,
    logo_path: &str,
    document: &PdfDocumentReference,
) -> std::result::Result<Mm, Box<dyn Error>> {
    // Dibujar el contenedor del logo
    let shape = Polygon {
        rings: vec![vec![
            (Point::new(start_x, start_y), false),
            (Point::new(start_x + width, start_y), false),
            (Point::new(start_x + width, start_y - height), false),
            (Point::new(start_x, start_y - height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(fill_color.clone());
    layer.add_polygon(shape);

    // 📌 Verificar si la imagen ya está optimizada
    let optimized_logo_path = format!("{}_optimized.jpg", logo_path);
    if !Path::new(&optimized_logo_path).exists() {
        compress_image(logo_path, &optimized_logo_path, 50)?; // Reducción de calidad para menos peso
    }

    let img = ImageReader::open(&optimized_logo_path)?
        .decode()? // Decodifica la imagen a un formato usable
        .resize(250, 250, imageops::FilterType::Lanczos3)
        .to_rgb8();

    let (img_width, img_height) = img.dimensions();
    let mut img_buffer = Vec::new();
    for pixel in img.pixels() {
        img_buffer.extend_from_slice(&pixel.0);
    }

    let image_xobj = ImageXObject {
        width: Px(img_width as usize),
        height: Px(img_height as usize),
        color_space: ColorSpace::Rgb,
        bits_per_component: ColorBits::Bit8, // 🔹 Reducción de bits para menor peso
        interpolate: false,
        image_data: img_buffer,
        image_filter: None,  // 🔹 Evitar filtros que `printpdf` no soporte
        smask: None,
        clipping_bbox: None,
    };

    let image = Image::from(image_xobj);

    // 📌 Ajustar margen y posicionamiento
    let margin_mm = Mm(2.0);
    let available_width = width - (margin_mm * 4.0);
    let available_height = height - (margin_mm * 1.5);
    let image_x = start_x.0 + margin_mm.0;
    let image_y = start_y.0 - margin_mm.0 - available_height.0;

    image.add_to_layer(
        layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(image_x)),
            translate_y: Some(Mm(image_y)),
            rotate: None,
            scale_x: Some(0.35), // 🔹 Reducción mayor para menos peso
            scale_y: Some(0.35),
            dpi: Some(60.0), // 🔹 Baja resolución para PDFs ligeros
        },
    );

    Ok(start_y - height)
}


fn compress_image(input_path: &str, output_path: &str, quality: u8) -> Result<()> {
    let img = ImageReader::open(input_path)
        .map_err(|e| IoError::new(ErrorKind::Other, format!("Error al abrir la imagen: {}", e)))?
        .decode()
        .map_err(|e| IoError::new(ErrorKind::Other, format!("Error al decodificar la imagen: {}", e)))?;

    let mut output_file = BufWriter::new(File::create(output_path)?);

    let mut encoder = JpegEncoder::new_with_quality(&mut output_file, quality);
    encoder
        .encode(
            img.to_rgb8().as_raw(),
            img.width(),
            img.height(),
            ExtendedColorType::from(ColorType::Rgb8),
        )
        .map_err(|e| IoError::new(ErrorKind::Other, format!("Error al escribir la imagen: {}", e)))?;

    Ok(())
}
pub fn draw_empresa_nombre_section(
    emisor: &Emisor,
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    fontBlod: &IndirectFontRef,
    start_x: Mm,
    start_y: Mm,
    width: Mm,
    height: Mm,
    fill_color: &Color,
    text_color: &Color,
) -> Mm {
    // Dibujar el rectángulo de fondo
    let empresa_nombre_shape = Polygon {
        rings: vec![vec![
            (Point::new(start_x, start_y), false),
            (Point::new(start_x + width, start_y), false),
            (Point::new(start_x + width, start_y - height), false),
            (Point::new(start_x, start_y - height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(fill_color.clone()); // Color de fondo
    layer.add_polygon(empresa_nombre_shape);

    // Posición del texto
    let text_x = (width / 2.0) - Mm(35.0); // Centrado horizontalmente
    let mut text_y = start_y - (height / 3.5); // Ajustado verticalmente

    layer.set_fill_color(text_color.clone()); // Color del texto
    layer.use_text(&emisor.nombre, 9.0, text_x, text_y, fontBlod);

    text_y -= Mm(5.0);
    layer.use_text(&emisor.nombreComercial, 7.0, text_x, text_y, font);

    start_y - height // Retorna la nueva posición de `current_y`
}


/// Dibuja la sección de información de la empresa emisora
pub fn draw_empresa_emisora_section(
    emisor: &Emisor,
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    fontBlod: &IndirectFontRef,
    margin: Mm,
    current_y: Mm,
    left_width: Mm,
    sub_left_empresa_emisora_height: Mm,
    background_color: &Color,
    text_color: &Color,
) -> Mm {
    // 🔹 **Dibujar el fondo de la sección de la empresa emisora**
    let sub_left_empresa_emisora = Polygon {
        rings: vec![vec![
            (Point::new(margin, current_y), false),
            (Point::new(margin + left_width, current_y), false),
            (Point::new(margin + left_width, current_y - sub_left_empresa_emisora_height), false),
            (Point::new(margin, current_y - sub_left_empresa_emisora_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(background_color.clone());
    layer.add_polygon(sub_left_empresa_emisora);

    // 🔹 **Definir la posición del texto**
    let text_x = margin + Mm(2.0); // Un poco de margen desde el borde izquierdo
    let mut text_y = current_y - Mm(3.0); // Baja un poco para evitar tocar el borde superior

    // 🔹 **Lista de datos de la empresa emisora**
    let emisor_info = vec![
        ("NIT", &emisor.nit),
        ("NRC", &emisor.nrc),
        ("COD ACTIVIDAD", &emisor.codActividad),
        ("ACTIVIDAD", &emisor.descActividad),
        ("DIRECCIÓN", &emisor.direccion.complemento),
        ("TELÉFONO", &emisor.telefono),
        ("CORREO", &emisor.correo),
    ];

    for (key, value) in emisor_info.iter() {
        layer.set_fill_color(text_color.clone());

        // Dibuja la clave (negrita)
        layer.use_text(format!("{}:", key), 6.0, text_x, text_y, fontBlod);

        // Dibuja el valor (regular), ajustando la posición horizontal si quieres alinear
        layer.use_text(format!(" {}", value), 6.0, text_x + Mm(18.0), text_y, font);

        text_y -= Mm(3.5);
    }

    // 🔹 **Reducir la altura disponible después de escribir en el bloque**
    current_y - sub_left_empresa_emisora_height
}
