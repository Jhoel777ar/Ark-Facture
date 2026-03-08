use crate::models::Factura;
use base64::Engine;

/// Genera un PDF base64 a partir del HTML
/// En WASM, esto devuelve el HTML que puede ser convertido a PDF en el cliente
pub fn generar_pdf_base64(html: &str) -> Result<String, String> {
    // En WASM no podemos generar PDF directamente
    // Devolvemos el HTML en base64 para que el cliente lo convierta
    let html_bytes = html.as_bytes();
    let encoded = base64::engine::general_purpose::STANDARD.encode(html_bytes);
    Ok(encoded)
}

/// Estructura para respuesta de PDF
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PdfResponse {
    pub exitoso: bool,
    pub pdf_base64: Option<String>,
    pub html: Option<String>,
    pub error: Option<String>,
}

/// Genera PDF en formato base64 (para cliente)
pub fn generar_pdf_para_cliente(html: &str) -> PdfResponse {
    match generar_pdf_base64(html) {
        Ok(pdf_base64) => PdfResponse {
            exitoso: true,
            pdf_base64: Some(pdf_base64),
            html: Some(html.to_string()),
            error: None,
        },
        Err(e) => PdfResponse {
            exitoso: false,
            pdf_base64: None,
            html: None,
            error: Some(e),
        },
    }
}
