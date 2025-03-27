## Benchmark iban_validation_py and iban_validation_polars against similar libraries
To give a perspective on how the python wrapper performs with regards to other similar libraries. The other libraries can have additional features that iban_validation_py does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

## Outcome
For now I tested only schiwty as this is the most prominent Python library. 
In the context of a single call through the python api, the iban_validation_py package is about 30 times faster.
In the context of calls through the Pandas dataframe, the iban_validation_py package is only 1.5 times faster.
In the context of calls through the Polars dataframe, the iban_validation_py package is about 3 times faster.
In the context of calls through the Polars dataframe, but using the iban_validation_polars plugin, then the plugin is about 75 times faster than the iban_validation_py, and about 200 times faster than schwifty. Which is were the real gain is, and the reason why the polars plugin exists.

Here is the output from pytest:
```
------------------------------------------------------------------------- benchmark 'pandas': 2 tests -------------------------------------------------------------------------
Name (time in s)        Min               Max              Mean            StdDev            Median               IQR            Outliers     OPS            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_pandas      1.8753 (1.0)      1.9565 (1.0)      1.9201 (1.0)      0.0296 (1.08)     1.9200 (1.0)      0.0327 (1.0)           2;0  0.5208 (1.0)           5           1
test_sch_pandas      2.7951 (1.49)     2.8583 (1.46)     2.8364 (1.48)     0.0275 (1.0)      2.8502 (1.48)     0.0425 (1.30)          1;0  0.3526 (0.68)          5           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------------- benchmark 'polars': 3 tests -----------------------------------------------------------------------------------
Name (time in ms)            Min                   Max                  Mean            StdDev                Median               IQR            Outliers       OPS            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ipl_polars           3.7290 (1.0)          4.7155 (1.0)          4.1126 (1.0)      0.3915 (1.0)          4.0305 (1.0)      0.5579 (1.03)          1;0  243.1547 (1.0)           5           1
test_ivp_polars         342.8622 (91.95)      343.8852 (72.93)      343.5144 (83.53)    0.4073 (1.04)       343.5705 (85.24)    0.5405 (1.0)           1;0    2.9111 (0.01)          5           1
test_sch_polars       1,129.2793 (302.84)   1,142.4447 (242.27)   1,132.9314 (275.48)   5.4005 (13.80)    1,130.6117 (280.51)   4.4374 (8.21)          1;1    0.8827 (0.00)          5           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

--------------------------------------------------------------------------------------- benchmark 'single': 2 tests ----------------------------------------------------------------------------------------
Name (time in ns)            Min                    Max                  Mean              StdDev                Median                 IQR             Outliers  OPS (Kops/s)            Rounds  Iterations
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_iban           120.0000 (1.0)         486.2500 (1.0)        140.2897 (1.0)        7.2469 (1.0)        139.1700 (1.0)        5.8300 (1.0)     11086;3735    7,128.1068 (1.0)       81084         100
test_sch_iban         5,250.0000 (43.75)    32,000.0000 (65.81)    6,099.4756 (43.48)    318.1919 (43.91)    6,084.0000 (43.72)    123.9999 (21.27)    7433;8795      163.9485 (0.02)     188965           1
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

For the details look in the test file in the iban_validation_bench_py package.
This report may not be updated for each release, it is more to give a general overview, users of hte library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [Pypi](https://pypi.org/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
