## Benchmark iban_validation_py and iban_validation_polars against similar libraries
To give a perspective on how the python wrapper performs with regards to other similar libraries. The other libraries can have additional features that iban_validation_py does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

## Outcome
For now I tested only schiwty as this is the most prominent Python library. 
In the context of a single call through the python api, the iban_validation_py package is about 30 times faster.
In the context of calls through the Pandas dataframe, the iban_validation_py package is only 1.5 times faster.
In the context of calls through the Polars dataframe, the iban_validation_py package is about 3 times faster.
In the context of calls through the Polars dataframe, but using the iban_validation_polars plugin, then the plugin is about 100 times faster than the iban_validation_py, and about 300 times faster than schwifty. Which is where the real gain is, and the reason why the polars plugin exists.

Here is the output from pytest:
```
------------------------------------------------------------------------- benchmark 'pandas': 2 tests -------------------------------------------------------------------------
Name (time in s)        Min               Max              Mean            StdDev            Median               IQR            Outliers     OPS            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_pandas      1.9127 (1.0)      2.0024 (1.0)      1.9631 (1.0)      0.0322 (1.30)     1.9661 (1.0)      0.0279 (1.0)           2;0  0.5094 (1.0)           5           1
test_sch_pandas      2.8341 (1.48)     2.8970 (1.45)     2.8671 (1.46)     0.0248 (1.0)      2.8596 (1.45)     0.0359 (1.29)          2;0  0.3488 (0.68)          5           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

----------------------------------------------------------------------------------- benchmark 'polars': 3 tests -----------------------------------------------------------------------------------
Name (time in ms)            Min                   Max                  Mean            StdDev                Median                IQR            Outliers       OPS            Rounds  Iterations
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ipl_polars           2.4626 (1.0)          3.0938 (1.0)          2.6312 (1.0)      0.0839 (1.0)          2.6347 (1.0)       0.1156 (1.0)          66;2  380.0531 (1.0)         210           1
test_ivp_polars         333.7688 (135.53)     337.0165 (108.93)     335.5211 (127.52)   1.3961 (16.63)      336.0793 (127.56)    2.3607 (20.42)         2;0    2.9804 (0.01)          5           1
test_sch_polars       1,132.7308 (459.97)   1,148.1930 (371.13)   1,140.9599 (433.63)   6.6653 (79.42)    1,141.1590 (433.13)   11.8951 (102.91)        2;0    0.8765 (0.00)          5           1
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

--------------------------------------------------------------------------------------- benchmark 'single': 2 tests ----------------------------------------------------------------------------------------
Name (time in ns)            Min                    Max                  Mean              StdDev                Median                 IQR             Outliers  OPS (Kops/s)            Rounds  Iterations
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_iban           120.0000 (1.0)         528.7500 (1.0)        141.6072 (1.0)       12.0353 (1.0)        139.1700 (1.0)        9.1700 (1.0)      8989;3265    7,061.7874 (1.0)       77670         100
test_sch_iban         5,250.0000 (43.75)    97,416.9998 (184.24)   6,096.8987 (43.06)    356.4848 (29.62)    6,083.0007 (43.71)    124.9991 (13.63)    5194;6320      164.0178 (0.02)     190477           1
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

For the details look in the test file in the iban_validation_bench_py package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [Pypi](https://pypi.org/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
