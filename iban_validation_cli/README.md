# iban_validation_cli

A command line front end for [iban_validation_rs](https://crates.io/crates/iban_validation_rs):
read IBANs from standard input or a file, print whether each one is valid along with the bank
and branch identifiers the registry defines for its country.

It exists so the library can be tried, or dropped into a shell pipeline, without writing any
code. Like the core crate it has no dependencies (argument parsing included).

## Install

```sh
cargo install iban_validation_cli
```

The installed binary carries the crate's name, `iban_validation_cli`.

## Use

```sh
$ echo DE44500105175407324931 | iban_validation_cli
DE44500105175407324931	valid	50010517	-

$ printf 'AL47212110090000000235698741\nAL4721211009000000023569874Q\n' | iban_validation_cli
AL47212110090000000235698741	valid	212	11009
AL4721211009000000023569874Q	invalid	The calculated mod97 for the iban indicates an incorrect Iban
```

Text output is tab separated: `iban`, `valid`/`invalid`, then the bank and branch identifiers
(`-` where the country defines none), or the reason when the IBAN is rejected.

CSV output adds a header and an error column, for loading straight into a dataframe:

```sh
$ iban_validation_cli --format csv ibans.txt
iban,valid,bank_id,branch_id,error
DE44500105175407324931,valid,50010517,,
FR1234,invalid,,,The length of the input Iban does match the length for that country
```

`--quiet` reports through the exit code only, which is what you want in a check step:

```sh
$ iban_validation_cli --quiet ibans.txt && echo "all good"
```

## Options

| Option | Effect |
| --- | --- |
| `-f`, `--format <text\|csv>` | Output format, default `text` |
| `-q`, `--quiet` | Print nothing; the exit code carries the result |
| `--print-format` | Accept print format input, removing spaces before validating |
| `--non-registry` | Also accept the 22 community-sourced, non-registry countries |
| `-h`, `--help` | Usage |
| `-V`, `--version` | Version, including the IBAN registry the data was generated from |

`FILE` is read one IBAN per line and empty lines are skipped. When it is absent or is `-`,
standard input is read.

## Exit codes

| Code | Meaning |
| --- | --- |
| `0` | Every IBAN read was valid (including when nothing was read) |
| `1` | At least one IBAN was invalid |
| `2` | The arguments or the input file could not be used |

## Notes

Validation is the strict electronic-format path of the core crate, so an IBAN written with
spaces is rejected unless `--print-format` is passed. The non-registry countries are
community-sourced rather than SWIFT-registered and are therefore opt-in, matching the
`CountrySet` behaviour of the other wrappers.

## Changes
 - 0.1.29: initial release.
