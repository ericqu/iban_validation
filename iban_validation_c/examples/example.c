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
    printf("Version %s\n", iban_version());

    return 0;
}