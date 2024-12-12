

ifeq ($(OS),Windows_NT)
	VENV_BIN=.venv/Scripts
else
	VENV_BIN=.venv/bin
endif

# Detect CPU architecture.
ifeq ($(OS),Windows_NT)
    ifeq ($(PROCESSOR_ARCHITECTURE),AMD64)
		ARCH := amd64
	else ifeq ($(PROCESSOR_ARCHITECTURE),x86)
		ARCH := x86
	else ifeq ($(PROCESSOR_ARCHITECTURE),ARM64)
		ARCH := arm64
	else
		ARCH := unknown
    endif
else
    UNAME_P := $(shell uname -p)
    ifeq ($(UNAME_P),x86_64)
		ARCH := amd64
	else ifneq ($(filter %86,$(UNAME_P)),)
		ARCH := x86
	else ifneq ($(filter arm%,$(UNAME_P)),)
		ARCH := arm64
	else
		ARCH := unknown
    endif
endif

.venv:	# Setup Python virtual env
	python3 -m venv .venv

.PHONY: requirements
requirements:	.venv
	$(VENV_BIN)/python -m pip install --upgrade uv
	$(VENV_BIN)/uv pip install --upgrade --compile-bytecode --no-build -r requirements-python.txt


.PHONY: iban_validation_preprocess
iban_validation_preprocess:
	$(MAKE) requirements
	$(VENV_BIN)/python iban_validation_preprocess/pre_process_registry.py

.PHONY: iban_validation_core
iban_validation_core:
	cargo build -p iban_validation_core

.PHONY: iban_validation_core_release
iban_validation_core_release:
	cargo build -p iban_validation_core -r

.PHONY: clean
clean:
	cargo clean
	rm -rf .venv/
	rm -rf .pytest_cache/

.PHONY: iban_validation_py
iban_validation_py:
	$(MAKE) requirements
	cd iban_validation_py
	$(VENV_BIN)/maturin develop -m iban_validation_py/Cargo.toml

.PHONY: iban_validation_py_release
iban_validation_py_release:
	$(MAKE) requirements
	cd iban_validation_py
	$(VENV_BIN)/maturin develop -m iban_validation_py/Cargo.toml --release

.PHONY: build_iban_validation_py
build_iban_validation_py:
	$(MAKE) requirements
	cd iban_validation_py
	$(VENV_BIN)/maturin build -m iban_validation_py/Cargo.toml

.PHONY: build_iban_validation_py_release
build_iban_validation_py_release:
	$(MAKE) requirements
	cd iban_validation_py
	$(VENV_BIN)/maturin build -m iban_validation_py/Cargo.toml --release

.PHONY: build_iban_validation_polars
build_iban_validation_polars:
	$(MAKE) requirements
	$(VENV_BIN)/maturin build -m iban_validation_polars/Cargo.toml 

.PHONY: build_iban_validation_polars_release
build_iban_validation_polars_release:
	$(MAKE) requirements
	$(VENV_BIN)/maturin build -m iban_validation_polars/Cargo.toml --release

.PHONY: test
test:
	$(MAKE) requirements
	cargo test
	$(VENV_BIN)/maturin develop -m iban_validation_polars/Cargo.toml
	$(VENV_BIN)/maturin develop -m iban_validation_py/Cargo.toml
	$(VENV_BIN)/pytest

