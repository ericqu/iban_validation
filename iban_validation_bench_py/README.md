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
test_ivp_pandas      1.9205 (1.0)      1.9621 (1.0)      1.9360 (1.0)      0.0160 (1.0)      1.9351 (1.0)      0.0183 (1.0)           1;0  0.5165 (1.0)           5           1
test_sch_pandas      2.7951 (1.46)     2.8829 (1.47)     2.8379 (1.47)     0.0358 (2.23)     2.8292 (1.46)     0.0581 (3.17)          2;0  0.3524 (0.68)          5           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

----------------------------------------------------------------------------------- benchmark 'polars': 3 tests -----------------------------------------------------------------------------------
Name (time in ms)            Min                   Max                  Mean            StdDev                Median                IQR            Outliers       OPS            Rounds  Iterations
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ipl_polars           2.4545 (1.0)          3.0551 (1.0)          2.6342 (1.0)      0.0787 (1.0)          2.6324 (1.0)       0.1027 (1.0)          65;3  379.6273 (1.0)         216           1
test_ivp_polars         329.5445 (134.26)     335.5868 (109.84)     332.0484 (126.05)   2.2640 (28.75)      331.3093 (125.86)    2.6344 (25.65)         2;0    3.0116 (0.01)          5           1
test_sch_polars       1,125.7790 (458.67)   1,143.9255 (374.43)   1,135.9050 (431.22)   7.9406 (100.85)   1,137.3792 (432.07)   14.1981 (138.24)        2;0    0.8804 (0.00)          5           1
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------------------- benchmark 'single': 2 tests -----------------------------------------------------------------------------------------
Name (time in ns)            Min                     Max                  Mean              StdDev                Median                IQR               Outliers  OPS (Kops/s)            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_iban           117.9100 (1.0)          400.4200 (1.0)        133.7164 (1.0)        8.1474 (1.0)        134.5800 (1.0)       3.7500 (1.0)      19365;19872    7,478.5134 (1.0)       82761         100
test_sch_iban         5,250.0002 (44.53)    123,458.0002 (308.32)   6,113.1705 (45.72)    445.9240 (54.73)    6,125.0000 (45.51)    84.0002 (22.40)    11048;17420      163.5812 (0.02)     187512           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

For the details look in the test file in the iban_validation_bench_py package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [Pypi](https://pypi.org/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
