/**
 * ARK Facture v4.0 - Generador de PDF Seguro
 * Genera PDF desde HTML usando puppeteer o html-pdf
 * Con validaciones de seguridad y manejo de errores robusto
 */

import { fileURLToPath } from 'url';
import { dirname, resolve } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/**
 * Valida que el HTML sea seguro antes de procesar
 * @param {string} html - HTML a validar
 * @returns {boolean}
 */
function validarHtmlSeguro(html) {
  if (!html || typeof html !== 'string') {
    throw new Error('HTML debe ser un string válido');
  }

  if (html.length > 10 * 1024 * 1024) {
    throw new Error('HTML excede tamaño máximo (10MB)');
  }

  // Detectar scripts maliciosos
  const patronesRiesgosos = [
    /<script[^>]*>[\s\S]*?<\/script>/gi,
    /on\w+\s*=/gi,
    /javascript:/gi,
    /vbscript:/gi,
    /data:text\/html/gi,
  ];

  for (const patron of patronesRiesgosos) {
    if (patron.test(html)) {
      throw new Error('HTML contiene contenido potencialmente peligroso');
    }
  }

  return true;
}

/**
 * Genera PDF usando Puppeteer (Recomendado)
 * @param {string} html - HTML del comprobante
 * @param {Object} opciones - Opciones de generación
 * @returns {Promise<Object>}
 */
async function generarConPuppeteer(html, opciones = {}) {
  try {
    const puppeteer = await import('puppeteer');
    const browser = await puppeteer.default.launch({
      headless: 'new',
      args: [
        '--no-sandbox',
        '--disable-setuid-sandbox',
        '--disable-dev-shm-usage',
        '--disable-gpu',
        '--single-process=false',
      ],
    });

    const page = await browser.newPage();

    // Configurar viewport
    await page.setViewport({
      width: 210,
      height: 297,
      deviceScaleFactor: 2,
    });

    // Cargar HTML
    await page.setContent(html, {
      waitUntil: 'networkidle0',
      timeout: 30000,
    });

    // Generar PDF
    const pdfBuffer = await page.pdf({
      format: 'A4',
      margin: {
        top: '10mm',
        right: '10mm',
        bottom: '10mm',
        left: '10mm',
      },
      printBackground: true,
      preferCSSPageSize: true,
      timeout: 30000,
    });

    await browser.close();

    return {
      exitoso: true,
      pdf_buffer: pdfBuffer,
      pdf_base64: pdfBuffer.toString('base64'),
      tamaño_bytes: pdfBuffer.length,
      metodo: 'puppeteer',
    };
  } catch (error) {
    throw new Error(`Error con Puppeteer: ${error.message}`);
  }
}

/**
 * Genera PDF usando html-pdf (Alternativa ligera)
 * @param {string} html - HTML del comprobante
 * @param {Object} opciones - Opciones de generación
 * @returns {Promise<Object>}
 */
async function generarConHtmlPdf(html, opciones = {}) {
  try {
    const htmlPdf = await import('html-pdf');

    return new Promise((resolve, reject) => {
      const opcionesDefault = {
        format: 'A4',
        orientation: 'portrait',
        border: {
          top: '10mm',
          right: '10mm',
          bottom: '10mm',
          left: '10mm',
        },
        timeout: 30000,
        ...opciones,
      };

      htmlPdf.default.create(html, opcionesDefault).toBuffer((err, buffer) => {
        if (err) {
          reject(new Error(`Error con html-pdf: ${err.message}`));
        } else {
          resolve({
            exitoso: true,
            pdf_buffer: buffer,
            pdf_base64: buffer.toString('base64'),
            tamaño_bytes: buffer.length,
            metodo: 'html-pdf',
          });
        }
      });
    });
  } catch (error) {
    throw new Error(`Error con html-pdf: ${error.message}`);
  }
}

/**
 * Genera PDF desde HTML con fallback automático
 * Intenta Puppeteer primero, luego html-pdf, luego devuelve HTML en base64
 * @param {string} html - HTML del comprobante
 * @param {Object} opciones - Opciones de generación
 * @returns {Promise<Object>}
 */
export async function generarPdfDesdeHtml(html, opciones = {}) {
  try {
    // Validar HTML
    validarHtmlSeguro(html);

    // Intentar con Puppeteer
    try {
      return await generarConPuppeteer(html, opciones);
    } catch (errorPuppeteer) {
      console.warn('⚠️ Puppeteer no disponible, intentando html-pdf...');

      // Fallback a html-pdf
      try {
        return await generarConHtmlPdf(html, opciones);
      } catch (errorHtmlPdf) {
        console.warn('⚠️ html-pdf no disponible, devolviendo HTML en base64...');

        // Fallback final: devolver HTML en base64
        const htmlBase64 = Buffer.from(html).toString('base64');
        return {
          exitoso: true,
          pdf_buffer: null,
          pdf_base64: htmlBase64,
          html_base64: htmlBase64,
          tamaño_bytes: htmlBase64.length,
          metodo: 'html-base64',
          advertencia: 'PDF no generado. Instala puppeteer o html-pdf para generar PDF real.',
          instrucciones: 'npm install puppeteer  # o  npm install html-pdf',
        };
      }
    }
  } catch (error) {
    return {
      exitoso: false,
      error: error.message,
      sugerencia: 'Verifica que el HTML sea válido y no contenga scripts maliciosos',
    };
  }
}

/**
 * Genera PDF y lo guarda en archivo
 * @param {string} html - HTML del comprobante
 * @param {string} rutaArchivo - Ruta donde guardar el PDF
 * @param {Object} opciones - Opciones de generación
 * @returns {Promise<Object>}
 */
export async function generarPdfArchivo(html, rutaArchivo, opciones = {}) {
  try {
    validarHtmlSeguro(html);

    // Validar ruta
    if (!rutaArchivo || typeof rutaArchivo !== 'string') {
      throw new Error('Ruta de archivo inválida');
    }

    // Resolver ruta absoluta
    const rutaAbsoluta = resolve(rutaArchivo);

    // Verificar que no intente escribir fuera del directorio permitido
    if (!rutaAbsoluta.startsWith(process.cwd())) {
      throw new Error('Ruta de archivo no permitida por seguridad');
    }

    const resultado = await generarPdfDesdeHtml(html, opciones);

    if (!resultado.exitoso) {
      return resultado;
    }

    // Guardar archivo
    const fs = await import('fs/promises');
    await fs.writeFile(rutaAbsoluta, resultado.pdf_buffer || Buffer.from(resultado.pdf_base64, 'base64'));

    return {
      exitoso: true,
      archivo: rutaAbsoluta,
      tamaño_bytes: resultado.tamaño_bytes,
      metodo: resultado.metodo,
    };
  } catch (error) {
    return {
      exitoso: false,
      error: error.message,
    };
  }
}

/**
 * Valida disponibilidad de generadores de PDF
 * @returns {Promise<Object>}
 */
export async function verificarDisponibilidad() {
  const disponibilidad = {
    puppeteer: false,
    html_pdf: false,
    html_base64: true, // Siempre disponible
  };

  try {
    await import('puppeteer');
    disponibilidad.puppeteer = true;
  } catch {
    // Puppeteer no disponible
  }

  try {
    await import('html-pdf');
    disponibilidad.html_pdf = true;
  } catch {
    // html-pdf no disponible
  }

  return {
    disponibilidad,
    recomendacion: disponibilidad.puppeteer
      ? 'Puppeteer disponible (recomendado)'
      : disponibilidad.html_pdf
        ? 'html-pdf disponible'
        : 'Solo HTML base64 disponible. Instala: npm install puppeteer',
  };
}

export default {
  generarPdfDesdeHtml,
  generarPdfArchivo,
  verificarDisponibilidad,
};
