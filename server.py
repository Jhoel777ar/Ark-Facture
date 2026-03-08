#!/usr/bin/env python3
"""
Servidor HTTP para ARK Facture v4.0
Sirve archivos con MIME types correctos para WASM
"""

import http.server
import socketserver
import os
from pathlib import Path

PORT = 8000

class WasmHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    """Handler que sirve WASM con MIME type correcto"""
    
    def end_headers(self):
        """Agrega headers CORS y MIME types correctos"""
        # CORS headers
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        
        # Cache headers
        self.send_header('Cache-Control', 'no-cache, no-store, must-revalidate')
        self.send_header('Pragma', 'no-cache')
        self.send_header('Expires', '0')
        
        super().end_headers()
    
    def guess_type(self, path):
        """Retorna MIME type correcto para archivos"""
        if path.endswith('.wasm'):
            return 'application/wasm'
        elif path.endswith('.js'):
            return 'application/javascript'
        elif path.endswith('.json'):
            return 'application/json'
        elif path.endswith('.xml'):
            return 'application/xml'
        elif path.endswith('.html'):
            return 'text/html'
        elif path.endswith('.css'):
            return 'text/css'
        
        result = super().guess_type(path)
        if isinstance(result, tuple):
            return result[0]
        return result
    
    def log_message(self, format, *args):
        """Log personalizado"""
        print(f"[{self.log_date_time_string()}] {format % args}")


def main():
    """Inicia el servidor"""
    os.chdir('.')
    
    handler = WasmHTTPRequestHandler
    
    with socketserver.TCPServer(("", PORT), handler) as httpd:
        print("=" * 60)
        print("🧾 ARK Facture v4.0 - Servidor HTTP")
        print("=" * 60)
        print()
        print(f"✅ Servidor iniciado en http://localhost:{PORT}")
        print()
        print("📍 URLs disponibles:")
        print(f"   - http://localhost:{PORT}/demo.html")
        print(f"   - http://localhost:{PORT}/index.html")
        print(f"   - http://localhost:{PORT}/pkg/facture_ark.js")
        print(f"   - http://localhost:{PORT}/pkg/facture_ark_bg.wasm")
        print()
        print("🛑 Presiona Ctrl+C para detener el servidor")
        print()
        
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n\n👋 Servidor detenido")


if __name__ == "__main__":
    main()

