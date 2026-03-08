/**
 * ARK Facture v4.0 - Wrapper simplificado
 * Uso fácil sin necesidad de Vite
 * Con soporte para generación de PDF segura
 */

import * as pdfGenerator from './pdf-generator.js';

let wasmModule = null;
let wasmReady = false;

/**
 * Inicializa el módulo WASM
 * @returns {Promise<boolean>}
 */
export async function init() {
  if (wasmReady) return true;
  
  try {
    wasmModule = await import('./facture_ark.js');
    await wasmModule.default();
    wasmReady = true;
    return true;
  } catch (error) {
    console.error('Error inicializando WASM:', error);
    return false;
  }
}

/**
 * Genera un comprobante completo
 * @param {Object} factura - Datos de la factura
 * @returns {Promise<Object>}
 */
export async function generarComprobante(factura) {
  if (!wasmReady) {
    const ok = await init();
    if (!ok) throw new Error('No se pudo inicializar WASM');
  }
  
  const resultado = JSON.parse(wasmModule.generar_comprobante_wasm(JSON.stringify(factura)));
  return resultado;
}

/**
 * Valida un comprobante
 * @param {Object} factura - Datos de la factura
 * @returns {Promise<Object>}
 */
export async function validarComprobante(factura) {
  if (!wasmReady) {
    const ok = await init();
    if (!ok) throw new Error('No se pudo inicializar WASM');
  }
  
  const resultado = JSON.parse(wasmModule.validar_comprobante_wasm(JSON.stringify(factura)));
  return resultado;
}

/**
 * Genera XML de un comprobante
 * @param {Object} factura - Datos de la factura
 * @returns {Promise<Object>}
 */
export async function generarXml(factura) {
  if (!wasmReady) {
    const ok = await init();
    if (!ok) throw new Error('No se pudo inicializar WASM');
  }
  
  const resultado = JSON.parse(wasmModule.generar_xml_wasm(JSON.stringify(factura)));
  return resultado;
}

/**
 * Obtiene la versión de la librería
 * @returns {Promise<string>}
 */
export async function obtenerVersion() {
  if (!wasmReady) {
    const ok = await init();
    if (!ok) throw new Error('No se pudo inicializar WASM');
  }
  
  return wasmModule.obtener_version();
}

/**
 * Genera PDF desde HTML de forma segura
 * Soporta Puppeteer, html-pdf, o HTML base64
 * @param {string} html - HTML del comprobante
 * @param {Object} opciones - Opciones de generación
 * @returns {Promise<Object>}
 */
export async function generarPdf(html, opciones = {}) {
  return await pdfGenerator.generarPdfDesdeHtml(html, opciones);
}

/**
 * Genera PDF y lo guarda en archivo
 * @param {string} html - HTML del comprobante
 * @param {string} rutaArchivo - Ruta donde guardar
 * @param {Object} opciones - Opciones de generación
 * @returns {Promise<Object>}
 */
export async function generarPdfArchivo(html, rutaArchivo, opciones = {}) {
  return await pdfGenerator.generarPdfArchivo(html, rutaArchivo, opciones);
}

/**
 * Verifica disponibilidad de generadores de PDF
 * @returns {Promise<Object>}
 */
export async function verificarPdfDisponibilidad() {
  return await pdfGenerator.verificarDisponibilidad();
}

// Exportar funciones de bajo nivel también
export { generar_comprobante_wasm, validar_comprobante_wasm, generar_xml_wasm, generar_pdf_wasm, obtener_version } from './facture_ark.js';

export default {
  init,
  generarComprobante,
  validarComprobante,
  generarXml,
  generarPdf,
  generarPdfArchivo,
  verificarPdfDisponibilidad,
  obtenerVersion
};
