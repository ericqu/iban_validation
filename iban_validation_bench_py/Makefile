# OS Specific command
ifeq ($(OS),Windows_NT)
	VENV_DIR := .venv
	VENV_BIN := .venv/Scripts
	RMRF := rm -rf
else
	VENV_DIR := .venv
	VENV_BIN := .venv/bin
	RMRF := rm -rf
endif

define create_venv
	$(RMRF) $(VENV_DIR)
	uv venv $(VENV_DIR)
	uv pip install --upgrade --compile-bytecode --no-build -r py_requirements.txt
endef

.PHONY: prep_py_benches
prep_py_benches:
	$(call create_venv)
	uv run gen_testfile.py

.PHONY: test_iban_validation_py
test_iban_validation_py:
	uv run pytest --benchmark-only --benchmark-group-by=group 