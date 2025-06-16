#![no_main]

use libfuzzer_sys::fuzz_target;
use iban_validation_rs::validate_iban_str;

fuzz_target!(|data: &str| {
    // libFuzzer gives &[u8], but cargo-fuzz maps it to &str here
    let _ = validate_iban_str(data);
});
