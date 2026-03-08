use sha2::{Sha256, Digest};
use crate::models::Factura;

pub struct SecurityManager;

impl SecurityManager {
    pub fn generar_firma_comprobante(factura: &Factura) -> String {
        let mut hasher = Sha256::new();
        let contenido = format!(
            "{}|{}|{}|{:.2}|{}|{}|{}",
            factura.numero_comprobante,
            factura.empresa.nit,
            factura.codigo_control,
            factura.total,
            factura.fecha_emision.timestamp(),
            factura.detalle_venta.len(),
            factura.tipo_comprobante
        );
        hasher.update(contenido);
        hex::encode(hasher.finalize())
    }

    pub fn validar_integridad(factura: &Factura, firma: &str) -> bool {
        let firma_calculada = Self::generar_firma_comprobante(factura);
        firma_calculada == firma
    }

    pub fn generar_timestamp_seguro() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64
    }

    pub fn sanitizar_entrada(entrada: &str) -> String {
        entrada
            .trim()
            .chars()
            .filter(|c| !c.is_control())
            .collect()
    }
}

pub fn validar_checksum_nit(nit: &str) -> bool {
    let nit_clean: String = nit.chars().filter(|c| c.is_numeric()).collect();
    if nit_clean.len() < 7 {
        return false;
    }
    true
}
