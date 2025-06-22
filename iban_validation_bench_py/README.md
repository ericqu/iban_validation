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
test_ivp_pandas      1.9385 (1.0)      2.0253 (1.0)      1.9856 (1.0)      0.0311 (1.0)      1.9864 (1.0)      0.0300 (1.0)           2;0  0.5036 (1.0)           5           1
test_sch_pandas      2.8144 (1.45)     2.9067 (1.44)     2.8691 (1.44)     0.0345 (1.11)     2.8696 (1.44)     0.0384 (1.28)          2;0  0.3485 (0.69)          5           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------------- benchmark 'polars': 3 tests -----------------------------------------------------------------------------------
Name (time in ms)            Min                   Max                  Mean            StdDev                Median               IQR            Outliers       OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ipl_polars           2.4337 (1.0)          3.0341 (1.0)          2.6100 (1.0)      0.0877 (1.0)          2.5983 (1.0)      0.1190 (1.0)          57;3  383.1448 (1.0)         206           1
test_ivp_polars         337.3497 (138.62)     343.9324 (113.36)     340.9178 (130.62)   2.3415 (26.70)      341.1209 (131.29)   1.7448 (14.67)         2;1    2.9333 (0.01)          5           1
test_sch_polars       1,130.0885 (464.35)   1,141.3496 (376.18)   1,136.4749 (435.43)   4.7916 (54.64)    1,137.1831 (437.66)   8.2715 (69.53)         2;0    0.8799 (0.00)          5           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------------------- benchmark 'single': 2 tests ----------------------------------------------------------------------------------------
Name (time in ns)            Min                     Max                  Mean              StdDev                Median                 IQR             Outliers  OPS (Kops/s)            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_iban           117.5000 (1.0)          556.6700 (1.0)        135.3630 (1.0)        5.7963 (1.0)        134.5800 (1.0)        2.5000 (1.0)      4708;6337    7,387.5453 (1.0)       82190         100
test_sch_iban         5,292.0077 (45.04)    158,625.0000 (284.95)   6,155.0580 (45.47)    586.5481 (101.19)   6,125.0030 (45.51)    124.9864 (49.99)    2150;5571      162.4680 (0.02)     187512           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

For the details look in the test file in the iban_validation_bench_py package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [Pypi](https://pypi.org/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
