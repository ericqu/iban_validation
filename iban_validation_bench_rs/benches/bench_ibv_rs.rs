use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use std::str::FromStr; // for iban_validate
pub fn iban_validate_sd(c: &mut Criterion) {
    c.bench_function("iban_validate_sd", |b| {
        b.iter(|| iban_validate::Iban::from_str(black_box("DE44500105175407324931")))
    });
}

// for Iban
pub fn iban_short_sd(c: &mut Criterion) {
    c.bench_function("iban_short_sd", |b| {
        b.iter(|| iban_short::Iban::from_str(black_box("DE44500105175407324931")))
    });
}

// for iban_parser
pub fn iban_parser_sd(c: &mut Criterion) {
    c.bench_function("iban_parser_sd", |b| {
        b.iter(|| iban_parser::parse_iban(black_box("DE44500105175407324931")))
    });
}

// for schwifty
pub fn schwifty_sd(c: &mut Criterion) {
    c.bench_function("schwifty_sd", |b| {
        b.iter(|| schwifty::validate(black_box("DE44500105175407324931")))
    });
}

pub fn iban_validation_rs_sd(c: &mut Criterion) {
    c.bench_function("iban_validation_rs_sd", |b| {
        b.iter(|| iban_validation_rs::validate_iban_get_numeric(black_box("DE44500105175407324931")))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = iban_validate_sd, iban_short_sd, iban_parser_sd, schwifty_sd, iban_validation_rs_sd
);
criterion_main!(benches);
