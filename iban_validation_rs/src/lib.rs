use rustc_hash::FxHashMap;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::sync::LazyLock;

/// indicate which information is expected from the Iban Registry and in the record.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IbanFields {
    /// two-letter country codes as per ISO 3166-1
    pub ctry_cd: [u8; 2],
    /// IBAN length, intentionnaly short, the length is sufficient but if something changes it will raise error quickly
    pub iban_len: u8,
    /// position of bank identifier starting point
    pub bank_id_pos_s: Option<usize>,
    /// position of bank identifier end point
    pub bank_id_pos_e: Option<usize>,
    /// position of branch identifier starting point
    pub branch_id_pos_s: Option<usize>,
    /// position of branch identifier end point
    pub branch_id_pos_e: Option<usize>,
    /// contains the structure the IBan for a specific country should be (generated from the python code)
    pub iban_struct: String,
}

/// indicate what types of error the iban validation can detect
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    /// the test Iban is too short for the country
    TooShort(usize),
    /// There is no country in the IBAN
    MissingCountry,
    /// There is no valid country in the IBAN
    InvalidCountry,
    /// Does not follow the structure for the country
    StructureIncorrectForCountry,
    /// The size of the IBAN is not what it should be for the country
    InvalidSizeForCountry,
    /// the modulo mod97 computation for the IBAN is invalid.
    ModuloIncorrect,
}
impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::TooShort(len) => write!(f, "The input Iban is too short to be an IBAN {} (minimum length is 4)", len),
            ValidationError::MissingCountry => write!(f, "The input Iban does not appear to start with 2 letters representing a two-letter country code"),
            ValidationError::InvalidCountry => write!(f,"the input Iban the first two-letter do not mathc a valid country"),
            ValidationError::StructureIncorrectForCountry => write!(f, "The characters founds in teh input Iban do not follow the country's Iban structure"),
            ValidationError::InvalidSizeForCountry => write!(f, "The length of the input Iban does match the length for that country"),
            ValidationError::ModuloIncorrect => write!(f, "The calculated mod97 for the iban indicates an incorrect Iban"),
        }
    }
}
impl Error for ValidationError {}

/// utility function to load the registry (as json) into a Hashmap
fn convert_to_hashmap(json_str: &str) -> Result<FxHashMap<[u8; 2], IbanFields>, serde_json::Error> {
    let items: Vec<IbanFields> = serde_json::from_str(json_str)?;

    let map: FxHashMap<[u8; 2], IbanFields> =
        items.into_iter().map(|item| (item.ctry_cd, item)).collect();

    Ok(map)
}

/// trigger the loading of the registry once need, and only once.
/// panics if failing as there is no other way forward.
static IB_REG: LazyLock<FxHashMap<[u8; 2], IbanFields>> = LazyLock::new(|| {
    convert_to_hashmap(include_str!("../data/iban_definitions.json"))
        .expect("Failed parsing JSON data into a HashMap")
});

const ALLOWED_E: &str = " ";

/// potential error for the per letter validation
#[derive(Debug, PartialEq)]
enum ValidationLetterError {
    NotPartOfRequiredSet,
}

/// internal utility
/// Check the character is a digit and return the value of that digit.
#[inline]
fn simple_contains_n(c: char) -> Result<u8, ValidationLetterError> {
    if c.is_ascii_digit() {
        Ok((c as u8) - 48) // 48 is the ascii value of '0'
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

/// internal utility
/// check the character is an uppercase A-Z and return a value between 10-36
#[inline]
fn simple_contains_a(c: char) -> Result<u8, ValidationLetterError> {
    if c.is_ascii_uppercase() {
        Ok((c as u8) - 55) // 55 is to get a 10 from a 'A'
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

/// internal utility
/// Check the character is alphanumeric an return the value (0-9 for digit,) 10-36 for letters.
#[inline]
fn simple_contains_c(c: char) -> Result<u8, ValidationLetterError> {
    if c.is_ascii_digit() {
        Ok((c as u8) - 48)
    } else if c.is_ascii_uppercase() {
        Ok((c as u8) - 55)
    } else if c.is_ascii_lowercase() {
        Ok((c as u8) - 87) // 87 is to get a 10 from a 'a'
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

/// internal utility
/// division method for modulo 97 >> faster than regular modulo
#[inline]
fn division_mod97(x: u32) -> u32 {
    let q = x / 97; // Quotient
    x - q * 97 // Remainder
}

/// indicate which file was used a source
pub const fn get_source_file() -> &'static str {
    include_str!("../data/iban_sourcefile.txt")
}

/// Validate than an Iban is valid according to the registry information
/// return true when Iban is fine, otherwise returns Error.
pub fn validate_iban_str(input_iban: &str) -> Result<bool, ValidationError> {
    let identified_country: [u8; 2] = match input_iban.get(..2) {
        Some(value) => value
            .as_bytes()
            .try_into()
            .map_err(|_| ValidationError::InvalidCountry)?,
        None => return Err(ValidationError::MissingCountry),
    };
    let pattern: &String = match &IB_REG.get(&identified_country) {
        Some(pattern) => &pattern.iban_struct,
        None => return Err(ValidationError::InvalidCountry),
    };

    if pattern.len() != input_iban.len() {
        return Err(ValidationError::InvalidSizeForCountry);
    }

    // There is a potental panic but it should be a dead code, as we should never find a non 2-letter country code given we search for 2-leter country code and found something before
    let pattern_start: [u8; 2] = pattern
        .get(..2)
        .unwrap()
        .as_bytes()
        .try_into()
        .map_err(|_| ValidationError::InvalidCountry)
        .expect("Error the built-in pattern is not starting with at least two characters");

    // first two letters do not match
    // technically unnecessary but performance is impacted negativey when not present
    match (pattern_start, identified_country) {
        (p_start, t_start) if p_start != t_start => return Err(ValidationError::InvalidCountry),
        _ => {}
    }

    let pat_re = pattern[4..].chars().chain(pattern[..4].chars());
    let input_re = input_iban[4..].chars().chain(input_iban[..4].chars());

    let mut acc: u32 = 0;

    for (p, t) in pat_re.zip(input_re) {
        let m97digit = match p {
            'n' => match simple_contains_n(t) {
                Ok(value) => value,
                _ => return Err(ValidationError::StructureIncorrectForCountry),
            },
            'a' => match simple_contains_a(t) {
                Ok(value) => value,
                _ => return Err(ValidationError::StructureIncorrectForCountry),
            },
            'c' => match simple_contains_c(t) {
                Ok(value) => value,
                _ => return Err(ValidationError::StructureIncorrectForCountry),
            },
            'e' => match ALLOWED_E.contains(t) {
                true => 0,
                _ => return Err(ValidationError::StructureIncorrectForCountry),
            },
            _ => {
                // the 2-letter country code should match
                if p == t {
                    match simple_contains_a(t) {
                        Ok(value) => value,
                        _ => return Err(ValidationError::StructureIncorrectForCountry),
                    }
                } else {
                    return Err(ValidationError::StructureIncorrectForCountry);
                }
            }
        };
        acc *= if m97digit < 10 { 10 } else { 100 }; // Multiply by 10 (or 100 for two-digit numbers)
        acc = division_mod97(acc + (m97digit as u32)); // and add new digit
    }
    if acc == 1 {
        Ok(true)
    } else {
        Err(ValidationError::ModuloIncorrect)
    }
}

/// indicate how a valid Iban is stored.
/// A owned String for the iban, so that if the String we tested is out of scope we have our own copy. TODO is it an issue?
/// If valid for the country the slice of the Iban representing the bank_id bank identifier.
/// If valid for the country the slice of the Iban representing the branch_id Branch identifier.
pub struct Iban<'a> {
    /// owned String not accessible to ensure read-only through reader
    stored_iban: String,
    /// Bank identifier when relevant
    pub iban_bank_id: Option<&'a str>,
    /// Branch identifier when relevant
    pub iban_branch_id: Option<&'a str>,
}

/// building a valid Iban (validate and take the relavant slices).
impl<'a> Iban<'a> {
    pub fn new(s: &'a str) -> Result<Self, ValidationError> {
        let _is_valid: bool = validate_iban_str(s)?;

        let identified_country: [u8; 2] = match s.get(..2) {
            Some(value) => value
                .as_bytes()
                .try_into()
                .map_err(|_| ValidationError::InvalidCountry)?,
            None => return Err(ValidationError::MissingCountry),
        };

        let iban_data: &IbanFields = match &IB_REG.get(&identified_country) {
            Some(pattern) => pattern,
            None => return Err(ValidationError::InvalidCountry),
        };

        let bank_id = if let Some(start) = iban_data.bank_id_pos_s {
            if let Some(end) = iban_data.bank_id_pos_e {
                if start <= end && (4 + start + (end - start)) <= s.len() {
                    Some(&s[start + 3..4 + start + (end - start)])
                } else {
                    None // Indices are invalid
                }
            } else {
                None
            }
        } else {
            None
        };

        let branch_id = if let Some(start) = iban_data.branch_id_pos_s {
            if let Some(end) = iban_data.branch_id_pos_e {
                if start <= end && (4 + start + (end - start)) <= s.len() {
                    Some(&s[start + 3..4 + start + (end - start)])
                } else {
                    None // Indices are invalid
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(Self {
            stored_iban: s.to_string(),
            iban_bank_id: bank_id,
            iban_branch_id: branch_id,
        })
    }

    /// get read-only access to the Iban
    pub fn get_iban(&self) -> &str {
        &self.stored_iban
    }
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn mini_test() {
        let al_test = "DE44500105175407324931";
        assert_eq!(validate_iban_str(al_test).unwrap_or(false), true);
    }

    #[test]
    fn al_iban() {
        let al_test = "";
        assert_eq!(validate_iban_str(al_test).unwrap_err(), 
            ValidationError::MissingCountry);
            let al_test = "DE44500105175407324931DE44500105175407324931";
        assert_eq!(validate_iban_str(al_test).unwrap_err(), 
            ValidationError::InvalidSizeForCountry);
        let al_test = "AL47212110090000000235698741";
        assert_eq!(validate_iban_str(al_test).unwrap_or(false), true);
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
        assert_eq!(validate_iban_str(al_test).unwrap_or(false), true);

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
        ];

        for al_test in &tc {
            assert_eq!(validate_iban_str(al_test).unwrap_or(false), true);
        }
    }

    #[test]
    fn check_map() {
        match IB_REG.get(&[b'F', b'R']) {
            Some(ib_data) => {
                println!("FR : {}", ib_data.iban_struct);
                assert_eq!(ib_data.iban_struct, "FRnnnnnnnnnnnncccccccccccnn");
            }
            _ => println!("FR IBan missing!"),
        }

        let al_ib_struct = &IB_REG
            .get(&[b'A', b'L'])
            .expect("country does not existin in registry")
            .iban_struct;
        assert_eq!("ALnnnnnnnnnncccccccccccccccc", al_ib_struct);

        println!("Successfully loaded {} countries", IB_REG.len());
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
    }

    #[test]
    fn charac_tests() {
        for c in ALLOWED_E.chars() {
            println!("{:?}", c as u8);
        }
    }

    #[test]
    fn test_mod97_equivalence() {
        // Test range of values to ensure equivalence
        for x in 0..10_000 {
            assert_eq!(division_mod97(x), x % 97, "Failed for value {}", x);
        }
    }

    #[test]
    fn test_filename() {
        assert_eq!(get_source_file(), "iban_registry_v99.txt");
    }
}
