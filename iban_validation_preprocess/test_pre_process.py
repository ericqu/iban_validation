# import polars as pl
import os
import pre_process_registry as ppr


def test_preprocess():
    inputfile = 'iban_validation_preprocess/iban_registry_v98.txt'
    output_iban_file = 'iban_validation_preprocess/test_out.json'

    df = ppr.pre_process(inputfile, output_iban_file)

    target_list = ['AD', 'AE', 'AL', 'AT', 'AZ', 'BA', 'BE', 'BG', 'BH', 'BI', 'BR', 'BY', 'CH', 'CR', 'CY', 'CZ', 'DE', 'DJ', 'DK', 'DO', 'EE', 'EG', 'ES', 'FI', 'FK', 'FO', 'FR', 'GB', 'GE', 'GI', 'GL', 'GR', 'GT', 'HR', 'HU', 'IE', 'IL', 'IQ', 'IS', 'IT', 'JO', 'KW', 'KZ', 'LB', 'LC', 'LI', 'LT', 'LU', 'LV', 'LY', 'MC', 'MD', 'ME', 'MK', 'MN', 'MR', 'MT', 'MU', 'NI', 'NL', 'NO', 'OM', 'PL', 'PS', 'PT', 'QA', 'RO', 'RS', 'RU', 'SA', 'SC', 'SD', 'SE', 'SI', 'SK', 'SM', 'SO', 'ST', 'SV', 'TL', 'TN', 'TR', 'UA', 'VA', 'VG', 'XK', 'YE']
    ctryl = df['ctry_cd'].to_list()

    assert (target_list == ctryl)

    try:
        os.remove(output_iban_file)
    except OSError:
        pass
