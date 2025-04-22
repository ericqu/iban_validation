/* IBAN Validation C API
 * for version number Cargo.toml file
  */

 #ifndef IBAN_VALIDATION_H
 #define IBAN_VALIDATION_H
 
 #ifdef __cplusplus
 extern "C" {
 #endif
 
 #include <stdint.h>
 #include <stddef.h>
 
 /**
  * Error codes for IBAN validation
  */
 enum IbanErrorCode {
     Valid = 1,             /* IBAN is valid */
     Invalid = 0,           /* IBAN is invalid for unspecified reason */
     TooShort = -1,         /* IBAN is too short */
     MissingCountry = -2,   /* IBAN is missing country code */
     InvalidCountry = -3,   /* IBAN has invalid country code */
     StructureIncorrect = -4, /* IBAN structure is incorrect for the country */
     InvalidSize = -5,      /* IBAN length is invalid for the country */
     ModuloFailed = -6,     /* IBAN checksum (mod-97) is incorrect */
 };
 
 /**
  * Structure to hold IBAN data
  */
 typedef struct {
     char* iban;       /* The full IBAN string */
     char* bank_id;    /* Bank identifier, NULL if not available */
     char* branch_id;  /* Branch identifier, NULL if not available */
 } IbanData;
 
 /**
  * Validates an IBAN string
  * 
  * @param iban_str A null-terminated string containing the IBAN to validate
  * @return Status code (see IbanErrorCode enum values)
  */
 int iban_validate(const char* iban_str);
 
 /**
  * Creates a new IBAN structure from a string
  * 
  * @param iban_str A null-terminated string containing the IBAN
  * @return A pointer to an IbanData structure if valid, NULL otherwise
  * 
  * Note: The caller is responsible for freeing the memory by calling iban_free
  */
 IbanData* iban_new(const char* iban_str);
 
 /**
  * Frees the memory allocated for an IbanData structure
  * 
  * @param iban_data Pointer to the IbanData structure to free
  */
 void iban_free(IbanData* iban_data);
 
 /**
  * Gets error message for a specific error code
  * 
  * @param error_code The error code returned by iban_validate
  * @return A null-terminated string with the error message (do not free this string)
  */
 const char* iban_error_message(int error_code);
 
 #ifdef __cplusplus
 }
 #endif
 
 #endif /* IBAN_VALIDATION_H */