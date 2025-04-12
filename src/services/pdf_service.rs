use printpdf::*;
use std::fs::File;
use crate::services::{footer_service, row_info_service, sub_total_service};
use crate::services::header_left_service;
use crate::services::header_right_service;
use crate::services::products_service;
use std::io::{BufWriter};
use std::process::Command;
use printpdf::{PdfDocument, Mm, Color};
use crate::routes::Pdf::{DteJson, ItemDocumento};

pub fn create_invoice_pdf(_data: &DteJson) -> String {
    let file_path = "./output/output.pdf".to_string();
    let compressed_file_path = "./output/output_compressed.pdf".to_string();

    // ──────────────────────────────────────────────────────────────────────────
    // 1. Dimensiones y márgenes
    // ──────────────────────────────────────────────────────────────────────────
    let width = Mm(216.0);   // Ancho carta
    let height = Mm(279.0);  // Alto carta

    let margin = Mm(10.0);
    let usable_width = width - (margin * 2.0);
    let usable_height = height - (margin * 2.0);


    // ──────────────────────────────────────────────────────────────────────────
    // 2. Proporciones de las secciones principales
    // ──────────────────────────────────────────────────────────────────────────
    let header_height = usable_height * 0.27;        // 27% de la página
    let client_info_height = usable_height * 0.07;   // 6% de la página
    let other_info_height = usable_height * 0.07;    // 7% de la página
    let other_docs_height = usable_height * 0.025;   // 3.5% de la página
    let products_height = usable_height * 0.45;      // 44% de la página
    let bottom_height = usable_height * 0.11;        // 10% de la página
    let footer_height = usable_height * 0.02;        // 2% de la página

    // Ancho para left_header vs right_header
    let left_width = usable_width * 0.48;            // 48% del ancho usable
    let right_width = usable_width * 0.52;           // 52% del ancho usable

    // Para el footer
    let left_bottom_width = usable_width * 0.70; // 70% de ancho usable
    let right_bottom_width = usable_width * 0.30; // 30% de ancho usable

    // ──────────────────────────────────────────────────────────────────────────
    // 3. Subdivisiones del right_header (como antes)
    //    3 + 12 + 10 = 25 “unidades” → se reparten en vertical
    // ──────────────────────────────────────────────────────────────────────────
    let sum_subsections = 3.0 + 12.0 + 12.0; // = 25.0
    let subright_title_factor = 4.0 / sum_subsections;      // 3/25
    let subright_docinfo_factor = 11.0 / sum_subsections;   // 12/25
    let subright_bottom_factor = 12.0 / sum_subsections;    // 10/25

    // ──────────────────────────────────────────────────────────────────────────
    // 4. Crear documento y capa
    // ──────────────────────────────────────────────────────────────────────────
    let (mut doc, page1, layer1) = PdfDocument::new("Documento", width, height, "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();

    // ──────────────────────────────────────────────────────────────────────────
    // 5. Definir colores
    // ──────────────────────────────────────────────────────────────────────────
    let grey_color = Color::Greyscale(Greyscale::new(0.3, None));     // Gris
    let black_color = Color::Greyscale(Greyscale::new(0.0, None));    // Negro
    // Definir colores
    let white_color = Color::Greyscale(Greyscale::new(1.0, None));
    // ──────────────────────────────────────────────────────────────────────────
    // 6. Iniciar en la parte superior (dentro del margen)
    // ──────────────────────────────────────────────────────────────────────────
    let mut current_y = height - margin;

    // ──────────────────────────────────────────────────────────────────────────
    // 7. Subdividir el left_header (25% de la página) en 3 secciones:
    //    10 + 5 + 10 = 25 unidades → cada una es un porcentaje de left_header
    // ──────────────────────────────────────────────────────────────────────────
    let left_header_sum = 10.0 + 5.0 + 12.0; // = 25
    // rojo
    let sub_left_empresa_nombre_height = header_height * (5.0  / left_header_sum); // negro
    let sub_left_empresa_emisora_height = header_height * (12.0 / left_header_sum); // rosa

    // Variables definidas previamente
    let margin = Mm(10.0);
    let left_width = usable_width * 0.48;
    let sub_left_header_logo_height = header_height * (10.0 / left_header_sum);

    // Dibujar el rectángulo del logo
    current_y = header_left_service::create_shape_logo(
        &current_layer,
        margin,
        current_y,
        left_width,
        sub_left_header_logo_height,
        &white_color,
        "./logos/logo.jpg",  // Ruta de la imagen
        &doc           // Referencia al documento PDF
    ).expect("REASON");




    current_y = header_left_service::draw_empresa_nombre_section(
        &_data.emisor,
        &current_layer,
        &font,
        &font_bold,
        margin,
        current_y,
        left_width,
        sub_left_empresa_nombre_height,
        &white_color,  // Color de fondo
        &black_color    // Color del texto
    );

    current_y = header_left_service::draw_empresa_emisora_section(
        &_data.emisor,
        &current_layer,
        &font,
        &font_bold,
        margin,
        current_y,
        left_width,
        sub_left_empresa_emisora_height,
        &white_color,  // Color de fondo
        &black_color   // Color del texto
    );


    let subright_title_h = header_height * subright_title_factor;       // 3/25
    let subright_docinfo_h = header_height * subright_docinfo_factor;   // 12/25
    let subright_bottom_h = header_height * subright_bottom_factor;     // 10/25

    let x0 = margin + left_width;
    let x1 = width - margin;
    // Top del right_header (igual que top del left_header al inicio)
    let right_header_top = height - margin;

    header_right_service::draw_right_header_section(
        &_data,
        &current_layer,
        &font,
        &font_bold,
        x0,
        x1,
        right_header_top,
        subright_title_h,
        subright_docinfo_h,
        &grey_color,    // Color del título
        &white_color,   // Color de la información
        &black_color    // Color del texto
    );

    let docinfo_top = right_header_top - subright_title_h;
    let bottom_row_top = docinfo_top - subright_docinfo_h;

    header_right_service::draw_right_extra_info_section(
        &_data,
        &current_layer,
        &font,
        &font_bold,
        x0,
        x1,
        right_width,
        bottom_row_top,
        subright_title_h,
        subright_docinfo_h,
        subright_bottom_h,
        &grey_color,    // Color del fondo de la sección
        &white_color,   // Color del fondo del QR
        &black_color    // Color del texto
    );

    current_y = row_info_service::draw_client_info_section(
        &_data.receptor,
        &current_layer,
        &font,
        &font_bold,
        margin,
        current_y,
        width,
        usable_width,
        client_info_height,
        &white_color,  // Color de fondo
        &black_color   // Color del texto
    );

    current_y = row_info_service::draw_other_info_section(
        &current_layer,
        margin,
        current_y,
        width,
        other_info_height,
        &white_color,
    );

    current_y = row_info_service::draw_other_docs_section(
        &current_layer,
        margin,
        current_y,
        width,
        other_docs_height,
        &white_color,
    );




    // 44% de la página
    let productos = &_data.cuerpoDocumento;
    let total_items = productos.len();
    
    let first_page_min_limit = 18;

    if total_items > first_page_min_limit {
        let first_page_limit = 23;
        let other_pages_limit = 42;
        let max_last_page_items = 36;

        let mut chunks: Vec<&[ItemDocumento]> = vec![];
        if total_items <= first_page_limit {
            chunks.push(&productos[..first_page_min_limit]);
            chunks.push(&productos[first_page_min_limit..total_items]);
        } else {
            chunks.push(&productos[..first_page_limit]);

            let mut start = first_page_limit;
            while start < total_items {
                let mut end = usize::min(start + other_pages_limit, total_items);

                if end == total_items && (end - start) > max_last_page_items {
                    end = start + max_last_page_items;
                }

                chunks.push(&productos[start..end]);
                start = end;
            }
        }

        let total_pages = chunks.len();
        println!("{}", chunks.len());

        for (i, chunk) in chunks.iter().enumerate() {
            println!("{}", chunk.len());

            let (page, layer) = if i == 0 {
                (page1, doc.get_page(page1).get_layer(layer1))
            } else {
                let (new_page, layer_id) = doc.add_page(width, height, &format!("Layer {}", i + 1));
                (new_page, doc.get_page(new_page).get_layer(layer_id))
            };
            
            let mut height_list =   Mm(((first_page_limit as f64) * 6.6) as f32) - Mm(8.0);
            if i != 0 {
                height_list =   Mm(((chunk.len() as f64) * 6.6) as f32) + Mm(7.0);
                current_y = height - margin;
            }
            if (chunk.len() >= max_last_page_items) {
                height_list =   Mm(((other_pages_limit as f64) * 6.6) as f32) - Mm(18.0);
                current_y = height - margin;
            }

            current_y = products_service::draw_products_section(
                chunk,
                &layer,
                &font,
                margin,
                current_y,
                width,
                height_list,
                &white_color,
            );

            if chunk.len() >= max_last_page_items {
                current_y = height - margin;
            }

            if (i == 0  &&  chunk.len() > 18){
                current_y = height - margin;
            }


            if i == chunks.len() - 1 {
                sub_total_service::draw_footer_left_section(
                    &_data,
                    &layer,
                    margin,
                    current_y,
                    left_bottom_width,
                    bottom_height,
                    &white_color,
                );

                sub_total_service::draw_footer_right_section(
                    &_data,
                    &layer,
                    &font,
                    margin,
                    current_y,
                    left_bottom_width,
                    width,
                    bottom_height,
                    &Color::Greyscale(Greyscale::new(0.3, None)), // Fondo gris oscuro para TOTAL A PAGAR
                    &Color::Greyscale(Greyscale::new(0.9, None)), // Fondo gris claro para el resto
                    &Color::Greyscale(Greyscale::new(0.0, None)), // Texto negro
                    &Color::Greyscale(Greyscale::new(1.0, None)), // Texto blanco (TOTAL A PAGAR)
                );

                footer_service::draw_footer_section(
                    &layer,
                    &font,
                    margin,
                    current_y - bottom_height,
                    width,
                    footer_height,
                    &Color::Greyscale(Greyscale::new(0.3, None)), // Color gris oscuro de fondo
                    &Color::Greyscale(Greyscale::new(1.0, None)), // Texto blanco
                );
            }
            draw_page_number(&layer, &font, i + 1, total_pages, width, margin);

        }
    } else {
        current_y = products_service::draw_products_section(
            productos,
            &current_layer,
            &font,
            margin,
            current_y,
            width,
            products_height,
            &white_color,
        );

        sub_total_service::draw_footer_left_section(
            &_data,
            &current_layer,
            margin,
            current_y,
            left_bottom_width,
            bottom_height,
            &white_color,
        );

        sub_total_service::draw_footer_right_section(
            &_data,
            &current_layer,
            &font,
            margin,
            current_y,
            left_bottom_width,
            width,
            bottom_height,
            &Color::Greyscale(Greyscale::new(0.3, None)), // Fondo gris oscuro para TOTAL A PAGAR
            &Color::Greyscale(Greyscale::new(0.9, None)), // Fondo gris claro para el resto
            &Color::Greyscale(Greyscale::new(0.0, None)), // Texto negro
            &Color::Greyscale(Greyscale::new(1.0, None)), // Texto blanco (TOTAL A PAGAR)
        );



        footer_service::draw_footer_section(
            &current_layer,
            &font,
            margin,
            current_y - bottom_height,
            width,
            footer_height,
            &Color::Greyscale(Greyscale::new(0.3, None)), // Color gris oscuro de fondo
            &Color::Greyscale(Greyscale::new(1.0, None)), // Texto blanco
        );
    }








    doc.save(&mut BufWriter::new(File::create(&file_path).unwrap())).unwrap();


    // // ──────────────────────────────────────────────────────────────────────────
    // // 4. Comprimir el PDF con lopdf
    // // ──────────────────────────────────────────────────────────────────────────
    // let mut doc = Document::load(&file_path).expect("Error al cargar el PDF");
    //
    // // Optimizar el PDF para reducir tamaño
    // doc.prune_objects(); // Elimina objetos no utilizados
    // doc.compress(); // Comprime streams e imágenes
    //
    // // Guardar el PDF comprimido
    // doc.save(&compressed_file_path).expect("Error al guardar PDF comprimido");
    //
    // compressed_file_path


    // ──────────────────────────────────────────────────────────────────────────
    // Comprimir PDF con Ghostscript
    // ──────────────────────────────────────────────────────────────────────────
    // let output = Command::new("gs")
    //     .args([
    //         "-sDEVICE=pdfwrite",
    //         "-dCompatibilityLevel=1.4",
    //         "-dPDFSETTINGS=/printer", // Puedes cambiar a /ebook, /printer, /prepress
    //         "-dNOPAUSE",
    //         "-dBATCH",
    //         &format!("-sOutputFile={}", compressed_file_path),
    //         &file_path,
    //     ])
    //     .output()
    //     .expect("Error ejecutando Ghostscript");
    // 
    // eprintln!("🔍 Ghostscript stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    // eprintln!("🔍 Ghostscript stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    // 
    // if !output.status.success() {
    //     eprintln!("Ghostscript falló con código de salida: {:?}", output.status);
    // } else {
    //     println!("PDF comprimido correctamente en {}", compressed_file_path);
    // }

    compressed_file_path

}

fn draw_page_number(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    page_number: usize,
    total_pages: usize,
    page_width: Mm,
    margin: Mm,
) {
    let text = format!("Página {} de {}", page_number, total_pages);
    let font_size = 8.0;

    // Posición centrada en el pie de página
    let text_x = (page_width + Mm(65.0)) / 2.0 - Mm((text.len() as f64 * 3.0) as f32); // Aproximación de centrado
    let text_y = margin - Mm(3.0); // Pie de página

    layer.set_fill_color(Color::Greyscale(Greyscale::new(0.0, None))); // Texto negro
    layer.use_text(text, font_size, text_x, text_y, font);
}