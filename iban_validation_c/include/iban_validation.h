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
 #include <stdbool.h>
 
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
  * Zero-copy string view structure
  */
 typedef struct {
     const char* ptr;       /* Pointer to string data */
     size_t len;            /* Length of string (excluding null terminator) */
 } StringView;
 
 /**
  * Zero-copy IBAN data view structure
  */
 typedef struct {
     StringView iban;       /* The full IBAN string view */
     StringView bank_id;    /* Bank identifier view */
     StringView branch_id;  /* Branch identifier view */
 } IbanDataView;
 
 /**
  * hold short return value for the iban validation avoiding copy of chars
  */
 typedef struct {
     bool is_valid;       /* is it a valid iban */
     uint8_t bank_s;      /* bank id starting point */
     uint8_t bank_e;      /* bank id end point, when zero it is not available */ 
     uint8_t branch_s;    /* branch id starting point */
     uint8_t branch_e;    /* branch id end point, when zero is not available */
 } IbanValidationResult;

 /**
  * Structure to hold IBAN data (with allocations)
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
  * Optimized IBAN validation using short zero-copy approach
  * 
  * @param iban_str A null-terminated string containing the IBAN to validate
  * @param len Length of the string (if known), pass 0 to auto-detect length
  * @param result the results needed to build the branch_id and bank_id (when available)
  * @return Status code (see IbanErrorCode enum values)
  */
 int iban_validate_short(const char* iban_str, size_t len, IbanValidationResult* result);

 /**
  * Optimized IBAN validation using zero-copy approach
  * 
  * @param iban_str A null-terminated string containing the IBAN to validate
  * @param len Length of the string (if known), pass 0 to auto-detect length
  * @return Status code (see IbanErrorCode enum values)
  */
 int iban_validate_optimized(const char* iban_str, size_t len);
 
 /**
  * Gets IBAN information without copying strings
  * Note: The returned data is only valid while iban_str is valid
  * 
  * @param iban_str A null-terminated string containing the IBAN
  * @param out_data Pointer to an IbanDataView structure to fill
  * @return 1 if valid, 0 or negative error code otherwise
  */
 int iban_get_view(const char* iban_str, IbanDataView* out_data);
 
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
 
 /**
  * Gets library version
  * 
  * @return A null-terminated string with the version number of the library (do not free this string)
  */
 const char* iban_version();

 #ifdef __cplusplus
 }
 #endif
 
 #endif /* IBAN_VALIDATION_H */