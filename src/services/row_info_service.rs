use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use printpdf::path::{PaintMode, WindingOrder};
use serde_json::Value;
use crate::routes::Pdf::{Emisor, Receptor};

/// Dibuja la sección de información del cliente en el PDF
pub fn draw_client_info_section(
    receptor: &Receptor,
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    fontBlod: &IndirectFontRef,
    margin: Mm,
    current_y: Mm,
    width: Mm,
    usable_width: Mm,
    client_info_height: Mm,
    background_color: &Color,
    text_color: &Color,
) -> Mm {
    // 🔹 **Dibujar el fondo de la sección de información del cliente**
    let client_info_rect = Polygon {
        rings: vec![vec![
            (Point::new(margin, current_y), false),
            (Point::new(width - margin, current_y), false),
            (Point::new(width - margin, current_y - client_info_height), false),
            (Point::new(margin, current_y - client_info_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(background_color.clone());
    layer.add_polygon(client_info_rect);

    // 🔹 **Definir las posiciones de texto**
    let text_x_left = margin + Mm(2.0);
    let text_x_right = margin + (usable_width / 2.0) + Mm(20.0);
    let mut text_y = current_y - Mm(3.0);

    // 🔹 **Lista de datos del cliente**
    let key_value_pairs: Vec<(&str, &str)> = vec![
        ("NOMBRE", &receptor.nombre),
        ("ACTIVIDAD", &receptor.descActividad),
        ("DIRECCIÓN", &receptor.direccion.complemento),
        ("REFERENCIAS", ""),
        ("TIPO Y Nº DOCUMENTO", &receptor.numDocumento),
        ("NRC", receptor.nrc.as_deref().unwrap_or("")),
        ("TELÉFONO", &receptor.telefono),
        ("CORREO", &receptor.correo),
    ];


    for i in 0..(key_value_pairs.len() / 2) {
        let (key_left, value_left) = &key_value_pairs[i];
        let (key_right, value_right) = &key_value_pairs[i + (key_value_pairs.len() / 2)];

        layer.set_fill_color(text_color.clone());
        layer.use_text(format!("{}:", key_left), 6.0, text_x_left, text_y, fontBlod);

        let offset_left = key_left.len() as f64 * 3.2; // ajusta si lo ves muy largo o corto
        layer.use_text(format!("{}", value_left), 6.0, Mm(27.0), text_y, font);

        layer.set_fill_color(text_color.clone());
        layer.use_text(format!("{}:", key_right), 6.0, text_x_right, text_y, fontBlod);

        let offset_right = key_right.len() as f64 * 3.2;
        layer.use_text(format!("{}", value_right), 6.0, text_x_right +  Mm(27.0), text_y, font);

        text_y -= Mm(4.0);
    }


    // 🔹 **Reducir la altura disponible después de escribir en el bloque**
    current_y - client_info_height
}

pub fn draw_other_info_section(
    layer: &PdfLayerReference,
    margin: Mm,
    current_y: Mm,
    width: Mm,
    other_info_height: Mm,
    background_color: &Color,
) -> Mm {
    // 🔹 **Dibujar el fondo de la sección**
    let other_info_rect = Polygon {
        rings: vec![vec![
            (Point::new(margin, current_y), false),
            (Point::new(width - margin, current_y), false),
            (Point::new(width - margin, current_y - other_info_height), false),
            (Point::new(margin, current_y - other_info_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(background_color.clone());
    layer.add_polygon(other_info_rect);

    // 🔹 **Reducir la altura disponible después de escribir en el bloque**
    current_y - other_info_height
}

pub fn draw_other_docs_section(
    layer: &PdfLayerReference,
    margin: Mm,
    current_y: Mm,
    width: Mm,
    other_docs_height: Mm,
    background_color: &Color,
) -> Mm {
    // 🔹 **Dibujar el fondo de la sección**
    let other_docs_rect = Polygon {
        rings: vec![vec![
            (Point::new(margin, current_y), false),
            (Point::new(width - margin, current_y), false),
            (Point::new(width - margin, current_y - other_docs_height), false),
            (Point::new(margin, current_y - other_docs_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(background_color.clone());
    layer.add_polygon(other_docs_rect);

    // 🔹 **Reducir la altura disponible después de escribir en el bloque**
    current_y - other_docs_height
}