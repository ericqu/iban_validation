import polars as pl
from polars.testing import assert_frame_equal
from iban_validation_polars import process_ibans

df = pl.DataFrame(
    {"ibans": ["AT611904300234573201", "CY17002001280000001200527600", "Test to fail"]}
)

res = (
    df.with_columns(
        validated=process_ibans("ibans")
        .str.split_exact(",", 2)
        .struct.rename_fields(["valid_ibans", "bank_id", "branch_id"])
    )
    .unnest("validated")
    .sort(by="ibans", descending=True)
)


def test_plugin():
    df = pl.DataFrame(
        {
            "ibans": [
                "AT611904300234573201",
                "CY17002001280000001200527600",
                "Test to fail",
            ]
        }
    )

    res = (
        df.with_columns(
            validated=process_ibans("ibans")
            .str.split_exact(",", 2)
            .struct.rename_fields(["valid_ibans", "bank_id", "branch_id"])
        )
        .unnest("validated")
        .sort(by="ibans", descending=True)
    )

    print(res)

    target_df = pl.DataFrame(
        {
            "ibans": [
                "Test to fail",
                "CY17002001280000001200527600",
                "AT611904300234573201",
            ],
            "valid_ibans": [
                "",
                "CY17002001280000001200527600",
                "AT611904300234573201",
            ],
            "bank_id": [None, "002", "19043"],
            "branch_id": [None, "00128", ""],
        }
    )
    print(target_df)
    assert_frame_equal(res, target_df)