use iban_validation_rs::{get_source_file, get_version, validate_iban_str};
use wasm_bindgen::prelude::*;

// JS/WASM wrapper
#[wasm_bindgen]
pub fn validate_iban_js(input: &str) -> Result<bool, JsValue> {
    match validate_iban_str(input) {
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
}

#[wasm_bindgen]
pub fn parse_iban_js(input: &str) -> Result<JsIban, JsValue> {
    match validate_iban_str(input) {
        Ok(true) => {
            let parsed = iban_validation_rs::Iban::new(input)
                .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;
            Ok(JsIban {
                iban: parsed.get_iban().to_string(),
                bank_id: parsed.iban_bank_id.map(|s| s.to_string()),
                branch_id: parsed.iban_branch_id.map(|s| s.to_string()),
            })
        }
        Ok(false) => Err(JsValue::from_str("Invalid IBAN")),
        Err(e) => Err(JsValue::from_str(&format!("Validation error: {}", e))),
    }
}
