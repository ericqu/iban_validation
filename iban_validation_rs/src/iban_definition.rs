// Auto-generated from iban_validation_preprocess/pre_process_registry.py, do not edit manually
use crate::{IbanFields, ValidationLetterError};
use crate::{simple_contains_a, simple_contains_c, simple_contains_n};

pub const _IBAN_MIN_LEN: u8 = 15;
pub const _IBAN_MAX_LEN: u8 = 33;

pub const IBAN_DEFINITIONS: [IbanFields; 104] = [
    IbanFields {
        ctry_cd: [65, 68], // "AD" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_a,
            literal_d,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [65, 69], // "AE" 23 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_a,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [65, 76], // "AL" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_a,
            literal_l,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [65, 84], // "AT" 20 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_a,
            literal_t,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [65, 90], // "AZ" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_a,
            literal_z,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [66, 65], // "BA" 20 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(6),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_b,
            literal_a,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [66, 69], // "BE" 16 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_b,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [66, 71], // "BG" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_b,
            literal_g,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [66, 72], // "BH" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_b,
            literal_h,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [66, 73], // "BI" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_b,
            literal_i,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [66, 82], // "BR" 29 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(8),
        branch_id_pos_s: Some(9),
        branch_id_pos_e: Some(13),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_a,
            simple_contains_c,
            literal_b,
            literal_r,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [66, 89], // "BY" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_b,
            literal_y,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [67, 72], // "CH" 21 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_c,
            literal_h,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [67, 82], // "CR" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_c,
            literal_r,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [67, 89], // "CY" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_c,
            literal_y,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [67, 90], // "CZ" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_c,
            literal_z,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [68, 69], // "DE" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(8),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_d,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [68, 74], // "DJ" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_d,
            literal_j,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [68, 75], // "DK" 18 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_d,
            literal_k,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [68, 79], // "DO" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_d,
            literal_o,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [69, 69], // "EE" 20 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_e,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [69, 71], // "EG" 29 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_e,
            literal_g,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [69, 83], // "ES" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_e,
            literal_s,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [70, 73], // "FI" 18 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_f,
            literal_i,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [70, 75], // "FK" 18 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_f,
            literal_k,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [70, 79], // "FO" 18 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_f,
            literal_o,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [70, 82], // "FR" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_f,
            literal_r,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [71, 80], // "GP" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_g,
            literal_p,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [77, 81], // "MQ" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_m,
            literal_q,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [71, 70], // "GF" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_g,
            literal_f,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [82, 69], // "RE" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_r,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [89, 84], // "YT" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_y,
            literal_t,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [78, 67], // "NC" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_n,
            literal_c,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [80, 70], // "PF" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_p,
            literal_f,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [80, 77], // "PM" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_p,
            literal_m,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [84, 70], // "TF" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_t,
            literal_f,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [87, 70], // "WF" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_w,
            literal_f,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [66, 76], // "BL" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_b,
            literal_l,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [77, 70], // "MF" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_m,
            literal_f,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [71, 66], // "GB" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_g,
            literal_b,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [73, 77], // "IM" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_i,
            literal_m,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [74, 69], // "JE" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_j,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [71, 71], // "GG" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_g,
            literal_g,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [71, 69], // "GE" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_g,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [71, 73], // "GI" 23 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_g,
            literal_i,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [71, 76], // "GL" 18 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_g,
            literal_l,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [71, 82], // "GR" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(7),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_g,
            literal_r,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [71, 84], // "GT" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_g,
            literal_t,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [72, 78], // "HN" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_h,
            literal_n,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [72, 82], // "HR" 21 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(7),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_h,
            literal_r,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [72, 85], // "HU" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(7),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_h,
            literal_u,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [73, 69], // "IE" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_i,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [73, 76], // "IL" 23 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(6),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_i,
            literal_l,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [73, 81], // "IQ" 23 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(7),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_i,
            literal_q,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [73, 83], // "IS" 26 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: Some(3),
        branch_id_pos_e: Some(4),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_i,
            literal_s,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [73, 84], // "IT" 27 characters
        bank_id_pos_s: Some(2),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(11),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_i,
            literal_t,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [74, 79], // "JO" 30 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_j,
            literal_o,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [75, 87], // "KW" 30 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_k,
            literal_w,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [75, 90], // "KZ" 20 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_k,
            literal_z,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [76, 66], // "LB" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_l,
            literal_b,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [76, 67], // "LC" 32 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_l,
            literal_c,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [76, 73], // "LI" 21 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_l,
            literal_i,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [76, 84], // "LT" 20 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_l,
            literal_t,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [76, 85], // "LU" 20 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_l,
            literal_u,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [76, 86], // "LV" 21 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_l,
            literal_v,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [76, 89], // "LY" 25 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(6),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_l,
            literal_y,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [77, 67], // "MC" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_m,
            literal_c,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [77, 68], // "MD" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_m,
            literal_d,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [77, 69], // "ME" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_m,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [77, 75], // "MK" 19 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_n,
            simple_contains_n,
            literal_m,
            literal_k,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [77, 78], // "MN" 20 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_m,
            literal_n,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [77, 82], // "MR" 27 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_m,
            literal_r,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [77, 84], // "MT" 31 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(9),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_m,
            literal_t,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [77, 85], // "MU" 30 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            literal_m,
            literal_u,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [78, 73], // "NI" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_n,
            literal_i,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [78, 76], // "NL" 18 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_n,
            literal_l,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [78, 79], // "NO" 15 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_n,
            literal_o,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [79, 77], // "OM" 23 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_o,
            literal_m,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [80, 75], // "PK" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_p,
            literal_k,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [80, 76], // "PL" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(8),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_p,
            literal_l,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [80, 83], // "PS" 29 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_p,
            literal_s,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [80, 84], // "PT" 25 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_p,
            literal_t,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [81, 65], // "QA" 29 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_q,
            literal_a,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [82, 79], // "RO" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_r,
            literal_o,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [82, 83], // "RS" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_r,
            literal_s,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [82, 85], // "RU" 33 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(9),
        branch_id_pos_s: Some(10),
        branch_id_pos_e: Some(14),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_r,
            literal_u,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [83, 65], // "SA" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_s,
            literal_a,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [83, 67], // "SC" 31 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            literal_s,
            literal_c,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [83, 68], // "SD" 18 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_s,
            literal_d,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [83, 69], // "SE" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_s,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [83, 73], // "SI" 19 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_s,
            literal_i,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [83, 75], // "SK" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_s,
            literal_k,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [83, 77], // "SM" 27 characters
        bank_id_pos_s: Some(2),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(11),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_s,
            literal_m,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [83, 79], // "SO" 23 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(7),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_s,
            literal_o,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [83, 84], // "ST" 25 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_s,
            literal_t,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [83, 86], // "SV" 28 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_s,
            literal_v,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [84, 76], // "TL" 23 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_t,
            literal_l,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [84, 78], // "TN" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: Some(3),
        branch_id_pos_e: Some(5),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_t,
            literal_n,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [84, 82], // "TR" 26 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_t,
            literal_r,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [85, 65], // "UA" 29 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_u,
            literal_a,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [86, 65], // "VA" 22 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_v,
            literal_a,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [86, 71], // "VG" 24 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_v,
            literal_g,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [88, 75], // "XK" 20 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: Some(3),
        branch_id_pos_e: Some(4),
        iban_struct_validators: &[
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            literal_x,
            literal_k,
            simple_contains_n,
            simple_contains_n,
        ],
    },
    IbanFields {
        ctry_cd: [89, 69], // "YE" 30 characters
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct_validators: &[
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_a,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_n,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            simple_contains_c,
            literal_y,
            literal_e,
            simple_contains_n,
            simple_contains_n,
        ],
    },
];

pub fn get_iban_fields(cc: [u8; 2]) -> Option<&'static IbanFields> {
    match cc {
        [65, 68] => Some(&IBAN_DEFINITIONS[0]),   // AD
        [65, 69] => Some(&IBAN_DEFINITIONS[1]),   // AE
        [65, 76] => Some(&IBAN_DEFINITIONS[2]),   // AL
        [65, 84] => Some(&IBAN_DEFINITIONS[3]),   // AT
        [65, 90] => Some(&IBAN_DEFINITIONS[4]),   // AZ
        [66, 65] => Some(&IBAN_DEFINITIONS[5]),   // BA
        [66, 69] => Some(&IBAN_DEFINITIONS[6]),   // BE
        [66, 71] => Some(&IBAN_DEFINITIONS[7]),   // BG
        [66, 72] => Some(&IBAN_DEFINITIONS[8]),   // BH
        [66, 73] => Some(&IBAN_DEFINITIONS[9]),   // BI
        [66, 82] => Some(&IBAN_DEFINITIONS[10]),  // BR
        [66, 89] => Some(&IBAN_DEFINITIONS[11]),  // BY
        [67, 72] => Some(&IBAN_DEFINITIONS[12]),  // CH
        [67, 82] => Some(&IBAN_DEFINITIONS[13]),  // CR
        [67, 89] => Some(&IBAN_DEFINITIONS[14]),  // CY
        [67, 90] => Some(&IBAN_DEFINITIONS[15]),  // CZ
        [68, 69] => Some(&IBAN_DEFINITIONS[16]),  // DE
        [68, 74] => Some(&IBAN_DEFINITIONS[17]),  // DJ
        [68, 75] => Some(&IBAN_DEFINITIONS[18]),  // DK
        [68, 79] => Some(&IBAN_DEFINITIONS[19]),  // DO
        [69, 69] => Some(&IBAN_DEFINITIONS[20]),  // EE
        [69, 71] => Some(&IBAN_DEFINITIONS[21]),  // EG
        [69, 83] => Some(&IBAN_DEFINITIONS[22]),  // ES
        [70, 73] => Some(&IBAN_DEFINITIONS[23]),  // FI
        [70, 75] => Some(&IBAN_DEFINITIONS[24]),  // FK
        [70, 79] => Some(&IBAN_DEFINITIONS[25]),  // FO
        [70, 82] => Some(&IBAN_DEFINITIONS[26]),  // FR
        [71, 80] => Some(&IBAN_DEFINITIONS[27]),  // GP
        [77, 81] => Some(&IBAN_DEFINITIONS[28]),  // MQ
        [71, 70] => Some(&IBAN_DEFINITIONS[29]),  // GF
        [82, 69] => Some(&IBAN_DEFINITIONS[30]),  // RE
        [89, 84] => Some(&IBAN_DEFINITIONS[31]),  // YT
        [78, 67] => Some(&IBAN_DEFINITIONS[32]),  // NC
        [80, 70] => Some(&IBAN_DEFINITIONS[33]),  // PF
        [80, 77] => Some(&IBAN_DEFINITIONS[34]),  // PM
        [84, 70] => Some(&IBAN_DEFINITIONS[35]),  // TF
        [87, 70] => Some(&IBAN_DEFINITIONS[36]),  // WF
        [66, 76] => Some(&IBAN_DEFINITIONS[37]),  // BL
        [77, 70] => Some(&IBAN_DEFINITIONS[38]),  // MF
        [71, 66] => Some(&IBAN_DEFINITIONS[39]),  // GB
        [73, 77] => Some(&IBAN_DEFINITIONS[40]),  // IM
        [74, 69] => Some(&IBAN_DEFINITIONS[41]),  // JE
        [71, 71] => Some(&IBAN_DEFINITIONS[42]),  // GG
        [71, 69] => Some(&IBAN_DEFINITIONS[43]),  // GE
        [71, 73] => Some(&IBAN_DEFINITIONS[44]),  // GI
        [71, 76] => Some(&IBAN_DEFINITIONS[45]),  // GL
        [71, 82] => Some(&IBAN_DEFINITIONS[46]),  // GR
        [71, 84] => Some(&IBAN_DEFINITIONS[47]),  // GT
        [72, 78] => Some(&IBAN_DEFINITIONS[48]),  // HN
        [72, 82] => Some(&IBAN_DEFINITIONS[49]),  // HR
        [72, 85] => Some(&IBAN_DEFINITIONS[50]),  // HU
        [73, 69] => Some(&IBAN_DEFINITIONS[51]),  // IE
        [73, 76] => Some(&IBAN_DEFINITIONS[52]),  // IL
        [73, 81] => Some(&IBAN_DEFINITIONS[53]),  // IQ
        [73, 83] => Some(&IBAN_DEFINITIONS[54]),  // IS
        [73, 84] => Some(&IBAN_DEFINITIONS[55]),  // IT
        [74, 79] => Some(&IBAN_DEFINITIONS[56]),  // JO
        [75, 87] => Some(&IBAN_DEFINITIONS[57]),  // KW
        [75, 90] => Some(&IBAN_DEFINITIONS[58]),  // KZ
        [76, 66] => Some(&IBAN_DEFINITIONS[59]),  // LB
        [76, 67] => Some(&IBAN_DEFINITIONS[60]),  // LC
        [76, 73] => Some(&IBAN_DEFINITIONS[61]),  // LI
        [76, 84] => Some(&IBAN_DEFINITIONS[62]),  // LT
        [76, 85] => Some(&IBAN_DEFINITIONS[63]),  // LU
        [76, 86] => Some(&IBAN_DEFINITIONS[64]),  // LV
        [76, 89] => Some(&IBAN_DEFINITIONS[65]),  // LY
        [77, 67] => Some(&IBAN_DEFINITIONS[66]),  // MC
        [77, 68] => Some(&IBAN_DEFINITIONS[67]),  // MD
        [77, 69] => Some(&IBAN_DEFINITIONS[68]),  // ME
        [77, 75] => Some(&IBAN_DEFINITIONS[69]),  // MK
        [77, 78] => Some(&IBAN_DEFINITIONS[70]),  // MN
        [77, 82] => Some(&IBAN_DEFINITIONS[71]),  // MR
        [77, 84] => Some(&IBAN_DEFINITIONS[72]),  // MT
        [77, 85] => Some(&IBAN_DEFINITIONS[73]),  // MU
        [78, 73] => Some(&IBAN_DEFINITIONS[74]),  // NI
        [78, 76] => Some(&IBAN_DEFINITIONS[75]),  // NL
        [78, 79] => Some(&IBAN_DEFINITIONS[76]),  // NO
        [79, 77] => Some(&IBAN_DEFINITIONS[77]),  // OM
        [80, 75] => Some(&IBAN_DEFINITIONS[78]),  // PK
        [80, 76] => Some(&IBAN_DEFINITIONS[79]),  // PL
        [80, 83] => Some(&IBAN_DEFINITIONS[80]),  // PS
        [80, 84] => Some(&IBAN_DEFINITIONS[81]),  // PT
        [81, 65] => Some(&IBAN_DEFINITIONS[82]),  // QA
        [82, 79] => Some(&IBAN_DEFINITIONS[83]),  // RO
        [82, 83] => Some(&IBAN_DEFINITIONS[84]),  // RS
        [82, 85] => Some(&IBAN_DEFINITIONS[85]),  // RU
        [83, 65] => Some(&IBAN_DEFINITIONS[86]),  // SA
        [83, 67] => Some(&IBAN_DEFINITIONS[87]),  // SC
        [83, 68] => Some(&IBAN_DEFINITIONS[88]),  // SD
        [83, 69] => Some(&IBAN_DEFINITIONS[89]),  // SE
        [83, 73] => Some(&IBAN_DEFINITIONS[90]),  // SI
        [83, 75] => Some(&IBAN_DEFINITIONS[91]),  // SK
        [83, 77] => Some(&IBAN_DEFINITIONS[92]),  // SM
        [83, 79] => Some(&IBAN_DEFINITIONS[93]),  // SO
        [83, 84] => Some(&IBAN_DEFINITIONS[94]),  // ST
        [83, 86] => Some(&IBAN_DEFINITIONS[95]),  // SV
        [84, 76] => Some(&IBAN_DEFINITIONS[96]),  // TL
        [84, 78] => Some(&IBAN_DEFINITIONS[97]),  // TN
        [84, 82] => Some(&IBAN_DEFINITIONS[98]),  // TR
        [85, 65] => Some(&IBAN_DEFINITIONS[99]),  // UA
        [86, 65] => Some(&IBAN_DEFINITIONS[100]), // VA
        [86, 71] => Some(&IBAN_DEFINITIONS[101]), // VG
        [88, 75] => Some(&IBAN_DEFINITIONS[102]), // XK
        [89, 69] => Some(&IBAN_DEFINITIONS[103]), // YE
        _ => None,
    }
}

#[inline]
fn literal_a(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 65 {
        // 'A'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_b(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 66 {
        // 'B'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_c(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 67 {
        // 'C'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_d(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 68 {
        // 'D'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_e(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 69 {
        // 'E'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_f(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 70 {
        // 'F'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_g(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 71 {
        // 'G'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_h(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 72 {
        // 'H'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_i(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 73 {
        // 'I'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_j(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 74 {
        // 'J'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_k(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 75 {
        // 'K'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_l(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 76 {
        // 'L'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_m(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 77 {
        // 'M'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_n(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 78 {
        // 'N'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_o(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 79 {
        // 'O'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_p(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 80 {
        // 'P'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_q(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 81 {
        // 'Q'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_r(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 82 {
        // 'R'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_s(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 83 {
        // 'S'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_t(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 84 {
        // 'T'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_u(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 85 {
        // 'U'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_v(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 86 {
        // 'V'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_w(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 87 {
        // 'W'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_x(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 88 {
        // 'X'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_y(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 89 {
        // 'Y'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

#[inline]
fn literal_z(c: u8) -> Result<usize, ValidationLetterError> {
    if c == 90 {
        // 'Z'
        Ok((c - 55) as usize)
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}
// Compile-time invariants
const _: () = {
    let _ = [(); (24 >= 4) as usize - 1]; // AD
    let _ = [(); (23 >= 4) as usize - 1]; // AE
    let _ = [(); (28 >= 4) as usize - 1]; // AL
    let _ = [(); (20 >= 4) as usize - 1]; // AT
    let _ = [(); (28 >= 4) as usize - 1]; // AZ
    let _ = [(); (20 >= 4) as usize - 1]; // BA
    let _ = [(); (16 >= 4) as usize - 1]; // BE
    let _ = [(); (22 >= 4) as usize - 1]; // BG
    let _ = [(); (22 >= 4) as usize - 1]; // BH
    let _ = [(); (27 >= 4) as usize - 1]; // BI
    let _ = [(); (29 >= 4) as usize - 1]; // BR
    let _ = [(); (28 >= 4) as usize - 1]; // BY
    let _ = [(); (21 >= 4) as usize - 1]; // CH
    let _ = [(); (22 >= 4) as usize - 1]; // CR
    let _ = [(); (28 >= 4) as usize - 1]; // CY
    let _ = [(); (24 >= 4) as usize - 1]; // CZ
    let _ = [(); (22 >= 4) as usize - 1]; // DE
    let _ = [(); (27 >= 4) as usize - 1]; // DJ
    let _ = [(); (18 >= 4) as usize - 1]; // DK
    let _ = [(); (28 >= 4) as usize - 1]; // DO
    let _ = [(); (20 >= 4) as usize - 1]; // EE
    let _ = [(); (29 >= 4) as usize - 1]; // EG
    let _ = [(); (24 >= 4) as usize - 1]; // ES
    let _ = [(); (18 >= 4) as usize - 1]; // FI
    let _ = [(); (18 >= 4) as usize - 1]; // FK
    let _ = [(); (18 >= 4) as usize - 1]; // FO
    let _ = [(); (27 >= 4) as usize - 1]; // FR
    let _ = [(); (27 >= 4) as usize - 1]; // GP
    let _ = [(); (27 >= 4) as usize - 1]; // MQ
    let _ = [(); (27 >= 4) as usize - 1]; // GF
    let _ = [(); (27 >= 4) as usize - 1]; // RE
    let _ = [(); (27 >= 4) as usize - 1]; // YT
    let _ = [(); (27 >= 4) as usize - 1]; // NC
    let _ = [(); (27 >= 4) as usize - 1]; // PF
    let _ = [(); (27 >= 4) as usize - 1]; // PM
    let _ = [(); (27 >= 4) as usize - 1]; // TF
    let _ = [(); (27 >= 4) as usize - 1]; // WF
    let _ = [(); (27 >= 4) as usize - 1]; // BL
    let _ = [(); (27 >= 4) as usize - 1]; // MF
    let _ = [(); (22 >= 4) as usize - 1]; // GB
    let _ = [(); (22 >= 4) as usize - 1]; // IM
    let _ = [(); (22 >= 4) as usize - 1]; // JE
    let _ = [(); (22 >= 4) as usize - 1]; // GG
    let _ = [(); (22 >= 4) as usize - 1]; // GE
    let _ = [(); (23 >= 4) as usize - 1]; // GI
    let _ = [(); (18 >= 4) as usize - 1]; // GL
    let _ = [(); (27 >= 4) as usize - 1]; // GR
    let _ = [(); (28 >= 4) as usize - 1]; // GT
    let _ = [(); (28 >= 4) as usize - 1]; // HN
    let _ = [(); (21 >= 4) as usize - 1]; // HR
    let _ = [(); (28 >= 4) as usize - 1]; // HU
    let _ = [(); (22 >= 4) as usize - 1]; // IE
    let _ = [(); (23 >= 4) as usize - 1]; // IL
    let _ = [(); (23 >= 4) as usize - 1]; // IQ
    let _ = [(); (26 >= 4) as usize - 1]; // IS
    let _ = [(); (27 >= 4) as usize - 1]; // IT
    let _ = [(); (30 >= 4) as usize - 1]; // JO
    let _ = [(); (30 >= 4) as usize - 1]; // KW
    let _ = [(); (20 >= 4) as usize - 1]; // KZ
    let _ = [(); (28 >= 4) as usize - 1]; // LB
    let _ = [(); (32 >= 4) as usize - 1]; // LC
    let _ = [(); (21 >= 4) as usize - 1]; // LI
    let _ = [(); (20 >= 4) as usize - 1]; // LT
    let _ = [(); (20 >= 4) as usize - 1]; // LU
    let _ = [(); (21 >= 4) as usize - 1]; // LV
    let _ = [(); (25 >= 4) as usize - 1]; // LY
    let _ = [(); (27 >= 4) as usize - 1]; // MC
    let _ = [(); (24 >= 4) as usize - 1]; // MD
    let _ = [(); (22 >= 4) as usize - 1]; // ME
    let _ = [(); (19 >= 4) as usize - 1]; // MK
    let _ = [(); (20 >= 4) as usize - 1]; // MN
    let _ = [(); (27 >= 4) as usize - 1]; // MR
    let _ = [(); (31 >= 4) as usize - 1]; // MT
    let _ = [(); (30 >= 4) as usize - 1]; // MU
    let _ = [(); (28 >= 4) as usize - 1]; // NI
    let _ = [(); (18 >= 4) as usize - 1]; // NL
    let _ = [(); (15 >= 4) as usize - 1]; // NO
    let _ = [(); (23 >= 4) as usize - 1]; // OM
    let _ = [(); (24 >= 4) as usize - 1]; // PK
    let _ = [(); (28 >= 4) as usize - 1]; // PL
    let _ = [(); (29 >= 4) as usize - 1]; // PS
    let _ = [(); (25 >= 4) as usize - 1]; // PT
    let _ = [(); (29 >= 4) as usize - 1]; // QA
    let _ = [(); (24 >= 4) as usize - 1]; // RO
    let _ = [(); (22 >= 4) as usize - 1]; // RS
    let _ = [(); (33 >= 4) as usize - 1]; // RU
    let _ = [(); (24 >= 4) as usize - 1]; // SA
    let _ = [(); (31 >= 4) as usize - 1]; // SC
    let _ = [(); (18 >= 4) as usize - 1]; // SD
    let _ = [(); (24 >= 4) as usize - 1]; // SE
    let _ = [(); (19 >= 4) as usize - 1]; // SI
    let _ = [(); (24 >= 4) as usize - 1]; // SK
    let _ = [(); (27 >= 4) as usize - 1]; // SM
    let _ = [(); (23 >= 4) as usize - 1]; // SO
    let _ = [(); (25 >= 4) as usize - 1]; // ST
    let _ = [(); (28 >= 4) as usize - 1]; // SV
    let _ = [(); (23 >= 4) as usize - 1]; // TL
    let _ = [(); (24 >= 4) as usize - 1]; // TN
    let _ = [(); (26 >= 4) as usize - 1]; // TR
    let _ = [(); (29 >= 4) as usize - 1]; // UA
    let _ = [(); (22 >= 4) as usize - 1]; // VA
    let _ = [(); (24 >= 4) as usize - 1]; // VG
    let _ = [(); (20 >= 4) as usize - 1]; // XK
    let _ = [(); (30 >= 4) as usize - 1]; // YE
};
