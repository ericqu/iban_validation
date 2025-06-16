// Auto-generated from iban_validation_preprocess/pre_process_registry.py, do not edit manually
use crate::IbanFields;

pub const _IBAN_MIN_LEN: u8 = 15 ;
pub const _IBAN_MAX_LEN: u8 = 33 ;

pub const IBAN_DEFINITIONS: [IbanFields; 99] = [
    IbanFields {
        ctry_cd: [65, 68], // "AD"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "ADnnnnnnnnnncccccccccccc",
    },
    IbanFields {
        ctry_cd: [65, 69], // "AE"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "AEnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [65, 76], // "AL"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(7),
        iban_struct: "ALnnnnnnnnnncccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [65, 84], // "AT"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "ATnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [65, 90], // "AZ"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "AZnnaaaacccccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [66, 65], // "BA"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(6),
        iban_struct: "BAnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [66, 69], // "BE"
        iban_len: 16,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "BEnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [66, 71], // "BG"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "BGnnaaaannnnnncccccccc",
    },
    IbanFields {
        ctry_cd: [66, 72], // "BH"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "BHnnaaaacccccccccccccc",
    },
    IbanFields {
        ctry_cd: [66, 73], // "BI"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct: "BInnnnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [66, 82], // "BR"
        iban_len: 29,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(8),
        branch_id_pos_s: Some(9),
        branch_id_pos_e: Some(13),
        iban_struct: "BRnnnnnnnnnnnnnnnnnnnnnnnnnac",
    },
    IbanFields {
        ctry_cd: [66, 89], // "BY"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "BYnnccccnnnncccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [67, 72], // "CH"
        iban_len: 21,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "CHnnnnnnncccccccccccc",
    },
    IbanFields {
        ctry_cd: [67, 82], // "CR"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "CRnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [67, 89], // "CY"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(8),
        iban_struct: "CYnnnnnnnnnncccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [67, 90], // "CZ"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "CZnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [68, 69], // "DE"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(8),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "DEnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [68, 74], // "DJ"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct: "DJnnnnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [68, 75], // "DK"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "DKnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [68, 79], // "DO"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "DOnnccccnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [69, 69], // "EE"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "EEnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [69, 71], // "EG"
        iban_len: 29,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "EGnnnnnnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [69, 83], // "ES"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "ESnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [70, 73], // "FI"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "FInnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [70, 75], // "FK"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "FKnnaannnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [70, 79], // "FO"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "FOnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [70, 82], // "FR"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "FRnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [71, 80], // "GP"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "GPnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [77, 81], // "MQ"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "MQnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [71, 70], // "GF"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "GFnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [82, 69], // "RE"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "REnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [89, 84], // "YT"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "YTnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [78, 67], // "NC"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "NCnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [80, 70], // "PF"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "PFnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [80, 77], // "PM"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "PMnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [84, 70], // "TF"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "TFnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [87, 70], // "WF"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "WFnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [71, 66], // "GB"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct: "GBnnaaaannnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [71, 69], // "GE"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "GEnnaannnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [71, 73], // "GI"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "GInnaaaaccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [71, 76], // "GL"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "GLnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [71, 82], // "GR"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(7),
        iban_struct: "GRnnnnnnnnncccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [71, 84], // "GT"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "GTnncccccccccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [72, 78], // "HN"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "HNnnaaaannnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [72, 82], // "HR"
        iban_len: 21,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(7),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "HRnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [72, 85], // "HU"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(7),
        iban_struct: "HUnnnnnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [73, 69], // "IE"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(10),
        iban_struct: "IEnnaaaannnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [73, 76], // "IL"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(6),
        iban_struct: "ILnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [73, 81], // "IQ"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(7),
        iban_struct: "IQnnaaaannnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [73, 83], // "IS"
        iban_len: 26,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: Some(3),
        branch_id_pos_e: Some(4),
        iban_struct: "ISnnnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [73, 84], // "IT"
        iban_len: 27,
        bank_id_pos_s: Some(2),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(11),
        iban_struct: "ITnnannnnnnnnnncccccccccccc",
    },
    IbanFields {
        ctry_cd: [74, 79], // "JO"
        iban_len: 30,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "JOnnaaaannnncccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [75, 87], // "KW"
        iban_len: 30,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "KWnnaaaacccccccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [75, 90], // "KZ"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "KZnnnnnccccccccccccc",
    },
    IbanFields {
        ctry_cd: [76, 66], // "LB"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "LBnnnnnncccccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [76, 67], // "LC"
        iban_len: 32,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "LCnnaaaacccccccccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [76, 73], // "LI"
        iban_len: 21,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "LInnnnnnncccccccccccc",
    },
    IbanFields {
        ctry_cd: [76, 84], // "LT"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "LTnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [76, 85], // "LU"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "LUnnnnnccccccccccccc",
    },
    IbanFields {
        ctry_cd: [76, 86], // "LV"
        iban_len: 21,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "LVnnaaaaccccccccccccc",
    },
    IbanFields {
        ctry_cd: [76, 89], // "LY"
        iban_len: 25,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: Some(4),
        branch_id_pos_e: Some(6),
        iban_struct: "LYnnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [77, 67], // "MC"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct: "MCnnnnnnnnnnnncccccccccccnn",
    },
    IbanFields {
        ctry_cd: [77, 68], // "MD"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "MDnncccccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [77, 69], // "ME"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "MEnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [77, 75], // "MK"
        iban_len: 19,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "MKnnnnnccccccccccnn",
    },
    IbanFields {
        ctry_cd: [77, 78], // "MN"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "MNnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [77, 82], // "MR"
        iban_len: 27,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: Some(6),
        branch_id_pos_e: Some(10),
        iban_struct: "MRnnnnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [77, 84], // "MT"
        iban_len: 31,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(9),
        iban_struct: "MTnnaaaannnnncccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [77, 85], // "MU"
        iban_len: 30,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(8),
        iban_struct: "MUnnaaaannnnnnnnnnnnnnnnnnnaaa",
    },
    IbanFields {
        ctry_cd: [78, 73], // "NI"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "NInnaaaannnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [78, 76], // "NL"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "NLnnaaaannnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [78, 79], // "NO"
        iban_len: 15,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "NOnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [79, 77], // "OM"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "OMnnnnncccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [80, 75], // "PK"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "PKnnaaaacccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [80, 76], // "PL"
        iban_len: 28,
        bank_id_pos_s: None,
        bank_id_pos_e: None,
        branch_id_pos_s: Some(1),
        branch_id_pos_e: Some(8),
        iban_struct: "PLnnnnnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [80, 83], // "PS"
        iban_len: 29,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "PSnnaaaaccccccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [80, 84], // "PT"
        iban_len: 25,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "PTnnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [81, 65], // "QA"
        iban_len: 29,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "QAnnaaaaccccccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [82, 79], // "RO"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "ROnnaaaacccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [82, 83], // "RS"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "RSnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [82, 85], // "RU"
        iban_len: 33,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(9),
        branch_id_pos_s: Some(10),
        branch_id_pos_e: Some(14),
        iban_struct: "RUnnnnnnnnnnnnnnnnccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [83, 65], // "SA"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "SAnnnncccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [83, 67], // "SC"
        iban_len: 31,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(8),
        iban_struct: "SCnnaaaannnnnnnnnnnnnnnnnnnnaaa",
    },
    IbanFields {
        ctry_cd: [83, 68], // "SD"
        iban_len: 18,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "SDnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [83, 69], // "SE"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "SEnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [83, 73], // "SI"
        iban_len: 19,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "SInnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [83, 75], // "SK"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "SKnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [83, 77], // "SM"
        iban_len: 27,
        bank_id_pos_s: Some(2),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: Some(7),
        branch_id_pos_e: Some(11),
        iban_struct: "SMnnannnnnnnnnncccccccccccc",
    },
    IbanFields {
        ctry_cd: [83, 79], // "SO"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(7),
        iban_struct: "SOnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [83, 84], // "ST"
        iban_len: 25,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "STnnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [83, 86], // "SV"
        iban_len: 28,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "SVnnaaaannnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [84, 76], // "TL"
        iban_len: 23,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "TLnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [84, 78], // "TN"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: Some(3),
        branch_id_pos_e: Some(5),
        iban_struct: "TNnnnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [84, 82], // "TR"
        iban_len: 26,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(5),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "TRnnnnnnnncccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [85, 65], // "UA"
        iban_len: 29,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(6),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "UAnnnnnnnnccccccccccccccccccc",
    },
    IbanFields {
        ctry_cd: [86, 65], // "VA"
        iban_len: 22,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(3),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "VAnnnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [86, 71], // "VG"
        iban_len: 24,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: None,
        branch_id_pos_e: None,
        iban_struct: "VGnnaaaannnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [88, 75], // "XK"
        iban_len: 20,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(2),
        branch_id_pos_s: Some(3),
        branch_id_pos_e: Some(4),
        iban_struct: "XKnnnnnnnnnnnnnnnnnn",
    },
    IbanFields {
        ctry_cd: [89, 69], // "YE"
        iban_len: 30,
        bank_id_pos_s: Some(1),
        bank_id_pos_e: Some(4),
        branch_id_pos_s: Some(5),
        branch_id_pos_e: Some(8),
        iban_struct: "YEnnaaaannnncccccccccccccccccc",
    },
];

pub fn get_iban_fields(cc: [u8; 2]) -> Option<&'static IbanFields> {
    match cc {
      [65, 68] => Some(&IBAN_DEFINITIONS[0]), // AD
      [65, 69] => Some(&IBAN_DEFINITIONS[1]), // AE
      [65, 76] => Some(&IBAN_DEFINITIONS[2]), // AL
      [65, 84] => Some(&IBAN_DEFINITIONS[3]), // AT
      [65, 90] => Some(&IBAN_DEFINITIONS[4]), // AZ
      [66, 65] => Some(&IBAN_DEFINITIONS[5]), // BA
      [66, 69] => Some(&IBAN_DEFINITIONS[6]), // BE
      [66, 71] => Some(&IBAN_DEFINITIONS[7]), // BG
      [66, 72] => Some(&IBAN_DEFINITIONS[8]), // BH
      [66, 73] => Some(&IBAN_DEFINITIONS[9]), // BI
      [66, 82] => Some(&IBAN_DEFINITIONS[10]), // BR
      [66, 89] => Some(&IBAN_DEFINITIONS[11]), // BY
      [67, 72] => Some(&IBAN_DEFINITIONS[12]), // CH
      [67, 82] => Some(&IBAN_DEFINITIONS[13]), // CR
      [67, 89] => Some(&IBAN_DEFINITIONS[14]), // CY
      [67, 90] => Some(&IBAN_DEFINITIONS[15]), // CZ
      [68, 69] => Some(&IBAN_DEFINITIONS[16]), // DE
      [68, 74] => Some(&IBAN_DEFINITIONS[17]), // DJ
      [68, 75] => Some(&IBAN_DEFINITIONS[18]), // DK
      [68, 79] => Some(&IBAN_DEFINITIONS[19]), // DO
      [69, 69] => Some(&IBAN_DEFINITIONS[20]), // EE
      [69, 71] => Some(&IBAN_DEFINITIONS[21]), // EG
      [69, 83] => Some(&IBAN_DEFINITIONS[22]), // ES
      [70, 73] => Some(&IBAN_DEFINITIONS[23]), // FI
      [70, 75] => Some(&IBAN_DEFINITIONS[24]), // FK
      [70, 79] => Some(&IBAN_DEFINITIONS[25]), // FO
      [70, 82] => Some(&IBAN_DEFINITIONS[26]), // FR
      [71, 80] => Some(&IBAN_DEFINITIONS[27]), // GP
      [77, 81] => Some(&IBAN_DEFINITIONS[28]), // MQ
      [71, 70] => Some(&IBAN_DEFINITIONS[29]), // GF
      [82, 69] => Some(&IBAN_DEFINITIONS[30]), // RE
      [89, 84] => Some(&IBAN_DEFINITIONS[31]), // YT
      [78, 67] => Some(&IBAN_DEFINITIONS[32]), // NC
      [80, 70] => Some(&IBAN_DEFINITIONS[33]), // PF
      [80, 77] => Some(&IBAN_DEFINITIONS[34]), // PM
      [84, 70] => Some(&IBAN_DEFINITIONS[35]), // TF
      [87, 70] => Some(&IBAN_DEFINITIONS[36]), // WF
      [71, 66] => Some(&IBAN_DEFINITIONS[37]), // GB
      [71, 69] => Some(&IBAN_DEFINITIONS[38]), // GE
      [71, 73] => Some(&IBAN_DEFINITIONS[39]), // GI
      [71, 76] => Some(&IBAN_DEFINITIONS[40]), // GL
      [71, 82] => Some(&IBAN_DEFINITIONS[41]), // GR
      [71, 84] => Some(&IBAN_DEFINITIONS[42]), // GT
      [72, 78] => Some(&IBAN_DEFINITIONS[43]), // HN
      [72, 82] => Some(&IBAN_DEFINITIONS[44]), // HR
      [72, 85] => Some(&IBAN_DEFINITIONS[45]), // HU
      [73, 69] => Some(&IBAN_DEFINITIONS[46]), // IE
      [73, 76] => Some(&IBAN_DEFINITIONS[47]), // IL
      [73, 81] => Some(&IBAN_DEFINITIONS[48]), // IQ
      [73, 83] => Some(&IBAN_DEFINITIONS[49]), // IS
      [73, 84] => Some(&IBAN_DEFINITIONS[50]), // IT
      [74, 79] => Some(&IBAN_DEFINITIONS[51]), // JO
      [75, 87] => Some(&IBAN_DEFINITIONS[52]), // KW
      [75, 90] => Some(&IBAN_DEFINITIONS[53]), // KZ
      [76, 66] => Some(&IBAN_DEFINITIONS[54]), // LB
      [76, 67] => Some(&IBAN_DEFINITIONS[55]), // LC
      [76, 73] => Some(&IBAN_DEFINITIONS[56]), // LI
      [76, 84] => Some(&IBAN_DEFINITIONS[57]), // LT
      [76, 85] => Some(&IBAN_DEFINITIONS[58]), // LU
      [76, 86] => Some(&IBAN_DEFINITIONS[59]), // LV
      [76, 89] => Some(&IBAN_DEFINITIONS[60]), // LY
      [77, 67] => Some(&IBAN_DEFINITIONS[61]), // MC
      [77, 68] => Some(&IBAN_DEFINITIONS[62]), // MD
      [77, 69] => Some(&IBAN_DEFINITIONS[63]), // ME
      [77, 75] => Some(&IBAN_DEFINITIONS[64]), // MK
      [77, 78] => Some(&IBAN_DEFINITIONS[65]), // MN
      [77, 82] => Some(&IBAN_DEFINITIONS[66]), // MR
      [77, 84] => Some(&IBAN_DEFINITIONS[67]), // MT
      [77, 85] => Some(&IBAN_DEFINITIONS[68]), // MU
      [78, 73] => Some(&IBAN_DEFINITIONS[69]), // NI
      [78, 76] => Some(&IBAN_DEFINITIONS[70]), // NL
      [78, 79] => Some(&IBAN_DEFINITIONS[71]), // NO
      [79, 77] => Some(&IBAN_DEFINITIONS[72]), // OM
      [80, 75] => Some(&IBAN_DEFINITIONS[73]), // PK
      [80, 76] => Some(&IBAN_DEFINITIONS[74]), // PL
      [80, 83] => Some(&IBAN_DEFINITIONS[75]), // PS
      [80, 84] => Some(&IBAN_DEFINITIONS[76]), // PT
      [81, 65] => Some(&IBAN_DEFINITIONS[77]), // QA
      [82, 79] => Some(&IBAN_DEFINITIONS[78]), // RO
      [82, 83] => Some(&IBAN_DEFINITIONS[79]), // RS
      [82, 85] => Some(&IBAN_DEFINITIONS[80]), // RU
      [83, 65] => Some(&IBAN_DEFINITIONS[81]), // SA
      [83, 67] => Some(&IBAN_DEFINITIONS[82]), // SC
      [83, 68] => Some(&IBAN_DEFINITIONS[83]), // SD
      [83, 69] => Some(&IBAN_DEFINITIONS[84]), // SE
      [83, 73] => Some(&IBAN_DEFINITIONS[85]), // SI
      [83, 75] => Some(&IBAN_DEFINITIONS[86]), // SK
      [83, 77] => Some(&IBAN_DEFINITIONS[87]), // SM
      [83, 79] => Some(&IBAN_DEFINITIONS[88]), // SO
      [83, 84] => Some(&IBAN_DEFINITIONS[89]), // ST
      [83, 86] => Some(&IBAN_DEFINITIONS[90]), // SV
      [84, 76] => Some(&IBAN_DEFINITIONS[91]), // TL
      [84, 78] => Some(&IBAN_DEFINITIONS[92]), // TN
      [84, 82] => Some(&IBAN_DEFINITIONS[93]), // TR
      [85, 65] => Some(&IBAN_DEFINITIONS[94]), // UA
      [86, 65] => Some(&IBAN_DEFINITIONS[95]), // VA
      [86, 71] => Some(&IBAN_DEFINITIONS[96]), // VG
      [88, 75] => Some(&IBAN_DEFINITIONS[97]), // XK
      [89, 69] => Some(&IBAN_DEFINITIONS[98]), // YE
     _ => None,
    }
}
