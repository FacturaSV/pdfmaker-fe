use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use printpdf::path::{PaintMode, WindingOrder};
use crate::services::table_service;
use serde_json::Value;
use crate::routes::Pdf::DteJson;

/// Dibuja la parte izquierda del footer (70% del ancho)
pub fn draw_footer_left_section(
    dteJson: &DteJson,
    layer: &PdfLayerReference,
    margin: Mm,
    current_y: Mm,
    left_bottom_width: Mm,
    bottom_height: Mm,
    fill_color: &Color,
) {
    let footer_left = Polygon {
        rings: vec![vec![
            (Point::new(margin, current_y), false),
            (Point::new(margin + left_bottom_width, current_y), false),
            (Point::new(margin + left_bottom_width, current_y - bottom_height), false),
            (Point::new(margin, current_y - bottom_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(fill_color.clone());
    layer.add_polygon(footer_left);
}

pub fn draw_footer_right_section(
    dteJson: &DteJson,
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    margin: Mm,
    mut current_y: Mm,
    left_bottom_width: Mm,
    width: Mm,
    bottom_height: Mm,
    sub_color: &Color,  // Color del último bloque (TOTAL A PAGAR)
    fill_color: &Color, // Color de los otros bloques
    text_color: &Color, // Color del texto normal
    highlight_text_color: &Color, // Color del texto de TOTAL A PAGAR
) {
    let section_width = width - margin - (margin + left_bottom_width);

    // 🔹 **Primera sección (Total Operaciones)**
    let first_section_height = Mm(7.0);
    let first_section = Polygon {
        rings: vec![vec![
            (Point::new(margin + left_bottom_width, current_y), false),
            (Point::new(width - margin, current_y), false),
            (Point::new(width - margin, current_y - first_section_height), false),
            (Point::new(margin + left_bottom_width, current_y - first_section_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };
    layer.set_fill_color(fill_color.clone());
    layer.add_polygon(first_section);

    let text_x_base = margin + left_bottom_width + Mm(3.0);
    let text_y_base = current_y - Mm(2.5);

    // 🔹 **Texto de "Total operaciones" en dos líneas**
    layer.set_fill_color(text_color.clone());
    layer.use_text("Total", 6.0, text_x_base, text_y_base, font);
    layer.use_text("operaciones.", 6.0, text_x_base, text_y_base - Mm(3.0), font);

    // 🔹 **Encabezados alineados arriba**
    let column_positions = vec![
        margin + left_bottom_width + Mm(19.0),
        margin + left_bottom_width + Mm(32.0),
        margin + left_bottom_width + Mm(48.0),
    ];
    let headers = vec!["No sujetas", "Exentas", "Gravadas"];

    for (i, header) in headers.iter().enumerate() {
        layer.use_text(*header, 6.0, column_positions[i], text_y_base, font);
    }

    // 🔹 **Valores alineados debajo de los títulos**
    let values = vec![dteJson.resumen.totalNoGravado.to_string(), dteJson.resumen.totalExenta.to_string(), dteJson.resumen.totalGravada.to_string()];
    for (i, value) in values.iter().enumerate() {
        layer.use_text(value, 6.0, column_positions[i], text_y_base - Mm(3.5), font);
    }

    current_y -= first_section_height;

    // 🔹 **Segunda sección (Otros datos intermedios)**
    let middle_section_height = Mm(16.0);
    let middle_section = Polygon {
        rings: vec![vec![
            (Point::new(margin + left_bottom_width, current_y), false),
            (Point::new(width - margin, current_y), false),
            (Point::new(width - margin, current_y - middle_section_height), false),
            (Point::new(margin + left_bottom_width, current_y - middle_section_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };
    layer.set_fill_color(fill_color.clone());
    layer.add_polygon(middle_section);

    let sinImpuesto = format!("{:.2}", dteJson.resumen.totalPagar - dteJson.resumen.totalIva);
    let subTotal = format!("{:.2}", dteJson.resumen.subTotal);
    let ivaRete1 = format!("{:.2}", dteJson.resumen.ivaRete1);
    let totalPagar = format!("{:.2}", dteJson.resumen.montoTotalOperacion);


    let middle_data = vec![
        ("Suma de operaciones sin impuesto", sinImpuesto),
        ("Sub total",subTotal),
        ("IVA retenido",ivaRete1),
        ("Monto total de la operación", totalPagar),
    ];

    let mut text_y = current_y - Mm(3.5);
    for (label, value) in middle_data {
        layer.set_fill_color(text_color.clone());
        layer.use_text(label, 6.0, margin + left_bottom_width + Mm(3.0), text_y, font);
        layer.use_text(value, 6.0, width - margin - Mm(9.5), text_y, font);
        text_y -= Mm(3.5);
    }

    current_y -= middle_section_height;

    // 🔹 **Última sección (TOTAL A PAGAR)**
    let last_section_height = Mm(5.5);
    let last_section = Polygon {
        rings: vec![vec![
            (Point::new(margin + left_bottom_width, current_y), false),
            (Point::new(width - margin, current_y), false),
            (Point::new(width - margin, current_y - last_section_height), false),
            (Point::new(margin + left_bottom_width, current_y - last_section_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };
    layer.set_fill_color(sub_color.clone()); // Fondo especial para TOTAL A PAGAR
    layer.add_polygon(last_section);

    let totalPagar = format!("{:.2}", dteJson.resumen.totalPagar);

    layer.set_fill_color(highlight_text_color.clone()); // Letras blancas
    layer.use_text("TOTAL A PAGAR", 6.0, margin + left_bottom_width + Mm(3.0), current_y - Mm(3.5), font);
    layer.use_text(totalPagar, 6.0, width - margin - Mm(9.5), current_y - Mm(3.5), font);
}
