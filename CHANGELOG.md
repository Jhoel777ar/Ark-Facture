# Changelog - ARK Facture

## [4.0.0] - 2025-03-08

### ✨ Nuevas Características
- 🎉 Generación de XML para integraciones B2B
- 🔐 Módulo de seguridad mejorado con firma de comprobantes
- 🛡️ Sanitización XSS automática en todos los campos
- 📊 Validadores bolivianos integrados (NIT, CI, teléfono, email)
- 🏗️ Arquitectura modular con módulos separados
- 📱 HTML template completamente rediseñado y responsive
- 🎨 Estilos mejorados para impresión y PDF
- 📚 Documentación completa con ejemplos para todos los frameworks
- 🧪 Suite de 24 tests exhaustivos
- ⚡ Performance optimizado (~2-3ms por comprobante)
- 🌐 Soporte para múltiples monedas (BOB, USD, EUR, ARS, CLP, PEN)
- 📋 Nuevos tipos de comprobante (nota_credito, nota_debito)
- 🔄 Cálculo automático de cambio
- 📦 Librería lista para NPM con tipos TypeScript

### 🔧 Mejoras Técnicas
- Migración a arquitectura modular (models, validators, generators, xml_export, security)
- Implementación de SHA-256 para códigos de control y verificación
- Hash de integridad para validación end-to-end
- Validación exhaustiva de datos antes de generar
- Soporte para WASM en navegadores y Node.js
- Optimización de tamaño WASM (~850KB)

### 📖 Documentación
- README.md completo con ejemplos
- INTEGRATION.md con guías para React, Vue, Angular, Svelte, Laravel, Express, Next.js
- Ejemplos de integración en index.html
- Comentarios en código
- Guía de instalación y uso

### 🐛 Correcciones
- Arreglo de cálculo de totales con descuentos e impuestos
- Validación correcta de campos obligatorios
- Manejo de errores mejorado
- Prevención de XSS en HTML generado

### 🚀 Performance
- Benchmark: ~2.667ms promedio por comprobante
- Velocidad: ~375 comprobantes/segundo
- Tamaño WASM: ~850KB (descargado una sola vez)
- Compatible con todos los navegadores modernos

### 📦 Distribución
- Compilación a WASM lista para NPM
- Tipos TypeScript incluidos
- Ejemplos de uso para múltiples frameworks
- Scripts de build para Windows y Linux/Mac

---

## [3.0.0] - 2025-02-15

### ✨ Características Iniciales
- Generación de comprobantes con SHA-256
- Códigos de control y verificación
- Generación de QR en SVG
- HTML template para impresión
- Validación básica
- CLI funcional
- Compilación a WASM

---

## [2.1.0] - 2024-12-01

### Versión Anterior
- Sistema básico de comprobantes
- Soporte para múltiples tipos de comprobante
- Generación de HTML y JSON

---

## Roadmap Futuro

### v4.1.0
- [ ] Firma digital RSA/ECDSA
- [ ] Timestamp de servidor confiable
- [ ] Historial de cambios
- [ ] Bloqueo de edición post-generación

### v4.2.0
- [ ] Cálculo de retenciones (IVA, IT)
- [ ] Descuentos por volumen/tiered
- [ ] Impuestos progresivos
- [ ] Campos de referencia a comprobantes anteriores

### v5.0.0
- [ ] API REST completa
- [ ] Webhooks para eventos
- [ ] Sincronización con sistemas externos
- [ ] Batch processing
- [ ] Reportes y análisis
- [ ] Dashboard de auditoría

---

## Notas de Compatibilidad

### Navegadores Soportados
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+
- Opera 44+

### Plataformas
- Windows
- macOS
- Linux
- Node.js 14+

### Frameworks
- React 16.8+
- Vue 3+
- Angular 12+
- Svelte 3+
- Next.js 12+
- Laravel 8+
- Express 4+

---

## Licencia

GPLv3 - Ver archivo LICENSE

---

**ARK Facture v4.0** - Potenciado por Rust 🦀 y WebAssembly 🕸️

Desarrollado por **ARK DEV SYSTEM**
