## Benchmark iban_validation_py and iban_validation_polars against similar libraries
To give a perspective on how the python wrapper performs with regards to other similar libraries. The other libraries can have additional features that iban_validation_py does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

## Outcome
For now I tested only schiwty as this is the most prominent Python library. 
In the context of a single call through the python api, the iban_validation_py package is about 30 times faster.
In the context of calls through the Pandas dataframe, the iban_validation_py package is only 1.5 times faster.
In the context of calls through the Polars dataframe, the iban_validation_py packahe is about 3 times faster.
In the context of calls through the Polars dataframe, but using the iban_validation_polars plugin, then the plugin is about 75 times faster than the iban_validation_py, and about 200 times faster than schwifty.

Here is the output from pytest:
```
------------------------------------------------------------------------- benchmark 'pandas': 2 tests -------------------------------------------------------------------------
Name (time in s)        Min               Max              Mean            StdDev            Median               IQR            Outliers     OPS            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_pandas      1.9130 (1.0)      1.9506 (1.0)      1.9258 (1.0)      0.0146 (1.0)      1.9210 (1.0)      0.0145 (1.0)           1;0  0.5193 (1.0)           5           1
test_sch_pandas      2.8088 (1.47)     2.8696 (1.47)     2.8378 (1.47)     0.0221 (1.52)     2.8348 (1.48)     0.0255 (1.75)          2;0  0.3524 (0.68)          5           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------------- benchmark 'polars': 3 tests -----------------------------------------------------------------------------------
Name (time in ms)            Min                   Max                  Mean            StdDev                Median               IQR            Outliers       OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ipl_polars           4.0872 (1.0)          4.9976 (1.0)          4.5319 (1.0)      0.3446 (1.0)          4.5309 (1.0)      0.4878 (1.0)           2;0  220.6556 (1.0)           5           1
test_ivp_polars         338.7387 (82.88)      347.4828 (69.53)      342.7749 (75.64)    4.3370 (12.59)      340.5337 (75.16)    8.0463 (16.49)         2;0    2.9174 (0.01)          5           1
test_sch_polars       1,132.3041 (277.04)   1,134.5975 (227.03)   1,132.8554 (249.97)   0.9785 (2.84)     1,132.4318 (249.94)   0.7133 (1.46)          1;1    0.8827 (0.00)          5           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------------------- benchmark 'single': 2 tests -----------------------------------------------------------------------------------------
Name (time in ns)            Min                    Max                  Mean              StdDev                Median                 IQR               Outliers  OPS (Kops/s)            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_iban           160.8298 (1.0)         492.5000 (1.0)        175.4948 (1.0)       11.3540 (1.0)        181.2499 (1.0)       18.3404 (1.0)        12756;535    5,698.1737 (1.0)       61069         100
test_sch_iban         5,332.9859 (33.16)    53,625.0009 (108.88)   6,161.2947 (35.11)    538.9544 (47.47)    6,167.0144 (34.02)    125.0010 (6.82)     17388;25373      162.3035 (0.03)     187513           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

For the details look in the test file in the iban_validation_bench_py package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [Pypi](https://pypi.org/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 