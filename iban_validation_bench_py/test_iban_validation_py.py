import iban_validation_py as iv
from iban_validation_polars import process_ibans as ipl_process_iban
from stdnum import iban as stdnum_iban
from schwifty import IBAN as sch_iban
import pandas as pd
import polars as pl

import pytest


test_csv = "data/test_file.csv"


def ivp_check_v_iban(iban_str="DE44500105175407324931"):
    return iv.validate_iban(iban_str)


def ivp_check_iban(iban_str="DE44500105175407324931"):
    return iv.IbanValidation(iban_str)


def ibv_get_iban_info_pandas(iban_str):
    iban = iv.IbanValidation(iban_str)
    iban_validated = iban.stored_iban or ""
    bank_code = iban.iban_bank_id or ""
    branch_code = iban.iban_branch_id or ""
    return pd.Series([iban_validated, bank_code, branch_code])


def ibv_get_iban_info_polars(iban_str):
    iban = iv.IbanValidation(iban_str)
    iban_validated = iban.stored_iban or ""
    bank_code = iban.iban_bank_id or ""
    branch_code = iban.iban_branch_id or ""
    return (iban_validated, bank_code, branch_code)


def ibv_enrich_df_pandas(csvfile=test_csv):
    df = pd.read_csv(csvfile, engine="pyarrow")
    df[["iban_validated", "bank_code", "branch_code"]] = df["IBAN Examples"].apply(
        ibv_get_iban_info_pandas
    )
    print(df)


def ibv_enrich_df_polars(csvfile=test_csv):
    df = (
        pl.scan_csv(csvfile)
        .with_columns(
            pl.col("IBAN Examples")
            .map_elements(ibv_get_iban_info_polars, return_dtype=pl.List(pl.String))
            .alias("iban_infos")
        )
        .with_columns(
            pl.col("iban_infos").list.get(0).alias("iban_validated"),
            pl.col("iban_infos").list.get(1).alias("bank_code"),
            pl.col("iban_infos").list.get(2).alias("branch_code"),
        )
        .drop("iban_infos")
        .collect(new_streaming=True)
    )

    print(df)


def ipl_enrich_df_polars(csvfile=test_csv):
    df = (
        pl.scan_csv(csvfile)
        .with_columns(
            iban_infos=ipl_process_iban("IBAN Examples")
            .str.split_exact(",", 2)
            .struct.rename_fields(["valid_ibans", "bank_id", "branch_id"])
        )
        .unnest("iban_infos")
        .collect(new_streaming=True)
    )

    print(df)


def sch_check_iban(iban_str="DE44500105175407324931"):
    return sch_iban(iban_str)


def sch_get_iban_info_pandas(iban_str):
    iban_validated = ""
    bank_code = ""
    branch_code = ""
    try:
        iban = sch_iban(iban_str)
        iban_validated = str(iban)
        bank_code = iban.bank_code
        branch_code = iban.branch_code or ""
    except Exception:
        pass
    return pd.Series([iban_validated, bank_code, branch_code])


def sch_get_iban_info_polars(iban_str):
    iban_validated = ""
    bank_code = ""
    branch_code = ""
    try:
        iban = sch_iban(iban_str)
        iban_validated = str(iban)
        bank_code = iban.bank_code
        branch_code = iban.branch_code or ""
    except Exception:
        pass
    return (iban_validated, bank_code, branch_code)


def sch_enrich_df_pandas(csvfile=test_csv):
    df = pd.read_csv(csvfile, engine="pyarrow")
    df[["iban_validated", "bank_code", "branch_code"]] = df["IBAN Examples"].apply(
        sch_get_iban_info_pandas
    )
    print(df)


def sch_enrich_df_polars(csvfile=test_csv):
    df = (
        pl.scan_csv(csvfile)
        .with_columns(
            pl.col("IBAN Examples")
            .map_elements(sch_get_iban_info_polars, return_dtype=pl.List(pl.String))
            .alias("iban_infos")
        )
        .with_columns(
            pl.col("iban_infos").list.get(0).alias("iban_validated"),
            pl.col("iban_infos").list.get(1).alias("bank_code"),
            pl.col("iban_infos").list.get(2).alias("branch_code"),
        )
        .drop("iban_infos")
        .collect(new_streaming=True)
    )

    print(df)


def stdnum_enrich_df_pandas(csvfile=test_csv):
    df = pd.read_csv(csvfile, engine="pyarrow")
    df[["iban_validated", "bank_code", "branch_code"]] = df["IBAN Examples"].apply(
        stdnum_get_iban_info_pandas
    )
    print(df)


def stdnum_enrich_df_polars(csvfile=test_csv):
    df = (
        pl.scan_csv(csvfile)
        .with_columns(
            pl.col("IBAN Examples")
            .map_elements(stdnum_get_iban_info_polars, return_dtype=pl.List(pl.String))
            .alias("iban_infos")
        )
        .with_columns(
            pl.col("iban_infos").list.get(0).alias("iban_validated"),
            pl.col("iban_infos").list.get(1).alias("bank_code"),
            pl.col("iban_infos").list.get(2).alias("branch_code"),
        )
        .drop("iban_infos")
        .collect(new_streaming=True)
    )

    print(df)


def stdnum_check_iban(iban_str="DE44500105175407324931"):
    stdnum_iban.validate(iban_str)


def stdnum_get_iban_info_pandas(iban_str):
    iban_validated = ""
    try:
        iban_validated = stdnum_iban.validate(iban_str)
    except Exception:
        pass
    return pd.Series([iban_validated, "", ""])


def stdnum_get_iban_info_polars(iban_str):
    iban_validated = ""
    try:
        iban_validated = stdnum_iban.validate(iban_str)
    except Exception:
        pass
    return (iban_validated, "", "")


@pytest.mark.benchmark(group="single", warmup=True)
def test_ivp_iban(benchmark):
    iban = benchmark(ivp_check_iban)
    assert "DE44500105175407324931" == iban.stored_iban
    assert "50010517" == iban.iban_bank_id
    assert iban.iban_branch_id is None


@pytest.mark.benchmark(group="single", warmup=True)
def test_stdnum_iban(benchmark):
    benchmark(stdnum_check_iban)


@pytest.mark.benchmark(group="pandas")
def test_ivp_pandas(benchmark):
    benchmark(ibv_enrich_df_pandas)


@pytest.mark.benchmark(group="polars")
def test_ivp_polars(benchmark):
    benchmark(ibv_enrich_df_polars)


@pytest.mark.benchmark(group="polars")
def test_ipl_polars(benchmark):
    benchmark(ipl_enrich_df_polars)


@pytest.mark.benchmark(group="single", warmup=True)
def test_sch_iban(benchmark):
    iban = benchmark(sch_check_iban)
    assert "DE44500105175407324931" == str(iban)
    assert "50010517" == iban.bank_code


@pytest.mark.benchmark(group="pandas")
def test_sch_pandas(benchmark):
    benchmark(sch_enrich_df_pandas)


@pytest.mark.benchmark(group="polars")
def test_sch_polars(benchmark):
    benchmark(sch_enrich_df_polars)


@pytest.mark.benchmark(group="pandas")
def test_stdnum_pandas(benchmark):
    benchmark(stdnum_enrich_df_pandas)


@pytest.mark.benchmark(group="polars")
def test_stdnum_polars(benchmark):
    benchmark(stdnum_enrich_df_polars)
