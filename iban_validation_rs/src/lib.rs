//! A short example illustrating a simple library usage
//!
//! ```
//! extern crate iban_validation_rs;
//! use iban_validation_rs::{validate_iban_str, Iban};
//!
//! // This function attempts to create an IBAN from the input string and prints the IBAN, bank ID, and branch ID if successful — or an error message if the creation fails.
//! fn print_iban_or_error(s: &str){
//!     match Iban::new(s) {
//!         Ok(iban) => {
//!             println!("IBAN: {}", iban.get_iban());
//!             match iban.iban_bank_id {
//!                 Some(bank_id) => println!("Bank ID: {}", bank_id),
//!                 None => println!("Bank ID: Not available"),
//!             }
//!             match iban.iban_branch_id {
//!                 Some(branch_id) => println!("Branch ID: {}", branch_id),
//!                 None => println!("Branch ID: Not available"),
//!             }
//!         }
//!         Err(e) => println!("Failed to create IBAN due to {:?} for input: {:?}", e, s),
//!     }
//! }
//!
//! fn main() {
//!     println!("okay? {:?}", validate_iban_str("DE44500105175407324931"));
//!     print_iban_or_error("DE44500105175407324931");
//!     print_iban_or_error("FR1234");
//! }
//! ```

use iban_definition::get_iban_fields;
use std::error::Error;
use std::fmt;

mod iban_definition;

/// indicate which information is expected from the Iban Registry and in the record.
#[derive(Default, Debug, Clone, PartialEq)]
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
    pub iban_struct: &'static str,
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
            ValidationError::TooShort(len) => write!(
                f,
                "The input Iban is too short to be an IBAN {} (minimum length is 4)",
                len
            ),
            ValidationError::MissingCountry => write!(
                f,
                "The input Iban does not appear to start with 2 letters representing a two-letter country code"
            ),
            ValidationError::InvalidCountry => write!(
                f,
                "the input Iban the first two-letter do not match a valid country"
            ),
            ValidationError::StructureIncorrectForCountry => write!(
                f,
                "The characters founds in the input Iban do not follow the country's Iban structure"
            ),
            ValidationError::InvalidSizeForCountry => write!(
                f,
                "The length of the input Iban does match the length for that country"
            ),
            ValidationError::ModuloIncorrect => write!(
                f,
                "The calculated mod97 for the iban indicates an incorrect Iban"
            ),
        }
    }
}
impl Error for ValidationError {}

/// potential error for the per letter validation
#[derive(Debug, PartialEq)]
enum ValidationLetterError {
    NotPartOfRequiredSet,
}

/// internal utility
/// Check the character (byte) is a digit and return the value of that digit.
#[inline]
fn simple_contains_n(c: u8) -> Result<usize, ValidationLetterError> {
    if c.is_ascii_digit() {
        Ok((c - 48) as usize) // 48 is the ascii value of '0'
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

/// internal utility
/// check the character is an uppercase A-Z and return a value between 10-36
#[inline]
fn simple_contains_a(c: u8) -> Result<usize, ValidationLetterError> {
    if c.is_ascii_uppercase() {
        Ok((c - 55) as usize) // 55 is to get a 10 from a 'A'
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

/// internal utility
/// Check the character is alphanumeric an return the value (0-9 for digit,) 10-36 for letters.
#[inline]
fn simple_contains_c(c: u8) -> Result<usize, ValidationLetterError> {
    if c.is_ascii_digit() {
        Ok((c - 48) as usize)
    } else if c.is_ascii_uppercase() {
        Ok((c - 55) as usize)
    } else if c.is_ascii_lowercase() {
        Ok((c - 87) as usize) // 87 is to get a 10 from a 'a'
    } else {
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }
}

/// const storage for the comprehensive modulo operation
const MFF_ARRAY: [[u8; 36]; 97] = generate_mff_array();

/// internal utility
/// build an array of precomputed modulo operations
/// the maximum should be 9635 (96 the largest previous, 35 a Z the largest possible)
const fn generate_mff_array() -> [[u8; 36]; 97] {
    let mut array: [[u8; 36]; 97] = [[0; 36]; 97];
    let mut pseudo_acc = 0u8;

    while pseudo_acc < 97 {
        let mut pseudo_newchar = 0u8;

        while pseudo_newchar < 36 {
            let mut result: u32 = pseudo_acc as u32;
            result *= if pseudo_newchar < 10 { 10 } else { 100 }; // Multiply by 10 (or 100 for two-digit numbers)
            result = (result + pseudo_newchar as u32) % 97; // and add new digit
            array[pseudo_acc as usize][pseudo_newchar as usize] = result as u8;

            pseudo_newchar += 1;
        }

        pseudo_acc += 1;
    }
    array
}

/// Indicates which file was used a source
pub const fn get_source_file() -> &'static str {
    include_str!("../data/iban_sourcefile.txt")
}

/// Indicates the version used. to be used in other modules like the c wrapper where this infomration is not available.
pub const fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Validate than an Iban is valid according to the registry information
/// return true when Iban is fine, together with information about the fields (so that it does not need to be looked up again later)
pub fn validate_iban_with_data(input_iban: &str) -> Result<(&IbanFields, bool), ValidationError> {
    let identified_country: [u8; 2] = match input_iban.get(..2) {
        Some(value) => value
            .as_bytes()
            .try_into()
            .map_err(|_| ValidationError::InvalidCountry)?,
        None => return Err(ValidationError::MissingCountry),
    };

    // let iban_data: &IbanFields = match &IB_REG.get(identified_country) {
    let iban_data: &IbanFields = match get_iban_fields(identified_country) {
        Some(pattern) => pattern,
        None => return Err(ValidationError::InvalidCountry),
    };

    let pattern = &iban_data.iban_struct;

    if pattern.len() != input_iban.len() {
        return Err(ValidationError::InvalidSizeForCountry);
    }

    let pat_re = pattern.bytes();

    // if we have invalid character at the boundary before starting to check them
    if !input_iban.is_char_boundary(4) {
        return Err(ValidationError::StructureIncorrectForCountry);
    }

    let input_re = input_iban[4..].bytes().chain(input_iban[..4].bytes());

    let mut acc: usize = 0;

    for (p, t) in pat_re.zip(input_re) {
        let m97digit = match p {
            b'n' => match simple_contains_n(t) {
                Ok(value) => value,
                _ => return Err(ValidationError::StructureIncorrectForCountry),
            },
            b'a' => match simple_contains_a(t) {
                Ok(value) => value,
                _ => return Err(ValidationError::StructureIncorrectForCountry),
            },
            b'c' => match simple_contains_c(t) {
                Ok(value) => value,
                _ => return Err(ValidationError::StructureIncorrectForCountry),
            },
            _ => {
                // the 2-letter country code should match, although it is unlikely to not match (still needed to compute the m97)
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
        acc = MFF_ARRAY[acc][m97digit] as usize;
    }
    if acc == 1 {
        Ok((iban_data, true))
    } else {
        Err(ValidationError::ModuloIncorrect)
    }
}

/// Validate than an Iban is valid according to the registry information
/// return true when Iban is fine, otherwise returns Error.
pub fn validate_iban_str(input_iban: &str) -> Result<bool, ValidationError> {
    validate_iban_with_data(input_iban).map(|(_, is_valid)| is_valid)
}

/// Validate than an Iban is valid according to the registry information
/// Give the results by numerical values (0,0 when the optional part is missing).
/// This is meant to be used in c wrapper when copying value is expensive.
pub fn validate_iban_get_numeric(
    input_iban: &str,
) -> Result<(bool, u8, u8, u8, u8), ValidationError> {
    let (iban_data, result) = validate_iban_with_data(input_iban)?;

    let (bank_s, bank_e) = match (iban_data.bank_id_pos_s, iban_data.bank_id_pos_e) {
        (Some(start), Some(end)) => (start + 3, end + 4),
        _ => (0, 0),
    };

    let (branch_s, branch_e) = match (iban_data.branch_id_pos_s, iban_data.branch_id_pos_e) {
        (Some(start), Some(end)) => (start + 3, end + 4),
        _ => (0, 0),
    };

    Ok((
        result,
        bank_s as u8,
        bank_e as u8,
        branch_s as u8,
        branch_e as u8,
    ))
}

/// Indicate how a valid Iban is stored.
/// A owned String for the iban, so that if the String we tested is out of scope we have our own copy. TODO is it an issue?
/// If valid for the country the slice of the Iban representing the bank_id bank identifier.
/// If valid for the country the slice of the Iban representing the branch_id Branch identifier.
#[derive(Debug)]
pub struct Iban<'a> {
    // /// owned String not accessible to ensure read-only through reader
    // stored_iban: String,
    stored_iban: &'a str,
    /// Bank identifier when relevant
    pub iban_bank_id: Option<&'a str>,
    /// Branch identifier when relevant
    pub iban_branch_id: Option<&'a str>,
}

/// building a valid Iban (validate and take the relavant slices).
impl<'a> Iban<'a> {
    pub fn new(s: &'a str) -> Result<Self, ValidationError> {
        let (iban_data, _) = validate_iban_with_data(s)?;

        let bank_id = Self::extract_identifier(s, iban_data.bank_id_pos_s, iban_data.bank_id_pos_e);
        let branch_id =
            Self::extract_identifier(s, iban_data.branch_id_pos_s, iban_data.branch_id_pos_e);

        Ok(Self {
            stored_iban: s,
            iban_bank_id: bank_id,
            iban_branch_id: branch_id,
        })
    }

    /// get read-only access to the Iban
    pub fn get_iban(&self) -> &str {
        self.stored_iban
    }

    /// helper function to fill the bank_id and branch_id
    #[inline]
    fn extract_identifier(
        s: &'a str,
        start_pos: Option<usize>,
        end_pos: Option<usize>,
    ) -> Option<&'a str> {
        match (start_pos, end_pos) {
            (Some(start), Some(end)) if start <= end && (4 + end) <= s.len() => {
                Some(&s[start + 3..end + 4])
            }
            _ => None,
        }
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
            "HN54PISA00000000000000123124",
        ];

        for al_test in &tc {
            assert_eq!(validate_iban_str(al_test).unwrap_or(false), true);
        }
    }

    #[test]
    fn lower_case_ibans() {
        let mt_test = "MT84MALT011000012345MTLCAST001S";
        assert_eq!(validate_iban_str(mt_test).unwrap_or(false), true);

        let mt_test = "MT84MALT011000012345MTLCAST001s";
        assert_eq!(validate_iban_str(mt_test).unwrap_or(false), true);

        let mt_test = "MT84MALT011000012345mtlCAST001s";
        assert_eq!(validate_iban_str(mt_test).unwrap_or(false), true);

        let mt_test = "MT84MALT011000012345mtlcast001s";
        assert_eq!(validate_iban_str(mt_test).unwrap_or(false), true);

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
    fn check_map() {
        // match IB_REG.get([b'F', b'R']) {
        match get_iban_fields([b'F', b'R']) {
            Some(ib_data) => {
                println!("FR : {}", ib_data.iban_struct);
                // assert_eq!(ib_data.iban_struct, "FRnnnnnnnnnnnncccccccccccnn");
                assert_eq!(ib_data.iban_struct, "nnnnnnnnnncccccccccccnnFRnn");
            }
            _ => println!("FR IBan missing!"),
        }

        let al_ib_struct = get_iban_fields([b'A', b'L'])
            .expect("country does not existin in registry")
            .iban_struct;
        // assert_eq!("ALnnnnnnnnnncccccccccccccccc", al_ib_struct);
        assert_eq!("nnnnnnnnccccccccccccccccALnn", al_ib_struct);

        // println!("Successfully loaded {} countries", IB_REG.len());
        println!("Successfully loaded countries");
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
        assert_eq!(the_test.iban_branch_id.unwrap(), "1100");
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
        assert_eq!(true, res);
        assert_eq!(bank_s, 4);
        assert_eq!(bank_e, 9);
        assert_eq!(branch_s, 0);
        assert_eq!(branch_e, 0); // not available
        assert_eq!("32000", &s[bank_s as usize..bank_e as usize]);

        let s = "AT611904300234573201";
        let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
        assert_eq!(true, res);
        assert_eq!(bank_s, 4);
        assert_eq!(bank_e, 9);
        assert_eq!(branch_s, 0);
        assert_eq!(branch_e, 0); // not available
        assert_eq!("19043", &s[bank_s as usize..bank_e as usize]);

        let s = "CY17002001280000001200527600";
        let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
        assert_eq!(true, res);
        assert_eq!(bank_s, 4);
        assert_eq!(bank_e, 7);
        assert_eq!(branch_s, 7);
        assert_eq!(branch_e, 12);
        assert_eq!("002", &s[bank_s as usize..bank_e as usize]);
        assert_eq!("00128", &s[branch_s as usize..branch_e as usize]);

        let s = "DE89370400440532013000";
        let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
        assert_eq!(true, res);
        assert_eq!(bank_s, 4);
        assert_eq!(bank_e, 12);
        assert_eq!(branch_s, 0);
        assert_eq!(branch_e, 0); // not available
        assert_eq!("37040044", &s[bank_s as usize..bank_e as usize]);

        let s = "FR1420041010050500013M02606";
        let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
        assert_eq!(true, res);
        assert_eq!(bank_s, 4);
        assert_eq!(bank_e, 9);
        assert_eq!(branch_s, 0);
        assert_eq!(branch_e, 0); // not available
        assert_eq!("20041", &s[bank_s as usize..bank_e as usize]);

        let s = "GB29NWBK60161331926819";
        let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
        assert_eq!(true, res);
        assert_eq!(bank_s, 4);
        assert_eq!(bank_e, 8);
        assert_eq!(branch_s, 8);
        assert_eq!(branch_e, 14); // not available
        assert_eq!("NWBK", &s[bank_s as usize..bank_e as usize]);
        assert_eq!("601613", &s[branch_s as usize..branch_e as usize]);

        let s = "IQ98NBIQ850123456789012";
        let (res, bank_s, bank_e, branch_s, branch_e) = validate_iban_get_numeric(s).unwrap();
        assert_eq!(true, res);
        assert_eq!(bank_s, 4);
        assert_eq!(bank_e, 8);
        assert_eq!(branch_s, 8);
        assert_eq!(branch_e, 11); // not available
        assert_eq!("NBIQ", &s[bank_s as usize..bank_e as usize]);
        assert_eq!("850", &s[branch_s as usize..branch_e as usize]);

        let s = "AZ21NABZ00000000137010001944";
        let (res, bank_s, bank_e, _branch_s, _branch_e) = validate_iban_get_numeric(s).unwrap();
        assert_eq!(true, res);
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

    // #[test]
    // fn test_mod97_equivalence() {
    //     // Test range of values to ensure equivalence
    //     for x in 0..9_700 {
    //         assert_eq!(div_arr_mod97(x), x % 97, "Failed for value {}", x);
    //     }
    // }

    #[test]
    fn test_filename() {
        assert_eq!(get_source_file(), "iban_registry_v99.txt");
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
        assert_eq!(validate_iban_str(spe_from_fuzz).unwrap_or(false), false);
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
}
