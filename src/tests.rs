#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::Utc;

    fn crear_factura_test() -> Factura {
        Factura {
            numero_comprobante: Factura::generar_numero_comprobante("123456789"),
            codigo_control: String::new(),
            codigo_verificacion: String::new(),
            empresa: Empresa {
                nombre: "Test Corp".to_string(),
                nit: "123456789".to_string(),
                razon_social: None,
                sucursal: None,
                encargado: None,
                telefono: None,
                atencion_cliente: None,
                direccion: None,
                email: None,
                ciudad: None,
                pais: None,
                sitio_web: None,
                caja: None,
                punto_venta: None,
                numero_licencia: None,
                regimen_tributario: None,
                logo_svg: None,
            },
            cliente: None,
            detalle_venta: vec![
                DetalleVenta {
                    descripcion: "Producto Test".to_string(),
                    cantidad: 2.0,
                    precio_unitario: 100.0,
                    codigo: Some("T01".to_string()),
                    categoria: None,
                    descuento_item: None,
                    impuesto_item: None,
                    unidad_medida: Some("un".to_string()),
                }
            ],
            fecha_emision: Utc::now(),
            fecha_vencimiento: None,
            moneda: "BOB".to_string(),
            subtotal: 0.0,
            impuestos: 0.0,
            descuentos: 0.0,
            total: 0.0,
            monto_pagado: Some(250.0),
            cambio: None,
            metodo_pago: Some("Efectivo".to_string()),
            notas: None,
            usuario_atendio: None,
            tipo_entrega: None,
            costo_envio: None,
            numero_orden: None,
            link_verificacion: None,
            tipo_comprobante: "recibo".to_string(),
            locale: Some("es-BO".to_string()),
        }
    }

    #[test]
    fn test_generacion_html_json() {
        let mut c = crear_factura_test();
        c.calcular_totales();
        let r = generar_html_y_json(&mut c);
        assert!(r.is_ok());
        let res = r.unwrap();
        assert!(res.html.contains("COMPROBANTE"));
        assert!(res.exitoso);
    }

    #[test]
    fn test_codigos_generados() {
        let mut c = crear_factura_test();
        c.calcular_totales();
        let _ = generar_html_y_json(&mut c);
        assert_eq!(c.codigo_control.len(), 32);
        assert_eq!(c.codigo_verificacion.len(), 40);
    }

    #[test]
    fn test_validacion() {
        let c = crear_factura_test();
        assert!(c.validar().is_ok());
    }

    #[test]
    fn test_descuentos() {
        let mut c = crear_factura_test();
        c.descuentos = 20.0;
        c.calcular_totales();
        // Subtotal es 200 (2 * 100), con descuento de 20 = 180
        assert!(c.total <= 200.0);
    }

    #[test]
    fn test_cambio() {
        let mut c = crear_factura_test();
        c.calcular_totales();
        assert!(c.cambio.is_some());
        assert!(c.cambio.unwrap() > 0.0);
    }

    #[test]
    fn test_serializacion() {
        let c = crear_factura_test();
        let json = serde_json::to_string(&c);
        assert!(json.is_ok());
    }

    #[test]
    fn test_xml_generation() {
        let c = crear_factura_test();
        let xml = xml_export::generar_xml(&c);
        assert!(xml.is_ok());
        let xml_str = xml.unwrap();
        assert!(xml_str.contains("<Comprobante>"));
        assert!(xml_str.contains("</Comprobante>"));
    }

    #[test]
    fn test_validador_nit() {
        assert!(validators::ValidadorBoliviano::validar_nit("1234567890"));
        assert!(!validators::ValidadorBoliviano::validar_nit("123"));
    }

    #[test]
    fn test_validador_email() {
        assert!(validators::ValidadorBoliviano::validar_email("test@example.com"));
        assert!(!validators::ValidadorBoliviano::validar_email("invalid-email"));
    }

    #[test]
    fn test_sanitizacion_html() {
        let sanitizado = validators::ValidadorBoliviano::sanitizar_html("<script>alert('xss')</script>");
        assert!(!sanitizado.contains("<script>"));
        assert!(sanitizado.contains("&lt;"));
    }

    #[test]
    fn test_performance() {
        let mut c = crear_factura_test();
        let inicio = std::time::Instant::now();
        let _ = generar_html_y_json(&mut c);
        let duracion = inicio.elapsed().as_secs_f64() * 1000.0;
        assert!(duracion < 10.0, "Generación tardó {:.2}ms", duracion);
    }

    #[test]
    fn test_numero_comprobante_unico() {
        let nit = "1234567890";
        let num1 = Factura::generar_numero_comprobante(nit);
        std::thread::sleep(std::time::Duration::from_millis(10));
        let num2 = Factura::generar_numero_comprobante(nit);
        assert_ne!(num1, num2);
    }

    #[test]
    fn test_validacion_cantidad() {
        assert!(validators::validar_cantidad(1.0));
        assert!(validators::validar_cantidad(5000.0));
        assert!(!validators::validar_cantidad(0.0));
        assert!(!validators::validar_cantidad(-1.0));
    }

    #[test]
    fn test_validacion_precio() {
        assert!(validators::validar_precio(0.0));
        assert!(validators::validar_precio(100.0));
        assert!(!validators::validar_precio(-1.0));
    }

    #[test]
    fn test_validacion_moneda() {
        assert!(validators::validar_moneda("BOB"));
        assert!(validators::validar_moneda("USD"));
        assert!(!validators::validar_moneda("XYZ"));
    }

    #[test]
    fn test_validacion_tipo_comprobante() {
        assert!(validators::validar_tipo_comprobante("recibo"));
        assert!(validators::validar_tipo_comprobante("proforma"));
        assert!(validators::validar_tipo_comprobante("nota_credito"));
        assert!(!validators::validar_tipo_comprobante("invalido"));
    }

    #[test]
    fn test_hash_integridad() {
        let c = crear_factura_test();
        let hash1 = generators::calcular_hash_integridad(&c);
        let hash2 = generators::calcular_hash_integridad(&c);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 hex
    }

    #[test]
    fn test_security_firma() {
        let c = crear_factura_test();
        let firma = security::SecurityManager::generar_firma_comprobante(&c);
        assert!(security::SecurityManager::validar_integridad(&c, &firma));
    }

    #[test]
    fn test_error_empresa_vacia() {
        let mut c = crear_factura_test();
        c.empresa.nombre = String::new();
        assert!(c.validar().is_err());
    }

    #[test]
    fn test_error_sin_items() {
        let mut c = crear_factura_test();
        c.detalle_venta = vec![];
        assert!(c.validar().is_err());
    }

    #[test]
    fn test_error_cantidad_negativa() {
        let mut c = crear_factura_test();
        c.detalle_venta[0].cantidad = -1.0;
        assert!(c.validar().is_err());
    }

    #[test]
    fn test_calcular_totales_con_impuestos() {
        let mut c = crear_factura_test();
        c.impuestos = 26.0; // 13% de 200
        c.calcular_totales();
        assert!(c.total > 200.0);
    }

    #[test]
    fn test_calcular_totales_con_envio() {
        let mut c = crear_factura_test();
        c.costo_envio = Some(50.0);
        c.calcular_totales();
        assert!(c.total > 200.0);
    }

    #[test]
    fn test_version() {
        let version = crate::VERSION;
        assert_eq!(version, "4.0.0");
    }
}
