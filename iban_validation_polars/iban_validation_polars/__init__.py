from pathlib import Path

import polars as pl
from polars.plugins import register_plugin_function
from polars._typing import IntoExpr

PLUGIN_PATH = Path(__file__).parent

def process_ibans(expr: IntoExpr, *, allow_non_registry: bool = False) -> pl.Expr:
    """validates IBAN and return struct with valid iban , bank identifier, and branch identifier when relevant

    allow_non_registry: when True, additionally accepts 22 IBAN-shaped account
    numbers that are not in the official SWIFT IBAN registry (community-sourced,
    weaker guarantees). Defaults to False (registry-only, same as before)."""
    return register_plugin_function(
        plugin_path=PLUGIN_PATH,
        function_name="process_ibans",
        args=expr,
        kwargs={"allow_non_registry": allow_non_registry},
        is_elementwise=True,
    )


def country_status(expr: IntoExpr) -> pl.Expr:
    """classifies each IBAN as "registry", "non_registry", or "invalid" """
    return register_plugin_function(
        plugin_path=PLUGIN_PATH,
        function_name="country_status",
        args=expr,
        is_elementwise=True,
    )

# def process_ibans_num(expr: IntoExpr) -> pl.Expr:
#     """validates IBAN and return struct with valid iban , bank identifier, and branch identifier when relevant"""
#     return register_plugin_function(
#         plugin_path=PLUGIN_PATH,
#         function_name="process_ibans_num",
#         args=expr,
#         is_elementwise=True,
#     )