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
test_ivp_pandas      1.9684 (1.0)      2.0716 (1.0)      2.0037 (1.0)      0.0398 (1.73)     1.9955 (1.0)      0.0376 (1.24)          1;0  0.4991 (1.0)           5           1
test_sch_pandas      2.8477 (1.45)     2.9080 (1.40)     2.8725 (1.43)     0.0230 (1.0)      2.8702 (1.44)     0.0304 (1.0)           2;0  0.3481 (0.70)          5           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------------- benchmark 'polars': 3 tests -----------------------------------------------------------------------------------
Name (time in ms)            Min                   Max                  Mean            StdDev                Median               IQR            Outliers       OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ipl_polars           3.4502 (1.0)          5.1083 (1.0)          3.9703 (1.0)      0.5996 (1.0)          3.7594 (1.0)      0.4690 (1.0)           1;1  251.8685 (1.0)           6           1
test_ivp_polars         342.3475 (99.23)      353.6371 (69.23)      346.2612 (87.21)    4.5349 (7.56)       344.1874 (91.55)    5.6837 (12.12)         1;0    2.8880 (0.01)          5           1
test_sch_polars       1,131.4400 (327.94)   1,139.7743 (223.12)   1,137.2151 (286.43)   3.3435 (5.58)     1,138.3879 (302.81)   3.3171 (7.07)          1;0    0.8793 (0.00)          5           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

--------------------------------------------------------------------------------------- benchmark 'single': 2 tests ---------------------------------------------------------------------------------------
Name (time in ns)            Min                    Max                  Mean              StdDev                Median                IQR             Outliers  OPS (Kops/s)            Rounds  Iterations
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_iban           122.0798 (1.0)         465.0000 (1.0)        144.9708 (1.0)       10.5567 (1.0)        145.0002 (1.0)      10.4198 (1.0)      9240;1529    6,897.9429 (1.0)       74075         100
test_sch_iban         5,250.0109 (43.00)    36,125.0131 (77.69)    6,126.9552 (42.26)    333.8576 (31.63)    6,124.9884 (42.24)    83.9937 (8.06)    6670;10883      163.2132 (0.02)     187513           1
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

For the details look in the test file in the iban_validation_bench_py package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [Pypi](https://pypi.org/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 