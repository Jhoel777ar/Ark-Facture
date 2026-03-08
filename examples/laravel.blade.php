{{-- ARK Facture v4.0 - Ejemplo Laravel Blade --}}
@extends('layouts.app')

@section('content')
<div class="container">
    <div class="row">
        <div class="col-md-8">
            <div class="card">
                <div class="card-header">
                    <h5>Generar Comprobante</h5>
                </div>
                <div class="card-body">
                    <form id="facturaForm">
                        <div class="form-group">
                            <label>Nombre Empresa</label>
                            <input type="text" class="form-control" id="empresaNombre" value="COMERCIAL ARK S.R.L." required>
                        </div>

                        <div class="form-group">
                            <label>NIT Empresa</label>
                            <input type="text" class="form-control" id="empresaNit" value="1234567890" required>
                        </div>

                        <div class="form-group">
                            <label>Cliente</label>
                            <input type="text" class="form-control" id="clienteNombre" value="Cliente Ejemplo">
                        </div>

                        <div class="form-group">
                            <label>Descripción</label>
                            <input type="text" class="form-control" id="descripcion" value="Producto">
                        </div>

                        <div class="form-group">
                            <label>Cantidad</label>
                            <input type="number" class="form-control" id="cantidad" value="1" step="0.01">
                        </div>

                        <div class="form-group">
                            <label>Precio</label>
                            <input type="number" class="form-control" id="precio" value="100" step="0.01">
                        </div>

                        <button type="button" class="btn btn-primary" onclick="generarComprobante()">
                            Generar Comprobante
                        </button>
                    </form>
                </div>
            </div>
        </div>

        <div class="col-md-4">
            <div class="card">
                <div class="card-header">
                    <h5>Resultado</h5>
                </div>
                <div class="card-body">
                    <div id="resultado">
                        <p class="text-muted">Haz clic en "Generar Comprobante"</p>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>

<script>
async function generarComprobante() {
    try {
        const factura = {
            numero_comprobante: "FAC-" + Date.now(),
            codigo_control: "",
            codigo_verificacion: "",
            empresa: {
                nombre: document.getElementById('empresaNombre').value,
                nit: document.getElementById('empresaNit').value,
                ciudad: "La Paz",
                pais: "Bolivia"
            },
            cliente: {
                nombre: document.getElementById('clienteNombre').value
            },
            detalle_venta: [
                {
                    descripcion: document.getElementById('descripcion').value,
                    cantidad: parseFloat(document.getElementById('cantidad').value),
                    precio_unitario: parseFloat(document.getElementById('precio').value)
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

        // Enviar al servidor
        const response = await fetch('/api/generar-comprobante', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'X-CSRF-TOKEN': document.querySelector('meta[name="csrf-token"]').content
            },
            body: JSON.stringify(factura)
        });

        const resultado = await response.json();
        if (resultado.exitoso) {
            document.getElementById('resultado').innerHTML = `
                <p><strong>Número:</strong> ${resultado.datos.numero_comprobante}</p>
                <p><strong>Total:</strong> ${resultado.datos.total} ${resultado.datos.moneda}</p>
                <p><strong>Control:</strong> ${resultado.datos.codigo_control}</p>
            `;
        } else {
            document.getElementById('resultado').innerHTML = `<p class="text-danger">${resultado.error}</p>`;
        }
    } catch (error) {
        document.getElementById('resultado').innerHTML = `<p class="text-danger">Error: ${error.message}</p>`;
    }
}
</script>
@endsection
