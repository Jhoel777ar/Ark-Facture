# 🚀 Guía de Integración - ARK Facture v4.0

## Instalación Rápida

```bash
npm install ark_facture
# o
yarn add ark_facture
# o
pnpm add ark_facture
```

## Uso Básico (3 líneas de código)

```javascript
import init, { generar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

await init(wasmUrl);
const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));
```

## Frameworks

### React 18+

```jsx
import { useState, useEffect } from 'react';
import init, { generar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

export function FacturaGenerator() {
  const [html, setHtml] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    init(wasmUrl).catch(console.error);
  }, []);

  const generar = async () => {
    setLoading(true);
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
        setHtml(resultado.html);
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <div>
      <button onClick={generar} disabled={loading}>
        {loading ? 'Generando...' : 'Generar Comprobante'}
      </button>
      {html && <div dangerouslySetInnerHTML={{ __html: html }} />}
    </div>
  );
}
```

### Vue 3

```vue
<script setup>
import { ref, onMounted } from 'vue';
import init, { generar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

const html = ref('');
const loading = ref(false);

onMounted(async () => {
  await init(wasmUrl);
});

const generar = async () => {
  loading.value = true;
  try {
    const factura = { /* datos */ };
    const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));
    if (resultado.exitoso) {
      html.value = resultado.html;
    }
  } finally {
    loading.value = false;
  }
};
</script>

<template>
  <div>
    <button @click="generar" :disabled="loading">
      {{ loading ? 'Generando...' : 'Generar Comprobante' }}
    </button>
    <div v-html="html"></div>
  </div>
</template>
```

### Angular

```typescript
import { Component, OnInit } from '@angular/core';
import init, { generar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

@Component({
  selector: 'app-factura',
  template: `
    <button (click)="generar()" [disabled]="loading">
      {{ loading ? 'Generando...' : 'Generar Comprobante' }}
    </button>
    <div [innerHTML]="html"></div>
  `
})
export class FacturaComponent implements OnInit {
  html = '';
  loading = false;

  async ngOnInit() {
    await init(wasmUrl);
  }

  async generar() {
    this.loading = true;
    try {
      const factura = { /* datos */ };
      const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));
      if (resultado.exitoso) {
        this.html = resultado.html;
      }
    } finally {
      this.loading = false;
    }
  }
}
```

### Svelte

```svelte
<script>
  import { onMount } from 'svelte';
  import init, { generar_comprobante_wasm } from 'ark_facture';
  import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

  let html = '';
  let loading = false;

  onMount(async () => {
    await init(wasmUrl);
  });

  async function generar() {
    loading = true;
    try {
      const factura = { /* datos */ };
      const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));
      if (resultado.exitoso) {
        html = resultado.html;
      }
    } finally {
      loading = false;
    }
  }
</script>

<button on:click={generar} disabled={loading}>
  {loading ? 'Generando...' : 'Generar Comprobante'}
</button>
{#if html}
  <div>{@html html}</div>
{/if}
```

### Laravel + Blade

```blade
<script type="module">
  import init, { generar_comprobante_wasm } from 'https://cdn.jsdelivr.net/npm/ark_facture@4.0.0/+esm';
  import wasmUrl from 'https://cdn.jsdelivr.net/npm/ark_facture@4.0.0/facture_ark_bg.wasm';

  document.getElementById('generar').addEventListener('click', async () => {
    await init(wasmUrl);
    
    const factura = @json($factura);
    const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));
    
    if (resultado.exitoso) {
      document.getElementById('resultado').innerHTML = resultado.html;
    }
  });
</script>

<button id="generar">Generar Comprobante</button>
<div id="resultado"></div>
```

### Express.js (Node.js)

```javascript
// Cliente (navegador)
async function generarEnServidor(factura) {
  const response = await fetch('/api/generar-comprobante', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(factura)
  });
  return response.json();
}

// Servidor (Express)
const express = require('express');
const app = express();

app.post('/api/generar-comprobante', express.json(), (req, res) => {
  try {
    const factura = req.body;
    // Usar WASM en Node.js
    const resultado = generarComprobanteWasm(JSON.stringify(factura));
    res.json(JSON.parse(resultado));
  } catch (error) {
    res.status(400).json({ error: error.message });
  }
});
```

### Next.js

```typescript
// pages/api/generar-comprobante.ts
import type { NextApiRequest, NextApiResponse } from 'next';

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  try {
    const factura = req.body;
    // Usar WASM en Node.js
    const resultado = generarComprobanteWasm(JSON.stringify(factura));
    res.status(200).json(JSON.parse(resultado));
  } catch (error) {
    res.status(400).json({ error: error.message });
  }
}
```

```typescript
// components/FacturaGenerator.tsx
'use client';

import { useState, useEffect } from 'react';
import init, { generar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

export function FacturaGenerator() {
  const [html, setHtml] = useState('');

  useEffect(() => {
    init(wasmUrl);
  }, []);

  const generar = async () => {
    const factura = { /* datos */ };
    const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));
    setHtml(resultado.html);
  };

  return (
    <div>
      <button onClick={generar}>Generar</button>
      {html && <div dangerouslySetInnerHTML={{ __html: html }} />}
    </div>
  );
}
```

## Configuración Vite

```javascript
// vite.config.js
export default {
  optimizeDeps: {
    exclude: ['ark_facture']
  }
}
```

## Convertir HTML a PDF

### html2pdf.js (Recomendado)

```bash
npm install html2pdf.js
```

```javascript
import html2pdf from 'html2pdf.js';

function descargarPDF() {
  const elemento = document.getElementById('comprobante');
  const opciones = {
    margin: 10,
    filename: 'comprobante.pdf',
    image: { type: 'jpeg', quality: 0.98 },
    html2canvas: { scale: 2 },
    jsPDF: { orientation: 'portrait', unit: 'mm', format: 'a4' }
  };
  
  html2pdf().set(opciones).from(elemento).save();
}
```

### jsPDF + html2canvas

```bash
npm install jspdf html2canvas
```

```javascript
import html2canvas from 'html2canvas';
import { jsPDF } from 'jspdf';

async function descargarPDF() {
  const canvas = await html2canvas(document.getElementById('comprobante'));
  const pdf = new jsPDF();
  const imgData = canvas.toDataURL('image/png');
  pdf.addImage(imgData, 'PNG', 10, 10);
  pdf.save('comprobante.pdf');
}
```

## Estructura de Datos Mínima

```javascript
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
```

## Funciones Disponibles

### generar_comprobante_wasm(factura_json)
Genera HTML + JSON con códigos SHA-256 y QR

### validar_comprobante_wasm(factura_json)
Valida datos sin generar HTML

### generar_xml_wasm(factura_json)
Genera XML para integraciones B2B

### obtener_version()
Retorna versión del WASM

## Manejo de Errores

```javascript
try {
  const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));
  
  if (!resultado.exitoso) {
    console.error('Errores:', resultado.errores);
    return;
  }
  
  console.log('Comprobante generado:', resultado.datos);
} catch (error) {
  console.error('Error:', error.message);
}
```

## Performance

- ⚡ ~2-3ms por comprobante
- 🚀 ~375-500 comprobantes/segundo
- 📦 ~850KB WASM (descargado una sola vez)
- 🌐 Funciona offline

## Soporte

- 📧 contacto@ark.com.bo
- 🐛 Reportar bugs en GitHub
- 📚 Documentación completa en README.md
