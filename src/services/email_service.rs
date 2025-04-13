use std::fs;
use lettre::{Message, SmtpTransport, AsyncTransport, Tokio1Executor, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::message::{header::ContentType, SinglePart, MultiPart, Attachment};
use std::path::Path;
use crate::routes::Pdf::DteJson;

pub async fn send_email_with_attachments(
    to: &str,
    pdf_path: &Path,
    json_path: &Path,
    _data: &DteJson
) -> Result<(), Box<dyn std::error::Error>> {
    let creds = Credentials::new(
        "8a4f6c001@smtp-brevo.com".to_string(),
        "dYKOcXNn6rabt8zm".to_string(),
    );

    let html_body = build_html_body(_data)?;

    let pdf_attachment = Attachment::new("factura.pdf".to_string())
        .body(std::fs::read(pdf_path)?, ContentType::parse("application/pdf")?);

    let json_attachment = Attachment::new("dte.json".to_string())
        .body(std::fs::read(json_path)?, ContentType::parse("application/json")?);

    let email = Message::builder()
        .from("FacturaSV <notificaciones@facturasv.online>".parse()?)
        .to(to.parse()?)
        .subject(format!("Factura electrónica - {}", _data.emisor.nombre))
        .multipart(
            MultiPart::mixed()
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(html_body)
                )
                .singlepart(pdf_attachment)
                .singlepart(json_attachment)
        )?;

    let mailer = SmtpTransport::relay("smtp-relay.brevo.com")?
        .credentials(creds)
        .build();

    mailer.send(&email).map(|_res| ()).map_err(|e| e.into())
}

fn build_html_body(_data: &DteJson) -> Result<String, Box<dyn std::error::Error>> {
    let template = fs::read_to_string("templates/factura_email.html")?;
    let html = template
        .replace("{{dte_number}}", &_data.identificacion.numeroControl)
        .replace("{{code_generation}}", &_data.identificacion.codigoGeneracion)
        .replace("{{sello_hacienda}}", &_data.recepcionMh.selloRecibido);

    Ok(html)
}
