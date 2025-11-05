# iban_validation_c
A package to facilitate validation of IBANs and getting the bank identifier and branch identifier in C/C++ as a wrapper from Rust.
This is experiemental changes to the API are likely to be without further notice.

## Short example
There are two ways to interact with the API:
 - Validate the iban with `iban_validate` which validates an IBAN string and returns a status code.
 - Use `iban_new` to creates a new IBAN structure from a string.
 
 See below code for illustration:

```c
#include <stdio.h>
#include <string.h>
#include "../include/iban_validation.h"

int main() {
    const char* valid_iban = "CY17002001280000001200527600";
    const char* invalid_iban = "FR1234";
    
    // Test IBAN validation
    int result = iban_validate(valid_iban);
    printf("Valid IBAN: %s - Result: %d (%s)\n", 
           valid_iban, result, iban_error_message(result));
    
    result = iban_validate(invalid_iban);
    printf("Invalid IBAN: %s - Result: %d (%s)\n", 
           invalid_iban, result, iban_error_message(result));
    
    // Test IBAN structure creation
    IbanData* iban_data = iban_new(valid_iban);
    if (iban_data != NULL) {
        printf("\nIBAN Details:\n");
        printf("  IBAN: %s\n", iban_data->iban);
        
        if (iban_data->bank_id != NULL) {
            printf("  Bank ID: %s\n", iban_data->bank_id);
        } else {
            printf("  Bank ID: Not available\n");
        }
        
        if (iban_data->branch_id != NULL) {
            printf("  Branch ID: %s\n", iban_data->branch_id);
        } else {
            printf("  Branch ID: Not available\n");
        }
        
        // Clean up
        iban_free(iban_data);
    } else {
        printf("Failed to create IBAN structure\n");
    }

    // preferred approach, keep the allocation on the c side.

    // calculate length once, otherwise pass 0
    size_t len = strlen(valid_iban);
    // prepare structure with results
    IbanValidationResult result_s;
    //validate
    int status = iban_validate_short(valid_iban, len, &result_s);
    if (status == 1) {
        printf("Valid Iban:\t%s\n", valid_iban);
        if (result_s.bank_e > 0) {
            printf("\tBank ID: %.*s\t\tPositions %d-%d\n",(result_s.bank_e - result_s.bank_s), 
                valid_iban + result_s.bank_s , result_s.bank_s, result_s.bank_e);
        }
        if (result_s.branch_e > 0) {
            printf("\tBranch ID: %.*s\t\tPositions %d-%d\n", (result_s.branch_e - result_s.branch_s),
                valid_iban + result_s.branch_s,  result_s.branch_s, result_s.branch_e);
        }
    } else {
        printf("Failed to validated IBAN structure\n");
    }

    // Print version
    printf("Version %s\n", iban_version());

    return 0;
}
```
See Makefile for compilation of the c/c++ examples, and the examples directory for more examples

## Changes
 - 0.1.21: upgraded to polars 0.52.0, rust 1.91, improved internal data structure. Enable modern CPU instruction on x86 (x86-64-v3) and Mac (M1) for python, polars and c packages.
 - 0.1.20: technical update upgraded to polars 0.51.0, rust 1.90
 - 0.1.19: technical update upgraded to polars 0.50.0, rust 1.89
 - 0.1.18: technical update upgraded to polars 0.49.1, pyo3 0.25, rust 1.88
 - 0.1.16: improved performance, added territories for GB and FR, and more tests, added WASM (experimental for now), added fuzzer
 - 0.1.15: improved performance (char to bytes) and improved c wrapper doc
 - 0.1.14: fixed error for country code IQ (using pdf instead of technicql input file)
 - 0.1.13: update to the wrapper for a version with less allocation (view-based)
 - 0.1.12: Initial release of the c wrapper