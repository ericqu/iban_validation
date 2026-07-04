import polars as pl
from polars.testing import assert_frame_equal
from iban_validation_polars import process_ibans, country_status


def test_plugin():
    df = pl.DataFrame(
        {
            "ibans": [
                "AT611904300234573201",
                "CY17002001280000001200527600",
                "Test to fail",
                "",
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
                "",
            ],
            "valid_ibans": [
                "",
                "CY17002001280000001200527600",
                "AT611904300234573201",
                "",
            ],
            "bank_id": [None, "002", "19043", None],
            "branch_id": [None, "00128", "", None],
        }
    )
    print(target_df)
    assert_frame_equal(res, target_df)


def test_non_registry_default_rejected():
    df = pl.DataFrame({"ibans": ["AO49012345678901234567890"]})
    res = df.with_columns(
        valid_iban=process_ibans("ibans").str.split_exact(",", 2).struct.field("field_0")
    )
    assert res["valid_iban"].to_list() == [""]


def test_non_registry_opt_in_accepted():
    df = pl.DataFrame(
        {"ibans": ["AO49012345678901234567890", "MA36012345678901234567890123"]}
    )
    res = (
        df.with_columns(
            validated=process_ibans("ibans", allow_non_registry=True)
            .str.split_exact(",", 2)
            .struct.rename_fields(["valid_ibans", "bank_id", "branch_id"])
        )
        .unnest("validated")
    )
    assert res["valid_ibans"].to_list() == df["ibans"].to_list()
    # AO has no bank/branch position data, so those fields are empty (not null)
    assert res["bank_id"].to_list() == ["", "01234"]
    assert res["branch_id"].to_list() == ["", "56789"]


def test_country_status_classification():
    df = pl.DataFrame(
        {
            "ibans": [
                "AT611904300234573201",  # registry
                "AO49012345678901234567890",  # non_registry
                "Test to fail",  # invalid
            ]
        }
    )
    res = df.with_columns(status=country_status("ibans"))
    assert res["status"].to_list() == ["registry", "non_registry", "invalid"]


def test_ipl_enrich_df_polars(csvfile="iban_validation_bench_py/data/test_file.csv"):
    df = (
        pl.scan_csv(csvfile)
        .with_columns(
            iban_infos=process_ibans("IBAN Examples")
            .str.split_exact(",", 2)
            .struct.rename_fields(["valid_ibans", "bank_id", "branch_id"])
        )
        .unnest("iban_infos")
        .collect(engine="streaming")
    )

    print(df)
