import polars as pl

pl.Config.set_tbl_cols(15)
pl.Config.set_tbl_rows(45)
inputfile = "iban_validation_preprocess/iban_registry_v101.txt"
output_source_file = "iban_validation_rs/data/iban_sourcefile.txt"
output_rust_codegen = "iban_validation_rs/src/iban_definition.rs"


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


def pre_process_to_rust(inputfile, output_rust_codegen):
    pre_df = get_df_from_input(inputfile)

    iban_min_len = pre_df.select(pl.min("iban_len")).item()
    iban_max_len = pre_df.select(pl.max("iban_len")).item()
    iban_len_assertions = []

    rs_code = """// Auto-generated from iban_validation_preprocess/pre_process_registry.py, do not edit manually
use crate::{{IbanFields, ValidationLetterError}};
use crate::{{simple_contains_a, simple_contains_c, simple_contains_n}};

pub const _IBAN_MIN_LEN: u8 = {};
pub const _IBAN_MAX_LEN: u8 = {};

pub const IBAN_DEFINITIONS: [IbanFields; {}] = [
""".format(iban_min_len, iban_max_len, len(pre_df))

    for row in pre_df.iter_rows(named=True):
        # Extract values and handle None values
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

        # Convert country code to ASCII representation for comment
        country_str = (
            chr(ctry_cd[0]) + chr(ctry_cd[1]) if isinstance(ctry_cd, list) else "??"
        )
        iban_len_assertions.append(
        f"""let _ = [(); ({iban_len} >= 4) as usize - 1]; // {country_str}"""
        )

        # Format the struct initialization
        rs_code += """    IbanFields {{
        ctry_cd: [{}, {}], // "{}"
        iban_len: {},
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

    # Close the array
    rs_code += "];\n"

    rs_code += """
pub fn get_iban_fields(cc: [u8; 2]) -> Option<&'static IbanFields> {
    match cc {
"""
    counter = 0
    for row in pre_df.iter_rows(named=True):
        ctry_cd = row["ctry_cd"]
        # Convert country code to ASCII representation for comment
        country_str = (
            chr(ctry_cd[0]) + chr(ctry_cd[1]) if isinstance(ctry_cd, list) else "??"
        )
        rs_code += """      [{}, {}] => Some(&IBAN_DEFINITIONS[{}]), // {}
""".format(
            ctry_cd[0],
            ctry_cd[1],
            counter,
            country_str,
        )
        counter += 1

    rs_code += """     _ => None,
    }
}

"""
    rs_code += generate_literal_validators()
    rs_code += """
// Compile-time invariants
const _: () = {
"""
    rs_code += "\n".join(iban_len_assertions)
    rs_code += """
};
"""

    # Write to output file
    with open(output_rust_codegen, "w") as f:
        f.write(rs_code)

    print(f"Rust code written to {output_rust_codegen}")


if __name__ == "__main__":
    pre_process_to_rust(inputfile, output_rust_codegen)
    pre_process_filename(inputfile, output_source_file)
