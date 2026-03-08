use crate::models::*;
use crate::validators::ValidadorBoliviano;
use chrono::Utc;
use qrcode::QrCode;
use qrcode::render::svg;
use sha2::{Sha256, Digest};

const HTML_TEMPLATE: &str = include_str!("../assets/factura.html");
const VERSION: &str = "4.0.0";

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
    let fecha_str = factura.fecha_emision.format("%d/%m/%Y %H:%M:%S").to_string();

    let detalle_items: String = factura.detalle_venta.iter()
        .map(|item| {
            let total = item.cantidad * item.precio_unitario
                - item.descuento_item.unwrap_or(0.0)
                + item.impuesto_item.unwrap_or(0.0);
            let codigo = item.codigo.as_ref()
                .map(|c| format!("[{}]", ValidadorBoliviano::sanitizar_html(c)))
                .unwrap_or_default();
            let unidad = item.unidad_medida.as_ref()
                .map(|u| format!(" {}", u))
                .unwrap_or_default();
            format!(
                "<tr><td>{} {}</td><td style='text-align:center'>{:.2}{}</td><td style='text-align:right'>{:.2}</td><td style='text-align:right'>{:.2}</td></tr>",
                codigo, ValidadorBoliviano::sanitizar_html(&item.descripcion), item.cantidad, unidad, item.precio_unitario, total
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
                    info.push_str(&format!("<span class='cliente-nombre'>{}</span>", ValidadorBoliviano::sanitizar_html(nombre)));
                }
                if !nit.is_empty() {
                    info.push_str(&format!("<br><small>NIT/CI: {}</small>", ValidadorBoliviano::sanitizar_html(nit)));
                }
                if let Some(dir) = &c.direccion {
                    if !dir.is_empty() {
                        info.push_str(&format!("<br><small>Dir: {}</small>", ValidadorBoliviano::sanitizar_html(dir)));
                    }
                }
                if let Some(ciudad) = &c.ciudad {
                    if !ciudad.is_empty() {
                        info.push_str(&format!("<br><small>Ciudad: {}</small>", ValidadorBoliviano::sanitizar_html(ciudad)));
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
                info.push_str(&format!("<p class='empresa-detalle'><strong>Razón Social:</strong> {}</p>", ValidadorBoliviano::sanitizar_html(razon)));
            }
        }
        if let Some(regimen) = &factura.empresa.regimen_tributario {
            if !regimen.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Régimen:</strong> {}</p>", ValidadorBoliviano::sanitizar_html(regimen)));
            }
        }
        if let Some(dir) = &factura.empresa.direccion {
            if !dir.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Dirección:</strong> {}</p>", ValidadorBoliviano::sanitizar_html(dir)));
            }
        }
        if let Some(tel) = &factura.empresa.telefono {
            if !tel.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Tel:</strong> {}</p>", ValidadorBoliviano::sanitizar_html(tel)));
            }
        }
        if let Some(email) = &factura.empresa.email {
            if !email.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Email:</strong> {}</p>", ValidadorBoliviano::sanitizar_html(email)));
            }
        }
        if let Some(caja) = &factura.empresa.caja {
            if !caja.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>Caja:</strong> {}</p>", ValidadorBoliviano::sanitizar_html(caja)));
            }
        }
        if let Some(pv) = &factura.empresa.punto_venta {
            if !pv.is_empty() {
                info.push_str(&format!("<p class='empresa-detalle'><strong>PV:</strong> {}</p>", ValidadorBoliviano::sanitizar_html(pv)));
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
        .map(|m| format!("<p class='pago-box'><strong>Método:</strong> {}</p>", ValidadorBoliviano::sanitizar_html(m)))
        .unwrap_or_default();

    let tipo_comprobante_label = match factura.tipo_comprobante.as_str() {
        "recibo" => "RECIBO DE PAGO",
        "proforma" => "PROFORMA",
        "nota_venta" => "NOTA DE VENTA",
        "compra" => "COMPROBANTE COMPRA",
        "nota_credito" => "NOTA DE CRÉDITO",
        "nota_debito" => "NOTA DE DÉBITO",
        _ => "COMPROBANTE INTERNO",
    };

    html = html.replace("{{TIPO_COMPROBANTE}}", tipo_comprobante_label);
    html = html.replace("{{EMPRESA_NOMBRE}}", &ValidadorBoliviano::sanitizar_html(&factura.empresa.nombre));
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

    let hash_integridad = calcular_hash_integridad(factura);

    let datos = FacturaResponse {
        numero_comprobante: factura.numero_comprobante.clone(),
        codigo_control: factura.codigo_control.clone(),
        codigo_verificacion: factura.codigo_verificacion.clone(),
        qr_data: qr_content,
        fecha_emision: fecha_str,
        total: factura.total,
        moneda: factura.moneda.clone(),
        hash_integridad,
    };

    Ok(GeneracionResult {
        exitoso: true,
        html,
        datos,
        version: VERSION.to_string(),
        tiempo_ms: None,
    })
}

pub fn calcular_hash_integridad(factura: &Factura) -> String {
    let mut hasher = Sha256::new();
    let contenido = format!(
        "{}|{}|{}|{:.2}|{}|{}",
        factura.numero_comprobante,
        factura.empresa.nit,
        factura.detalle_venta.len(),
        factura.total,
        factura.fecha_emision.timestamp(),
        factura.tipo_comprobante
    );
    hasher.update(contenido);
    hex::encode(hasher.finalize())
}
