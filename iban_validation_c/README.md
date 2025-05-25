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
    
    return 0;
}
```
See Makefile for compilation of the c/c++ examples, and the examples directory for more examples

## Changes
 - 0.1.14: fixed error for country code IQ (using pdf instead of technicql input file)
 - 0.1.13: update to the wrapper for a version with less allocation (view-based)
 - 0.1.12: Initial release of the c wrapper