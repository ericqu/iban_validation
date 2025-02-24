extern crate iban_validation_rs;
use iban_validation_rs::{validate_iban_str, Iban};

/// This function attempts to create an IBAN from the input string and prints the IBAN, bank ID, and branch ID if successful — or an error message if the creation fails.
fn print_iban_or_error(s: &str){
    match Iban::new(s) {
        Ok(iban) => {
            println!("IBAN: {}", iban.get_iban());
            match iban.iban_bank_id {
                Some(bank_id) => println!("Bank ID: {}", bank_id),
                None => println!("Bank ID: Not available"),
            }
            match iban.iban_branch_id {
                Some(branch_id) => println!("Branch ID: {}", branch_id),
                None => println!("Branch ID: Not available"),
            }
        }
        Err(e) => println!("Failed to create IBAN due to {:?} for input: {:?}", e, s),
    }
}

fn main() {
    println!("okay? {:?}", validate_iban_str("DE44500105175407324931"));
    print_iban_or_error("DE44500105175407324931");
    print_iban_or_error("FR1234");
}