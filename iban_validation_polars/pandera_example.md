# Validating IBAN columns with pandera

[pandera](https://pandera.readthedocs.io/) is a dataframe validation library:
you declare a schema for your dataframe (columns, dtypes, checks) and pandera
enforces it at runtime, producing a detailed failure report when data does not
conform. Since pandera 0.19 it supports Polars natively.

`iban_validation` plugs into pandera as a custom check, giving you
registry-accurate IBAN validation (structure, length, mod-97 checksum, based on
the SWIFT IBAN registry) inside your existing pandera schemas.

Two variants below:

- **Polars** (recommended): uses the `iban_validation_polars` plugin, fully
  vectorized and multi-threaded — suitable for millions of rows.
- **pandas**: uses `iban_validation_py` element-wise. Simple, but roughly 100x
  slower than the Polars plugin on large data (see the
  [benchmarks](https://github.com/ericqu/iban_validation/blob/main/iban_validation_bench_py/README.md)).

## Installation

```bash
pip install pandera[polars] polars iban-validation-polars     # Polars variant
pip install pandera[pandas] pandas iban-validation-py         # pandas variant
```

## Polars (vectorized, recommended)

A pandera custom check for Polars receives a `PolarsData` object and must
return a LazyFrame with a single boolean column. `process_ibans` returns
`"validated_iban,bank_id,branch_id"` per row, with an **empty first field when
the IBAN is invalid** — so validity is simply "first field is non-empty":

```python
import polars as pl
import pandera.polars as pa
from pandera.polars import PolarsData
from iban_validation_polars import process_ibans


def valid_iban(data: PolarsData) -> pl.LazyFrame:
    """Vectorized IBAN check (structure, length, mod-97 checksum)
    powered by the iban_validation Rust core."""
    return data.lazyframe.select(
        process_ibans(data.key)
        .str.split_exact(",", 2)
        .struct.field("field_0")
        .ne("")
    )


schema = pa.DataFrameSchema(
    {
        "iban": pa.Column(
            str,
            pa.Check(
                valid_iban,
                error="invalid IBAN (structure/length/checksum per SWIFT registry)",
            ),
        ),
        "amount": pa.Column(float, pa.Check.ge(0)),
    }
)

df = pl.DataFrame(
    {
        "iban": [
            "DE44500105175407324931",       # valid
            "FR7630006000011234567890189",  # valid
            "DE44500105175407324932",       # bad checksum
        ],
        "amount": [10.0, 25.5, 3.0],
    }
)

schema.validate(df)
```

Running this raises a `SchemaError` pointing at the offending row:

```text
pandera.errors.SchemaError: Column 'iban' failed validator ...
invalid IBAN (structure/length/checksum per SWIFT registry)
failure cases: DE44500105175407324932
```

Remove the bad row (or fix the check digit to `...31`) and `schema.validate(df)`
returns the dataframe unchanged.

> **LazyFrame note:** by default pandera only runs *schema-level* checks
> (columns, dtypes) on a `pl.LazyFrame` and runs data-level checks like this
> one on eager `pl.DataFrame`s. To force data checks on LazyFrames, set the
> environment variable `PANDERA_VALIDATION_DEPTH=SCHEMA_AND_DATA`.

### Dropping invalid rows instead of raising

For data-cleaning pipelines, let pandera filter instead of fail:

```python
schema_dropping = pa.DataFrameSchema(
    {"iban": pa.Column(str, pa.Check(valid_iban), drop_invalid_rows=True)},
    drop_invalid_rows=True,
)
clean = schema_dropping.validate(df, lazy=True)
```

## pandas (element-wise)

With pandas, use the `iban_validation_py` package element-wise. This is fine
for small dataframes; prefer the Polars variant for bulk data.

```python
import pandas as pd
import pandera.pandas as pa
import iban_validation_py as iv


def is_valid_iban(iban: str) -> bool:
    return iv.IbanValidation(iban).stored_iban is not None


schema = pa.DataFrameSchema(
    {
        "iban": pa.Column(
            str,
            pa.Check(is_valid_iban, element_wise=True, error="invalid IBAN"),
        ),
    }
)

df = pd.DataFrame(
    {
        "iban": [
            "DE44500105175407324931",
            "FR7630006000011234567890189",
            "DE44500105175407324932",  # bad checksum
        ]
    }
)

schema.validate(df, lazy=True)
```

With `lazy=True` pandera collects all failures into a `SchemaErrors` report
listing each failing value and its row index.

## Going further

- Extract `bank_id` / `branch_id` alongside validation with the same
  `process_ibans` call — see the
  [iban_validation_polars README](https://github.com/ericqu/iban_validation/blob/main/iban_validation_polars/README.md).
- Classify IBANs as `"registry"` / `"non_registry"` / `"invalid"` with
  `country_status`, or accept non-registry IBAN-shaped identifiers with
  `process_ibans(..., allow_non_registry=True)`.
- Note: a passing check means the IBAN is structurally valid per the SWIFT
  registry — it does not guarantee the account exists.
