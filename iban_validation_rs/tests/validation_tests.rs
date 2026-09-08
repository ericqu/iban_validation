use iban_validation_rs::*;

#[test]
fn mini_test() {
    let al_test = "DE44500105175407324931";
    assert!(validate_iban_str(al_test).unwrap_or(false));
}

#[test]
fn forbiden_checksum_test() {
    let al_test = "IQ98NBIQ850123456789012";
    assert!(validate_iban_str(al_test).unwrap_or(false));
    let al_test = "IQ01NBIQ850123456789012";
    assert!(!validate_iban_str(al_test).unwrap_or(false));
}

#[test]
fn al_iban() {
    let al_test = "";
    assert_eq!(
        validate_iban_str(al_test).unwrap_err(),
        ValidationError::MissingCountry
    );
    let al_test = "DE44500105175407324931DE44500105175407324931";
    assert_eq!(
        validate_iban_str(al_test).unwrap_err(),
        ValidationError::InvalidSizeForCountry
    );
    let al_test = "AL47212110090000000235698741";
    assert!(validate_iban_str(al_test).unwrap_or(false));
    let al_test = "A7212110090000000235698741";
    assert_eq!(
        validate_iban_str(al_test).unwrap_err(),
        ValidationError::InvalidCountry
    );
    let al_test = "AL4721211009000000023569874Q";
    assert_eq!(
        validate_iban_str(al_test).unwrap_err(),
        ValidationError::ModuloIncorrect
    );
    let al_test = "NI04BAPR00000013000003558124";
    assert_eq!(
        validate_iban_str(al_test).unwrap_err(),
        ValidationError::ModuloIncorrect
    );
    let al_test = "RU1704452522540817810538091310419";
    assert_eq!(
        validate_iban_str(al_test).unwrap_err(),
        ValidationError::ModuloIncorrect
    );
    let al_test = "ST68000200010192194210112";
    assert_eq!(
        validate_iban_str(al_test).unwrap_err(),
        ValidationError::ModuloIncorrect
    );
    let al_test = "AL47ZZ211009000000023569874Q";
    assert_eq!(
        validate_iban_str(al_test).unwrap_err(),
        ValidationError::StructureIncorrectForCountry
    );
    let al_test = "AL4721211009000000023569874QQ";
    assert_eq!(
        validate_iban_str(al_test).unwrap_err(),
        ValidationError::InvalidSizeForCountry
    );
    let al_test = "AD1200012030200359100100";
    assert!(validate_iban_str(al_test).unwrap_or(false));

    let tc = vec![
        "AD1200012030200359100100",
        "AE070331234567890123456",
        "AL47212110090000000235698741",
        "AT611904300234573201",
        "AZ21NABZ00000000137010001944",
        "BA391290079401028494",
        "BE68539007547034",
        "BG80BNBG96611020345678",
        "BH67BMAG00001299123456",
        "BI4210000100010000332045181",
        "BR1800360305000010009795493C1",
        "BY13NBRB3600900000002Z00AB00",
        "CH9300762011623852957",
        "CR05015202001026284066",
        "CY17002001280000001200527600",
        "CZ6508000000192000145399",
        "DE89370400440532013000",
        "DJ2100010000000154000100186",
        "DK5000400440116243",
        "DO28BAGR00000001212453611324",
        "EE382200221020145685",
        "EG380019000500000000263180002",
        "ES9121000418450200051332",
        "FI2112345600000785",
        "FK88SC123456789012",
        "FO6264600001631634",
        "FR1420041010050500013M02606",
        "GB29NWBK60161331926819",
        "GE29NB0000000101904917",
        "GI75NWBK000000007099453",
        "GL8964710001000206",
        "GR1601101250000000012300695",
        "GT82TRAJ01020000001210029690",
        "HR1210010051863000160",
        "HU42117730161111101800000000",
        "IE29AIBK93115212345678",
        "IL620108000000099999999",
        "IQ98NBIQ850123456789012",
        "IS140159260076545510730339",
        "IT60X0542811101000000123456",
        "JO94CBJO0010000000000131000302",
        "KW81CBKU0000000000001234560101",
        "KZ86125KZT5004100100",
        "LB62099900000001001901229114",
        "LC55HEMM000100010012001200023015",
        "LI21088100002324013AA",
        "LT121000011101001000",
        "LU280019400644750000",
        "LV80BANK0000435195001",
        "LY83002048000020100120361",
        "MC5811222000010123456789030",
        "MD24AG000225100013104168",
        "ME25505000012345678951",
        "MK07250120000058984",
        "MN121234123456789123",
        "MR1300020001010000123456753",
        "MT84MALT011000012345MTLCAST001S",
        "MU17BOMM0101101030300200000MUR",
        "NL91ABNA0417164300",
        "NO9386011117947",
        "OM810180000001299123456",
        "PK36SCBL0000001123456702",
        "PL61109010140000071219812874",
        "PS92PALS000000000400123456702",
        "PT50000201231234567890154",
        "QA58DOHB00001234567890ABCDEFG",
        "RO49AAAA1B31007593840000",
        "RS35260005601001611379",
        "SA0380000000608010167519",
        "SC18SSCB11010000000000001497USD",
        "SD2129010501234001",
        "SE4550000000058398257466",
        "SI56263300012039086",
        "SK3112000000198742637541",
        "SM86U0322509800000000270100",
        "SO211000001001000100141",
        "SV62CENR00000000000000700025",
        "TL380080012345678910157",
        "TN5910006035183598478831",
        "TR330006100519786457841326",
        "UA213223130000026007233566001",
        "VA59001123000012345678",
        "VG96VPVG0000012345678901",
        "XK051212012345678906",
        "YE15CBYE0001018861234567891234",
        "GB82WEST12345698765432",
        "HN88CABF00000000000250005469",
        "HN54PISA00000000000000123124",
    ];

    for al_test in &tc {
        assert!(validate_iban_str(al_test).unwrap_or(false));
    }
}

#[test]
fn lower_case_ibans() {
    let mt_test = "MT84MALT011000012345MTLCAST001S";
    assert!(validate_iban_str(mt_test).unwrap_or(false));

    let mt_test = "MT84MALT011000012345MTLCAST001s";
    assert!(validate_iban_str(mt_test).unwrap_or(false));

    let mt_test = "MT84MALT011000012345mtlCAST001s";
    assert!(validate_iban_str(mt_test).unwrap_or(false));

    let mt_test = "MT84MALT011000012345mtlcast001s";
    assert!(validate_iban_str(mt_test).unwrap_or(false));

    let mt_test = "MT84malt011000012345mtlcast001s";
    assert_eq!(
        validate_iban_str(mt_test).unwrap_err(),
        ValidationError::StructureIncorrectForCountry
    );

    let mt_test = "MT84MALT0110000%2345MTLCAST001S"; // the percent is not a digit or letter
    assert_eq!(
        validate_iban_str(mt_test).unwrap_err(),
        ValidationError::StructureIncorrectForCountry
    );
}

#[test]
fn validate_iban_tostruct() {
    let the_test = Iban::new("AT483200000012345864").unwrap();
    assert_eq!(the_test.get_iban(), "AT483200000012345864");
    assert_eq!(the_test.iban_bank_id.unwrap(), "32000");
    assert_eq!(the_test.iban_branch_id, None);
    let the_test = Iban::new("AT611904300234573201").unwrap();
    assert_eq!(the_test.get_iban(), "AT611904300234573201");
    assert_eq!(the_test.iban_bank_id.unwrap(), "19043");
    assert_eq!(the_test.iban_branch_id, None);
    let the_test = Iban::new("CY17002001280000001200527600").unwrap();
    assert_eq!(the_test.get_iban(), "CY17002001280000001200527600");
    assert_eq!(the_test.iban_bank_id.unwrap(), "002");
    assert_eq!(the_test.iban_branch_id.unwrap(), "00128");
    let the_test = Iban::new("AL47212110090000000235698741").unwrap();
    assert_eq!(the_test.get_iban(), "AL47212110090000000235698741");
    assert_eq!(the_test.iban_bank_id.unwrap(), "212");
    assert_eq!(the_test.iban_branch_id.unwrap(), "11009");
    let the_test = Iban::new("DE89370400440532013000").unwrap();
    assert_eq!(the_test.get_iban(), "DE89370400440532013000");
    assert_eq!(the_test.iban_bank_id.unwrap(), "37040044");
    let the_test = Iban::new("FR1420041010050500013M02606").unwrap();
    assert_eq!(the_test.get_iban(), "FR1420041010050500013M02606");
    assert_eq!(the_test.iban_bank_id.unwrap(), "20041");
    let the_test = Iban::new("GB29NWBK60161331926819").unwrap();
    assert_eq!(the_test.get_iban(), "GB29NWBK60161331926819");
    assert_eq!(the_test.iban_bank_id.unwrap(), "NWBK");
    assert_eq!(the_test.iban_branch_id.unwrap(), "601613");
    let the_test = Iban::new("GE29NB0000000101904917").unwrap();
    assert_eq!(the_test.get_iban(), "GE29NB0000000101904917");
    assert_eq!(the_test.iban_bank_id.unwrap(), "NB");
    assert_eq!(the_test.iban_branch_id, None);
    let the_test = Iban::new("IQ98NBIQ850123456789012").unwrap();
    assert_eq!(the_test.get_iban(), "IQ98NBIQ850123456789012");
    assert_eq!(the_test.iban_bank_id.unwrap(), "NBIQ");
    assert_eq!(the_test.iban_branch_id.unwrap(), "850");
    let the_test = Iban::new("DEFR").unwrap_err();
    assert_eq!(the_test, ValidationError::InvalidSizeForCountry);
    let the_test = Iban::new("D").unwrap_err();
    assert_eq!(the_test, ValidationError::MissingCountry);
    let the_test = Iban::new("").unwrap_err();
    assert_eq!(the_test, ValidationError::MissingCountry);
}

#[test]
fn validate_iban_to_nums() {
    let s = "AT483200000012345864";
    let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
    assert!(res);
    assert_eq!(bank_s, 4);
    assert_eq!(bank_e, 9);
    assert_eq!(branch_s, 0);
    assert_eq!(branch_e, 0); // not available
    assert_eq!("32000", &s[bank_s as usize..bank_e as usize]);

    let s = "AT611904300234573201";
    let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
    assert!(res);
    assert_eq!(bank_s, 4);
    assert_eq!(bank_e, 9);
    assert_eq!(branch_s, 0);
    assert_eq!(branch_e, 0); // not available
    assert_eq!("19043", &s[bank_s as usize..bank_e as usize]);

    let s = "CY17002001280000001200527600";
    let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
    assert!(res);
    assert_eq!(bank_s, 4);
    assert_eq!(bank_e, 7);
    assert_eq!(branch_s, 7);
    assert_eq!(branch_e, 12);
    assert_eq!("002", &s[bank_s as usize..bank_e as usize]);
    assert_eq!("00128", &s[branch_s as usize..branch_e as usize]);

    let s = "DE89370400440532013000";
    let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
    assert!(res);
    assert_eq!(bank_s, 4);
    assert_eq!(bank_e, 12);
    assert_eq!(branch_s, 0);
    assert_eq!(branch_e, 0); // not available
    assert_eq!("37040044", &s[bank_s as usize..bank_e as usize]);

    let s = "FR1420041010050500013M02606";
    let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
    assert!(res);
    assert_eq!(bank_s, 4);
    assert_eq!(bank_e, 9);
    assert_eq!(branch_s, 0);
    assert_eq!(branch_e, 0); // not available
    assert_eq!("20041", &s[bank_s as usize..bank_e as usize]);

    let s = "GB29NWBK60161331926819";
    let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
    assert!(res);
    assert_eq!(bank_s, 4);
    assert_eq!(bank_e, 8);
    assert_eq!(branch_s, 8);
    assert_eq!(branch_e, 14); // not available
    assert_eq!("NWBK", &s[bank_s as usize..bank_e as usize]);
    assert_eq!("601613", &s[branch_s as usize..branch_e as usize]);

    let s = "IQ98NBIQ850123456789012";
    let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
    assert!(res);
    assert_eq!(bank_s, 4);
    assert_eq!(bank_e, 8);
    assert_eq!(branch_s, 8);
    assert_eq!(branch_e, 11); // not available
    assert_eq!("NBIQ", &s[bank_s as usize..bank_e as usize]);
    assert_eq!("850", &s[branch_s as usize..branch_e as usize]);

    let s = "AZ21NABZ00000000137010001944";
    let (res, bank_s, bank_e, _branch_s, _branch_e) = validate_iban_get_numeric(s).unwrap();
    assert!(res);
    assert_eq!("NABZ", &s[bank_s as usize..bank_e as usize]);

    let s = "DEFR";
    let the_error = validate_iban_get_numeric(s).unwrap_err();
    assert_eq!(the_error, ValidationError::InvalidSizeForCountry);

    let s = "D";
    let the_error = validate_iban_get_numeric(s).unwrap_err();
    assert_eq!(the_error, ValidationError::MissingCountry);

    let s = "";
    let the_error = validate_iban_get_numeric(s).unwrap_err();
    assert_eq!(the_error, ValidationError::MissingCountry);
}

#[test]
fn test_filename() {
    assert_eq!(get_source_file(), "iban_registry_v102.txt");
}

#[test]
fn test_version() {
    assert_eq!(get_version(), env!("CARGO_PKG_VERSION"));
}

#[test]
fn test_fmt_display() {
    let error = ValidationError::TooShort(3);
    assert_eq!(
        format!("{}", error),
        "The input Iban is too short to be an IBAN 3 (minimum length is 4)"
    );
    let error = ValidationError::MissingCountry;
    assert_eq!(
        format!("{}", error),
        "The input Iban does not appear to start with 2 letters representing a two-letter country code"
    );
    let error = ValidationError::InvalidCountry;
    assert_eq!(
        format!("{}", error),
        "the input Iban the first two-letter do not match a valid country"
    );
    let error = ValidationError::StructureIncorrectForCountry;
    assert_eq!(
        format!("{}", error),
        "The characters founds in the input Iban do not follow the country's Iban structure"
    );
    let error = ValidationError::InvalidSizeForCountry;
    assert_eq!(
        format!("{}", error),
        "The length of the input Iban does match the length for that country"
    );
}

#[test]
fn test_modulo_incorrect() {
    let error = ValidationError::ModuloIncorrect;
    assert_eq!(
        format!("{}", error),
        "The calculated mod97 for the iban indicates an incorrect Iban"
    );
}

#[test]
fn test_from_fuzz() {
    let spe_from_fuzz = "ILM\u{7bd}\0\0\0\0\0\0\0\0\0M\0\0\0J\0\0\0I";
    assert!(!validate_iban_str(spe_from_fuzz).unwrap_or(false));
}

/// Regression test from fuzz/artifacts/fuzz_target_1/crash-11e2231fa03ef6cc4fd61b3b7894a9bcc6a180dc
// The trailing `\0\00` is copied verbatim from the fuzz crash artifact. Rewriting it as
// `\x0000` to satisfy clippy::octal_escapes would keep the same bytes but lose the
// byte-for-byte correspondence with the artifact, so the literal stays as the fuzzer produced it.
#[allow(clippy::octal_escapes)]
#[test]
fn test_from_fuzz_egem() {
    let spe_from_fuzz = "EGE\u{7bd}0q0j\0\0\0\0\0\0\0\0,00~000000\0\00";
    assert!(validate_iban_str(spe_from_fuzz).is_err());
}

#[test]
fn validate_iban_tostruc_additional() {
    let the_test = Iban::new("BA391290079401028494").unwrap();
    assert_eq!(the_test.get_iban(), "BA391290079401028494");
    assert_eq!(the_test.iban_bank_id.unwrap(), "129");
    assert_eq!(the_test.iban_branch_id.unwrap(), "007");

    let the_test = Iban::new("BE68539007547034").unwrap();
    assert_eq!(the_test.get_iban(), "BE68539007547034");
    assert_eq!(the_test.iban_bank_id.unwrap(), "539");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("BG80BNBG96611020345678").unwrap();
    assert_eq!(the_test.get_iban(), "BG80BNBG96611020345678");
    assert_eq!(the_test.iban_bank_id.unwrap(), "BNBG");
    assert_eq!(the_test.iban_branch_id.unwrap(), "9661");

    let the_test = Iban::new("BH67BMAG00001299123456").unwrap();
    assert_eq!(the_test.get_iban(), "BH67BMAG00001299123456");
    assert_eq!(the_test.iban_bank_id.unwrap(), "BMAG");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("BI4210000100010000332045181").unwrap();
    assert_eq!(the_test.get_iban(), "BI4210000100010000332045181");
    assert_eq!(the_test.iban_bank_id.unwrap(), "10000");
    assert_eq!(the_test.iban_branch_id.unwrap(), "10001");

    let the_test = Iban::new("BR1800360305000010009795493C1").unwrap();
    assert_eq!(the_test.get_iban(), "BR1800360305000010009795493C1");
    assert_eq!(the_test.iban_bank_id.unwrap(), "00360305");
    assert_eq!(the_test.iban_branch_id.unwrap(), "00001");

    let the_test = Iban::new("BY13NBRB3600900000002Z00AB00").unwrap();
    assert_eq!(the_test.get_iban(), "BY13NBRB3600900000002Z00AB00");
    assert_eq!(the_test.iban_bank_id.unwrap(), "NBRB");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("CH9300762011623852957").unwrap();
    assert_eq!(the_test.get_iban(), "CH9300762011623852957");
    assert_eq!(the_test.iban_bank_id.unwrap(), "00762");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("CR05015202001026284066").unwrap();
    assert_eq!(the_test.get_iban(), "CR05015202001026284066");
    assert_eq!(the_test.iban_bank_id.unwrap(), "0152");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("CY17002001280000001200527600").unwrap();
    assert_eq!(the_test.get_iban(), "CY17002001280000001200527600");
    assert_eq!(the_test.iban_bank_id.unwrap(), "002");
    assert_eq!(the_test.iban_branch_id.unwrap(), "00128");

    let the_test = Iban::new("CZ6508000000192000145399").unwrap();
    assert_eq!(the_test.get_iban(), "CZ6508000000192000145399");
    assert_eq!(the_test.iban_bank_id.unwrap(), "0800");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("DJ2100010000000154000100186").unwrap();
    assert_eq!(the_test.get_iban(), "DJ2100010000000154000100186");
    assert_eq!(the_test.iban_bank_id.unwrap(), "00010");
    assert_eq!(the_test.iban_branch_id.unwrap(), "00000");

    let the_test = Iban::new("DK5000400440116243").unwrap();
    assert_eq!(the_test.get_iban(), "DK5000400440116243");
    assert_eq!(the_test.iban_bank_id.unwrap(), "0040");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("DO28BAGR00000001212453611324").unwrap();
    assert_eq!(the_test.get_iban(), "DO28BAGR00000001212453611324");
    assert_eq!(the_test.iban_bank_id.unwrap(), "BAGR");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("EE382200221020145685").unwrap();
    assert_eq!(the_test.get_iban(), "EE382200221020145685");
    assert_eq!(the_test.iban_bank_id.unwrap(), "22");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("EG380019000500000000263180002").unwrap();
    assert_eq!(the_test.get_iban(), "EG380019000500000000263180002");
    assert_eq!(the_test.iban_bank_id.unwrap(), "0019");
    assert_eq!(the_test.iban_branch_id.unwrap(), "0005");

    let the_test = Iban::new("ES9121000418450200051332").unwrap();
    assert_eq!(the_test.get_iban(), "ES9121000418450200051332");
    assert_eq!(the_test.iban_bank_id.unwrap(), "2100");
    assert_eq!(the_test.iban_branch_id.unwrap(), "0418");

    let the_test = Iban::new("FI2112345600000785").unwrap();
    assert_eq!(the_test.get_iban(), "FI2112345600000785");
    assert_eq!(the_test.iban_bank_id.unwrap(), "123");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("FK88SC123456789012").unwrap();
    assert_eq!(the_test.get_iban(), "FK88SC123456789012");
    assert_eq!(the_test.iban_bank_id.unwrap(), "SC");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("FO6264600001631634").unwrap();
    assert_eq!(the_test.get_iban(), "FO6264600001631634");
    assert_eq!(the_test.iban_bank_id.unwrap(), "6460");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("FR1420041010050500013M02606").unwrap();
    assert_eq!(the_test.get_iban(), "FR1420041010050500013M02606");
    assert_eq!(the_test.iban_bank_id.unwrap(), "20041");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("GP1120041010050500013M02606").unwrap();
    assert_eq!(the_test.get_iban(), "GP1120041010050500013M02606");
    assert_eq!(the_test.iban_bank_id.unwrap(), "20041");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("GE29NB0000000101904917").unwrap();
    assert_eq!(the_test.get_iban(), "GE29NB0000000101904917");
    assert_eq!(the_test.iban_bank_id.unwrap(), "NB");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("GI75NWBK000000007099453").unwrap();
    assert_eq!(the_test.get_iban(), "GI75NWBK000000007099453");
    assert_eq!(the_test.iban_bank_id.unwrap(), "NWBK");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("GL8964710001000206").unwrap();
    assert_eq!(the_test.get_iban(), "GL8964710001000206");
    assert_eq!(the_test.iban_bank_id.unwrap(), "6471");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("GR1601101250000000012300695").unwrap();
    assert_eq!(the_test.get_iban(), "GR1601101250000000012300695");
    assert_eq!(the_test.iban_bank_id.unwrap(), "011");
    assert_eq!(the_test.iban_branch_id.unwrap(), "0125");

    let the_test = Iban::new("GT82TRAJ01020000001210029690").unwrap();
    assert_eq!(the_test.get_iban(), "GT82TRAJ01020000001210029690");
    assert_eq!(the_test.iban_bank_id.unwrap(), "TRAJ");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("HN88CABF00000000000250005469").unwrap();
    assert_eq!(the_test.get_iban(), "HN88CABF00000000000250005469");
    assert_eq!(the_test.iban_bank_id.unwrap(), "CABF");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("HR1210010051863000160").unwrap();
    assert_eq!(the_test.get_iban(), "HR1210010051863000160");
    assert_eq!(the_test.iban_bank_id.unwrap(), "1001005");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("HU42117730161111101800000000").unwrap();
    assert_eq!(the_test.get_iban(), "HU42117730161111101800000000");
    assert_eq!(the_test.iban_bank_id.unwrap(), "117");
    assert_eq!(the_test.iban_branch_id.unwrap(), "7301");

    let the_test = Iban::new("IE29AIBK93115212345678").unwrap();
    assert_eq!(the_test.get_iban(), "IE29AIBK93115212345678");
    assert_eq!(the_test.iban_bank_id.unwrap(), "AIBK");
    assert_eq!(the_test.iban_branch_id.unwrap(), "931152");

    let the_test = Iban::new("IL620108000000099999999").unwrap();
    assert_eq!(the_test.get_iban(), "IL620108000000099999999");
    assert_eq!(the_test.iban_bank_id.unwrap(), "010");
    assert_eq!(the_test.iban_branch_id.unwrap(), "800");

    let the_test = Iban::new("IQ98NBIQ850123456789012").unwrap();
    assert_eq!(the_test.get_iban(), "IQ98NBIQ850123456789012");
    assert_eq!(the_test.iban_bank_id.unwrap(), "NBIQ");
    assert_eq!(the_test.iban_branch_id.unwrap(), "850");

    let the_test = Iban::new("IS140159260076545510730339").unwrap();
    assert_eq!(the_test.get_iban(), "IS140159260076545510730339");
    assert_eq!(the_test.iban_bank_id.unwrap(), "01");
    assert_eq!(the_test.iban_branch_id.unwrap(), "59");

    let the_test = Iban::new("IT60X0542811101000000123456").unwrap();
    assert_eq!(the_test.get_iban(), "IT60X0542811101000000123456");
    assert_eq!(the_test.iban_bank_id.unwrap(), "05428");
    assert_eq!(the_test.iban_branch_id.unwrap(), "11101");

    let the_test = Iban::new("JO94CBJO0010000000000131000302").unwrap();
    assert_eq!(the_test.get_iban(), "JO94CBJO0010000000000131000302");
    assert_eq!(the_test.iban_bank_id.unwrap(), "CBJO");
    assert_eq!(the_test.iban_branch_id.unwrap(), "0010"); // PDF incorrect; online and txt version okay

    let the_test = Iban::new("KW81CBKU0000000000001234560101").unwrap();
    assert_eq!(the_test.get_iban(), "KW81CBKU0000000000001234560101");
    assert_eq!(the_test.iban_bank_id.unwrap(), "CBKU");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("KZ86125KZT5004100100").unwrap();
    assert_eq!(the_test.get_iban(), "KZ86125KZT5004100100");
    assert_eq!(the_test.iban_bank_id.unwrap(), "125");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("LB62099900000001001901229114").unwrap();
    assert_eq!(the_test.get_iban(), "LB62099900000001001901229114");
    assert_eq!(the_test.iban_bank_id.unwrap(), "0999");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("LC55HEMM000100010012001200023015").unwrap();
    assert_eq!(the_test.get_iban(), "LC55HEMM000100010012001200023015");
    assert_eq!(the_test.iban_bank_id.unwrap(), "HEMM");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("LI21088100002324013AA").unwrap();
    assert_eq!(the_test.get_iban(), "LI21088100002324013AA");
    assert_eq!(the_test.iban_bank_id.unwrap(), "08810");
    assert_eq!(the_test.iban_branch_id, None);

    let the_test = Iban::new("YE15CBYE0001018861234567891234").unwrap();
    assert_eq!(the_test.get_iban(), "YE15CBYE0001018861234567891234");
    assert_eq!(the_test.iban_bank_id.unwrap(), "CBYE");
    assert_eq!(the_test.iban_branch_id.unwrap(), "0001");
}

#[test]
fn print_format_with_spaces() {
    //                DE44 5001 0517 5407 3249 31
    let iban = "DE44 5001 0517 5407 3249 31";
    assert!(validate_iban_str_print(iban).unwrap());
}

#[test]
fn print_format_irregular_spaces() {
    let iban = "D E 4 4   5001 0517   5407 3249 31 ";
    assert!(validate_iban_str_print(iban).unwrap());
    let iban = "DE44   5001 0517   5407 3249 31 ";
    assert!(validate_iban_str_print(iban).unwrap());
    let iban = "DE 44   5001 0517   5407 3249 31 ";
    assert!(validate_iban_str_print(iban).unwrap());
    let iban = "DE44   5001 0517   5407 3249 31                                                                             a";
    assert_eq!(
        validate_iban_str_print(iban).unwrap_err(),
        ValidationError::InvalidSizeForCountry
    );
}

#[test]
fn strict_rejects_spaces() {
    let iban = "DE44 5001 0517 5407 3249 31";
    assert!(validate_iban_str(iban).is_err());
}

#[test]
fn flexible_rejects_excessively_long_input() {
    let mut s = String::from("DE44");
    s.push_str(&" ".repeat(10_000));
    s.push_str("500105175407324931");

    assert!(
        validate_iban_str_print(&s).is_err(),
        "should fail fast on pathological input"
    );
}

#[test]
fn flexible_rejects_extra_logical_chars() {
    let iban = "DE44500105175407324931123";
    assert!(
        validate_iban_str_print(iban).is_err(),
        "too many logical characters"
    );
}

#[test]
fn print_format_rejects_forbidden_checksum() {
    let iban = "IQ01 NBIQ 8501 2345 6789 012";
    assert_eq!(
        validate_iban_str_print(iban).unwrap_err(),
        ValidationError::InvalidChecksum
    );
}

#[test]
fn every_valid_strict_iban_also_validates_in_print_format() {
    let tc = vec![
        "AD1200012030200359100100",
        "AE070331234567890123456",
        "AL47212110090000000235698741",
        "AT611904300234573201",
        "DE89370400440532013000",
        "FR1420041010050500013M02606",
        "GB29NWBK60161331926819",
        "IQ98NBIQ850123456789012",
        "MT84MALT011000012345MTLCAST001S",
    ];

    for iban in &tc {
        assert!(validate_iban_str(iban).unwrap_or(false));
        assert!(validate_iban_str_print(iban).unwrap_or(false));
    }
}

#[test]
fn round_trip_iban_new_preserves_original_string() {
    let tc = vec![
        "AD1200012030200359100100",
        "AT611904300234573201",
        "CY17002001280000001200527600",
        "DE89370400440532013000",
        "GB29NWBK60161331926819",
        "MT84MALT011000012345MTLCAST001S",
    ];

    for s in &tc {
        let iban = Iban::new(s).unwrap();
        assert_eq!(iban.get_iban(), *s);
    }
}

#[test]
fn lowercase_country_code_is_rejected() {
    let iban = "de44500105175407324931";
    assert_eq!(
        validate_iban_str(iban).unwrap_err(),
        ValidationError::InvalidCountry
    );
}

#[test]
fn print_format_rejects_non_space_whitespace() {
    let iban = "DE44\t500105175407324931";
    assert_eq!(
        validate_iban_str_print(iban).unwrap_err(),
        ValidationError::StructureIncorrectForCountry
    );

    let iban = "DE44\n500105175407324931";
    assert_eq!(
        validate_iban_str_print(iban).unwrap_err(),
        ValidationError::StructureIncorrectForCountry
    );
}

/// Guards against the non-registry country codes leaking into the
/// default, registry-only country set.
#[test]
#[cfg(not(feature = "non_registry"))]
fn non_registry_countries_rejected_without_feature() {
    let tc = vec![
        "AO49012345678901234567890",
        "MA36012345678901234567890123",
        "DZ090123456789012345678901",
    ];

    for iban in &tc {
        assert_eq!(
            validate_iban_str(iban).unwrap_err(),
            ValidationError::InvalidCountry
        );
    }
}

/// Every country example from the IBAN registry's official examples file validates,
/// giving broad, low-maintenance coverage across all supported countries.
#[test]
fn all_registry_examples_validate() {
    const EXAMPLES: &str = include_str!("../data/IBAN Examples.txt");

    let ibans: Vec<&str> = EXAMPLES
        .lines()
        .skip(1) // header line "IBAN Examples"
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    assert_eq!(ibans.len(), 89, "expected fixture to list 89 example IBANs");

    for iban in &ibans {
        assert!(
            validate_iban_str(iban).unwrap_or(false),
            "expected {iban} to be a valid IBAN"
        );
    }
}
