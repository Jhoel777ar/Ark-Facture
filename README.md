# 🧾 ARK Facture v4.0

**Sistema Enterprise de generación de comprobantes de venta internos para Bolivia**

Compilado a WebAssembly desde Rust con arquitectura Zero-Knowledge. Genera comprobantes de venta, recibos, proformas y notas de venta con códigos de control SHA-256, códigos QR SVG y HTML profesional listo para convertir a PDF.

Ideal para sistemas POS, tiendas online, aplicaciones SaaS y plataformas de facturación.

## ✨ Características v4.0

- 🚀 **Ultra rápido**: ~2-3ms por comprobante (compilado a WASM)
- 🔒 **Seguro**: Códigos de control SHA-256 (FIPS 180-4) y validación end-to-end
- 📱 **QR Code**: Generación nativa de códigos QR en SVG (verificables)
- 🎨 **HTML + CSS**: Templates responsive listos para imprimir o convertir a PDF
- ✅ **Validación integrada**: Validación exhaustiva antes de generar
- 📦 **Zero Dependencies**: Funciona en cualquier navegador moderno
- 🏗️ **Zero-Knowledge Architecture**: No necesita servidor, todo en cliente
- 🌐 **TypeScript ready**: Tipos completos incluidos
- 📊 **Formato agnóstico**: Salida HTML + JSON + XML para máxima flexibilidad
- ⚡ **1000 comps/segundo**: Rendimiento enterprise-grade
- 🛡️ **XSS Prevention**: Sanitización automática de HTML
- 🇧🇴 **Validadores bolivianos**: NIT, CI, teléfono, email

## 📦 Instalación

### NPM
```bash
npm install ark_facture
```

### Yarn
```bash
yarn add ark_facture
```

### PNPM
```bash
pnpm add ark_facture
```

## 🚀 Uso Rápido

### En HTML5 (Vanilla JavaScript + Vite)

Setup en `vite.config.js`:
```javascript
export default {
  optimizeDeps: {
    exclude: ['ark_facture']
  }
}
```

Uso:
```html
<!DOCTYPE html>
<html lang="es">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ARK Facture Demo</title>
</head>
<body>
  <h1>Generador de Comprobantes v4.0</h1>
  <button id="generar">Generar Comprobante</button>
  <button id="exportar-pdf">Exportar a PDF</button>
  <div id="resultado"></div>

  <script type="module">
    import init, { generar_comprobante_wasm } from 'ark_facture';
    import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

    let wasmReady = false;

    async function initWasm() {
      if (!wasmReady) {
        await init(wasmUrl);
        wasmReady = true;
        console.log('✅ WASM initialized');
      }
    }

    document.getElementById('generar').addEventListener('click', async () => {
      await initWasm();

      const factura = {
        numero_comprobante: "FAC-2025-00001",
        codigo_control: "",
        codigo_verificacion: "",
        empresa: {
          nombre: "MI EMPRESA S.R.L.",
          nit: "1234567890",
          direccion: "Av. Principal #123, La Paz",
          ciudad: "La Paz",
          pais: "Bolivia"
        },
        cliente: {
          nombre: "Cliente Ejemplo",
          nit_ci: "1234567 LP"
        },
        detalle_venta: [
          {
            descripcion: "Producto de prueba",
            cantidad: 2,
            precio_unitario: 150.00,
            codigo: "PROD-001"
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

      const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));

      if (resultado.exitoso) {
        document.getElementById('resultado').innerHTML = resultado.html;
        console.log('Datos de seguridad:', resultado.datos);
      } else {
        console.error('Errores:', resultado.errores);
      }
    });
  </script>
</body>
</html>
```

### En React 18+

```jsx
import { useState, useEffect } from 'react';
import init, { generar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

export default function FacturaGenerator() {
  const [html, setHtml] = useState('');
  const [datos, setDatos] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    init(wasmUrl).catch(console.error);
  }, []);

  const generarFactura = async () => {
    setLoading(true);
    try {
      const factura = {
        numero_comprobante: "FAC-2025-00001",
        codigo_control: "",
        codigo_verificacion: "",
        empresa: {
          nombre: "COMERCIAL ARK S.R.L.",
          nit: "1234567890"
        },
        detalle_venta: [
          {
            descripcion: "Laptop HP",
            cantidad: 1,
            precio_unitario: 4500.00
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

      const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));

      if (resultado.exitoso) {
        setHtml(resultado.html);
        setDatos(resultado.datos);
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <button onClick={generarFactura} disabled={loading}>
        {loading ? 'Generando...' : 'Generar Comprobante'}
      </button>
      {html && <div dangerouslySetInnerHTML={{ __html: html }} />}
    </div>
  );
}
```

### En Vue 3 + Vite

```vue
<script setup>
import { ref, onMounted } from 'vue';
import init, { generar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

const html = ref('');
const datos = ref(null);
const loading = ref(false);

onMounted(async () => {
  await init(wasmUrl);
});

const generarFactura = async () => {
  loading.value = true;
  try {
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

    const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));

    if (resultado.exitoso) {
      html.value = resultado.html;
      datos.value = resultado.datos;
    }
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div>
    <button @click="generarFactura" :disabled="loading">
      {{ loading ? 'Generando...' : 'Generar Comprobante' }}
    </button>
    <div v-html="html"></div>
  </div>
</template>
```

## 📄 Convertir HTML a PDF

La librería genera HTML + CSS, no PDF directamente. Usa una de estas opciones recomendadas:

### Opción 1: html2pdf.js (Recomendado - Ligero)

```bash
npm install html2pdf.js
```

```javascript
import html2pdf from 'html2pdf.js';

const elemento = document.getElementById('resultado');
const opciones = {
  margin: 10,
  filename: 'comprobante.pdf',
  image: { type: 'jpeg', quality: 0.98 },
  html2canvas: { scale: 2 },
  jsPDF: { orientation: 'portrait', unit: 'mm', format: 'a4' }
};

html2pdf().set(opciones).from(elemento).save();
```

### Opción 2: jsPDF + html2canvas

```bash
npm install jspdf html2canvas
```

```javascript
import html2canvas from 'html2canvas';
import { jsPDF } from 'jspdf';

const canvas = await html2canvas(document.getElementById('resultado'));
const pdf = new jsPDF();
const imgData = canvas.toDataURL('image/png');
pdf.addImage(imgData, 'PNG', 10, 10);
pdf.save('comprobante.pdf');
```

## 📋 Estructura de Datos

### Objeto Factura (Entrada)

**Campos Requeridos ✅**

```javascript
{
  numero_comprobante: string,        // Ej: "FAC-2025-00001"
  codigo_control: string,            // Dejar vacío "", se genera automáticamente
  codigo_verificacion: string,       // Dejar vacío "", se genera automáticamente
  empresa: {
    nombre: string,                  // OBLIGATORIO
    nit: string                      // OBLIGATORIO (validado)
  },
  detalle_venta: [                   // OBLIGATORIO (mínimo 1 item)
    {
      descripcion: string,           // OBLIGATORIO
      cantidad: number,              // OBLIGATORIO (> 0)
      precio_unitario: number        // OBLIGATORIO (>= 0)
    }
  ],
  fecha_emision: string,             // ISO 8601: "2025-10-17T20:34:14Z"
  moneda: string,                    // Ej: "BOB", "USD", "EUR"
  subtotal: number,                  // Se calcula automáticamente
  impuestos: number,                 // Puede ser 0
  descuentos: number,                // Puede ser 0
  total: number,                     // Se calcula automáticamente
  tipo_comprobante: string           // "recibo", "proforma", "nota_venta", "compra"
}
```

**Campos Opcionales 📝**

```javascript
{
  empresa: {
    razon_social?: string,
    sucursal?: string,
    encargado?: string,
    telefono?: string,
    atencion_cliente?: string,
    direccion?: string,
    email?: string,
    ciudad?: string,
    pais?: string,
    sitio_web?: string,
    caja?: string,
    punto_venta?: string,
    numero_licencia?: string,
    regimen_tributario?: string,
    logo_svg?: string                 // SVG inline (logo personalizado)
  },
  cliente?: {
    nombre?: string,
    nit_ci?: string,
    direccion?: string,
    telefono?: string,
    email?: string,
    empresa?: string,
    ciudad?: string,
    codigo_postal?: string,
    pais?: string
  },
  detalle_venta: [
    {
      codigo?: string,
      categoria?: string,
      descuento_item?: number,
      impuesto_item?: number,
      unidad_medida?: string          // "un", "kg", "lts", etc.
    }
  ],
  fecha_vencimiento?: string,         // ISO 8601
  monto_pagado?: number,
  cambio?: number,                    // Se calcula automáticamente
  metodo_pago?: string,               // "Efectivo", "Tarjeta", "QR"
  notas?: string,
  usuario_atendio?: string,
  tipo_entrega?: string,
  costo_envio?: number,
  numero_orden?: string,
  link_verificacion?: string,         // URL personalizada para QR
  locale?: string                     // "es-BO", "es", "en"
}
```

### Objeto Resultado (Salida)

**Éxito ✅**

```javascript
{
  exitoso: true,
  html: string,                       // HTML + CSS completo listo para imprimir/convertir
  datos: {
    numero_comprobante: string,
    codigo_control: string,           // SHA-256: 32 caracteres hex
    codigo_verificacion: string,      // SHA-256: 40 caracteres hex
    qr_data: string,                  // Contenido del código QR
    fecha_emision: string,            // Formato: "17/10/2025 20:34:14"
    total: number,
    moneda: string,
    hash_integridad: string           // SHA-256 para validación
  },
  version: string,                    // "4.0.0"
  tiempo_ms: null
}
```

**Error ❌**

```javascript
{
  exitoso: false,
  error?: string,                     // Error de parsing JSON
  errores?: string[]                  // Lista de errores de validación
}
```

## 🔧 API de Funciones

### `generar_comprobante_wasm(factura_json: string): string`

Genera comprobante completo con HTML, códigos SHA-256 y QR SVG.

**Parámetros:**
- `factura_json`: String JSON con datos de la factura

**Retorna:**
- String JSON (parsear con `JSON.parse()`)

**Rendimiento:** ~2-3ms por comprobante

### `validar_comprobante_wasm(factura_json: string): string`

Valida datos sin generar HTML.

**Parámetros:**
- `factura_json`: String JSON con datos de la factura

**Retorna:**
- String JSON con resultado de validación

### `generar_xml_wasm(factura_json: string): string`

Genera XML para integraciones B2B.

**Parámetros:**
- `factura_json`: String JSON con datos de la factura

**Retorna:**
- String JSON con XML generado

### `obtener_version(): string`

Retorna versión del WASM

## ⚠️ Errores Comunes

### Error: "Nombre de empresa es obligatorio"

```javascript
// ❌ Incorrecto
empresa: { nombre: "", nit: "123" }

// ✅ Correcto
empresa: { nombre: "MI EMPRESA S.R.L.", nit: "123" }
```

### Error: "WASM not initialized"

```javascript
// ❌ Incorrecto
import { generar_comprobante_wasm } from 'ark_facture';
generar_comprobante_wasm(JSON.stringify(factura));

// ✅ Correcto
import init, { generar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

await init(wasmUrl);
generar_comprobante_wasm(JSON.stringify(factura));
```

### Error: "Debe haber al menos un artículo"

```javascript
// ❌ Incorrecto
detalle_venta: []

// ✅ Correcto
detalle_venta: [
  { descripcion: "Producto", cantidad: 1, precio_unitario: 100 }
]
```

### Error: "Cantidad debe ser > 0"

```javascript
// ❌ Incorrecto
{ descripcion: "Producto", cantidad: 0, precio_unitario: 100 }

// ✅ Correcto
{ descripcion: "Producto", cantidad: 1, precio_unitario: 100 }
```

## 📊 Tipos de Comprobante

| Tipo | Valor | Descripción |
|------|-------|-------------|
| Recibo | `"recibo"` | Recibo de pago estándar |
| Proforma | `"proforma"` | Cotización o presupuesto |
| Nota de Venta | `"nota_venta"` | Nota de venta simple |
| Compra | `"compra"` | Comprobante de compra |
| Nota de Crédito | `"nota_credito"` | Nota de crédito |
| Nota de Débito | `"nota_debito"` | Nota de débito |
| Interno | `"interno"` | Comprobante interno genérico |

## 🏗️ Arquitectura Zero-Knowledge

ARK Facture implementa arquitectura Zero-Knowledge:

- **Todo en cliente**: No requiere servidor para generar comprobantes
- **Sin tracking**: Los datos no se envían a servidores externos
- **Privacidad garantizada**: Procesamiento 100% local en el navegador
- **Offline-ready**: Funciona sin conexión a internet
- **Determinístico**: Mismos datos = mismos códigos de control

## 🔐 Seguridad Criptográfica

- **SHA-256 (FIPS 180-4)**: Códigos de control y verificación
- **Timestamps inmutables**: Previene duplicados
- **QR verificable**: Contiene datos completos del comprobante
- **Hash de integridad**: Validación end-to-end
- **Sanitización XSS**: Prevención de inyección de código

## ⚡ Rendimiento

- **Velocidad**: ~2-3ms por comprobante en navegador
- **Throughput**: ~375-500 comprobantes/segundo
- **Tamaño WASM**: ~850KB (descargado una sola vez)
- **Compatibilidad**: Todos los navegadores modernos con WebAssembly

### Benchmark v4.0

```
Iteraciones: 1000
Exitosas: 1000
Promedio: 2.667 ms/op
Velocidad: 375 comps/seg
Total: 2.67 segundos
Rendimiento: EXCELENTE ✓
```

## 🌍 Compatibilidad de Navegadores

| Navegador | Versión Mínima | WebAssembly |
|-----------|----------------|------------|
| Chrome | 57+ | ✅ |
| Firefox | 52+ | ✅ |
| Safari | 11+ | ✅ |
| Edge | 16+ | ✅ |
| Opera | 44+ | ✅ |

## 🔐 Seguridad y Cumplimiento

### Validación Integrada

- NIT empresarial (validación de formato)
- Cantidad de artículos (1-5000 máximo)
- Descripción de artículos (máx 1000 caracteres)
- Límites de monto (hasta 10 mil millones)
- Sanitización de HTML (previene XSS)

### WASM Signature

```
SHA256: 6A3D1E54176B299DFDD07E73D287D18166DBB75F52BD0C35A1B5DAC7D5C61659
```

### Cumplimiento Normativo - Bolivia

**Válido para:**
- ✓ Control interno y auditoría
- ✓ Inventario y gestión contable
- ✓ Respaldo digital certificado
- ✓ Sistemas SaaS
- ✓ Mercados internacionales
- ✓ Registro administrativo empresarial

**No válido para:**
- ✗ Crédito fiscal IVA ante SIN
- ✗ Devoluciones tributarias
- ✗ Operaciones sujetas a fiscalización SIAT

**Referencias:**
- Ley 843 - Código Tributario Boliviano
- D.S. 24051 - Régimen General
- Resolución SIN - Facturación Electrónica

## 📦 Integración en Producción

### Recomendaciones

- Cache el WASM: Se descarga una sola vez
- Manejo de errores: Siempre valida respuesta
- Conversión a PDF: Usa librería de preferencia
- Almacenamiento: Guarda JSON para auditoría
- Backups: Mantén copias de las facturas generadas

### Ejemplo Producción

```javascript
import init, { generar_comprobante_wasm, validar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

let wasmReady = false;

async function inicializarWasm() {
  if (!wasmReady) {
    try {
      await init(wasmUrl);
      wasmReady = true;
      console.log('✅ ARK Facture v4.0 ready');
    } catch (error) {
      console.error('❌ Error inicializando WASM:', error);
      throw error;
    }
  }
}

async function generarFacturaProduccion(factura) {
  await inicializarWasm();

  // Validar primero
  const validacion = JSON.parse(validar_comprobante_wasm(JSON.stringify(factura)));
  if (!validacion.valido) {
    throw new Error(`Errores de validación: ${validacion.errores.join(', ')}`);
  }

  // Generar
  const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));

  if (!resultado.exitoso) {
    throw new Error(`Error generando comprobante: ${resultado.errores?.join(', ')}`);
  }

  // Guardar para auditoría
  localStorage.setItem(`factura_${resultado.datos.numero_comprobante}`, JSON.stringify(resultado.datos));

  return resultado;
}
```

## 📚 Ejemplos Completos

### Tienda Online (React)

```jsx
import { generateInvoice } from 'ark_facture-react';

export function Checkout({ carrito }) {
  const generarRecibo = async () => {
    const factura = {
      numero_comprobante: `FAC-${Date.now()}`,
      codigo_control: "",
      codigo_verificacion: "",
      empresa: {
        nombre: "Tu Tienda S.R.L.",
        nit: "1234567890"
      },
      cliente: {
        nombre: carrito.cliente,
        nit_ci: carrito.ci
      },
      detalle_venta: carrito.items.map(item => ({
        descripcion: item.nombre,
        cantidad: item.cantidad,
        precio_unitario: item.precio,
        codigo: item.sku
      })),
      fecha_emision: new Date().toISOString(),
      moneda: "BOB",
      subtotal: 0,
      impuestos: 0,
      descuentos: carrito.descuento || 0,
      total: 0,
      monto_pagado: carrito.pagado,
      metodo_pago: "Tarjeta",
      tipo_comprobante: "recibo"
    };

    const resultado = await generateInvoice(factura);
    // Descargar como PDF...
  };
}
```

### Sistema POS

```javascript
// Generar múltiples facturas rápidamente
async function procesoLote(facturas) {
  await init(wasmUrl);

  const resultados = facturas.map(f => {
    const json = generar_comprobante_wasm(JSON.stringify(f));
    return JSON.parse(json);
  });

  console.log(`✅ ${resultados.length} comprobantes en ${Date.now()}ms`);
  return resultados;
}
```

## 📄 Licencia

GPLv3 - Ver archivo LICENSE

---

**ARK Facture v4.0** - Potenciado por Rust 🦀 y WebAssembly 🕸️

Desarrollado por **ARK DEV SYSTEM**
