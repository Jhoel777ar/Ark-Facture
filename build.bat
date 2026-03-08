@echo off
REM ARK Facture v4.0 - Build Script (Windows)
REM Compila Rust a WASM y prepara para NPM

setlocal enabledelayedexpansion

echo.
echo 🔨 ARK Facture v4.0 - Build Script (Windows)
echo ============================================
echo.

REM Verificar dependencias
echo ✓ Verificando dependencias...
where wasm-pack >nul 2>nul
if errorlevel 1 (
    echo ❌ wasm-pack no está instalado
    echo    Instala desde: https://rustwasm.org/wasm-pack/installer/
    exit /b 1
)

where cargo >nul 2>nul
if errorlevel 1 (
    echo ❌ Rust no está instalado
    echo    Instala desde: https://rustup.rs/
    exit /b 1
)

echo ✓ Dependencias OK
echo.

REM Limpiar builds anteriores
echo 🧹 Limpiando builds anteriores...
if exist pkg rmdir /s /q pkg
if exist target rmdir /s /q target

echo ✓ Limpieza completada
echo.

REM Compilar a WASM
echo 🦀 Compilando Rust a WASM...
call wasm-pack build --target bundler --release
if errorlevel 1 (
    echo ❌ Error en la compilación
    exit /b 1
)

echo ✓ Compilación completada
echo.

REM Verificar archivos generados
echo 📦 Verificando archivos generados...
if exist "pkg\facture_ark.js" (
    if exist "pkg\facture_ark_bg.wasm" (
        echo ✓ Archivos WASM generados correctamente
        
        REM Mostrar tamaño
        for %%A in (pkg\facture_ark_bg.wasm) do (
            set size=%%~zA
            echo    Tamaño WASM: !size! bytes
        )
    ) else (
        echo ❌ Error: facture_ark_bg.wasm no encontrado
        exit /b 1
    )
) else (
    echo ❌ Error: facture_ark.js no encontrado
    exit /b 1
)

echo.

REM Ejecutar tests
echo 🧪 Ejecutando tests...
cargo test --lib 2>&1 | findstr "test result passed failed"

echo.

REM Información final
echo ✅ Build completado exitosamente!
echo.
echo 📋 Próximos pasos:
echo    1. Revisar pkg\package.json
echo    2. Ejecutar: npm publish pkg\
echo    3. O usar localmente: npm install .\pkg
echo.
echo 📚 Documentación:
echo    - README.md - Documentación completa
echo    - pkg\INTEGRATION.md - Guía de integración
echo    - index.html - Demo interactiva
echo.

pause
