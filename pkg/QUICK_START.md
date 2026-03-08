# 🚀 Quick Start - ARK Facture v4.0

## Instalación

```bash
npm install ark_facture
```

## Uso Básico (Sin Vite)

```javascript
import ArkFacture from 'ark_facture';

// Inicializar
await ArkFacture.init();

// Crear factura
const factura = {
  numero_comprobante: "FAC-2025-00001",
  codigo_control: "",
  codigo_verificacion: "",
  empresa: {
    nombre: "MI EMPRESA S.R.L.",
    nit: "1234567890"
  },
  detalle_venta: [
    {
      descripcion: "Producto",
      cantidad: 1,
      precio_unitario: 100
    }
  ],
  fecha_emision: new Date().toISOString(),
  moneda: "BOB",
  subtotal: 0,
  impuestos: 0,
  descuentos: 0,
  total: 0,
  tipo_comprobante: "recibo"
};

// Generar
const resultado = await ArkFacture.generarComprobante(factura);
console.log(resultado.html);  // HTML renderizado
console.log(resultado.datos); // Datos de seguridad
```

## Funciones Disponibles

### `init()`
Inicializa el módulo WASM

```javascript
await ArkFacture.init();
```

### `generarComprobante(factura)`
Genera HTML + JSON + QR

```javascript
const resultado = await ArkFacture.generarComprobante(factura);
// resultado.html - HTML completo
// resultado.datos - Datos de seguridad
```

### `validarComprobante(factura)`
Valida datos sin generar

```javascript
const validacion = await ArkFacture.validarComprobante(factura);
// validacion.valido - boolean
// validacion.errores - array de errores
```

### `generarXml(factura)`
Genera XML para integraciones

```javascript
const xml = await ArkFacture.generarXml(factura);
// xml.xml - Contenido XML
```

### `generarPdf(html, opciones)`
Genera PDF desde HTML (requiere Puppeteer o html-pdf)

```javascript
const pdf = await ArkFacture.generarPdf(resultado.html);
if (pdf.exitoso) {
  // pdf.pdf_base64 - Base64 del PDF
  // pdf.tamaño_bytes - Tamaño en bytes
  // pdf.metodo - 'puppeteer', 'html-pdf', o 'html-base64'
}
```

### `generarPdfArchivo(html, ruta, opciones)`
Genera PDF y lo guarda en archivo

```javascript
const resultado = await ArkFacture.generarPdfArchivo(
  html,
  './comprobante.pdf'
);
```

### `verificarPdfDisponibilidad()`
Verifica qué generadores de PDF están disponibles

```javascript
const disponibilidad = await ArkFacture.verificarPdfDisponibilidad();
console.log(disponibilidad.recomendacion);
```

## Ejemplo Completo

```javascript
import ArkFacture from 'ark_facture';

async function main() {
  // Inicializar
  await ArkFacture.init();

  // Crear factura
  const factura = {
    numero_comprobante: "FAC-" + Date.now(),
    codigo_control: "",
    codigo_verificacion: "",
    empresa: {
      nombre: "COMERCIAL ARK S.R.L.",
      nit: "1234567890",
      ciudad: "La Paz",
      pais: "Bolivia"
    },
    cliente: {
      nombre: "Cliente Ejemplo",
      nit_ci: "1234567"
    },
    detalle_venta: [
      {
        descripcion: "Laptop HP",
        cantidad: 2,
        precio_unitario: 4500
      },
      {
        descripcion: "Mouse",
        cantidad: 3,
        precio_unitario: 120
      }
    ],
    fecha_emision: new Date().toISOString(),
    moneda: "BOB",
    subtotal: 0,
    impuestos: 0,
    descuentos: 150,
    total: 0,
    monto_pagado: 13000,
    metodo_pago: "Efectivo",
    tipo_comprobante: "recibo"
  };

  // Validar
  const validacion = await ArkFacture.validarComprobante(factura);
  if (!validacion.valido) {
    console.error('Errores:', validacion.errores);
    return;
  }

  // Generar
  const resultado = await ArkFacture.generarComprobante(factura);
  
  // Mostrar HTML
  document.getElementById('resultado').innerHTML = resultado.html;
  
  // Mostrar datos
  console.log('Número:', resultado.datos.numero_comprobante);
  console.log('Total:', resultado.datos.total);
  console.log('Control:', resultado.datos.codigo_control);
  
  // Generar XML
  const xml = await ArkFacture.generarXml(factura);
  console.log('XML:', xml.xml);
}

main().catch(console.error);
```

## Convertir a PDF

### Con Puppeteer (Recomendado)

```bash
npm install puppeteer
```

```javascript
import ArkFacture from 'ark_facture';

async function main() {
  await ArkFacture.init();

  const factura = { /* ... */ };
  const resultado = await ArkFacture.generarComprobante(factura);

  // Generar PDF
  const pdf = await ArkFacture.generarPdf(resultado.html);

  if (pdf.exitoso) {
    // Guardar archivo
    const fs = require('fs');
    fs.writeFileSync('comprobante.pdf', Buffer.from(pdf.pdf_base64, 'base64'));
    console.log('✅ PDF guardado:', pdf.tamaño_bytes, 'bytes');
  }
}

main().catch(console.error);
```

### Con html-pdf (Alternativa ligera)

```bash
npm install html-pdf
```

Uso igual que Puppeteer, la librería detecta automáticamente.

### En Navegador (html2pdf.js)

```bash
npm install html2pdf.js
```

```javascript
import html2pdf from 'html2pdf.js';

const resultado = await ArkFacture.generarComprobante(factura);
html2pdf().from(resultado.html).save('comprobante.pdf');
```

## Performance

- ⚡ ~2-3ms por comprobante
- 🚀 ~375-500 comprobantes/segundo
- 📦 ~850KB WASM (descargado una sola vez)
- 🌐 Funciona offline

## Soporte

- 📧 contacto@ark.com.bo
- 📚 Ver README.md para documentación completa
- 📖 Ver INTEGRATION.md para ejemplos por framework

---

**ARK Facture v4.0** - Listo para producción 🚀
