import os
import pre_process_registry as ppr

def test_preprocess():
    inputfile = "iban_validation_preprocess/iban_registry_v101.txt"
    output_iban_file = "iban_validation_preprocess/test_out.json"

    df = ppr.get_df_from_input(inputfile)

    ctryl = df["ctry_cd"].to_list()

    assert 104 == len(ctryl)

    try:
        os.remove(output_iban_file)
    except OSError:
        pass
