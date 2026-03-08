use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use qrcode::QrCode;
use qrcode::render::svg;
use regex::Regex;
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod models;
pub mod validators;
pub mod generators;
pub mod xml_export;
pub mod security;
pub mod pdf_export;

#[cfg(test)]
mod tests;

pub use models::*;
pub use validators::*;
pub use generators::*;
pub use xml_export::*;
pub use security::*;
pub use pdf_export::*;

const HTML_TEMPLATE: &str = include_str!("../assets/factura.html");
const VERSION: &str = "4.0.0";

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn obtener_version() -> String {
    VERSION.to_string()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generar_comprobante_wasm(factura_json: &str) -> String {
    console_error_panic_hook::set_once();
    
    match serde_json::from_str::<Factura>(factura_json) {
        Ok(mut factura) => {
            match generar_html_y_json(&mut factura) {
                Ok(resultado) => {
                    serde_json::to_string(&resultado).unwrap_or_else(|_| 
                        serde_json::json!({
                            "exitoso": false,
                            "error": "Error serializando resultado"
                        }).to_string()
                    )
                },
                Err(errores) => {
                    serde_json::json!({
                        "exitoso": false,
                        "errores": errores
                    }).to_string()
                }
            }
        },
        Err(e) => {
            serde_json::json!({
                "exitoso": false,
                "error": format!("Error parseando JSON: {}", e)
            }).to_string()
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn validar_comprobante_wasm(factura_json: &str) -> String {
    console_error_panic_hook::set_once();
    
    match serde_json::from_str::<Factura>(factura_json) {
        Ok(factura) => {
            match factura.validar() {
                Ok(_) => serde_json::json!({
                    "valido": true,
                    "mensaje": "Comprobante válido"
                }).to_string(),
                Err(errores) => serde_json::json!({
                    "valido": false,
                    "errores": errores
                }).to_string()
            }
        },
        Err(e) => serde_json::json!({
            "valido": false,
            "error": format!("Error: {}", e)
        }).to_string()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generar_xml_wasm(factura_json: &str) -> String {
    console_error_panic_hook::set_once();
    
    match serde_json::from_str::<Factura>(factura_json) {
        Ok(mut factura) => {
            factura.validar().ok();
            factura.calcular_totales();
            match generar_xml(&factura) {
                Ok(xml) => serde_json::json!({
                    "exitoso": true,
                    "xml": xml
                }).to_string(),
                Err(e) => serde_json::json!({
                    "exitoso": false,
                    "error": e
                }).to_string()
            }
        },
        Err(e) => serde_json::json!({
            "exitoso": false,
            "error": format!("Error parseando JSON: {}", e)
        }).to_string()
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_wasm() {
    console_error_panic_hook::set_once();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generar_pdf_wasm(html: &str) -> String {
    console_error_panic_hook::set_once();
    
    match pdf_export::generar_pdf_base64(html) {
        Ok(pdf_base64) => serde_json::json!({
            "exitoso": true,
            "pdf_base64": pdf_base64,
            "instrucciones": "Usa html2pdf.js o jsPDF en el cliente para convertir a PDF"
        }).to_string(),
        Err(e) => serde_json::json!({
            "exitoso": false,
            "error": e
        }).to_string()
    }
}
