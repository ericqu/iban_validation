import polars as pl
from iban_validation_polars import process_ibans
import time

import os

pl.Config.set_tbl_rows(5)

inputfile = r"iban_validation_rs/data/IBAN Examples.txt"
generatedfile = r"iban_validation_polars/examples/test_file.csv"
sample_size = 100_000_000

# generate a csv file for testing
df = pl.read_csv(inputfile).sample(sample_size, with_replacement=True)
df.write_csv(generatedfile)
print("writing to file complete")

start = time.perf_counter()
df = (
    pl.scan_csv(generatedfile)
    .with_columns(
        validated=process_ibans("IBAN Examples")
        .str.split_exact(",", 2)
        .struct.rename_fields(["valid_ibans", "bank_id", "branch_id"])
    )
    .unnest("validated")
    # .sort(by="IBAN Examples", descending=True)
)
# trigger the processing
print(df.collect(engine='streaming'))
duration = time.perf_counter() - start
print(f'process_ibans for {sample_size} took {duration:.6f}')

# cleanup
os.remove(generatedfile)


def sample_with_condition(csv_file, sample_size, random_state=None):
    df = pl.read_csv(csv_file)
    
    target_rows = df.filter(pl.col('IBAN Examples') == 'DE75512108001245126199')
    other_rows = df.filter(pl.col('IBAN Examples') != 'DE75512108001245126199')
    
    n_from_target = int(sample_size * 0.8)
    n_from_others = sample_size - n_from_target
    
    sampled_target = target_rows.sample(n=n_from_target, with_replacement=True, seed=random_state)
    sampled_others = other_rows.sample(n=n_from_others, with_replacement=True, seed=random_state)
    
    # Just concatenate - already at sample_size, no extra memory
    return pl.concat([sampled_target, sampled_others], how="vertical").sample(n=sample_size, with_replacement=True)

sample_with_condition(inputfile, sample_size).write_csv(generatedfile)

start = time.perf_counter()
df = (
    pl.scan_csv(generatedfile)
    .with_columns(
        validated=process_ibans("IBAN Examples")
        .str.split_exact(",", 2)
        .struct.rename_fields(["valid_ibans", "bank_id", "branch_id"])
    )
    .unnest("validated")
    # .sort(by="IBAN Examples", descending=True)
)
# trigger the processing
print(df.collect(engine='streaming'))
duration = time.perf_counter() - start
print(f'process_ibans 80/20 for {sample_size} took {duration:.6f}')

# cleanup
os.remove(generatedfile)



df = pl.read_csv(inputfile).sample(sample_size, with_replacement=True)
df.write_csv(generatedfile)
print("writing to file complete")

start = time.perf_counter()
df = (
    pl.scan_csv(generatedfile)
    .with_columns(
        validated=process_ibans("IBAN Examples")
        .str.split_exact(",", 2)
        .struct.rename_fields(["valid_ibans", "bank_id", "branch_id"])
    )
    .unnest("validated")
    # .sort(by="IBAN Examples", descending=True)
)
# trigger the processing
print(df.collect(engine='streaming'))
duration = time.perf_counter() - start
print(f'process_ibans for {sample_size} took {duration:.6f}')

# cleanup
os.remove(generatedfile)
