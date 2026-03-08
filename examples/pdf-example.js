/**
 * ARK Facture v4.0 - Ejemplo de Generación de PDF
 * Demuestra cómo generar PDF desde HTML
 * 
 * Uso:
 * npm install ark_facture puppeteer
 * node examples/pdf-example.js
 */

import ArkFacture from '../pkg/index.js';
import { writeFileSync } from 'fs';
import { resolve } from 'path';

async function main() {
  console.log('🧾 ARK Facture v4.0 - Ejemplo de PDF\n');

  try {
    // 1. Inicializar
    console.log('📦 Inicializando WASM...');
    await ArkFacture.init();
    console.log('✅ WASM inicializado\n');

    // 2. Verificar disponibilidad de PDF
    console.log('🔍 Verificando disponibilidad de generadores de PDF...');
    const disponibilidad = await ArkFacture.verificarPdfDisponibilidad();
    console.log('Disponibilidad:', disponibilidad.disponibilidad);
    console.log('Recomendación:', disponibilidad.recomendacion);
    console.log();

    // 3. Crear factura
    console.log('📝 Creando factura...');
    const factura = {
      numero_comprobante: `FAC-${Date.now()}`,
      codigo_control: '',
      codigo_verificacion: '',
      empresa: {
        nombre: 'COMERCIAL ARK S.R.L.',
        nit: '1234567890',
        razon_social: 'Comercial ARK - Distribuidora',
        direccion: 'Av. 6 de Agosto 1234, La Paz',
        telefono: '+591 2 2345678',
        email: 'contacto@ark.com.bo',
        ciudad: 'La Paz',
        pais: 'Bolivia',
      },
      cliente: {
        nombre: 'María González López',
        nit_ci: '7654321',
        direccion: 'Calle Principal 456',
        ciudad: 'La Paz',
      },
      detalle_venta: [
        {
          descripcion: 'Laptop HP Pavilion 15',
          cantidad: 2,
          precio_unitario: 4500,
          codigo: 'PROD-001',
        },
        {
          descripcion: 'Mouse Inalámbrico',
          cantidad: 3,
          precio_unitario: 120,
          codigo: 'PROD-002',
        },
      ],
      fecha_emision: new Date().toISOString(),
      moneda: 'BOB',
      subtotal: 0,
      impuestos: 0,
      descuentos: 150,
      total: 0,
      monto_pagado: 13000,
      metodo_pago: 'Efectivo',
      tipo_comprobante: 'recibo',
    };
    console.log('✅ Factura creada\n');

    // 4. Validar
    console.log('✔️ Validando factura...');
    const validacion = await ArkFacture.validarComprobante(factura);
    if (!validacion.valido) {
      console.error('❌ Errores de validación:', validacion.errores);
      return;
    }
    console.log('✅ Factura válida\n');

    // 5. Generar comprobante
    console.log('🚀 Generando comprobante...');
    const resultado = await ArkFacture.generarComprobante(factura);

    if (!resultado.exitoso) {
      console.error('❌ Error generando comprobante:', resultado.errores);
      return;
    }

    console.log('✅ Comprobante generado');
    console.log('   Número:', resultado.datos.numero_comprobante);
    console.log('   Total:', resultado.datos.total, resultado.datos.moneda);
    console.log('   Control:', resultado.datos.codigo_control);
    console.log();

    // 6. Generar PDF
    console.log('📄 Generando PDF...');
    const pdf = await ArkFacture.generarPdf(resultado.html);

    if (!pdf.exitoso) {
      console.error('❌ Error generando PDF:', pdf.error);
      console.log('💡 Sugerencia:', pdf.instrucciones);
      return;
    }

    console.log('✅ PDF generado');
    console.log('   Método:', pdf.metodo);
    console.log('   Tamaño:', (pdf.tamaño_bytes / 1024).toFixed(2), 'KB');
    console.log();

    // 7. Guardar PDF
    console.log('💾 Guardando PDF...');
    const rutaPdf = resolve('./comprobante_ejemplo.pdf');
    const pdfBuffer = Buffer.from(pdf.pdf_base64, 'base64');
    writeFileSync(rutaPdf, pdfBuffer);
    console.log('✅ PDF guardado en:', rutaPdf);
    console.log();

    // 8. Generar XML
    console.log('📋 Generando XML...');
    const xml = await ArkFacture.generarXml(factura);

    if (xml.exitoso) {
      const rutaXml = resolve('./comprobante_ejemplo.xml');
      writeFileSync(rutaXml, xml.xml);
      console.log('✅ XML guardado en:', rutaXml);
    }
    console.log();

    // 9. Resumen
    console.log('═══════════════════════════════════════════════════════');
    console.log('✅ PROCESO COMPLETADO EXITOSAMENTE');
    console.log('═══════════════════════════════════════════════════════');
    console.log('📊 Resumen:');
    console.log('   • Comprobante:', resultado.datos.numero_comprobante);
    console.log('   • Total:', resultado.datos.total, resultado.datos.moneda);
    console.log('   • PDF:', (pdf.tamaño_bytes / 1024).toFixed(2), 'KB');
    console.log('   • Método PDF:', pdf.metodo);
    console.log('   • Versión:', resultado.version);
    console.log();
    console.log('📁 Archivos generados:');
    console.log('   • PDF:', rutaPdf);
    console.log('   • XML:', rutaXml);
    console.log();
  } catch (error) {
    console.error('❌ Error:', error.message);
    console.error(error);
  }
}

main();
