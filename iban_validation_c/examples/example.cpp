#include <iostream>
#include <string>
#include "../include/iban_validation.h"

int main() {
    const std::string valid_iban = "CY17002001280000001200527600";
    const std::string invalid_iban = "FR1234";
    
    // Test IBAN validation
    int result = iban_validate(valid_iban.c_str());
    std::cout << "Valid IBAN: " << valid_iban << " - Result: " << result 
              << " (" << iban_error_message(result) << ")" << std::endl;
    
    result = iban_validate(invalid_iban.c_str());
    std::cout << "Invalid IBAN: " << invalid_iban << " - Result: " << result 
              << " (" << iban_error_message(result) << ")" << std::endl;
    
    // Test IBAN structure creation
    IbanData* iban_data = iban_new(valid_iban.c_str());
    if (iban_data != nullptr) {
        std::cout << "\nIBAN Details:" << std::endl;
        std::cout << "  IBAN: " << iban_data->iban << std::endl;
        
        if (iban_data->bank_id != nullptr) {
            std::cout << "  Bank ID: " << iban_data->bank_id << std::endl;
        } else {
            std::cout << "  Bank ID: Not available" << std::endl;
        }
        
        if (iban_data->branch_id != nullptr) {
            std::cout << "  Branch ID: " << iban_data->branch_id << std::endl;
        } else {
            std::cout << "  Branch ID: Not available" << std::endl;
        }
        
        // Clean up
        iban_free(iban_data);
    } else {
        std::cout << "Failed to create IBAN structure" << std::endl;
    }
    
    return 0;
}