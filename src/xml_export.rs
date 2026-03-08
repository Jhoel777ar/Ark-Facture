use crate::models::*;
use crate::validators::ValidadorBoliviano;

pub fn generar_xml(factura: &Factura) -> Result<String, String> {
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<Comprobante>\n");
    xml.push_str("  <Encabezado>\n");
    xml.push_str(&format!("    <NumeroComprobante>{}</NumeroComprobante>\n", factura.numero_comprobante));
    xml.push_str(&format!("    <CodigoControl>{}</CodigoControl>\n", factura.codigo_control));
    xml.push_str(&format!("    <CodigoVerificacion>{}</CodigoVerificacion>\n", factura.codigo_verificacion));
    xml.push_str(&format!("    <TipoComprobante>{}</TipoComprobante>\n", factura.tipo_comprobante));
    xml.push_str(&format!("    <FechaEmision>{}</FechaEmision>\n", factura.fecha_emision.to_rfc3339()));
    xml.push_str(&format!("    <Moneda>{}</Moneda>\n", factura.moneda));
    xml.push_str("  </Encabezado>\n");

    xml.push_str("  <Empresa>\n");
    xml.push_str(&format!("    <Nombre>{}</Nombre>\n", ValidadorBoliviano::sanitizar_html(&factura.empresa.nombre)));
    xml.push_str(&format!("    <NIT>{}</NIT>\n", factura.empresa.nit));
    if let Some(razon) = &factura.empresa.razon_social {
        xml.push_str(&format!("    <RazonSocial>{}</RazonSocial>\n", ValidadorBoliviano::sanitizar_html(razon)));
    }
    if let Some(dir) = &factura.empresa.direccion {
        xml.push_str(&format!("    <Direccion>{}</Direccion>\n", ValidadorBoliviano::sanitizar_html(dir)));
    }
    if let Some(tel) = &factura.empresa.telefono {
        xml.push_str(&format!("    <Telefono>{}</Telefono>\n", ValidadorBoliviano::sanitizar_html(tel)));
    }
    if let Some(email) = &factura.empresa.email {
        xml.push_str(&format!("    <Email>{}</Email>\n", ValidadorBoliviano::sanitizar_html(email)));
    }
    xml.push_str("  </Empresa>\n");

    if let Some(cliente) = &factura.cliente {
        xml.push_str("  <Cliente>\n");
        if let Some(nombre) = &cliente.nombre {
            xml.push_str(&format!("    <Nombre>{}</Nombre>\n", ValidadorBoliviano::sanitizar_html(nombre)));
        }
        if let Some(nit_ci) = &cliente.nit_ci {
            xml.push_str(&format!("    <NITCI>{}</NITCI>\n", ValidadorBoliviano::sanitizar_html(nit_ci)));
        }
        if let Some(dir) = &cliente.direccion {
            xml.push_str(&format!("    <Direccion>{}</Direccion>\n", ValidadorBoliviano::sanitizar_html(dir)));
        }
        if let Some(tel) = &cliente.telefono {
            xml.push_str(&format!("    <Telefono>{}</Telefono>\n", ValidadorBoliviano::sanitizar_html(tel)));
        }
        if let Some(email) = &cliente.email {
            xml.push_str(&format!("    <Email>{}</Email>\n", ValidadorBoliviano::sanitizar_html(email)));
        }
        xml.push_str("  </Cliente>\n");
    }

    xml.push_str("  <DetalleVenta>\n");
    for (idx, item) in factura.detalle_venta.iter().enumerate() {
        xml.push_str(&format!("    <Item numero=\"{}\">\n", idx + 1));
        xml.push_str(&format!("      <Descripcion>{}</Descripcion>\n", ValidadorBoliviano::sanitizar_html(&item.descripcion)));
        xml.push_str(&format!("      <Cantidad>{:.2}</Cantidad>\n", item.cantidad));
        xml.push_str(&format!("      <PrecioUnitario>{:.2}</PrecioUnitario>\n", item.precio_unitario));
        if let Some(codigo) = &item.codigo {
            xml.push_str(&format!("      <Codigo>{}</Codigo>\n", ValidadorBoliviano::sanitizar_html(codigo)));
        }
        if let Some(unidad) = &item.unidad_medida {
            xml.push_str(&format!("      <UnidadMedida>{}</UnidadMedida>\n", ValidadorBoliviano::sanitizar_html(unidad)));
        }
        let total = item.cantidad * item.precio_unitario
            - item.descuento_item.unwrap_or(0.0)
            + item.impuesto_item.unwrap_or(0.0);
        xml.push_str(&format!("      <Total>{:.2}</Total>\n", total));
        xml.push_str("    </Item>\n");
    }
    xml.push_str("  </DetalleVenta>\n");

    xml.push_str("  <Totales>\n");
    xml.push_str(&format!("    <Subtotal>{:.2}</Subtotal>\n", factura.subtotal));
    xml.push_str(&format!("    <Descuentos>{:.2}</Descuentos>\n", factura.descuentos));
    xml.push_str(&format!("    <Impuestos>{:.2}</Impuestos>\n", factura.impuestos));
    if let Some(envio) = factura.costo_envio {
        xml.push_str(&format!("    <CostoEnvio>{:.2}</CostoEnvio>\n", envio));
    }
    xml.push_str(&format!("    <Total>{:.2}</Total>\n", factura.total));
    xml.push_str("  </Totales>\n");

    if let Some(metodo) = &factura.metodo_pago {
        xml.push_str(&format!("  <MetodoPago>{}</MetodoPago>\n", ValidadorBoliviano::sanitizar_html(metodo)));
    }
    if let Some(notas) = &factura.notas {
        xml.push_str(&format!("  <Notas>{}</Notas>\n", ValidadorBoliviano::sanitizar_html(notas)));
    }

    xml.push_str("</Comprobante>\n");
    Ok(xml)
}
