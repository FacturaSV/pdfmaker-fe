use printpdf::*;
use printpdf::path::{PaintMode, WindingOrder};
use crate::routes::Pdf::{DteJson, Emisor, Identificacion};
use crate::services::qr_service;

pub fn draw_right_header_section(
    dteJson: &DteJson,
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    fontBlod: &IndirectFontRef,
    x0: Mm,
    x1: Mm,
    right_header_top: Mm,
    subright_title_h: Mm,
    subright_docinfo_h: Mm,
    title_color: &Color,
    docinfo_color: &Color,
    text_color: &Color,
) -> Mm {
    // Dibujar el rectángulo del título (gris)
    let subright_title = Polygon {
        rings: vec![vec![
            (Point::new(x0, right_header_top), false),
            (Point::new(x1, right_header_top), false),
            (Point::new(x1, right_header_top - subright_title_h), false),
            (Point::new(x0, right_header_top - subright_title_h), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };
    layer.set_fill_color(title_color.clone());
    layer.add_polygon(subright_title);

    // Dibujar el rectángulo de información (blanco)
    let docinfo_top = right_header_top - subright_title_h;
    let subright_docinfo = Polygon {
        rings: vec![vec![
            (Point::new(x0, docinfo_top), false),
            (Point::new(x1, docinfo_top), false),
            (Point::new(x1, docinfo_top - subright_docinfo_h), false),
            (Point::new(x0, docinfo_top - subright_docinfo_h), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };
    layer.set_fill_color(docinfo_color.clone());
    layer.add_polygon(subright_docinfo);

    // Añadir texto centrado en el título
    let text_x = x0 + ((x1 - x0) / 2.0) - Mm(38.0); // Centrado horizontalmente
    let text_y = right_header_top - (subright_title_h / 2.5); // Ajustado verticalmente

    layer.set_fill_color(docinfo_color.clone());
    layer.use_text("DOCUMENTO TRIBUTARIO ELECTRÓNICO", 11.0, text_x, text_y, fontBlod);
    layer.use_text("FACTURA", 11.0, text_x + Mm(30.0), text_y - Mm(5.0), fontBlod);

    // Datos clave del documento
    let tipoModelo = dteJson.identificacion.tipoModelo.to_string();
    let version = dteJson.identificacion.version.to_string();
    let key_value_pairs = vec![
        ("CÓDIGO DE GENERACIÓN", &dteJson.identificacion.codigoGeneracion),
        ("SELLO RECEPCIÓN", &dteJson.recepcionMh.selloRecibido),
        ("NÚMERO DE CONTROL", &dteJson.identificacion.numeroControl),
        ("MODELO FACTURACIÓN",  &tipoModelo),
        ("VERSIÓN DEL JSON",  &version),
        ("TIPO DE TRANSMISIÓN", &dteJson.recepcionMh.estado),
        ("FECHA DE EMISIÓN",  &dteJson.identificacion.fecEmi),
        ("MONEDA", &dteJson.identificacion.tipoMoneda),
        ("HORA DE EMISIÓN",  &dteJson.identificacion.horEmi),
    ];

    // Posición del texto en la sección de información
    let mut text_y = docinfo_top - Mm(4.5);
    for (i, (key, value)) in key_value_pairs.iter().enumerate() {
        layer.set_fill_color(text_color.clone());

        if i < 6 {
            layer.use_text(format!("{}:", key), 7.0, x0 + Mm(2.0), text_y, fontBlod);
            layer.use_text(*value, 7.0, x0 + Mm(35.0), text_y, font);
        } else {
            layer.use_text(format!("{}:", key), 7.0, x0 + Mm(55.0), text_y - Mm(12.0), fontBlod);
            layer.use_text(*value, 7.0, x0 + Mm(80.0), text_y - Mm(12.0), font);
        }

        if i == 5 {
            text_y = docinfo_top - Mm(4.0);
        } else {
            text_y -= Mm(4.0);
        }
    }

    docinfo_top - subright_docinfo_h // Retorna la nueva posición de `current_y`
}


/// Dibuja la sección de "Información Extra" y agrega el código QR
pub fn draw_right_extra_info_section(
    dteJson: &DteJson,
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    fontBlod: &IndirectFontRef,
    x0: Mm,
    x1: Mm,
    right_width: Mm,
    bottom_row_top: Mm,
    subright_title_h: Mm,
    subright_docinfo_h: Mm,
    subright_bottom_h: Mm,
    grey_color: &Color,
    white_color: &Color,
    black_color: &Color,
) -> Mm {
    let subright_extra_width = right_width * 0.70;

    let subright_extra_info = Polygon {
        rings: vec![vec![
            (Point::new(x0+Mm(0.5), bottom_row_top-Mm(0.5)), false),
            (Point::new(x0-Mm(0.5) + subright_extra_width, bottom_row_top-Mm(0.5)), false),
            (Point::new(x0-Mm(0.5) + subright_extra_width, bottom_row_top-Mm(0.5) - Mm(3.0)), false),
            (Point::new(x0+Mm(0.5), bottom_row_top-Mm(0.5) - Mm(3.0)), false),
        ]],
        mode: PaintMode::Fill,
        winding_order: WindingOrder::NonZero,
    };

    layer.set_fill_color(grey_color.clone());
    layer.add_polygon(subright_extra_info);

    // 🔹 **Texto dentro de la sección**
    let key_value_pairs = vec![
        ("CÓDIGO CLIENTE", ""),
        ("NÚMERO DE CONTROL", ""),
        ("VENDEDOR", ""),
        ("CÓDIGO ALMACÉN", ""),
        ("Nº DOCUMENTO", ""),
        ("ORDEN DE COMPRA", ""),
    ];

    let mut text_y = bottom_row_top - Mm(6.0);
    for (i, (key, value)) in key_value_pairs.iter().enumerate() {
        layer.set_fill_color(black_color.clone());
        layer.use_text(format!("{}:", key), 6.0, x0 + Mm(2.0), text_y, fontBlod);
        layer.use_text(*value, 6.0, x0 + Mm(30.0), text_y, font);
        text_y -= Mm(3.5);
    }

    // 🔹 **Dibujar área del QR (sección derecha)**
    let subright_qr = Polygon {
        rings: vec![vec![
            (Point::new(x0 + subright_extra_width, bottom_row_top), false),
            (Point::new(x1, bottom_row_top), false),
            (Point::new(x1, bottom_row_top - subright_bottom_h), false),
            (Point::new(x0 + subright_extra_width, bottom_row_top - subright_bottom_h), false),
        ]],
        mode: PaintMode::FillStroke,
        winding_order: WindingOrder::NonZero,
    };
    layer.set_fill_color(white_color.clone());
    layer.add_polygon(subright_qr);

    // 🔹 **Añadir el título "INFORMACIÓN EXTRA" centrado en la sección**
    let text_x = x0 + (right_width / 2.0) - Mm(27.0); // Ajustado para centrar mejor
    let text_y = bottom_row_top - (subright_title_h / 3.5); // Ajustado para mejor alineación

    layer.set_fill_color(white_color.clone());
    layer.use_text("INFORMACIÓN EXTRA", 8.0, text_x, text_y, font);

    // 🔹 **Generar y añadir el código QR**
    let qr_url = format!(
        "https://admin.factura.gob.sv/consultaPublica?ambiente={}&codGen={}&fechaEmi={}",
        dteJson.recepcionMh.ambiente,
        dteJson.recepcionMh.codigoGeneracion,
        dteJson.identificacion.fecEmi
    );
    let qr_image = qr_service::generate_qr_code_image(&*qr_url, 7);


    if let Ok(pdf_image) = qr_service::convert_image_for_pdf(&qr_image) {
        pdf_image.add_to_layer(layer.clone(), ImageTransform {
            translate_x: Some(x0 + subright_extra_width + Mm(0.5)), // Posición ajustada
            translate_y: Some(bottom_row_top - Mm(30.0)), // Ajustar verticalmente
            scale_x: Some(1.1),
            scale_y: Some(1.1),
            ..Default::default()
        });
    } else {
        println!("❌ Error al convertir la imagen para el PDF");
    }

    bottom_row_top - subright_bottom_h
}

