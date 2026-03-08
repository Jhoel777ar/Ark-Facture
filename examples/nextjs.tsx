// ARK Facture v4.0 - Ejemplo Next.js 13+
'use client';

import { useState, useEffect } from 'react';
import init, { generar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

export default function FacturaPage() {
  const [html, setHtml] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  useEffect(() => {
    init(wasmUrl).catch(err => setError(err.message));
  }, []);

  const generarFactura = async () => {
    setLoading(true);
    setError('');
    try {
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
      } else {
        setError(resultado.errores?.join(', ') || 'Error desconocido');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Error desconocido');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="container mx-auto p-4">
      <h1 className="text-3xl font-bold mb-4">ARK Facture v4.0</h1>
      
      <button
        onClick={generarFactura}
        disabled={loading}
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
      >
        {loading ? 'Generando...' : 'Generar Comprobante'}
      </button>

      {error && <p className="text-red-500 mt-4">{error}</p>}
      
      {html && (
        <div
          className="mt-4 border rounded p-4"
          dangerouslySetInnerHTML={{ __html: html }}
        />
      )}
    </div>
  );
}
