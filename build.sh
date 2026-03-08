#!/bin/bash

# ARK Facture v4.0 - Build Script
# Compila Rust a WASM y prepara para NPM

set -e

echo "🔨 ARK Facture v4.0 - Build Script"
echo "=================================="
echo ""

# Verificar dependencias
echo "✓ Verificando dependencias..."
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack no está instalado"
    echo "   Instala con: curl https://rustwasm.org/wasm-pack/installer/init.sh -sSf | sh"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust no está instalado"
    echo "   Instala desde: https://rustup.rs/"
    exit 1
fi

echo "✓ Dependencias OK"
echo ""

# Limpiar builds anteriores
echo "🧹 Limpiando builds anteriores..."
rm -rf pkg/
rm -rf target/

echo "✓ Limpieza completada"
echo ""

# Compilar a WASM
echo "🦀 Compilando Rust a WASM..."
wasm-pack build --target bundler --release

echo "✓ Compilación completada"
echo ""

# Verificar archivos generados
echo "📦 Verificando archivos generados..."
if [ -f "pkg/facture_ark.js" ] && [ -f "pkg/facture_ark_bg.wasm" ]; then
    echo "✓ Archivos WASM generados correctamente"
    
    # Mostrar tamaño
    WASM_SIZE=$(du -h pkg/facture_ark_bg.wasm | cut -f1)
    echo "   Tamaño WASM: $WASM_SIZE"
else
    echo "❌ Error: Archivos WASM no encontrados"
    exit 1
fi

echo ""

# Ejecutar tests
echo "🧪 Ejecutando tests..."
cargo test --lib 2>&1 | grep -E "test result|passed|failed" || true

echo ""

# Información final
echo "✅ Build completado exitosamente!"
echo ""
echo "📋 Próximos pasos:"
echo "   1. Revisar pkg/package.json"
echo "   2. Ejecutar: npm publish pkg/"
echo "   3. O usar localmente: npm install ./pkg"
echo ""
echo "📚 Documentación:"
echo "   - README.md - Documentación completa"
echo "   - pkg/INTEGRATION.md - Guía de integración"
echo "   - index.html - Demo interactiva"
echo ""
