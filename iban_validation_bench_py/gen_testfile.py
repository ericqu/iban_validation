import polars as pl

inputfile = r'../iban_validation_rs/data/IBAN Examples.txt'
outputfile = r"data/test_file.csv"

# generate a csv file for testing
df = pl.read_csv(inputfile).sample(100_000, with_replacement=True)
df.write_csv(outputfile)
print("writing to file complete")