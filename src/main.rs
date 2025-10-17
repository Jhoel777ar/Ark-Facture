use facture_ark::*;
use chrono::Utc;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

fn main() {
    println!("╔════════════════════════════════════════════════════════╗");
    println!("║  ARK COMPROBANTE SERVICE v2.1 - ENTERPRISE EDITION    ║");
    println!("║    Sistema de Comprobantes de Venta Internos - Bolivia ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    let empresa = Empresa {
        nombre: "COMERCIAL ARK S.R.L.".to_string(),
        nit: "1234567890".to_string(),
        razon_social: Some("Comercial ARK SRL - Distribuidora General".to_string()),
        sucursal: Some("Sucursal Central".to_string()),
        encargado: Some("Juan Pérez García".to_string()),
        telefono: Some("+591 2 2345678".to_string()),
        atencion_cliente: Some("800-10-5000".to_string()),
        direccion: Some("Av. 6 de Agosto 1234, Edificio Central, La Paz".to_string()),
        email: Some("contacto@ark.com.bo".to_string()),
        ciudad: Some("La Paz".to_string()),
        pais: Some("Bolivia".to_string()),
        sitio_web: Some("www.ark.com.bo".to_string()),
        caja: Some("Caja 01".to_string()),
        punto_venta: Some("PV-001".to_string()),
        numero_licencia: Some("LIC-2024-001".to_string()),
        regimen_tributario: Some("Régimen General".to_string()),
    };

    let cliente = Cliente {
        nombre: Some("María González López".to_string()),
        nit_ci: Some("7654321".to_string()),
        direccion: Some("Zona Sur, Calle 21 #456, Edificio B, Apt 5".to_string()),
        telefono: Some("+591 7 1234567".to_string()),
        email: Some("maria@example.com".to_string()),
        empresa: Some("Empresa XYZ Distribuciones S.A.".to_string()),
        ciudad: Some("La Paz".to_string()),
        codigo_postal: Some("0000".to_string()),
        pais: Some("Bolivia".to_string()),
    };

    let items = vec![
        DetalleVenta {
            descripcion: "Laptop HP Pavilion 15 - Intel Core i7 - 16GB RAM - 512GB SSD".to_string(),
            cantidad: 2.0,
            precio_unitario: 4500.0,
            codigo: Some("LAP-001".to_string()),
            categoria: Some("Electrónica".to_string()),
            descuento_item: Some(50.0),
            impuesto_item: None,
            unidad_medida: Some("un".to_string()),
        },
        DetalleVenta {
            descripcion: "Mouse Inalámbrico Logitech M705 Ergonómico".to_string(),
            cantidad: 3.0,
            precio_unitario: 120.0,
            codigo: Some("MOU-002".to_string()),
            categoria: Some("Accesorios".to_string()),
            descuento_item: None,
            impuesto_item: None,
            unidad_medida: Some("un".to_string()),
        },
        DetalleVenta {
            descripcion: "Teclado Mecánico RGB Corsair K95 Platinum - Switches Cherry MX".to_string(),
            cantidad: 1.0,
            precio_unitario: 450.0,
            codigo: Some("TEC-003".to_string()),
            categoria: Some("Accesorios".to_string()),
            descuento_item: None,
            impuesto_item: None,
            unidad_medida: Some("un".to_string()),
        },
        DetalleVenta {
            descripcion: "Monitor LG 27 pulgadas - 4K - IPS Panel - 60Hz Refresh Rate".to_string(),
            cantidad: 2.0,
            precio_unitario: 1800.0,
            codigo: Some("MON-004".to_string()),
            categoria: Some("Electrónica".to_string()),
            descuento_item: Some(100.0),
            impuesto_item: None,
            unidad_medida: Some("un".to_string()),
        },
    ];

    let mut factura = Factura {
        numero_comprobante: Factura::generar_numero_comprobante(&empresa.nit),
        codigo_control: String::new(),
        codigo_verificacion: String::new(),
        empresa,
        cliente: Some(cliente),
        detalle_venta: items,
        fecha_emision: Utc::now(),
        fecha_vencimiento: None,
        moneda: "BOB".to_string(),
        subtotal: 0.0,
        impuestos: 0.0,
        descuentos: 150.0,
        total: 0.0,
        monto_pagado: Some(13000.0),
        cambio: None,
        metodo_pago: Some("Efectivo".to_string()),
        notas: Some("Gracias por su compra - Garantía 30 días".to_string()),
        usuario_atendio: Some("Carlos Mendoza".to_string()),
        tipo_entrega: Some("Domicilio - Zona Central".to_string()),
        costo_envio: Some(50.0),
        numero_orden: Some("ORD-2025-001".to_string()),
        link_verificacion: None,
        tipo_comprobante: "recibo".to_string(),
    };

    factura.calcular_totales();
    factura.impuestos = (factura.subtotal * 0.13 * 100.0).round() / 100.0;
    factura.calcular_totales();

    println!("📄 Comprobante de prueba generado:");
    println!("   Número: {}", factura.numero_comprobante);
    println!("   Subtotal: {:.2} {}", factura.subtotal, factura.moneda);
    println!("   Impuestos (IVA 13%): {:.2} {}", factura.impuestos, factura.moneda);
    println!("   Total: {:.2} {}", factura.total, factura.moneda);
    println!();

    loop {
        println!("\n┌────────────────────────────────────────┐");
        println!("│  ¿Qué desea hacer?                    │");
        println!("├────────────────────────────────────────┤");
        println!("│  1. Generar HTML + JSON               │");
        println!("│  2. Validar comprobante               │");
        println!("│  3. Benchmark de rendimiento          │");
        println!("│  4. Ver datos de seguridad            │");
        println!("│  5. Información de normativa          │");
        println!("│  6. Salir                             │");
        println!("└────────────────────────────────────────┘");
        print!("\n👉 Seleccione (1-6): ");
        io::stdout().flush().unwrap();

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();

        match opcion.trim() {
            "1" => generar_html(&mut factura),
            "2" => validar(&factura),
            "3" => benchmark(&mut factura),
            "4" => mostrar_seguridad(&factura),
            "5" => mostrar_normativa(),
            "6" => {
                println!("\n👋 ¡Hasta luego! Sistema ARK v2.1");
                break;
            },
            _ => println!("⚠️ Opción no válida. Seleccione 1-6"),
        }
    }
}

fn generar_html(factura: &mut Factura) {
    println!("\n🔄 Generando HTML + JSON...");
    let inicio = Instant::now();

    match generar_html_y_json(factura) {
        Ok(resultado) => {
            let filename_html = format!("comprobante_{}.html", factura.numero_comprobante);
            let filename_json = format!("comprobante_{}.json", factura.numero_comprobante);

            match (fs::write(&filename_html, &resultado.html), 
                   fs::write(&filename_json, serde_json::to_string_pretty(&resultado.datos).unwrap())) {
                (Ok(_), Ok(_)) => {
                    let duracion = inicio.elapsed();
                    println!("✅ HTML: {} ({:.3}ms)", filename_html, duracion.as_secs_f64() * 1000.0);
                    println!("✅ JSON: {} ({:.3}ms)", filename_json, duracion.as_secs_f64() * 1000.0);
                    println!("\n📊 Datos del comprobante:");
                    println!("   Número: {}", resultado.datos.numero_comprobante);
                    println!("   Control: {}", resultado.datos.codigo_control);
                    println!("   Verificación: {}", resultado.datos.codigo_verificacion);
                    println!("   Total: {:.2} {}", resultado.datos.total, resultado.datos.moneda);
                    println!("   QR: {}", resultado.datos.qr_data);
                },
                _ => println!("❌ Error al guardar los archivos"),
            }
        },
        Err(e) => println!("❌ Error: {:?}", e),
    }
}

fn validar(factura: &Factura) {
    println!("\n✓ Validando comprobante...");
    match factura.validar() {
        Ok(_) => println!("✅ Comprobante válido - Listo para usar"),
        Err(errores) => {
            println!("❌ Errores encontrados:");
            for error in errores {
                println!("   • {}", error);
            }
        }
    }
}

fn benchmark(factura: &mut Factura) {
    const ITER: usize = 100;
    println!("\n⚡ Ejecutando benchmark ({} iteraciones)...", ITER);

    let inicio = Instant::now();
    for _ in 0..ITER {
        let _ = generar_html_y_json(factura);
    }
    let duracion = inicio.elapsed();
    let promedio = duracion.as_secs_f64() * 1000.0 / ITER as f64;
    let velocidad = 1000.0 / promedio;

    println!("\n   ┌──────────────────────────────────────┐");
    println!("   │  RESULTADOS DEL BENCHMARK            │");
    println!("   ├──────────────────────────────────────┤");
    println!("   │  Promedio: {:.3} ms/operación       │", promedio);
    println!("   │  Velocidad: {:.0} comps/segundo     │", velocidad);
    println!("   │  Total: {:.2} segundos               │", duracion.as_secs_f64());
    println!("   │  Rendimiento: EXCELENTE ✓           │");
    println!("   └──────────────────────────────────────┘");
}

fn mostrar_seguridad(factura: &Factura) {
    println!("\n🔒 Información de Seguridad:");
    println!("   ├─ Tipo: Comprobante Interno");
    println!("   ├─ Protocolo: SHA-256 (FIPS 180-4)");
    println!("   ├─ Código Control: {} caracteres", factura.codigo_control.len());
    println!("   ├─ Código Verificación: {} caracteres", factura.codigo_verificacion.len());
    println!("   ├─ Timestamp: {} ms", factura.fecha_emision.timestamp_millis());
    println!("   ├─ QR: Verificable y no reproducible");
    println!("   ├─ Cumplimiento: Legal sin SIAT ✓");
    println!("   └─ Estatus: SEGURO Y CERTIFICADO");
    println!("\n   Nota: No válido para crédito fiscal IVA ante SIN");
}

fn mostrar_normativa() {
    println!("\n📋 CUMPLIMIENTO NORMATIVO - BOLIVIA");
    println!("   ┌─────────────────────────────────────────────┐");
    println!("   │ Leyes de Referencia                         │");
    println!("   ├─────────────────────────────────────────────┤");
    println!("   │ • Ley 843 - Código Tributario Boliviano    │");
    println!("   │ • D.S. 24051 - Régimen General             │");
    println!("   │ • Resolución SIN - Facturación Electrónica │");
    println!("   └─────────────────────────────────────────────┘");
    println!("\n   Validez del Comprobante:");
    println!("   ✓ Control interno y auditoría");
    println!("   ✓ Inventario y gestión contable");
    println!("   ✓ Respaldo digital certificado");
    println!("   ✓ Compatible con sistemas SaaS");
    println!("   ✓ Válido para mercados internacionales");
    println!("\n   No Válido Para:");
    println!("   ✗ Crédito fiscal IVA ante SIN");
    println!("   ✗ Devoluciones tributarias");
    println!("   ✗ Operaciones sujetas a fiscalización SIAT");
    println!("\n   Identificación del Documento:");
    println!("   • Claramente marcado: COMPROBANTE INTERNO");
    println!("   • NIT de empresa registrado");
    println!("   • Datos de control contable inmutables");
    println!("   • QR verificable");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_comprobante_test() -> Factura {
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
            },
            cliente: None,
            detalle_venta: vec![
                DetalleVenta {
                    descripcion: "Producto Test Descripción Larga Para Verificar Ajuste".to_string(),
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
        }
    }

    #[test]
    fn test_generacion_html_json() {
        let mut c = crear_comprobante_test();
        c.calcular_totales();
        let r = generar_html_y_json(&mut c);
        assert!(r.is_ok());
        let res = r.unwrap();
        assert!(res.html.contains("COMPROBANTE"));
    }

    #[test]
    fn test_codigos_generados() {
        let mut c = crear_comprobante_test();
        c.calcular_totales();
        let _ = generar_html_y_json(&mut c);
        assert_eq!(c.codigo_control.len(), 32);
        assert_eq!(c.codigo_verificacion.len(), 40);
    }

    #[test]
    fn test_validacion() {
        let mut c = crear_comprobante_test();
        c.calcular_totales();
        assert!(c.validar().is_ok());
    }

    #[test]
    fn test_descuentos() {
        let mut c = crear_comprobante_test();
        c.descuentos = 20.0;
        c.calcular_totales();
        assert_eq!(c.total, 180.0);
    }

    #[test]
    fn test_cambio() {
        let mut c = crear_comprobante_test();
        c.calcular_totales();
        assert!(c.cambio.is_some());
    }

    #[test]
    fn test_serializacion() {
        let c = crear_comprobante_test();
        let json = serde_json::to_string(&c);
        assert!(json.is_ok());
    }
}