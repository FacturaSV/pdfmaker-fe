use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use printpdf::path::{PaintMode, WindingOrder};
use crate::services::table_service;
use serde_json::Value;

pub fn draw_footer_section(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    margin: Mm,
    current_y: Mm,
    width: Mm,
    footer_height: Mm,
    background_color: &Color,
    text_color: &Color,
) -> Mm {
    // 🔹 **Dibujar el fondo del footer**
    let footer_rect = Polygon {
        rings: vec![vec![
            (Point::new(margin, current_y), false),
            (Point::new(width - margin, current_y), false),
            (Point::new(width - margin, current_y - footer_height), false),
            (Point::new(margin, current_y - footer_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(background_color.clone());
    layer.add_polygon(footer_rect);

    // 📌 **Texto a colocar en el footer**
    let footer_text = "Este documento tributario electrónico ha sido generado a través de la plataforma FacturaLink para la empresa X";

    // 🔹 **Calcular ancho del texto para centrarlo correctamente**
    let avg_char_width = 4.0 * 0.4; // Aproximación del ancho de cada carácter (40% del tamaño de la fuente)
    let text_width = footer_text.len() as f32 * avg_char_width;

    // 🔹 **Mover el texto más a la derecha**
    let offset_right = 55.0; // Ajusta este valor según necesites
    let text_x = ((width.0 - text_width) / 2.0) + offset_right;
    let text_y = current_y.0 - (footer_height.0 / 2.5); // Centrar verticalmente

    // 🔹 **Asegurar que el texto es del color correcto**
    layer.set_fill_color(text_color.clone());

    // 📌 **Agregar el texto sobre el fondo del footer**
    layer.use_text(footer_text, 4.0, Mm(text_x), Mm(text_y), font);

    // 🔹 **Reducir la altura disponible después de escribir en el bloque**
    current_y - footer_height
}