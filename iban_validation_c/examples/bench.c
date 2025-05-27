#include <stdio.h>
#include <string.h>
#include <time.h>
#include "../include/iban_validation.h"

void benchmark_standard_approach(const char* iban, int iterations) {
    printf("Benchmarking standard approach:\n");
    
    // Warmup
    iban_validate(iban);
    
    // Benchmark
    clock_t start = clock();
    for (int i = 0; i < iterations; i++) {
        iban_validate(iban);
    }
    clock_t end = clock();
    
    double time_elapsed = (double)(end - start) / CLOCKS_PER_SEC;
    printf("Standard approach: %f seconds for %d iterations\n", 
           time_elapsed, iterations);
}

void benchmark_short_approach(const char* iban, int iterations) {
    printf("Benchmarking Short approach:\n");
    
    // Pre-calculate length once
    size_t len = strlen(iban);
    // prepare structure with results
    IbanValidationResult result;
    // Warmup
    iban_validate_short(iban, len, &result);
    
    // Benchmark
    clock_t start = clock();
    for (int i = 0; i < iterations; i++) {
        iban_validate_short(iban, len, &result);
    }
    clock_t end = clock();
    
    double time_elapsed = (double)(end - start) / CLOCKS_PER_SEC;
    printf("Short approach: %f seconds for %d iterations\n", 
           time_elapsed, iterations);
}

void benchmark_optimized_approach(const char* iban, int iterations) {
    printf("Benchmarking zero-copy approach:\n");
    
    // Pre-calculate length once
    size_t len = strlen(iban);
    
    // Warmup
    iban_validate_optimized(iban, len);
    
    // Benchmark
    clock_t start = clock();
    for (int i = 0; i < iterations; i++) {
        iban_validate_optimized(iban, len);
    }
    clock_t end = clock();
    
    double time_elapsed = (double)(end - start) / CLOCKS_PER_SEC;
    printf("Zero-copy approach: %f seconds for %d iterations\n", 
           time_elapsed, iterations);
}

void benchmark_view_approach(const char* iban, int iterations) {
    printf("Benchmarking view approach:\n");
    
    IbanDataView view;
    
    // Warmup
    iban_get_view(iban, &view);
    
    // Benchmark
    clock_t start = clock();
    for (int i = 0; i < iterations; i++) {
        iban_get_view(iban, &view);
    }
    clock_t end = clock();
    
    double time_elapsed = (double)(end - start) / CLOCKS_PER_SEC;
    printf("View approach: %f seconds for %d iterations\n", 
           time_elapsed, iterations);
}

void benchmark_compared_to_allocation(const char* iban, int iterations) {
    printf("Benchmarking allocation approach:\n");
    
    // Warmup
    IbanData* data = iban_new(iban);
    iban_free(data);
    
    // Benchmark
    clock_t start = clock();
    for (int i = 0; i < iterations; i++) {
        IbanData* data = iban_new(iban);
        iban_free(data);
    }
    clock_t end = clock();
    
    double time_elapsed = (double)(end - start) / CLOCKS_PER_SEC;
    printf("Allocation approach: %f seconds for %d iterations\n", 
           time_elapsed, iterations);
    
    // Compare with view approach
    IbanDataView view;
    start = clock();
    for (int i = 0; i < iterations; i++) {
        iban_get_view(iban, &view);
    }
    end = clock();
    
    double view_time = (double)(end - start) / CLOCKS_PER_SEC;
    printf("View approach: %f seconds for %d iterations\n", 
           view_time, iterations);
    
    double speedup = time_elapsed / view_time;
    printf("Zero-copy speedup: %.2fx faster\n", speedup);


    // Compare with short approach
    IbanValidationResult result;
    size_t len = strlen(iban);
    start = clock();
    for (int i = 0; i < iterations; i++) {
        iban_validate_short(iban, len, &result);
    }
    end = clock();
    
    double short_time = (double)(end - start) / CLOCKS_PER_SEC;
    printf("Short approach: %f seconds for %d iterations\n", 
           short_time, iterations);
    
    speedup = time_elapsed / short_time;
    printf("Short approach speedup: %.2fx faster\n", speedup);    
}

void demo_view_usage() {
    const char* iban = "DE89370400440532013000";
    
    // Using the view-based approach
    IbanDataView view;
    int result = iban_get_view(iban, &view);
    
    if (result == Valid) {
        printf("Valid IBAN: %.*s\n", (int)view.iban.len, view.iban.ptr);
        printf("Bank ID: %.*s\n", (int)view.bank_id.len, view.bank_id.ptr);
        printf("Branch ID: %.*s\n", (int)view.branch_id.len, view.branch_id.ptr);
    } else {
        printf("Invalid IBAN: %s\n", iban_error_message(result));
    }
}

int main() {
    printf("IBAN Validation Library Version: %s\n", iban_version());
    
    const char* iban = "DE89370400440532013000"; // valid IBAN
    
    // Demo the zero-copy approach
    demo_view_usage();
    
    // Run benchmarks with 10,000,000 iterations
    int iterations = 10000000;
    benchmark_standard_approach(iban, iterations);
    benchmark_short_approach(iban, iterations);
    benchmark_optimized_approach(iban, iterations);
    benchmark_view_approach(iban, iterations);
    benchmark_compared_to_allocation(iban, iterations);
    
    return 0;
}