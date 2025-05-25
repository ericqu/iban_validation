from pathlib import Path

import polars as pl
from polars.plugins import register_plugin_function
from polars._typing import IntoExpr

PLUGIN_PATH = Path(__file__).parent

def process_ibans(expr: IntoExpr) -> pl.Expr:
    """validates IBAN and return struct with valid iban , bank identifier, and branch identifier when relevant"""
    return register_plugin_function(
        plugin_path=PLUGIN_PATH,
        function_name="process_ibans",
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