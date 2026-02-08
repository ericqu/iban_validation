## Benchmark iban_validation_py and iban_validation_polars against similar libraries
To give a perspective on how the python wrapper performs with regards to other similar libraries. The other libraries can have additional features that iban_validation_py does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

## Outcome
I tested only [schiwty](https://github.com/mdomke/schwifty) as this is the most prominent Python library, and also [python-stdnum](https://arthurdejong.org/python-stdnum/)
In the context of a single call through the python api, the iban_validation_py package is about 50 times faster.
In the context of calls through the Pandas dataframe, the iban_validation_py package is only 1.5 times faster.
In the context of calls through the Polars dataframe, the iban_validation_py package is about 3 times faster.
In the context of calls through the Polars dataframe, but using the iban_validation_polars plugin, then the plugin is about 100 times faster than the iban_validation_py, and about 400 times faster than schwifty. Which is where the real gain is, and the reason why the polars plugin exists.

Here is the output from pytest:
```
-------------------------------------------------------------------------- benchmark 'pandas': 3 tests --------------------------------------------------------------------------
Name (time in s)          Min               Max              Mean            StdDev            Median               IQR            Outliers     OPS            Rounds  Iterations
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_pandas        3.3086 (1.0)      3.4751 (1.0)      3.3832 (1.0)      0.0644 (1.65)     3.3891 (1.0)      0.0924 (2.66)          2;0  0.2956 (1.0)           5           1
test_sch_pandas        4.2881 (1.30)     4.3911 (1.26)     4.3545 (1.29)     0.0391 (1.0)      4.3654 (1.29)     0.0348 (1.0)           1;1  0.2296 (0.78)          5           1
test_stdnum_pandas     4.5365 (1.37)     4.7365 (1.36)     4.6418 (1.37)     0.0758 (1.94)     4.6353 (1.37)     0.1054 (3.03)          2;0  0.2154 (0.73)          5           1
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

------------------------------------------------------------------------------------ benchmark 'polars': 4 tests ------------------------------------------------------------------------------------
Name (time in ms)             Min                   Max                  Mean             StdDev                Median                IQR            Outliers       OPS            Rounds  Iterations
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ipl_polars            4.6265 (1.0)          5.7368 (1.0)          4.9285 (1.0)       0.1246 (1.0)          4.9084 (1.0)       0.0482 (1.0)         15;21  202.9014 (1.0)         137           1
test_ivp_polars          266.5875 (57.62)      314.5044 (54.82)      279.1138 (56.63)    20.1512 (161.70)     270.5945 (55.13)    18.6382 (386.37)        1;1    3.5828 (0.02)          5           1
test_sch_polars        1,051.1647 (227.21)   1,089.2430 (189.87)   1,064.9964 (216.09)   15.6890 (125.89)   1,057.4526 (215.44)   22.2621 (461.49)        1;0    0.9390 (0.00)          5           1
test_stdnum_polars     1,311.0402 (283.38)   1,345.2390 (234.49)   1,322.3831 (268.31)   13.6569 (109.58)   1,316.1482 (268.14)   15.4392 (320.05)        1;0    0.7562 (0.00)          5           1
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------------------- benchmark 'single': 3 tests -----------------------------------------------------------------------------------------
Name (time in ns)            Min                     Max                  Mean              StdDev                Median                 IQR              Outliers  OPS (Kops/s)            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_iban           108.7505 (1.0)          608.7497 (1.0)        124.2803 (1.0)       10.5288 (1.0)        125.8403 (1.0)        9.1689 (1.0)      14410;2034    8,046.3251 (1.0)       90227         100
test_sch_iban         5,250.3310 (48.28)     41,792.1692 (68.65)    6,162.7907 (49.59)    349.8125 (33.22)    6,165.8211 (49.00)    125.7285 (13.71)    6367;11676      162.2642 (0.02)     186043           1
test_stdnum_iban      7,457.5655 (68.57)    100,790.9887 (165.57)   8,529.6988 (68.63)    587.8684 (55.83)    8,583.0688 (68.21)    124.7972 (13.61)   13486;16373      117.2374 (0.01)     134842           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

For the details look in the test file in the iban_validation_bench_py package.
This report may not be updated for each release, it is more to give a general overview, users of the library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [Pypi](https://pypi.org/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
