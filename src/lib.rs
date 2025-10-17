use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use qrcode::QrCode;
use qrcode::render::svg;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const HTML_TEMPLATE: &str = include_str!("../assets/factura.html");

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneracionResult {
    pub exitoso: bool,
    pub html: String,
    pub datos: FacturaResponse,
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

pub fn generar_html_y_json(factura: &mut Factura) -> Result<GeneracionResult, Vec<String>> {
    factura.validar()?;
    factura.calcular_totales();

    let timestamp = Utc::now().timestamp_millis();
    factura.codigo_control = Factura::generar_codigo_control(
        &factura.empresa.nit,
        &factura.numero_comprobante,
        timestamp
    );
    factura.codigo_verificacion = Factura::generar_codigo_verificacion(
        &factura.empresa.nit,
        &factura.numero_comprobante,
        factura.total
    );

    let qr_content = factura.link_verificacion.as_ref().cloned().unwrap_or_else(|| {
        format!(
            "{}|{}|{}|{:.2}|{}",
            factura.numero_comprobante,
            factura.codigo_control,
            factura.empresa.nit,
            factura.total,
            factura.codigo_verificacion
        )
    });

    let code = QrCode::new(qr_content.as_bytes())
        .map_err(|e| vec![format!("Error generando QR: {}", e)])?;
    let svg_qr = code.render::<svg::Color>()
        .min_dimensions(100, 100)
        .module_dimensions(2, 2)
        .build();

    let mut html = HTML_TEMPLATE.to_string();
    // La fecha ya viene del usuario, solo formateamos
    let fecha_str = factura.fecha_emision.format("%d/%m/%Y %H:%M:%S").to_string();

    let detalle_items: String = factura.detalle_venta.iter()
        .map(|item| {
            let total = item.cantidad * item.precio_unitario
                - item.descuento_item.unwrap_or(0.0)
                + item.impuesto_item.unwrap_or(0.0);
            let codigo = item.codigo.as_ref()
                .map(|c| format!("[{}]", c))
                .unwrap_or_default();
            let unidad = item.unidad_medida.as_ref()
                .map(|u| format!(" {}", u))
                .unwrap_or_default();
            format!(
                "<tr><td>{} {}</td><td style='text-align:center'>{:.2}{}</td><td style='text-align:right'>{:.2}</td><td style='text-align:right'>{:.2}</td></tr>",
                codigo, item.descripcion, item.cantidad, unidad, item.precio_unitario, total
            )
        })
        .collect();

    let cliente_info = factura.cliente.as_ref()
        .and_then(|c| {
            let nombre = c.nombre.as_ref().map(|s| s.trim()).unwrap_or("");
            let nit = c.nit_ci.as_ref().map(|s| s.trim()).unwrap_or("");
            if !nombre.is_empty() || !nit.is_empty() {
                let mut info = String::from("<div class='cliente-box'><strong>Cliente</strong><br>");
                if !nombre.is_empty() {
                    info.push_str(&format!("<span class='cliente-nombre'>{}</span>", nombre));
                }
                if !nit.is_empty() {
                    info.push_str(&format!("<br><small>NIT/CI: {}</small>", nit));
                }
                if let Some(dir) = &c.direccion {
                    if !dir.is_empty() {
                        info.push_str(&format!("<br><small>Dir: {}</small>", dir));
                    }
                }
                if let Some(ciudad) = &c.ciudad {
                    if !ciudad.is_empty() {
                        info.push_str(&format!("<br><small>Ciudad: {}</small>", ciudad));
                    }
                }
                info.push_str("</div>");
                Some(info)
            } else {
                None
            }
        })
        .unwrap_or_default();

    let empresa_info = {
        let mut info = String::new();
        if let Some(razon) = &factura.empresa.razon_social {
            if !razon.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Razón Social:</strong> {}</p>", razon));
            }
        }
        if let Some(regimen) = &factura.empresa.regimen_tributario {
            if !regimen.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Régimen:</strong> {}</p>", regimen));
            }
        }
        if let Some(dir) = &factura.empresa.direccion {
            if !dir.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Dirección:</strong> {}</p>", dir));
            }
        }
        if let Some(tel) = &factura.empresa.telefono {
            if !tel.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Tel:</strong> {}</p>", tel));
            }
        }
        if let Some(email) = &factura.empresa.email {
            if !email.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Email:</strong> {}</p>", email));
            }
        }
        if let Some(caja) = &factura.empresa.caja {
            if !caja.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Caja:</strong> {}</p>", caja));
            }
        }
        if let Some(pv) = &factura.empresa.punto_venta {
            if !pv.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>PV:</strong> {}</p>", pv));
            }
        }
        info
    };

    let descuentos_html = if factura.descuentos > 0.01 {
        format!("<div class='total-row'><span>Descuentos:</span><span>-{:.2}</span></div>", factura.descuentos)
    } else {
        String::new()
    };

    let impuestos_html = if factura.impuestos > 0.01 {
        format!("<div class='total-row'><span>Impuestos (IVA):</span><span>+{:.2}</span></div>", factura.impuestos)
    } else {
        String::new()
    };

    let envio_html = factura.costo_envio
        .filter(|&e| e > 0.01)
        .map(|e| format!("<div class='total-row'><span>Envío:</span><span>+{:.2}</span></div>", e))
        .unwrap_or_default();

    let monto_pagado_html = factura.monto_pagado
        .map(|p| format!("<div class='total-row'><span>Pago:</span><span>{:.2}</span></div>", p))
        .unwrap_or_default();

    let cambio_html = factura.cambio
        .filter(|&c| c > 0.01)
        .map(|c| format!("<div class='total-row'><span>Cambio:</span><span>{:.2}</span></div>", c))
        .unwrap_or_default();

    let metodo_pago_html = factura.metodo_pago.as_ref()
        .map(|m| format!("<p class='pago-box'><strong>Método:</strong> {}</p>", m))
        .unwrap_or_default();

    let tipo_comprobante_label = match factura.tipo_comprobante.as_str() {
        "recibo" => "RECIBO DE PAGO",
        "proforma" => "PROFORMA",
        "nota_venta" => "NOTA DE VENTA",
        "compra" => "COMPROBANTE COMPRA",
        _ => "COMPROBANTE INTERNO",
    };

    html = html.replace("{{TIPO_COMPROBANTE}}", tipo_comprobante_label);
    html = html.replace("{{EMPRESA_NOMBRE}}", &factura.empresa.nombre);
    html = html.replace("{{EMPRESA_NIT}}", &factura.empresa.nit);
    html = html.replace("{{EMPRESA_INFO}}", &empresa_info);
    html = html.replace("{{NUMERO_COMPROBANTE}}", &factura.numero_comprobante);
    html = html.replace("{{FECHA_EMISION}}", &fecha_str);
    html = html.replace("{{CODIGO_CONTROL}}", &factura.codigo_control);
    html = html.replace("{{CODIGO_VERIFICACION}}", &factura.codigo_verificacion);
    html = html.replace("{{MONEDA}}", &factura.moneda);
    html = html.replace("{{CLIENTE_INFO}}", &cliente_info);
    html = html.replace("{{DETALLE_ITEMS}}", &detalle_items);
    html = html.replace("{{SUBTOTAL}}", &format!("{:.2}", factura.subtotal));
    html = html.replace("{{DESCUENTOS}}", &descuentos_html);
    html = html.replace("{{IMPUESTOS}}", &impuestos_html);
    html = html.replace("{{ENVIO}}", &envio_html);
    html = html.replace("{{TOTAL}}", &format!("{:.2}", factura.total));
    html = html.replace("{{MONTO_PAGADO}}", &monto_pagado_html);
    html = html.replace("{{CAMBIO}}", &cambio_html);
    html = html.replace("{{QR_CODE}}", &svg_qr);
    html = html.replace("{{METODO_PAGO}}", &metodo_pago_html);

    let datos = FacturaResponse {
        numero_comprobante: factura.numero_comprobante.clone(),
        codigo_control: factura.codigo_control.clone(),
        codigo_verificacion: factura.codigo_verificacion.clone(),
        qr_data: qr_content,
        fecha_emision: fecha_str,
        total: factura.total,
        moneda: factura.moneda.clone(),
    };

    Ok(GeneracionResult {
        exitoso: true,
        html,
        datos,
        tiempo_ms: None, // No calculamos tiempo en WASM
    })
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
#[wasm_bindgen(start)]
pub fn main_wasm() {
    console_error_panic_hook::set_once();
}