use iban_validation_rs::{CountrySet, get_source_file, get_version, validate_iban_str_with};
use wasm_bindgen::prelude::*;

#[inline]
fn country_set(allow_non_registry: Option<bool>) -> CountrySet {
    allow_non_registry.unwrap_or(false).into()
}

/// Validates an IBAN. `allowNonRegistry` (default `false`) additionally accepts 22
/// IBAN-shaped account numbers that are not in the official SWIFT IBAN registry
/// (community-sourced, weaker guarantees).
// JS/WASM wrapper
#[wasm_bindgen]
pub fn validate_iban_js(input: &str, allow_non_registry: Option<bool>) -> Result<bool, JsValue> {
    match validate_iban_str_with(input, country_set(allow_non_registry)) {
        Ok(valid) => Ok(valid),
        Err(e) => Err(JsValue::from_str(&format!("Validation error: {}", e))),
    }
}

#[wasm_bindgen]
pub fn get_source_file_js() -> String {
    get_source_file().to_string()
}

#[wasm_bindgen]
pub fn get_version_js() -> String {
    get_version().to_string()
}

#[wasm_bindgen]
pub struct JsIban {
    iban: String,
    bank_id: Option<String>,
    branch_id: Option<String>,
    is_non_registry: bool,
}

#[wasm_bindgen]
impl JsIban {
    #[wasm_bindgen(getter)]
    pub fn iban(&self) -> String {
        self.iban.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn bank_id(&self) -> Option<String> {
        self.bank_id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn branch_id(&self) -> Option<String> {
        self.branch_id.clone()
    }

    /// Whether the matched country is one of the non-registry countries.
    #[wasm_bindgen(getter)]
    pub fn is_non_registry(&self) -> bool {
        self.is_non_registry
    }
}

/// Validates and parses an IBAN. `allowNonRegistry` (default `false`) additionally
/// accepts 22 IBAN-shaped account numbers that are not in the official SWIFT IBAN
/// registry (community-sourced, weaker guarantees).
#[wasm_bindgen]
pub fn parse_iban_js(input: &str, allow_non_registry: Option<bool>) -> Result<JsIban, JsValue> {
    let set = country_set(allow_non_registry);
    match iban_validation_rs::Iban::new_with(input, set) {
        Ok(parsed) => {
            let cc: [u8; 2] = input.as_bytes()[0..2].try_into().unwrap();
            Ok(JsIban {
                iban: parsed.get_iban().to_string(),
                bank_id: parsed.iban_bank_id.map(|s| s.to_string()),
                branch_id: parsed.iban_branch_id.map(|s| s.to_string()),
                is_non_registry: iban_validation_rs::is_non_registry_country(cc),
            })
        }
        Err(e) => Err(JsValue::from_str(&format!("Validation error: {}", e))),
    }
}

/// Lists the two-letter codes of the opt-in, non-registry countries.
#[wasm_bindgen]
pub fn non_registry_countries_js() -> Vec<String> {
    iban_validation_rs::NON_REGISTRY_COUNTRIES
        .iter()
        .map(|cc| std::str::from_utf8(cc).unwrap().to_string())
        .collect()
}
