

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
	$(MAKE) requirements

.PHONY: requirements
requirements:	.venv
	$(VENV_BIN)/python -m pip install --upgrade uv
	$(VENV_BIN)/uv pip install --upgrade --compile-bytecode --no-build -r requirements-python.txt


.PHONY: registry_preprocess
registry_preprocess:
	$(MAKE) requirements
	$(VENV_BIN)/python registry_preprocess/pre_process_registry.py

.PHONY: core_iban_valid
core_iban_valid:
	cargo build -p core_iban_valid

.PHONY: core_iban_valid_release
core_iban_valid_release:
	cargo build -p core_iban_valid -r

.PHONY: clean
clean:
	cargo clean
	rm -rf .venv/

.PHONY: python_wrapper
python_wrapper:
	$(MAKE) requirements
	cd python_wrapper
	$(VENV_BIN)/maturin develop -m python_wrapper/Cargo.toml

.PHONY: python_wrapper_release
python_wrapper_release:
	$(MAKE) requirements
	cd python_wrapper
	$(VENV_BIN)/maturin develop -m python_wrapper/Cargo.toml --release

.PHONY: build_python_wrapper
build_python_wrapper:
	$(MAKE) requirements
	cd python_wrapper
	$(VENV_BIN)/maturin build -m python_wrapper/Cargo.toml

.PHONY: build_python_wrapper_release
build_python_wrapper_release:
	$(MAKE) requirements
	cd python_wrapper
	$(VENV_BIN)/maturin build -m python_wrapper/Cargo.toml --release

.PHONY: build_polars_plugin
build_polars_plugin:
	$(MAKE) requirements
	$(VENV_BIN)/maturin build -m polars_plugin/Cargo.toml 

.PHONY: build_polars_plugin_release
build_polars_plugin_release:
	$(MAKE) requirements
	$(VENV_BIN)/maturin build -m polars_plugin/Cargo.toml --release

.PHONY: test
test:
	$(MAKE) requirements
	cargo test
	$(VENV_BIN)/maturin develop -m polars_plugin/Cargo.toml 
	$(VENV_BIN)/pytest polars_plugin/tests/test_polars_plugin.py



