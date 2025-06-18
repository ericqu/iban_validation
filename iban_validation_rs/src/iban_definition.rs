// Auto-generated from iban_validation_preprocess/pre_process_registry.py, do not edit manually
use crate::IbanFields;

pub const _IBAN_MIN_LEN: u8 = 15;
pub const _IBAN_MAX_LEN: u8 = 33;

pub const IBAN_DEFINITIONS: [IbanFields; 104] = [
    IbanFields {
        ctry_cd: [65, 68], // "AD"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "nnnnnnnnccccccccccccADnn",
    },
    IbanFields {
        ctry_cd: [65, 69], // "AE"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnnnAEnn",
    },
    IbanFields {
        ctry_cd: [65, 76], // "AL"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(7),
        iban_struct: "nnnnnnnnccccccccccccccccALnn",
    },
    IbanFields {
        ctry_cd: [65, 84], // "AT"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnATnn",
    },
    IbanFields {
        ctry_cd: [65, 90], // "AZ"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaaccccccccccccccccccccAZnn",
    },
    IbanFields {
        ctry_cd: [66, 65], // "BA"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(6),
        iban_struct: "nnnnnnnnnnnnnnnnBAnn",
    },
    IbanFields {
        ctry_cd: [66, 69], // "BE"
        iban_len: 16,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnBEnn",
    },
    IbanFields {
        ctry_cd: [66, 71], // "BG"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "aaaannnnnnccccccccBGnn",
    },
    IbanFields {
        ctry_cd: [66, 72], // "BH"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaaccccccccccccccBHnn",
    },
    IbanFields {
        ctry_cd: [66, 73], // "BI"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnnnBInn",
    },
    IbanFields {
        ctry_cd: [66, 82], // "BR"
        iban_len: 29,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(8),
        branch_id_pos_s: Some(9),
        branch_id_pos_e: Some(13),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnnnacBRnn",
    },
    IbanFields {
        ctry_cd: [66, 89], // "BY"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "ccccnnnnccccccccccccccccBYnn",
    },
    IbanFields {
        ctry_cd: [67, 72], // "CH"
        iban_len: 21,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnccccccccccccCHnn",
    },
    IbanFields {
        ctry_cd: [67, 82], // "CR"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnnCRnn",
    },
    IbanFields {
        ctry_cd: [67, 89], // "CY"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(8),
        iban_struct: "nnnnnnnnccccccccccccccccCYnn",
    },
    IbanFields {
        ctry_cd: [67, 90], // "CZ"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnnnnCZnn",
    },
    IbanFields {
        ctry_cd: [68, 69], // "DE"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(8),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnnDEnn",
    },
    IbanFields {
        ctry_cd: [68, 74], // "DJ"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnnnDJnn",
    },
    IbanFields {
        ctry_cd: [68, 75], // "DK"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnDKnn",
    },
    IbanFields {
        ctry_cd: [68, 79], // "DO"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "ccccnnnnnnnnnnnnnnnnnnnnDOnn",
    },
    IbanFields {
        ctry_cd: [69, 69], // "EE"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnEEnn",
    },
    IbanFields {
        ctry_cd: [69, 71], // "EG"
        iban_len: 29,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnnnnnEGnn",
    },
    IbanFields {
        ctry_cd: [69, 83], // "ES"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnESnn",
    },
    IbanFields {
        ctry_cd: [70, 73], // "FI"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnFInn",
    },
    IbanFields {
        ctry_cd: [70, 75], // "FK"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aannnnnnnnnnnnFKnn",
    },
    IbanFields {
        ctry_cd: [70, 79], // "FO"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnFOnn",
    },
    IbanFields {
        ctry_cd: [70, 82], // "FR"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnFRnn",
    },
    IbanFields {
        ctry_cd: [71, 80], // "GP"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnGPnn",
    },
    IbanFields {
        ctry_cd: [77, 81], // "MQ"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnMQnn",
    },
    IbanFields {
        ctry_cd: [71, 70], // "GF"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnGFnn",
    },
    IbanFields {
        ctry_cd: [82, 69], // "RE"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnREnn",
    },
    IbanFields {
        ctry_cd: [89, 84], // "YT"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnYTnn",
    },
    IbanFields {
        ctry_cd: [78, 67], // "NC"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnNCnn",
    },
    IbanFields {
        ctry_cd: [80, 70], // "PF"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnPFnn",
    },
    IbanFields {
        ctry_cd: [80, 77], // "PM"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnPMnn",
    },
    IbanFields {
        ctry_cd: [84, 70], // "TF"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnTFnn",
    },
    IbanFields {
        ctry_cd: [87, 70], // "WF"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnWFnn",
    },
    IbanFields {
        ctry_cd: [66, 76], // "BL"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnBLnn",
    },
    IbanFields {
        ctry_cd: [77, 70], // "MF"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnncccccccccccnnMFnn",
    },
    IbanFields {
        ctry_cd: [71, 66], // "GB"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct: "aaaannnnnnnnnnnnnnGBnn",
    },
    IbanFields {
        ctry_cd: [73, 77], // "IM"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct: "aaaannnnnnnnnnnnnnIMnn",
    },
    IbanFields {
        ctry_cd: [74, 69], // "JE"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct: "aaaannnnnnnnnnnnnnJEnn",
    },
    IbanFields {
        ctry_cd: [71, 71], // "GG"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct: "aaaannnnnnnnnnnnnnGGnn",
    },
    IbanFields {
        ctry_cd: [71, 69], // "GE"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aannnnnnnnnnnnnnnnGEnn",
    },
    IbanFields {
        ctry_cd: [71, 73], // "GI"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaacccccccccccccccGInn",
    },
    IbanFields {
        ctry_cd: [71, 76], // "GL"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnGLnn",
    },
    IbanFields {
        ctry_cd: [71, 82], // "GR"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(7),
        iban_struct: "nnnnnnnccccccccccccccccGRnn",
    },
    IbanFields {
        ctry_cd: [71, 84], // "GT"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "ccccccccccccccccccccccccGTnn",
    },
    IbanFields {
        ctry_cd: [72, 78], // "HN"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaannnnnnnnnnnnnnnnnnnnHNnn",
    },
    IbanFields {
        ctry_cd: [72, 82], // "HR"
        iban_len: 21,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(7),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnHRnn",
    },
    IbanFields {
        ctry_cd: [72, 85], // "HU"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(7),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnnnnHUnn",
    },
    IbanFields {
        ctry_cd: [73, 69], // "IE"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct: "aaaannnnnnnnnnnnnnIEnn",
    },
    IbanFields {
        ctry_cd: [73, 76], // "IL"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(6),
        iban_struct: "nnnnnnnnnnnnnnnnnnnILnn",
    },
    IbanFields {
        ctry_cd: [73, 81], // "IQ"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(7),
        iban_struct: "aaaannnnnnnnnnnnnnnIQnn",
    },
    IbanFields {
        ctry_cd: [73, 83], // "IS"
        iban_len: 26,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: Some(3),
        branch_id_pos_e: Some(4),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnnISnn",
    },
    IbanFields {
        ctry_cd: [73, 84], // "IT"
        iban_len: 27,
        bank_id_pos_s: Some(2),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(11),
        iban_struct: "annnnnnnnnnccccccccccccITnn",
    },
    IbanFields {
        ctry_cd: [74, 79], // "JO"
        iban_len: 30,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "aaaannnnccccccccccccccccccJOnn",
    },
    IbanFields {
        ctry_cd: [75, 87], // "KW"
        iban_len: 30,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaaccccccccccccccccccccccKWnn",
    },
    IbanFields {
        ctry_cd: [75, 90], // "KZ"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnncccccccccccccKZnn",
    },
    IbanFields {
        ctry_cd: [76, 66], // "LB"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnccccccccccccccccccccLBnn",
    },
    IbanFields {
        ctry_cd: [76, 67], // "LC"
        iban_len: 32,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaaccccccccccccccccccccccccLCnn",
    },
    IbanFields {
        ctry_cd: [76, 73], // "LI"
        iban_len: 21,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnccccccccccccLInn",
    },
    IbanFields {
        ctry_cd: [76, 84], // "LT"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnLTnn",
    },
    IbanFields {
        ctry_cd: [76, 85], // "LU"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnncccccccccccccLUnn",
    },
    IbanFields {
        ctry_cd: [76, 86], // "LV"
        iban_len: 21,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaacccccccccccccLVnn",
    },
    IbanFields {
        ctry_cd: [76, 89], // "LY"
        iban_len: 25,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(6),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnLYnn",
    },
    IbanFields {
        ctry_cd: [77, 67], // "MC"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct: "nnnnnnnnnncccccccccccnnMCnn",
    },
    IbanFields {
        ctry_cd: [77, 68], // "MD"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "ccccccccccccccccccccMDnn",
    },
    IbanFields {
        ctry_cd: [77, 69], // "ME"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnnMEnn",
    },
    IbanFields {
        ctry_cd: [77, 75], // "MK"
        iban_len: 19,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnccccccccccnnMKnn",
    },
    IbanFields {
        ctry_cd: [77, 78], // "MN"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnMNnn",
    },
    IbanFields {
        ctry_cd: [77, 82], // "MR"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnnnMRnn",
    },
    IbanFields {
        ctry_cd: [77, 84], // "MT"
        iban_len: 31,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(9),
        iban_struct: "aaaannnnnccccccccccccccccccMTnn",
    },
    IbanFields {
        ctry_cd: [77, 85], // "MU"
        iban_len: 30,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(8),
        iban_struct: "aaaannnnnnnnnnnnnnnnnnnaaaMUnn",
    },
    IbanFields {
        ctry_cd: [78, 73], // "NI"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaannnnnnnnnnnnnnnnnnnnNInn",
    },
    IbanFields {
        ctry_cd: [78, 76], // "NL"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaannnnnnnnnnNLnn",
    },
    IbanFields {
        ctry_cd: [78, 79], // "NO"
        iban_len: 15,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnNOnn",
    },
    IbanFields {
        ctry_cd: [79, 77], // "OM"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnccccccccccccccccOMnn",
    },
    IbanFields {
        ctry_cd: [80, 75], // "PK"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaaccccccccccccccccPKnn",
    },
    IbanFields {
        ctry_cd: [80, 76], // "PL"
        iban_len: 28,
        bank_id_pos_s: None,
        bank_id_pos_e: None,
        branch_id_pos_s: Some(1),
        branch_id_pos_e: Some(8),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnnnnPLnn",
    },
    IbanFields {
        ctry_cd: [80, 83], // "PS"
        iban_len: 29,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaacccccccccccccccccccccPSnn",
    },
    IbanFields {
        ctry_cd: [80, 84], // "PT"
        iban_len: 25,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnPTnn",
    },
    IbanFields {
        ctry_cd: [81, 65], // "QA"
        iban_len: 29,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaacccccccccccccccccccccQAnn",
    },
    IbanFields {
        ctry_cd: [82, 79], // "RO"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaaccccccccccccccccROnn",
    },
    IbanFields {
        ctry_cd: [82, 83], // "RS"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnnRSnn",
    },
    IbanFields {
        ctry_cd: [82, 85], // "RU"
        iban_len: 33,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(9),
        branch_id_pos_s: Some(10),
        branch_id_pos_e: Some(14),
        iban_struct: "nnnnnnnnnnnnnncccccccccccccccRUnn",
    },
    IbanFields {
        ctry_cd: [83, 65], // "SA"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnccccccccccccccccccSAnn",
    },
    IbanFields {
        ctry_cd: [83, 67], // "SC"
        iban_len: 31,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(8),
        iban_struct: "aaaannnnnnnnnnnnnnnnnnnnaaaSCnn",
    },
    IbanFields {
        ctry_cd: [83, 68], // "SD"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnSDnn",
    },
    IbanFields {
        ctry_cd: [83, 69], // "SE"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnnnnSEnn",
    },
    IbanFields {
        ctry_cd: [83, 73], // "SI"
        iban_len: 19,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnSInn",
    },
    IbanFields {
        ctry_cd: [83, 75], // "SK"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnnnnSKnn",
    },
    IbanFields {
        ctry_cd: [83, 77], // "SM"
        iban_len: 27,
        bank_id_pos_s: Some(2),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(11),
        iban_struct: "annnnnnnnnnccccccccccccSMnn",
    },
    IbanFields {
        ctry_cd: [83, 79], // "SO"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(7),
        iban_struct: "nnnnnnnnnnnnnnnnnnnSOnn",
    },
    IbanFields {
        ctry_cd: [83, 84], // "ST"
        iban_len: 25,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnnSTnn",
    },
    IbanFields {
        ctry_cd: [83, 86], // "SV"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaannnnnnnnnnnnnnnnnnnnSVnn",
    },
    IbanFields {
        ctry_cd: [84, 76], // "TL"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnnnTLnn",
    },
    IbanFields {
        ctry_cd: [84, 78], // "TN"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: Some(3),
        branch_id_pos_e: Some(5),
        iban_struct: "nnnnnnnnnnnnnnnnnnnnTNnn",
    },
    IbanFields {
        ctry_cd: [84, 82], // "TR"
        iban_len: 26,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnccccccccccccccccTRnn",
    },
    IbanFields {
        ctry_cd: [85, 65], // "UA"
        iban_len: 29,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnncccccccccccccccccccUAnn",
    },
    IbanFields {
        ctry_cd: [86, 65], // "VA"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "nnnnnnnnnnnnnnnnnnVAnn",
    },
    IbanFields {
        ctry_cd: [86, 71], // "VG"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "aaaannnnnnnnnnnnnnnnVGnn",
    },
    IbanFields {
        ctry_cd: [88, 75], // "XK"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: Some(3),
        branch_id_pos_e: Some(4),
        iban_struct: "nnnnnnnnnnnnnnnnXKnn",
    },
    IbanFields {
        ctry_cd: [89, 69], // "YE"
        iban_len: 30,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "aaaannnnccccccccccccccccccYEnn",
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
