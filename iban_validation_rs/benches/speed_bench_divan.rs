use divan::{AllocProfiler, Bencher, black_box};

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}

#[divan::bench(sample_count = 1000)]
fn simple_bench(bencher: Bencher) {
    bencher.bench(|| iban_validation_rs::validate_iban_str(black_box("DE44500105175407324931")));
}

#[divan::bench(sample_count = 1000)]
fn simple_bench_print(bencher: Bencher) {
    bencher.bench(|| {
        iban_validation_rs::validate_iban_str_print(black_box("DE44 5001 0517 5407 3249 31"))
    });
}

#[divan::bench(sample_count = 1000)]
fn simple_bench_num(bencher: Bencher) {
    bencher.bench(|| {
        iban_validation_rs::validate_iban_get_numeric(black_box("DE44500105175407324931"))
    });
}

#[divan::bench(sample_count = 1000)]
fn struct_bench() {
    let _ = iban_validation_rs::Iban::new(black_box("DE44500105175407324931")).unwrap();
}
