<script setup>
import { ref, onMounted } from 'vue';
import init, { generar_comprobante_wasm, validar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

const html = ref('');
const datos = ref(null);
const loading = ref(false);
const error = ref('');

onMounted(async () => {
  try {
    await init(wasmUrl);
  } catch (err) {
    error.value = err.message;
  }
});

const generarFactura = async () => {
  loading.value = true;
  error.value = '';
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
      html.value = resultado.html;
      datos.value = resultado.datos;
    } else {
      error.value = resultado.errores?.join(', ') || 'Error desconocido';
    }
  } catch (err) {
    error.value = err.message;
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
    <p v-if="error" style="color: red;">{{ error }}</p>
    <div v-if="html" v-html="html"></div>
    <pre v-if="datos">{{ JSON.stringify(datos, null, 2) }}</pre>
  </div>
</template>
