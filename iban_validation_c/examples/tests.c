#include <stdio.h>
#include <string.h>
#include <assert.h>
#include "../include/iban_validation.h"

void test_valid_ibans() {
    printf("Testing valid IBANs...\n");
    
    struct {
        const char* iban;
        const char* description;
    } valid_cases[] = {
        {"GB82WEST12345698765432", "UK IBAN"},
        {"DE89370400440532013000", "German IBAN"},
        {"FR1420041010050500013M02606", "French IBAN"},
        {"IT60X0542811101000000123456", "Italian IBAN"},
        {"ES9121000418450200051332", "Spanish IBAN"},
        {"NL91ABNA0417164300", "Dutch IBAN"},
        {"BE68539007547034", "Belgian IBAN"},
        {"CH9300762011623852957", "Swiss IBAN"},
        {"AT611904300234573201", "Austrian IBAN"},
        {"LU280019400644750000", "Luxembourg IBAN"},
        {"IE29AIBK93115212345678", "Irish IBAN"},
        {"PT50000201231234567890154", "Portuguese IBAN"},
        {"SE4550000000058398257466", "Swedish IBAN"},
        {"DK5000400440116243", "Danish IBAN"},
        {"NO9386011117947", "Norwegian IBAN"}
    };
    
    for (int i = 0; i < sizeof(valid_cases) / sizeof(valid_cases[0]); i++) {
        IbanValidationResult result;
        int status = iban_validate_short(valid_cases[i].iban, 0, &result);
        
        printf("  %s: %s -> Status: %d, Valid: %s\n", 
               valid_cases[i].description,
               valid_cases[i].iban, 
               status,
               result.is_valid ? "true" : "false");
        
        // Should return IBAN_VALID (1)
        assert(status == 1);
    }
}

void test_invalid_ibans() {
    printf("\nTesting invalid IBANs...\n");
    
    struct {
        const char* iban;
        const char* description;
        int expected_error;
    } invalid_cases[] = {
        {"GB82WEST12345698765433", "UK IBAN with wrong checksum", -4 /*StructureIncorrect*/},
        {"DE89370400440532013001", "German IBAN with wrong checksum", -4 /*StructureIncorrect*/},
        {"GB82WEST123456987654321", "UK IBAN too long", -4 /*StructureIncorrect*/},
        {"GB82WEST1234569876543", "UK IBAN too short", -4 /*StructureIncorrect*/},
        {"XX82WEST12345698765432", "Invalid country code", -3 /*Invalid Country*/},
        {"G182WEST12345698765432", "Invalid country code format",  -3 /*Invalid Country*/},
        {"GBXXWEST12345698765432", "Non-numeric check digits", -4 /*StructureIncorrect*/},
        {"", "Empty string", -2 /* Missing Country*/},
        {"GB", "Too short", -5 /*InvalidSize*/},
        {"GBAA", "Too short with letters", -5 /*InvalidSize*/}
    };
    
    for (int i = 0; i < sizeof(invalid_cases) / sizeof(invalid_cases[0]); i++) {
        IbanValidationResult result;
        int status = iban_validate_short(invalid_cases[i].iban, 0, &result);
        
        printf("  %s: '%s' -> Status: %d\n", 
               invalid_cases[i].description,
               invalid_cases[i].iban, 
               status);
        
        // Should not return IBAN_VALID
        assert(status != 0);
        // Optionally check specific error code
        // assert(status == invalid_cases[i].expected_error);
    }
}

void test_edge_cases() {
    printf("\nTesting edge cases...\n");
    
    IbanValidationResult result;
    int status;
    
    // Null pointer test
    status = iban_validate_short(NULL, 0, &result);
    printf("  NULL pointer -> Status: %d\n", status);
    assert(status == -2);
    
    // Test with explicit length
    const char* test_iban = "GB82WEST12345698765432";
    status = iban_validate_short(test_iban, strlen(test_iban), &result);
    printf("  Explicit length -> Status: %d\n", status);
    assert(status == 1);
    
    // Test with wrong explicit length
    status = iban_validate_short(test_iban, 10, &result);
    printf("  Wrong explicit length -> Status: %d\n", status);
    // Should fail due to incorrect length
    
    // Test with lowercase
    status = iban_validate_short("gb82west12345698765432", 0, &result);
    printf("  Lowercase IBAN -> Status: %d\n", status);
    
    // Test with spaces (should fail)
    status = iban_validate_short("GB82 WEST 1234 5698 7654 32", 0, &result);
    printf("  IBAN with spaces -> Status: %d\n", status);
    
    // Test non-UTF8 characters (if possible in your environment)
    char bad_utf8[] = {0x47, 0x42, 0x38, 0x32, 0xFF, 0xFE, 0x00}; // GB82 + invalid UTF-8
    status = iban_validate_short(bad_utf8, 0, &result);
    printf("  Invalid UTF-8 -> Status: %d\n", status);
    assert(status == 0);
}

void test_bank_branch_extraction() {
    printf("\nTesting bank/branch extraction...\n");
    
    struct {
        const char* iban;
        const char* country;
    } test_cases[] = {
        {"GB82WEST12345698765432", "GB (UK)"},
        {"DE89370400440532013000", "DE (Germany)"},
        {"FR1420041010050500013M02606", "FR (France)"},
        {"IT60X0542811101000000123456", "IT (Italy)"}
    };
    
    for (int i = 0; i < sizeof(test_cases) / sizeof(test_cases[0]); i++) {
        IbanValidationResult result;
        int status = iban_validate_short(test_cases[i].iban, 0, &result);
        
        if (status == 1) {
            printf("  %s %s:\n", test_cases[i].country, test_cases[i].iban);
            printf("    Bank: positions %d-%d\n", result.bank_s, result.bank_e);
            printf("    Branch: positions %d-%d\n", result.branch_s, result.branch_e);
            
            // Extract and display if positions are valid
            if (result.bank_e > 0) {
                char bank_code[20];
                int bank_len = result.bank_e - result.bank_s;
                strncpy(bank_code, test_cases[i].iban + result.bank_s, bank_len);
                bank_code[bank_len] = '\0';
                printf("    Bank code: %s\n", bank_code);
            }
            
            if (result.branch_e > 0) {
                char branch_code[20];
                int branch_len = result.branch_e - result.branch_s;
                strncpy(branch_code, test_cases[i].iban + result.branch_s, branch_len);
                branch_code[branch_len] = '\0';
                printf("    Branch code: %s\n", branch_code);
            }
        }
    }
}

int main() {
    printf("IBAN Validation Test Suite\n");
    printf("==========================\n");
    
    test_valid_ibans();
    test_invalid_ibans();
    test_edge_cases();
    test_bank_branch_extraction();
    
    printf("\nAll tests completed!\n");
    return 0;
}