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
test_ivp_pandas        1.9345 (1.0)      2.0246 (1.0)      1.9849 (1.0)      0.0331 (1.0)      1.9837 (1.0)      0.0373 (1.0)           2;0  0.5038 (1.0)           5           1
test_sch_pandas        2.8541 (1.48)     2.9487 (1.46)     2.8905 (1.46)     0.0363 (1.10)     2.8756 (1.45)     0.0426 (1.14)          2;0  0.3460 (0.69)          5           1
test_stdnum_pandas     3.0921 (1.60)     3.1975 (1.58)     3.1536 (1.59)     0.0516 (1.56)     3.1811 (1.60)     0.0944 (2.53)          1;0  0.3171 (0.63)          5           1
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

----------------------------------------------------------------------------------- benchmark 'polars': 4 tests -----------------------------------------------------------------------------------
Name (time in ms)             Min                   Max                  Mean            StdDev                Median               IQR            Outliers       OPS            Rounds  Iterations
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ipl_polars            2.5323 (1.0)          2.8884 (1.0)          2.6876 (1.0)      0.0649 (1.0)          2.6880 (1.0)      0.0874 (1.0)          65;1  372.0744 (1.0)         204           1
test_ivp_polars          339.9341 (134.24)     357.7403 (123.86)     347.5845 (129.33)   6.5836 (101.45)     347.6349 (129.33)   7.5319 (86.22)         2;0    2.8770 (0.01)          5           1
test_sch_polars        1,147.4601 (453.13)   1,161.4929 (402.13)   1,156.4574 (430.29)   5.2979 (81.64)    1,157.6732 (430.68)   4.4043 (50.42)         1;1    0.8647 (0.00)          5           1
test_stdnum_polars     1,414.9822 (558.78)   1,423.9784 (493.00)   1,419.5266 (528.17)   4.1052 (63.26)    1,421.6250 (528.87)   7.0543 (80.76)         3;0    0.7045 (0.00)          5           1
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------------------- benchmark 'single': 3 tests ----------------------------------------------------------------------------------------
Name (time in ns)            Min                    Max                  Mean              StdDev                Median                 IQR              Outliers  OPS (Kops/s)            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_ivp_iban           118.7483 (1.0)         417.5003 (1.0)        136.4646 (1.0)        9.6953 (1.0)        136.6693 (1.0)        6.2468 (1.0)     18953;12708    7,327.9088 (1.0)       83051         100
test_sch_iban         5,292.0077 (44.56)    81,457.8962 (195.11)   6,167.7274 (45.20)    472.4557 (48.73)    6,165.8211 (45.11)    125.0301 (20.01)    8347;11514      162.1343 (0.02)     184635           1
test_stdnum_iban      7,583.0612 (63.86)    50,334.0270 (120.56)   8,864.5184 (64.96)    660.4401 (68.12)    8,750.0084 (64.02)    167.6381 (26.84)   27315;28606      112.8093 (0.02)     131873           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

For the details look in the test file in the iban_validation_bench_py package.
This report may not be updated for each release, it is more to give a general overview, users of the library should benchmark the crate in scenario relevant for their use case. 

The crates selected were found by looking for "Iban" on [Pypi](https://pypi.org/), filtered to the ones with similar feature as this library. 
If there is any issue please to report it. 
