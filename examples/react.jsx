// ARK Facture v4.0 - Ejemplo React 18+
import { useState, useEffect } from 'react';
import init, { generar_comprobante_wasm, validar_comprobante_wasm, generar_xml_wasm, generar_pdf_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

export function FacturaGenerator() {
  const [html, setHtml] = useState('');
  const [datos, setDatos] = useState(null);
  const [xml, setXml] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [tab, setTab] = useState('preview');

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

      // Generar HTML
      const resultado = JSON.parse(generar_comprobante_wasm(JSON.stringify(factura)));
      if (resultado.exitoso) {
        setHtml(resultado.html);
        setDatos(resultado.datos);
      } else {
        setError(resultado.errores?.join(', ') || 'Error desconocido');
        return;
      }

      // Generar XML
      const xmlResultado = JSON.parse(generar_xml_wasm(JSON.stringify(factura)));
      if (xmlResultado.exitoso) {
        setXml(xmlResultado.xml);
      }
    } catch (err) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const descargarJSON = () => {
    if (!datos) return;
    const json = JSON.stringify(datos, null, 2);
    const blob = new Blob([json], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `comprobante_${datos.numero_comprobante}.json`;
    a.click();
  };

  const descargarXML = () => {
    if (!xml) return;
    const blob = new Blob([xml], { type: 'application/xml' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `comprobante_${datos?.numero_comprobante}.xml`;
    a.click();
  };

  const descargarPDF = () => {
    if (!html) return;
    const elemento = document.createElement('div');
    elemento.innerHTML = html;
    elemento.style.display = 'none';
    document.body.appendChild(elemento);
    
    if (typeof html2pdf !== 'undefined') {
      const opciones = {
        margin: 10,
        filename: `comprobante_${datos?.numero_comprobante}.pdf`,
        image: { type: 'jpeg', quality: 0.98 },
        html2canvas: { scale: 2 },
        jsPDF: { orientation: 'portrait', unit: 'mm', format: 'a4' }
      };
      html2pdf().set(opciones).from(elemento).save();
    } else {
      const ventana = window.open('', '', 'width=800,height=600');
      ventana.document.write(html);
      ventana.document.close();
      ventana.print();
    }
    
    document.body.removeChild(elemento);
  };

  return (
    <div style={{ padding: '20px' }}>
      <h1>ARK Facture v4.0 - React</h1>
      
      <button onClick={generarFactura} disabled={loading} style={{ padding: '10px 20px', marginBottom: '20px' }}>
        {loading ? 'Generando...' : 'Generar Comprobante'}
      </button>

      {error && <p style={{ color: 'red' }}>{error}</p>}

      {html && (
        <div>
          <div style={{ marginBottom: '20px' }}>
            <button onClick={() => setTab('preview')} style={{ marginRight: '10px', fontWeight: tab === 'preview' ? 'bold' : 'normal' }}>
              👁️ Vista Previa
            </button>
            <button onClick={() => setTab('json')} style={{ marginRight: '10px', fontWeight: tab === 'json' ? 'bold' : 'normal' }}>
              📋 JSON
            </button>
            <button onClick={() => setTab('xml')} style={{ marginRight: '10px', fontWeight: tab === 'xml' ? 'bold' : 'normal' }}>
              📄 XML
            </button>
            <button onClick={() => setTab('pdf')} style={{ fontWeight: tab === 'pdf' ? 'bold' : 'normal' }}>
              📑 PDF
            </button>
          </div>

          {tab === 'preview' && (
            <div>
              <div dangerouslySetInnerHTML={{ __html: html }} />
              {datos && (
                <div style={{ marginTop: '20px', padding: '15px', background: '#f5f5f5', borderRadius: '4px' }}>
                  <h3>Datos de Seguridad</h3>
                  <p><strong>Número:</strong> {datos.numero_comprobante}</p>
                  <p><strong>Control:</strong> {datos.codigo_control}</p>
                  <p><strong>Verificación:</strong> {datos.codigo_verificacion}</p>
                  <p><strong>Total:</strong> {datos.total} {datos.moneda}</p>
                  <p><strong>Hash:</strong> {datos.hash_integridad}</p>
                </div>
              )}
            </div>
          )}

          {tab === 'json' && (
            <div>
              <pre style={{ background: '#f5f5f5', padding: '15px', borderRadius: '4px', overflow: 'auto' }}>
                {JSON.stringify(datos, null, 2)}
              </pre>
              <button onClick={descargarJSON} style={{ padding: '10px 20px', marginTop: '10px' }}>
                ⬇️ Descargar JSON
              </button>
            </div>
          )}

          {tab === 'xml' && (
            <div>
              <pre style={{ background: '#f5f5f5', padding: '15px', borderRadius: '4px', overflow: 'auto' }}>
                {xml}
              </pre>
              <button onClick={descargarXML} style={{ padding: '10px 20px', marginTop: '10px' }}>
                ⬇️ Descargar XML
              </button>
            </div>
          )}

          {tab === 'pdf' && (
            <div>
              <p>Convierte el HTML a PDF usando html2pdf.js o jsPDF</p>
              <button onClick={descargarPDF} style={{ padding: '10px 20px' }}>
                📥 Descargar como PDF
              </button>
              <p style={{ marginTop: '10px', fontSize: '12px', color: '#666' }}>
                Instala: npm install html2pdf.js
              </p>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
