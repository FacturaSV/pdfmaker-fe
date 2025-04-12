use axum::{Json, extract::State};
use axum::response::IntoResponse;
use serde::{Serializer, Deserialize, Deserializer, Serialize};
use crate::services::pdf_service;



pub async fn generate_invoice(Json(payload): Json<DteJson>) -> impl IntoResponse {
    println!("Número de control: {}", payload.identificacion.numeroControl);

    let file_path = pdf_service::create_invoice_pdf(&payload);
    Json(serde_json::json!({
        "message": "PDF generado exitosamente",
        "data": file_path
    }))
}


#[derive(Deserialize)]
pub struct PdfRequest {
    pub title: String,
    #[serde(default)]
    pub content: String,
}

#[derive(Serialize)]
pub struct PdfResponse {
    pub message: String,
    pub file_path: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub  struct DteJson {
    pub identificacion: Identificacion,
    pub documentoRelacionado: Option<serde_json::Value>,
    pub emisor: Emisor,
    pub receptor: Receptor,
    pub otrosDocumentos: Option<serde_json::Value>,
    pub ventaTercero: Option<serde_json::Value>,
    pub cuerpoDocumento: Vec<ItemDocumento>,
    pub resumen: Resumen,
    pub extension: Option<serde_json::Value>,
    pub apendice: Option<serde_json::Value>,
    pub firmaElectronica: String,
    pub recepcionMh: RecepcionMh,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identificacion {
    pub version: u8,
    pub ambiente: String,
    pub tipoDte: String,
    pub numeroControl: String,
    pub codigoGeneracion: String,
    pub tipoModelo: u8,
    pub tipoOperacion: u8,
    pub tipoContingencia: Option<String>,
    pub motivoContin: Option<String>,
    pub fecEmi: String,
    pub horEmi: String,
    pub tipoMoneda: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emisor {
    pub nit: String,
    pub nrc: String,
    pub nombre: String,
    pub codActividad: String,
    pub descActividad: String,
    pub nombreComercial: String,
    pub tipoEstablecimiento: String,
    pub direccion: Direccion,
    pub telefono: String,
    pub correo: String,
    pub codEstableMH: String,
    pub codEstable: String,
    pub codPuntoVentaMH: Option<String>,
    pub codPuntoVenta: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Receptor {
    pub tipoDocumento: String,
    pub numDocumento: String,
    pub nrc: Option<String>,
    pub nombre: String,
    pub codActividad: String,
    pub descActividad: String,
    pub direccion: Direccion,
    pub telefono: String,
    pub correo: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Direccion {
    pub departamento: String,
    pub municipio: String,
    pub complemento: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemDocumento {
    pub numItem: u32,
    pub tipoItem: u8,
    pub numeroDocumento: Option<String>,
    pub cantidad: f64,
    pub codigo: String,
    pub codTributo: Option<String>,
    pub uniMedida: u32,
    pub descripcion: String,
    pub precioUni: f64,
    pub montoDescu: f64,
    pub ventaNoSuj: f64,
    pub ventaExenta: f64,
    pub ventaGravada: f64,
    pub tributos: Option<serde_json::Value>,
    pub psv: f64,
    pub noGravado: f64,
    pub ivaItem: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resumen {
    pub totalNoSuj: f64,
    pub totalExenta: f64,
    pub totalGravada: f64,
    pub subTotalVentas: f64,
    pub descuNoSuj: f64,
    pub descuExenta: f64,
    pub descuGravada: f64,
    pub porcentajeDescuento: f64,
    pub totalDescu: f64,
    pub tributos: Vec<serde_json::Value>,
    pub subTotal: f64,
    pub ivaRete1: f64,
    pub reteRenta: f64,
    pub montoTotalOperacion: f64,
    pub totalNoGravado: f64,
    pub totalPagar: f64,
    pub totalIva: f64,
    pub saldoFavor: f64,
    pub condicionOperacion: u8,
    pub totalLetras: String,
    pub pagos: Vec<Pago>,
    pub numPagoElectronico: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pago {
    pub codigo: String,
    pub montoPago: f64,
    pub plazo: String,
    pub referencia: String,
    pub periodo: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecepcionMh {
    pub version: u8,
    pub ambiente: String,
    pub versionApp: u8,
    pub estado: String,
    pub codigoGeneracion: String,
    pub selloRecibido: String,
    pub fhProcesamiento: String,
    pub clasificaMsg: String,
    pub codigoMsg: String,
    pub descripcionMsg: String,
    pub observaciones: Vec<serde_json::Value>,
}

/// Serializa un f64 con 2 decimales
pub fn serialize<S>(num: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("{:.2}", num))
}

/// Deserializa directamente como f64 (sin necesidad de formato)
pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    f64::deserialize(deserializer)
}