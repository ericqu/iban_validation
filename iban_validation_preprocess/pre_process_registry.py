import polars as pl
pl.Config.set_tbl_cols(15)
pl.Config.set_tbl_rows(45)
inputfile = 'iban_validation_preprocess/iban_registry_v98.txt'
output_iban_file = 'iban_validation_rs/data/iban_definitions.json'
output_source_file = 'iban_validation_rs/data/iban_sourcefile.txt'

def pre_process_filename(inputfile, output_source_file) :
    from pathlib import Path
    file_path = Path(inputfile)
    filename = file_path.name     # Get the filename from the path
    with open(output_source_file, "w") as f:
        f.write(filename)


def pre_process(inputfile, output_iban_file):
    df = pl.scan_csv(inputfile, separator='\t', quote_char='"', n_rows=25)

    header = df.select(df.collect_schema().names()[0]).collect().to_series().to_list()        
    df = df.collect().transpose(include_header=False, column_names=header).slice(1)

    # preprocess and check iban structure
    def process_iban_structure(i_structure_e: str):
        iso3166 = i_structure_e[0:2]
        i = 2
        next_exclamation = 0 
        while(i_structure_e.find('!',i) > 0):
            next_exclamation = i_structure_e.find('!',i)
            num = int(i_structure_e[i:next_exclamation])
            letter = str(i_structure_e[next_exclamation+1:next_exclamation+2])
            iso3166 = str(iso3166) + num * letter
            i = next_exclamation + 2
        return iso3166

    pre_df = df.with_columns(
                    pl.col('IBAN structure').map_elements(process_iban_structure, return_dtype=pl.String).alias('iban_struct'),
                    pl.when(pl.col('IBAN prefix country code (ISO 3166)') == 'JO')\
                        .then(pl.lit('1-4')).otherwise(pl.col('Bank identifier position within the BBAN').str.strip_chars()).alias('Bank identifier position within the BBAN'),
                    pl.col('Branch identifier position within the BBAN').str.split_exact(by='-',n=1).struct.rename_fields(['branch_id_pos_s','branch_id_pos_e'])\
                        .alias('fields').struct.unnest(),
                    pl.col('IBAN length').cast(pl.UInt16))\
            .with_columns(
                pl.col('Bank identifier position within the BBAN').str.slice(0,1).str.to_integer(strict=False).cast(pl.UInt16).alias('bank_id_pos_s'),
                pl.col('Bank identifier position within the BBAN').str.slice(2).str.to_integer(strict=False).cast(pl.UInt16).alias('bank_id_pos_e'),
                pl.col('branch_id_pos_s').str.to_integer(strict=False).cast(pl.UInt16),
                pl.col('branch_id_pos_e').str.to_integer(strict=False).cast(pl.UInt16),
                )\
            .with_columns(
                pl.col('IBAN electronic format example').str.slice(3+pl.col('bank_id_pos_s'), pl.col('bank_id_pos_e')+1-pl.col('bank_id_pos_s')).alias('bank_id')
            ).rename({'IBAN prefix country code (ISO 3166)':'ctry_cd', 'IBAN length':'iban_len'})\
            .with_columns(
                 pl.col('ctry_cd').map_elements(lambda x: [ord(c) for c in x], return_dtype=pl.List(pl.UInt16))
            )\
            .select(['ctry_cd', 'iban_len', 'bank_id_pos_s','bank_id_pos_e', 'branch_id_pos_s', 'branch_id_pos_e', 'iban_struct' ])
    
    pre_df.write_json(output_iban_file)
    print(f'preprocessing: {inputfile} -> completed', )
    print(pre_df['ctry_cd'])
    return pre_df

if __name__ == "__main__":
    pre_process(inputfile, output_iban_file)
    pre_process_filename(inputfile, output_source_file)
