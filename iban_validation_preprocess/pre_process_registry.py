import polars as pl

pl.Config.set_tbl_cols(15)
pl.Config.set_tbl_rows(45)
inputfile = "iban_validation_preprocess/iban_registry_v102.txt"
output_source_file = "iban_validation_rs/data/iban_sourcefile.txt"
output_rust_codegen = "iban_validation_rs/src/iban_definition.rs"
non_registry_inputfile = "iban_validation_preprocess/iban_registry_non_registry.csv"


# preprocess and check iban structure
def process_iban_structure(i_structure_e: str):
    iso3166 = i_structure_e[0:2]
    i = 2
    next_exclamation = 0
    while i_structure_e.find("!", i) > 0:
        next_exclamation = i_structure_e.find("!", i)
        num = int(i_structure_e[i:next_exclamation])
        letter = str(i_structure_e[next_exclamation + 1 : next_exclamation + 2])
        iso3166 = str(iso3166) + num * letter
        i = next_exclamation + 2
    return iso3166


def pre_process_filename(inputfile, output_source_file):
    from pathlib import Path

    file_path = Path(inputfile)
    filename = file_path.name  # Get the filename from the path
    with open(output_source_file, "w") as f:
        f.write(filename)


def get_df_from_input(inputfile):
    df = pl.scan_csv(inputfile, separator="\t", quote_char='"', n_rows=25)

    header = df.select(df.collect_schema().names()[0]).collect().to_series().to_list()
    df = df.collect().transpose(include_header=False, column_names=header).slice(1)

    territory_mapping = pl.DataFrame(
        {
            "original_code": [
                "FR",
                "FR",
                "FR",
                "FR",
                "FR",
                "FR",
                "FR",
                "FR",
                "FR",
                "FR",
                "FR",
                "FR",
                "FR",
                "GB",
                "GB",
                "GB",
                "GB",
            ],
            "new_code": [
                "FR",
                "GP",
                "MQ",
                "GF",
                "RE",
                "YT",
                "NC",
                "PF",
                "PM",
                "TF",
                "WF",
                "BL",
                "MF",
                "GB",
                "IM",
                "JE",
                "GG",
            ],
        }
    )

    pre_df = (
        df.with_columns(
            pl.when(pl.col("IBAN prefix country code (ISO 3166)") == "IQ")
            .then(pl.lit("5-7"))
            .otherwise(pl.col("Branch identifier position within the BBAN"))
            .alias("Branch identifier position within the BBAN")
        )
        .with_columns(
            pl.col("IBAN structure")
            .map_elements(process_iban_structure, return_dtype=pl.String)
            .alias("iban_struct"),
            pl.when(pl.col("IBAN prefix country code (ISO 3166)") == "IQ")
            .then(pl.lit("1-4"))
            .otherwise(
                pl.col("Bank identifier position within the BBAN").str.strip_chars()
            )
            .alias("Bank identifier position within the BBAN"),
            pl.col("Branch identifier position within the BBAN")
            .str.split_exact(by="-", n=1)
            .struct.rename_fields(["branch_id_pos_s", "branch_id_pos_e"])
            .alias("fields")
            .struct.unnest(),
            pl.col("IBAN length").cast(pl.UInt16),
        )
        .with_columns(
            pl.col("Bank identifier position within the BBAN")
            .str.slice(0, 1)
            .str.to_integer(strict=False)
            .cast(pl.UInt16)
            .alias("bank_id_pos_s"),
            pl.col("Bank identifier position within the BBAN")
            .str.slice(2)
            .str.to_integer(strict=False)
            .cast(pl.UInt16)
            .alias("bank_id_pos_e"),
            pl.col("branch_id_pos_s").str.to_integer(strict=False).cast(pl.UInt16),
            pl.col("branch_id_pos_e").str.to_integer(strict=False).cast(pl.UInt16),
        )
        .with_columns(
            pl.col("IBAN electronic format example")
            .str.slice(
                3 + pl.col("bank_id_pos_s"),
                pl.col("bank_id_pos_e") + 1 - pl.col("bank_id_pos_s"),
            )
            .alias("bank_id")
        )
        .rename(
            {
                "IBAN prefix country code (ISO 3166)": "ctry_cd",
                "IBAN length": "iban_len",
            }
        )
        .join(
            territory_mapping, left_on="ctry_cd", right_on="original_code", how="left"
        )
        .with_columns(
            pl.coalesce([pl.col("new_code"), pl.col("ctry_cd")]).alias("ctry_cd")
        )
        .drop("new_code")
        .with_columns(
            (pl.col("ctry_cd") + pl.col("iban_struct").str.slice(2)).alias(
                "iban_struct"
            )
        )
        .with_columns(
            (
                pl.col("iban_struct").str.slice(4)
                + pl.col("iban_struct").str.slice(0, 4)  # .alias('temp_is')
            )
        )
        .with_columns(
            pl.col("ctry_cd").map_elements(
                lambda x: [ord(c) for c in x], return_dtype=pl.List(pl.UInt16)
            )
        )
        .select(
            [
                "ctry_cd",
                "iban_len",
                "bank_id_pos_s",
                "bank_id_pos_e",
                "branch_id_pos_s",
                "branch_id_pos_e",
                "iban_struct",
            ]
        )
    )
    return pre_df


def get_non_registry_df_from_input(inputfile):
    """Load the non-registry IBAN countries CSV into the same
    schema produced by get_df_from_input, so the two can feed the same
    codegen helpers. No territory remapping applies to this data source."""
    import csv

    def parse_pos(pos: str):
        if not pos:
            return None, None
        start, end = pos.split("-")
        return int(start), int(end)

    rows = []
    with open(inputfile, newline="") as f:
        for record in csv.DictReader(f):
            iban_struct = process_iban_structure(record["iban_structure"])
            iban_struct = iban_struct[4:] + iban_struct[0:4]
            bank_id_pos_s, bank_id_pos_e = parse_pos(record["bank_id_pos"])
            branch_id_pos_s, branch_id_pos_e = parse_pos(record["branch_id_pos"])
            rows.append(
                {
                    "ctry_cd": [ord(c) for c in record["country_code"]],
                    "iban_len": int(record["iban_length"]),
                    "bank_id_pos_s": bank_id_pos_s,
                    "bank_id_pos_e": bank_id_pos_e,
                    "branch_id_pos_s": branch_id_pos_s,
                    "branch_id_pos_e": branch_id_pos_e,
                    "iban_struct": iban_struct,
                }
            )

    return pl.DataFrame(
        rows,
        schema={
            "ctry_cd": pl.List(pl.UInt16),
            "iban_len": pl.UInt16,
            "bank_id_pos_s": pl.UInt16,
            "bank_id_pos_e": pl.UInt16,
            "branch_id_pos_s": pl.UInt16,
            "branch_id_pos_e": pl.UInt16,
            "iban_struct": pl.String,
        },
    )


def generate_iban_fields_array(rows, array_name: str) -> tuple[str, list[str]]:
    """Build the `pub const {array_name}: [IbanFields; N] = [...]` literal plus
    a compile-time length assertion per row."""
    entries = []
    assertions = []
    for row in rows:
        ctry_cd = row["ctry_cd"]
        iban_len = row["iban_len"]
        bank_id_pos_s = (
            f"Some({row['bank_id_pos_s']})"
            if row["bank_id_pos_s"] is not None
            else "None"
        )
        bank_id_pos_e = (
            f"Some({row['bank_id_pos_e']})"
            if row["bank_id_pos_e"] is not None
            else "None"
        )
        branch_id_pos_s = (
            f"Some({row['branch_id_pos_s']})"
            if row["branch_id_pos_s"] is not None
            else "None"
        )
        branch_id_pos_e = (
            f"Some({row['branch_id_pos_e']})"
            if row["branch_id_pos_e"] is not None
            else "None"
        )
        iban_struct = row["iban_struct"]

        country_str = (
            chr(ctry_cd[0]) + chr(ctry_cd[1]) if isinstance(ctry_cd, list) else "??"
        )
        assertions.append(
            f"""let _ = [(); ({iban_len} >= 4) as usize - 1]; // {country_str}"""
        )

        entries.append(
            """    IbanFields {{
        ctry_cd: [{}, {}], // "{}" {} characters
        bank_id_pos_s: {},
        bank_id_pos_e: {},
        branch_id_pos_s: {},
        branch_id_pos_e: {},
        iban_struct_validators: &{}
    }},""".format(
                ctry_cd[0],
                ctry_cd[1],
                country_str,
                iban_len,
                bank_id_pos_s,
                bank_id_pos_e,
                branch_id_pos_s,
                branch_id_pos_e,
                generate_from_struct_to_validator(iban_struct),
            )
        )

    array_code = "pub const {}: [IbanFields; {}] = [\n".format(array_name, len(entries))
    array_code += "\n".join(entries)
    array_code += "\n];\n"
    return array_code, assertions


def generate_lookup_arms(rows, array_name: str) -> str:
    """Build the `[cc0, cc1] => Some(&{array_name}[i]), // CC` match arms."""
    arms = []
    for counter, row in enumerate(rows):
        ctry_cd = row["ctry_cd"]
        country_str = (
            chr(ctry_cd[0]) + chr(ctry_cd[1]) if isinstance(ctry_cd, list) else "??"
        )
        arms.append(
            "      [{}, {}] => Some(&{}[{}]), // {}".format(
                ctry_cd[0], ctry_cd[1], array_name, counter, country_str
            )
        )
    return "\n".join(arms) + "\n"


def generate_literal_validators() -> str:
    """Generate all literal_XX functions for A-Z"""
    functions = []

    for ascii_val in range(65, 91):  # A-Z
        char = chr(ascii_val).lower()
        func = f"""#[inline]
fn literal_{char}(c: u8) -> Result<usize, ValidationLetterError> {{
    if c == {ascii_val} {{ // '{char.upper()}'
        Ok((c - 55) as usize)
    }} else {{
        Err(ValidationLetterError::NotPartOfRequiredSet)
    }}
}}"""
        functions.append(func)

    return "\n\n".join(functions)


def generate_from_struct_to_validator(iban_struct: str) -> str:
    """ "Given an iban_gives a validator code"""
    rust_code = "["
    for idx, c in enumerate(iban_struct):
        if c.islower():
            if c == "n":
                rust_code += "\n\t\t\tsimple_contains_n,"
            elif c == "c":
                rust_code += "\n\t\t\tsimple_contains_c,"
            elif c == "a":
                rust_code += "\n\t\t\tsimple_contains_a,"
        else:
            rust_code += "\n\t\t\tliteral_{},".format(c.lower())

    rust_code += "\n\t\t],"
    return rust_code


def pre_process_to_rust(inputfile, output_rust_codegen, non_registry_inputfile=None):
    pre_df = get_df_from_input(inputfile)

    # official min/max drive the public consts; non-registry data must fit
    # within these bounds but never widens them.
    iban_min_len = pre_df.select(pl.min("iban_len")).item()
    iban_max_len = pre_df.select(pl.max("iban_len")).item()

    non_registry_df = None
    if non_registry_inputfile is not None:
        non_registry_df = get_non_registry_df_from_input(non_registry_inputfile)
        if len(non_registry_df) > 0:
            non_registry_min_len = non_registry_df.select(pl.min("iban_len")).item()
            non_registry_max_len = non_registry_df.select(pl.max("iban_len")).item()
            assert iban_min_len <= non_registry_min_len and non_registry_max_len <= iban_max_len, (
                f"non-registry iban_len range [{non_registry_min_len}, {non_registry_max_len}] "
                f"falls outside official [{iban_min_len}, {iban_max_len}]"
            )

    rs_code = """// Auto-generated from iban_validation_preprocess/pre_process_registry.py, do not edit manually
use crate::{{IbanFields, ValidationLetterError}};
use crate::{{simple_contains_a, simple_contains_c, simple_contains_n}};

pub const IBAN_MIN_LEN: u8 = {};
pub const IBAN_MAX_LEN: u8 = {};

""".format(iban_min_len, iban_max_len)

    official_rows = list(pre_df.iter_rows(named=True))
    official_array, official_assertions = generate_iban_fields_array(
        official_rows, "IBAN_DEFINITIONS"
    )
    rs_code += official_array

    rs_code += """
pub fn get_iban_fields(cc: [u8; 2]) -> Option<&'static IbanFields> {
    match cc {
"""
    rs_code += generate_lookup_arms(official_rows, "IBAN_DEFINITIONS")
    rs_code += """     _ => None,
    }
}

/// Look up a country's `IbanFields` across the given [`crate::CountrySet`]: the
/// official registry first, falling back to the opt-in non-registry table when
/// `set` is [`crate::CountrySet::WithNonRegistry`] (a no-op when the `non_registry`
/// feature is not compiled in).
pub fn get_iban_fields_with(cc: [u8; 2], set: crate::CountrySet) -> Option<&'static IbanFields> {
    get_iban_fields(cc).or_else(|| match set {
        crate::CountrySet::Registry => None,
        crate::CountrySet::WithNonRegistry => get_non_registry_fields(cc),
    })
}

/// Whether `cc` identifies one of the opt-in, non-registry countries (always
/// `false` when the `non_registry` feature is not compiled in).
pub fn is_non_registry_country(cc: [u8; 2]) -> bool {
    get_non_registry_fields(cc).is_some()
}

"""

    has_non_registry = non_registry_df is not None and len(non_registry_df) > 0
    if not has_non_registry:
        rs_code += """fn get_non_registry_fields(_cc: [u8; 2]) -> Option<&'static IbanFields> {
    None
}

"""

    rs_code += generate_literal_validators()
    rs_code += """
// Compile-time invariants
const _: () = {
"""
    rs_code += "\n".join(official_assertions)
    rs_code += """
};
"""

    if has_non_registry:
        non_registry_rows = list(non_registry_df.iter_rows(named=True))
        non_registry_array, non_registry_assertions = generate_iban_fields_array(
            non_registry_rows, "NON_REGISTRY_IBAN_DEFINITIONS"
        )

        countries_entries = ",\n    ".join(
            "[b'{}', b'{}']".format(chr(row["ctry_cd"][0]), chr(row["ctry_cd"][1]))
            for row in non_registry_rows
        )

        rs_code += """
#[cfg(not(feature = "non_registry"))]
fn get_non_registry_fields(_cc: [u8; 2]) -> Option<&'static IbanFields> {{
    None
}}

/// Two-letter codes of the opt-in, non-registry countries, in the same order as
/// [`NON_REGISTRY_IBAN_DEFINITIONS`].
#[cfg(feature = "non_registry")]
pub const NON_REGISTRY_COUNTRIES: &[[u8; 2]] = &[
    {}
];

#[cfg(feature = "non_registry")]
""".format(countries_entries)
        rs_code += non_registry_array
        rs_code += """
#[cfg(feature = "non_registry")]
fn get_non_registry_fields(cc: [u8; 2]) -> Option<&'static IbanFields> {
    match cc {
"""
        rs_code += generate_lookup_arms(
            non_registry_rows, "NON_REGISTRY_IBAN_DEFINITIONS"
        )
        rs_code += """     _ => None,
    }
}

#[cfg(feature = "non_registry")]
const _: () = {
"""
        rs_code += "\n".join(non_registry_assertions)
        rs_code += """
};
"""

    # Write to output file
    with open(output_rust_codegen, "w") as f:
        f.write(rs_code)

    print(f"Rust code written to {output_rust_codegen}")


if __name__ == "__main__":
    pre_process_to_rust(inputfile, output_rust_codegen, non_registry_inputfile)
    pre_process_filename(inputfile, output_source_file)
