use printpdf::*;
use printpdf::path::{PaintMode, WindingOrder};
use crate::services::table_service;
use crate::routes::Pdf::{DteJson, ItemDocumento};

pub fn draw_products_section(
    items: &[ItemDocumento], // <- solo los productos de esta "página"
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    margin: Mm,
    current_y: Mm,
    width: Mm,
    products_height: Mm,
    background_color: &Color,
) -> Mm {
    // 🔹 **Dibujar el fondo de la sección de productos**
    let products_rect = Polygon {
        rings: vec![vec![
            (Point::new(margin, current_y), false),
            (Point::new(width - margin, current_y), false),
            (Point::new(width - margin, current_y - products_height), false),
            (Point::new(margin, current_y - products_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(background_color.clone());
    layer.add_polygon(products_rect);

    // 🔹 **Definir las coordenadas para la tabla de productos**
    let start_x = Mm(10.0);
    let start_y = current_y - Mm(0.0); // Ajuste para la tabla dentro de la sección

    let column_widths = vec![
        Mm(8.0), Mm(10.0), Mm(18.0), Mm(26.0), Mm(64.0),
        Mm(14.0), Mm(14.0), Mm(14.0), Mm(14.0), Mm(14.0),
    ];

    // 🔹 **Generar la tabla de productos**
    table_service::generate_product_table(items, layer, font, start_x, start_y, &column_widths);

    // 🔹 **Reducir la altura disponible después de escribir en el bloque**
    current_y - products_height
}