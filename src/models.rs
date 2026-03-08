use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Empresa {
    pub nombre: String,
    pub nit: String,
    pub razon_social: Option<String>,
    pub sucursal: Option<String>,
    pub encargado: Option<String>,
    pub telefono: Option<String>,
    pub atencion_cliente: Option<String>,
    pub direccion: Option<String>,
    pub email: Option<String>,
    pub ciudad: Option<String>,
    pub pais: Option<String>,
    pub sitio_web: Option<String>,
    pub caja: Option<String>,
    pub punto_venta: Option<String>,
    pub numero_licencia: Option<String>,
    pub regimen_tributario: Option<String>,
    pub logo_svg: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cliente {
    pub nombre: Option<String>,
    pub nit_ci: Option<String>,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub empresa: Option<String>,
    pub ciudad: Option<String>,
    pub codigo_postal: Option<String>,
    pub pais: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetalleVenta {
    pub descripcion: String,
    pub cantidad: f64,
    pub precio_unitario: f64,
    pub codigo: Option<String>,
    pub categoria: Option<String>,
    pub descuento_item: Option<f64>,
    pub impuesto_item: Option<f64>,
    pub unidad_medida: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factura {
    pub numero_comprobante: String,
    pub codigo_control: String,
    pub codigo_verificacion: String,
    pub empresa: Empresa,
    pub cliente: Option<Cliente>,
    pub detalle_venta: Vec<DetalleVenta>,
    pub fecha_emision: DateTime<Utc>,
    pub fecha_vencimiento: Option<DateTime<Utc>>,
    pub moneda: String,
    pub subtotal: f64,
    pub impuestos: f64,
    pub descuentos: f64,
    pub total: f64,
    pub monto_pagado: Option<f64>,
    pub cambio: Option<f64>,
    pub metodo_pago: Option<String>,
    pub notas: Option<String>,
    pub usuario_atendio: Option<String>,
    pub tipo_entrega: Option<String>,
    pub costo_envio: Option<f64>,
    pub numero_orden: Option<String>,
    pub link_verificacion: Option<String>,
    pub tipo_comprobante: String,
    pub locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneracionResult {
    pub exitoso: bool,
    pub html: String,
    pub datos: FacturaResponse,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiempo_ms: Option<u128>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FacturaResponse {
    pub numero_comprobante: String,
    pub codigo_control: String,
    pub codigo_verificacion: String,
    pub qr_data: String,
    pub fecha_emision: String,
    pub total: f64,
    pub moneda: String,
    pub hash_integridad: String,
}

impl Factura {
    pub fn generar_codigo_control(nit: &str, numero: &str, fecha_ms: i64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}_{}_{}", nit, numero, fecha_ms));
        let resultado = hasher.finalize();
        hex::encode(&resultado[0..16]).to_uppercase()
    }

    pub fn generar_codigo_verificacion(nit: &str, numero: &str, total: f64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}_{}_{}_{}", nit, numero, total, Utc::now().timestamp()));
        let resultado = hasher.finalize();
        hex::encode(&resultado[0..20]).to_uppercase()
    }

    pub fn generar_numero_comprobante(nit: &str) -> String {
        let timestamp = Utc::now().timestamp_millis();
        let nit_digits: String = nit.chars().filter(|c| c.is_numeric()).collect();
        let prefix = nit_digits.chars().take(6).collect::<String>();
        format!("{}{}", prefix, timestamp % 100000000000)
    }

    pub fn calcular_totales(&mut self) {
        self.subtotal = self.detalle_venta.iter()
            .map(|item| {
                let total_item = item.cantidad * item.precio_unitario;
                let descuento = item.descuento_item.unwrap_or(0.0);
                let impuesto = item.impuesto_item.unwrap_or(0.0);
                total_item - descuento + impuesto
            })
            .sum::<f64>();

        let subtotal_base = self.subtotal + self.descuentos;
        let costo_envio = self.costo_envio.unwrap_or(0.0);
        self.total = ((subtotal_base + self.impuestos - self.descuentos + costo_envio) * 100.0).round() / 100.0;

        if let Some(pagado) = self.monto_pagado {
            let cambio_calc = pagado - self.total;
            if cambio_calc > 0.001 {
                self.cambio = Some((cambio_calc * 100.0).round() / 100.0);
            }
        }
    }

    pub fn validar(&self) -> Result<(), Vec<String>> {
        let mut errores = Vec::new();

        if self.empresa.nombre.trim().is_empty() {
            errores.push("Nombre de empresa es obligatorio".to_string());
        }
        if self.empresa.nit.trim().is_empty() {
            errores.push("NIT de empresa es obligatorio".to_string());
        }
        if self.detalle_venta.is_empty() {
            errores.push("Debe haber al menos un artículo en la venta".to_string());
        }

        for (idx, item) in self.detalle_venta.iter().enumerate() {
            if item.descripcion.trim().is_empty() {
                errores.push(format!("Descripción del artículo {} vacía", idx + 1));
            }
            if item.cantidad <= 0.0 {
                errores.push(format!("Cantidad del artículo {} debe ser > 0", idx + 1));
            }
            if item.precio_unitario < 0.0 {
                errores.push(format!("Precio del artículo {} no válido", idx + 1));
            }
        }

        if errores.is_empty() { Ok(()) } else { Err(errores) }
    }
}
