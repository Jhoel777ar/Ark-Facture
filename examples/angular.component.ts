// ARK Facture v4.0 - Ejemplo Angular
import { Component, OnInit } from '@angular/core';
import init, { generar_comprobante_wasm } from 'ark_facture';
import wasmUrl from "ark_facture/facture_ark_bg.wasm?url";

@Component({
  selector: 'app-factura',
  template: `
    <div class="container">
      <h1>ARK Facture v4.0</h1>
      
      <button (click)="generarFactura()" [disabled]="loading">
        {{ loading ? 'Generando...' : 'Generar Comprobante' }}
      </button>

      <p *ngIf="error" class="error">{{ error }}</p>
      
      <div *ngIf="html" class="resultado" [innerHTML]="html"></div>
    </div>
  `,
  styles: [`
    .container { padding: 20px; }
    button { padding: 10px 20px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer; }
    button:disabled { background: #ccc; cursor: not-allowed; }
    .error { color: red; margin-top: 10px; }
    .resultado { margin-top: 20px; border: 1px solid #ddd; padding: 20px; border-radius: 4px; }
  `]
})
export class FacturaComponent implements OnInit {
  html = '';
  loading = false;
  error = '';

  async ngOnInit() {
    try {
      await init(wasmUrl);
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Error desconocido';
    }
  }

  async generarFactura() {
    this.loading = true;
    this.error = '';
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
        this.html = resultado.html;
      } else {
        this.error = resultado.errores?.join(', ') || 'Error desconocido';
      }
    } catch (err) {
      this.error = err instanceof Error ? err.message : 'Error desconocido';
    } finally {
      this.loading = false;
    }
  }
}
