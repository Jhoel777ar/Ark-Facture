use regex::Regex;

pub struct ValidadorBoliviano;

impl ValidadorBoliviano {
    pub fn validar_nit(nit: &str) -> bool {
        let nit_clean = nit.trim().replace("-", "").replace(" ", "");
        if nit_clean.len() < 7 || nit_clean.len() > 13 {
            return false;
        }
        nit_clean.chars().all(|c| c.is_numeric())
    }

    pub fn validar_ci(ci: &str) -> bool {
        let ci_clean = ci.trim().replace(" ", "");
        if ci_clean.len() < 5 || ci_clean.len() > 12 {
            return false;
        }
        let parts: Vec<&str> = ci_clean.split(|c| c == '-' || c == ' ').collect();
        if parts.is_empty() {
            return false;
        }
        parts[0].chars().all(|c| c.is_numeric())
    }

    pub fn validar_email(email: &str) -> bool {
        let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        re.is_match(email)
    }

    pub fn validar_telefono(telefono: &str) -> bool {
        let re = Regex::new(r"^\+?[0-9\s\-\(\)]{7,}$").unwrap();
        re.is_match(telefono)
    }

    pub fn validar_descripcion(desc: &str) -> bool {
        let desc_clean = desc.trim();
        !desc_clean.is_empty() && desc_clean.len() <= 1000
    }

    pub fn sanitizar_html(input: &str) -> String {
        input
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#39;")
    }
}

pub fn validar_cantidad(cantidad: f64) -> bool {
    cantidad > 0.0 && cantidad <= 5000.0
}

pub fn validar_precio(precio: f64) -> bool {
    precio >= 0.0 && precio <= 10_000_000_000.0
}

pub fn validar_moneda(moneda: &str) -> bool {
    matches!(moneda, "BOB" | "USD" | "EUR" | "ARS" | "CLP" | "PEN")
}

pub fn validar_tipo_comprobante(tipo: &str) -> bool {
    matches!(
        tipo,
        "recibo" | "proforma" | "nota_venta" | "compra" | "interno" | "nota_credito" | "nota_debito"
    )
}
