// ARK Facture v4.0 - Ejemplo Express.js
const express = require('express');
const app = express();

app.use(express.json());

// Endpoint para generar comprobante
app.post('/api/generar-comprobante', (req, res) => {
  try {
    const factura = req.body;

    // Validar datos básicos
    if (!factura.empresa || !factura.empresa.nombre || !factura.empresa.nit) {
      return res.status(400).json({
        exitoso: false,
        error: 'Datos de empresa incompletos'
      });
    }

    if (!factura.detalle_venta || factura.detalle_venta.length === 0) {
      return res.status(400).json({
        exitoso: false,
        error: 'Debe haber al menos un artículo'
      });
    }

    // Calcular totales
    const subtotal = factura.detalle_venta.reduce((sum, item) => {
      return sum + (item.cantidad * item.precio_unitario);
    }, 0);

    const impuestos = (subtotal * (factura.impuestos || 0)) / 100;
    const descuentos = factura.descuentos || 0;
    const total = subtotal + impuestos - descuentos;

    // Respuesta simulada (en producción usarías WASM)
    const resultado = {
      exitoso: true,
      datos: {
        numero_comprobante: factura.numero_comprobante || "FAC-" + Date.now(),
        codigo_control: "ABC123DEF456",
        codigo_verificacion: "XYZ789UVW012",
        qr_data: "QR_DATA_AQUI",
        fecha_emision: new Date().toLocaleString('es-BO'),
        total: total,
        moneda: factura.moneda || "BOB",
        hash_integridad: "SHA256_HASH"
      },
      html: `
        <div style="font-family: Arial; padding: 20px;">
          <h2>COMPROBANTE DE VENTA</h2>
          <p><strong>Empresa:</strong> ${factura.empresa.nombre}</p>
          <p><strong>NIT:</strong> ${factura.empresa.nit}</p>
          <p><strong>Total:</strong> ${total.toFixed(2)} ${factura.moneda || 'BOB'}</p>
        </div>
      `
    };

    res.json(resultado);
  } catch (error) {
    res.status(500).json({
      exitoso: false,
      error: error.message
    });
  }
});

// Endpoint para validar comprobante
app.post('/api/validar-comprobante', (req, res) => {
  try {
    const factura = req.body;
    const errores = [];

    if (!factura.empresa?.nombre) errores.push('Nombre de empresa requerido');
    if (!factura.empresa?.nit) errores.push('NIT requerido');
    if (!factura.detalle_venta?.length) errores.push('Debe haber artículos');

    res.json({
      valido: errores.length === 0,
      errores: errores
    });
  } catch (error) {
    res.status(500).json({
      valido: false,
      error: error.message
    });
  }
});

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`✅ Servidor ARK Facture en http://localhost:${PORT}`);
  console.log(`📍 POST /api/generar-comprobante`);
  console.log(`📍 POST /api/validar-comprobante`);
});
