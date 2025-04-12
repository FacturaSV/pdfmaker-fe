use printpdf::*;
use printpdf::path::{PaintMode, WindingOrder};
use crate::routes::Pdf::{ItemDocumento};

pub fn generate_product_table(
    items: &[ItemDocumento],
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    start_x: Mm,
    mut start_y: Mm,
    column_widths: &[Mm]
) {
    let rows: Vec<(String, String, String, &str, &str, String, String, String, String, String)> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            (
                format!("{}",  i),                          // numItem
                format!("{:.2}", item.cantidad),              // cantidad
                item.uniMedida.to_string(),                          // &String -> &str
                item.codigo.as_str(),                          // &String -> &str
                item.descripcion.as_str(),                     // &String -> &str
                format!("{:.2}", item.precioUni),             // precio unitario
                format!("{:.2}", item.ventaNoSuj),            // no sujeta
                format!("{:.2}", item.ventaExenta),           // exenta
                format!("{:.2}", item.ventaGravada),          // gravada
                format!("{:.2}", item.precioUni * item.cantidad), // total línea
            )
        })
        .collect();


    let headers = vec![
        "Num", " Cant", "Unidad Medida", "Código", "Descripción",
        "Precio Unit", "Descuento", "No Sujetas", "Exentas", "Gravadas"
    ];
    
    let header_height = Mm(8.0);
    let row_height = Mm(6.0);

    let x1 = start_x;
    let x2 = start_x + column_widths.iter().cloned().fold(Mm(0.0), |acc, mm| acc + mm);
    let y1 = start_y;
    let y2 = start_y - header_height;

    // 🔹 **Dibujar fondo gris oscuro de los encabezados usando Polygon**
    let header_bg = Polygon {
        rings: vec![vec![
            (Point::new(x1, y1), false),
            (Point::new(x2, y1), false),
            (Point::new(x2, y2), false),
            (Point::new(x1, y2), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(Color::Greyscale(Greyscale::new(0.3, None))); // **Color gris oscuro**
    layer.add_polygon(header_bg);

    // 📌 **Cambiar el color del texto a blanco**
    layer.set_fill_color(Color::Greyscale(Greyscale::new(1.0, None))); // **Texto blanco**

    let mut x_position = start_x;
    let font_size = 7.0;

    let mut x_position = start_x;
    let font_size = 7.0;

    for (i, header) in headers.iter().enumerate() {
        let column_width = column_widths[i];

        let text_y = y1 - (header_height / 2.0);

        let text_x = if i >= 5 {
            // 🔹 Alineación derecha
            let text_width = estimate_text_width(header, font_size);
            x_position + column_width - text_width - Mm(1.5) // margen derecho
        } else {
            // 🔹 Alineación izquierda
            x_position + Mm(2.0)
        };

        layer.use_text(*header, font_size as f32, text_x, text_y, font);
        x_position += column_width;
    }
    
    start_y -= header_height;

    for row in rows {
        let formatted_row: Vec<String> = vec![
            row.0.to_string(),
            row.1.to_string(),
            row.2.to_string(),
            row.3.to_string(),
            row.4.to_string(),
            row.5.to_string(),
            row.6.to_string(),
            row.7.to_string(),
            row.8.to_string(),
            row.9.to_string(),
        ];

        x_position = start_x;
        
        for (i, text) in formatted_row.iter().enumerate() {
            if text.is_empty() {
                x_position += column_widths[i];
                continue;
            }

            let font_size = 7.0;

            // 🔹 Para columnas 5 a 9 (índices 5 al 9 inclusive), alineamos a la derecha
            let text_x = if i >= 5 {
                let text_width = estimate_text_width(text, font_size);
                x_position + column_widths[i] - text_width - Mm(1.5) // margen derecho
            } else {
                // 🔹 Para columnas 0 a 4, alineamos a la izquierda
                x_position + Mm(2.0)
            };

            let text_y = start_y - (row_height / 2.2);

            layer.set_fill_color(Color::Greyscale(Greyscale::new(0.0, None)));
            layer.use_text(text, font_size as f32, text_x, text_y, font);

            x_position += column_widths[i];
        }

        start_y -= row_height;
    }
}

fn estimate_text_width(text: &str, font_size: f64) -> Mm {
    // Aproximación de ancho de texto (usa 0.5 o 0.6 dependiendo del font)
    let avg_char_width = 0.5 * font_size;
    let width_pt = text.len() as f64 * avg_char_width;
    Mm((width_pt / 2.83465) as f32) // Convertir pt a mm
}


fn fill_rect(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm, color: Color) {
    layer.set_fill_color(color);
    let points = vec![
        (Point::new(x1, y1), false),
        (Point::new(x2, y1), false),
        (Point::new(x2, y2), false),
        (Point::new(x1, y2), false),
        (Point::new(x1, y1), false),
    ];

    let rect = Line {
        points,
        is_closed: true,
    };

    layer.add_line(rect);
}

// 📌 **Dibujar los bordes negros del encabezado**
fn draw_border_rect(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm, border_color: Color) {
    layer.set_outline_color(border_color);

    let points = vec![
        (Point::new(x1, y1), false),
        (Point::new(x2, y1), false),
        (Point::new(x2, y2), false),
        (Point::new(x1, y2), false),
        (Point::new(x1, y1), false),
    ];

    let border = Line {
        points,
        is_closed: true,
    };

    layer.add_line(border);
}

// 📌 **Ocultar valores 0 o vacíos**
fn filter_zero(value: &str) -> String {
    if value == "0.00" || value == "0" || value.trim().is_empty() {
        return "".to_string();
    }
    value.to_string()
}


pub(crate) fn draw_totals_table(layer: &PdfLayerReference, font: &IndirectFontRef, start_x: Mm, mut start_y: Mm, column_widths: &[Mm]) {
    let row_height = Mm(7.0);
    let total_label_width = column_widths.iter().take(column_widths.len() - 1).map(|mm| mm.0).sum::<f32>();
    let x1 = start_x;
    let x2 = start_x + Mm(column_widths.iter().map(|mm| mm.0).sum::<f32>());

    let headers = vec!["Total operaciones", "No Sujetas", "Exentas", "Gravadas"];
    let totals = vec!["", "0.00", "0.00", "289.87"];
    let summary_rows = vec![
        ("Suma de operaciones sin impuesto", "289.87"),
        ("Sub total", "289.87"),
        ("IVA retenido", "0.00"),
        ("Monto total de la operación", "289.87"),
    ];

    let total_final_label = "TOTAL A PAGAR";
    let total_final_value = "289.87";

    // 🔹 **Dibujar línea superior**
    draw_line(layer, x1, start_y, x2, start_y);
    start_y -= row_height;

    let mut x_pos = start_x;

    // 🔹 **Encabezados**
    for (i, header) in headers.iter().enumerate() {
        let text_x = x_pos + (column_widths[i] / 2.0) - Mm(6.0);
        let text_y = start_y + (row_height / 2.2);
        layer.set_fill_color(Color::Greyscale(Greyscale::new(0.0, None))); // Negro
        layer.use_text(*header, 8.0, text_x, text_y, font);
        x_pos += column_widths[i];
    }

    // 🔹 **Dibujar línea después de encabezados**
    draw_line(layer, x1, start_y, x2, start_y);
    start_y -= row_height;

    // 🔹 **Fila de totales**
    x_pos = start_x;
    for (i, total) in totals.iter().enumerate() {
        let text_x = x_pos + (column_widths[i] / 2.0) - Mm(6.0);
        let text_y = start_y + (row_height / 2.2);
        layer.use_text(*total, 8.0, text_x, text_y, font);
        x_pos += column_widths[i];
    }

    // 🔹 **Dibujar línea después de la fila de totales**
    draw_line(layer, x1, start_y, x2, start_y);
    start_y -= row_height;

    // 🔹 **Resumen de totales**
    for (label, value) in summary_rows.iter() {
        let text_x = start_x + Mm(total_label_width) - Mm(70.0);
        let text_y = start_y + (row_height / 2.2);
        layer.set_fill_color(Color::Greyscale(Greyscale::new(0.0, None))); // Negro
        layer.use_text(*label, 8.0, text_x, text_y, font);
        layer.use_text(*value, 8.0, x2 - Mm(20.0), text_y, font);
        start_y -= row_height;
    }

    // 🔹 **Fondo gris oscuro para "TOTAL A PAGAR"**
    let total_bg = Polygon {
        rings: vec![vec![
            (Point::new(start_x, start_y), false),
            (Point::new(x2, start_y), false),
            (Point::new(x2, start_y - row_height), false),
            (Point::new(start_x, start_y - row_height), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(Color::Greyscale(Greyscale::new(0.3, None))); // Gris oscuro
    layer.add_polygon(total_bg);

    // 🔹 **Texto blanco en "TOTAL A PAGAR"**
    layer.set_fill_color(Color::Greyscale(Greyscale::new(1.0, None))); // Blanco
    let text_x_label = start_x + Mm(total_label_width) - Mm(70.0);
    let text_x_value = x2 - Mm(20.0);
    let text_y = start_y + (row_height / 2.2);
    layer.use_text(total_final_label, 9.0, text_x_label, text_y, font);
    layer.use_text(total_final_value, 9.0, text_x_value, text_y, font);

    // 🔹 **Línea inferior final**
    draw_line(layer, x1, start_y - row_height, x2, start_y - row_height);
}

// 📌 **Dibujar una línea horizontal**
fn draw_line(layer: &PdfLayerReference, x1: Mm, y1: Mm, x2: Mm, y2: Mm) {
    let line = Line {
        points: vec![
            (Point::new(x1, y1), false),
            (Point::new(x2, y2), false),
        ],
        is_closed: false,
    };
    layer.set_outline_color(Color::Greyscale(Greyscale::new(0.0, None))); // Negro
    layer.add_line(line);
}

