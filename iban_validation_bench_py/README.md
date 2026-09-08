## Benchmark iban_validation_py and iban_validation_polars against similar libraries
To give a perspective on how the python wrapper performs with regards to other similar libraries. The other libraries can have additional features that iban_validation_py does not have. Only creating an Iban structure with the validated iban, the bank identifier when present and the branch identifier when present is test.

## Outcome
I tested only [schiwty](https://github.com/mdomke/schwifty) as this is the most prominent Python library, and also [python-stdnum](https://arthurdejong.org/python-stdnum/)
In the context of a single call through the python api, the iban_validation_py package is about 48 times faster.
In the context of calls through the Pandas dataframe, the iban_validation_py package is only 1.3 times faster.
In the context of calls through the Polars dataframe, the iban_validation_py package is about 3.6 times faster.
In the context of calls through the Polars dataframe, but using the iban_validation_polars plugin, then the plugin is about 61 times faster than the iban_validation_py, and about 220 times faster than schwifty. Which is where the real gain is, and the reason why the polars plugin exists.

Here is the output from pytest:
```
-------------------------------------------------------------------------- benchmark 'pandas': 3 tests --------------------------------------------------------------------------
Name (time in s)          Min               Max              Mean            StdDev            Median               IQR            Outliers     OPS            Rounds  Iterations
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_pandas        3.4417 (1.0)      3.5311 (1.0)      3.4974 (1.0)      0.0369 (1.0)      3.5031 (1.0)      0.0568 (1.81)          1;0  0.2859 (1.0)           5           1
test_sch_pandas        4.3691 (1.27)     4.4741 (1.27)     4.4338 (1.27)     0.0388 (1.05)     4.4425 (1.27)     0.0315 (1.0)           2;1  0.2255 (0.79)          5           1
test_stdnum_pandas     4.6211 (1.34)     4.7750 (1.35)     4.6933 (1.34)     0.0568 (1.54)     4.6884 (1.34)     0.0721 (2.29)          2;0  0.2131 (0.75)          5           1
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

------------------------------------------------------------------------------------ benchmark 'polars': 4 tests ------------------------------------------------------------------------------------
Name (time in ms)             Min                   Max                  Mean             StdDev                Median                IQR            Outliers       OPS            Rounds  Iterations
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ipl_polars            4.9259 (1.0)          8.1741 (1.0)          5.3362 (1.0)       0.5413 (1.0)          5.1493 (1.0)       0.3238 (1.0)         10;10  187.3998 (1.0)         123           1
test_ivp_polars          311.4053 (63.22)      337.1680 (41.25)      326.1195 (61.11)    11.2825 (20.84)      328.3845 (63.77)    20.0442 (61.90)         1;0    3.0664 (0.02)          5           1
test_sch_polars        1,153.5320 (234.18)   1,205.8719 (147.52)   1,176.0603 (220.39)   21.1509 (39.08)    1,174.1092 (228.02)   33.6291 (103.86)        2;0    0.8503 (0.00)          5           1
test_stdnum_polars     1,416.8073 (287.62)   1,434.1973 (175.46)   1,426.6174 (267.35)    7.8888 (14.57)    1,430.8995 (277.89)   13.5207 (41.76)         1;0    0.7010 (0.00)          5           1
-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------------------- benchmark 'single': 3 tests ----------------------------------------------------------------------------------------
Name (time in ns)            Min                     Max                  Mean              StdDev                Median                 IQR             Outliers  OPS (Kops/s)            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_iban           117.9101 (1.0)        1,016.6608 (1.0)        135.4836 (1.0)        8.3646 (1.0)        134.5901 (1.0)        5.0000 (1.0)      4836;3375    7,380.9698 (1.0)       76676         100
test_sch_iban         5,665.9337 (48.05)    104,084.0289 (102.38)   6,519.1654 (48.12)    440.6022 (52.67)    6,499.9331 (48.29)    125.0301 (25.01)    1590;6701      153.3939 (0.02)     177780           1
test_stdnum_iban      7,583.9926 (64.32)     46,708.0390 (45.94)    8,764.9538 (64.69)    422.4834 (50.51)    8,750.0084 (65.01)    125.0301 (25.01)    2853;4171      114.0907 (0.02)     129719           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

For the details look in the test file in the iban_validation_bench_py package.
This report may not be updated for each release, it is more to give a general overview, users of the library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [Pypi](https://pypi.org/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
