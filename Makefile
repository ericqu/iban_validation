DIST_DIR ?= dist
C_WRAPPER_DIR := iban_validation_c
DIST_C_DIR := $(DIST_DIR)/c
DIST_WHL_DIR := $(DIST_DIR)/whl

# OS Specific command
ifeq ($(OS),Windows_NT)
	VENV_DIR := .venv
	VENV_BIN := .venv/Scripts
	RMRF := rm -rf
	MVF := move /Y
else
	VENV_DIR := .venv
	VENV_BIN := .venv/bin
	RMRF := rm -rf
	MVF := mv -f
endif

# Cross-compilation targets
MACOS_TARGETS := aarch64-apple-darwin x86_64-apple-darwin
LINUX_TARGETS := x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu
WINDOWS_TARGETS := x86_64-pc-windows-msvc
PYTHON_VERSIONS := 3.9 3.10 3.11 3.12 3.13

# Create a virtual environment
define create_venv
	$(RMRF) $(VENV_DIR)
	uv venv $(VENV_DIR)
	uv pip install --upgrade --compile-bytecode --no-build -r requirements-python.txt
endef

# Create a virtual environment for a specific platform
define create_venv_py
	$(RMRF) $(VENV_DIR)
	uv venv $(VENV_DIR) --python $(1)
	uv pip install --upgrade --compile-bytecode --no-build -r requirements-python.txt
endef

.PHONY: iban_validation_preprocess
iban_validation_preprocess:
	$(call create_venv)
	$(VENV_BIN)/python iban_validation_preprocess/pre_process_registry.py

.PHONY: iban_validation_rs_release
iban_validation_rs_release:	iban_validation_preprocess clippy
	cargo build -p iban_validation_rs -r

.PHONY: clean
clean:
	rustup update
	rustup component add llvm-tools-preview
	cargo clean
	$(MAKE) clean_wheels
	$(RMRF) .pytest_cache
	$(RMRF) .venv
	$(RMRF) target
	$(RMRF) $(DIST_DIR)

.PHONY: clean_wheels
clean_wheels:
	@echo "Cleaning wheels, distribution files, and Python extension modules..."
ifeq ($(OS),Windows_NT)
	powershell -Command "Get-ChildItem -Path . -Recurse -Include *.whl,*.tar.gz,*.pyd | Remove-Item -Force"
else
	find . -type f \( -name "*.whl" -o -name "*.tar.gz" -o -name "*.so" \) -delete
endif

.PHONY: iban_validation_wasm
iban_validation_wasm: iban_validation_rs_release
	wasm-pack build iban_validation_wasm --target web --out-dir ../docs/pkg --release
	cp iban_validation_wasm/index.html docs/
	wasm-pack build iban_validation_wasm --target bundler --release


.PHONY: iban_validation_c
iban_validation_c:
	cargo build -p $(C_WRAPPER_DIR) --release

.PHONY: iban_validation_c_release
iban_validation_c_release: iban_validation_rs_release
	$(call create_venv)
	mkdir -p $(DIST_C_DIR)
# current machine
	cargo build -p $(C_WRAPPER_DIR) --release ;\
	mkdir -p $(DIST_C_DIR) ;\
	cp target/release/lib$(C_WRAPPER_DIR).a $(DIST_C_DIR) ;\
	cp target/release/lib$(C_WRAPPER_DIR).dylib $(DIST_C_DIR) ;\
	cp $(C_WRAPPER_DIR)/include/*.h $(DIST_C_DIR)/ ;
# macos gnu
	rustup target add aarch64-apple-darwin
	cargo build -p $(C_WRAPPER_DIR) --release --target aarch64-apple-darwin
	mkdir -p $(DIST_C_DIR)/aarch64-apple-darwin
	cp target/aarch64-apple-darwin/release/lib$(C_WRAPPER_DIR).a $(DIST_C_DIR)/aarch64-apple-darwin/
	cp target/aarch64-apple-darwin/release/lib$(C_WRAPPER_DIR).dylib $(DIST_C_DIR)/aarch64-apple-darwin/ || true
	cp $(C_WRAPPER_DIR)/include/*.h $(DIST_C_DIR)/aarch64-apple-darwin/
# linux gnu
	rustup target add x86_64-unknown-linux-gnu
	cargo build -p $(C_WRAPPER_DIR) --release --target x86_64-unknown-linux-gnu
	mkdir -p $(DIST_C_DIR)/x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/release/lib$(C_WRAPPER_DIR).a $(DIST_C_DIR)/x86_64-unknown-linux-gnu/
	cp target/x86_64-unknown-linux-gnu/release/lib$(C_WRAPPER_DIR).so $(DIST_C_DIR)/x86_64-unknown-linux-gnu/ || true
	cp $(C_WRAPPER_DIR)/include/*.h $(DIST_C_DIR)/x86_64-unknown-linux-gnu/
# build-windows:
	rustup target add x86_64-pc-windows-gnu
	cargo build -p $(C_WRAPPER_DIR) --release --target x86_64-pc-windows-gnu
	mkdir -p $(DIST_C_DIR)/x86_64-pc-windows-gnu
	cp target/x86_64-pc-windows-gnu/release/lib$(C_WRAPPER_DIR).a $(DIST_C_DIR)/x86_64-pc-windows-gnu/
	cp target/x86_64-pc-windows-gnu/release/$(C_WRAPPER_DIR).dll $(DIST_C_DIR)/x86_64-pc-windows-gnu/ || true
	cp $(C_WRAPPER_DIR)/include/*.h $(DIST_C_DIR)/x86_64-pc-windows-gnu/

.PHONY: iban_validation_c_examples
iban_validation_c_examples: iban_validation_c_release
	cc iban_validation_c/examples/example.c -o iban_validation_c/examples/example_c -liban_validation_c -L./$(DIST_C_DIR)
	cc iban_validation_c/examples/bench.c -o iban_validation_c/examples/bench_c -liban_validation_c -L./$(DIST_C_DIR)
	cc iban_validation_c/examples/tests.c -o iban_validation_c/examples/tests_c -liban_validation_c -L./$(DIST_C_DIR)
	g++ iban_validation_c/examples/example.cpp -o iban_validation_c/examples/example_cpp -liban_validation_c -L./$(DIST_C_DIR)

.PHONY: iban_validation_py
iban_validation_py:
	$(call create_venv)
	$(VENV_BIN)/maturin develop -m iban_validation_py/Cargo.toml

.PHONY: iban_validation_py_release
iban_validation_py_release:	clippy
	$(call create_venv)
	$(VENV_BIN)/maturin develop -m iban_validation_py/Cargo.toml --release

.PHONY: iban_validation_polars_release
iban_validation_polars_release:	clippy
	$(call create_venv)
	$(VENV_BIN)/maturin develop -m iban_validation_polars/Cargo.toml --release 

.PHONY: build_iban_validation_py_release
build_iban_validation_py_release:	clippy
	$(call create_venv)
	$(VENV_BIN)/maturin sdist -m iban_validation_py/Cargo.toml --out $(DIST_WHL_DIR)
	$(foreach target,$(MACOS_TARGETS),\
		$(foreach pyver,$(PYTHON_VERSIONS),\
			$(call create_venv_py, $(pyver)) ;\
			$(VENV_BIN)/uv run --python $(pyver) python -m maturin build -m iban_validation_py/Cargo.toml --release --strip --target $(target) --out $(DIST_WHL_DIR) ;\
		)\
	)
	$(foreach target,$(LINUX_TARGETS),\
		$(foreach pyver,$(PYTHON_VERSIONS),\
			$(call create_venv_py, $(pyver)) ;\
			$(VENV_BIN)/uv run --python $(pyver) python -m maturin build -m iban_validation_py/Cargo.toml --release -i python$(pyver) --strip --target $(target) --manylinux 2014 --zig --out $(DIST_WHL_DIR) ;\
		)\
	)
	$(foreach target,$(WINDOWS_TARGETS),\
		$(foreach pyver,$(PYTHON_VERSIONS),\
			$(call create_venv_py, $(pyver)) ;\
			$(VENV_BIN)/uv run --python $(pyver) python -m maturin build -m iban_validation_py/Cargo.toml --release -i python$(pyver) --strip --target $(target) --out $(DIST_WHL_DIR) ;\
		)\
	)

.PHONY: build_iban_validation_polars_release
build_iban_validation_polars_release:	clippy
ifeq ($(OS),Windows_NT)
	powershell -Command "Remove-Item -Path iban_validation_polars\*.pyd -Force"
endif
	$(call create_venv)
	$(VENV_BIN)/maturin sdist -m iban_validation_polars/Cargo.toml --out $(DIST_WHL_DIR)
	$(foreach target,$(MACOS_TARGETS),\
		$(foreach pyver,$(PYTHON_VERSIONS),\
			$(call create_venv_py, $(pyver)) ;\
			$(VENV_BIN)/uv run --python $(pyver) python -m maturin build -m iban_validation_polars/Cargo.toml --release --strip --target $(target) --out $(DIST_WHL_DIR) ;\
		)\
	)
	$(foreach target,$(LINUX_TARGETS),\
		$(foreach pyver,$(PYTHON_VERSIONS),\
			$(call create_venv_py, $(pyver)) ;\
			$(VENV_BIN)/uv run --python $(pyver) python -m maturin build -m iban_validation_polars/Cargo.toml --release -i python$(pyver) --strip --target $(target) --manylinux 2014 --zig --out $(DIST_WHL_DIR) ;\
		)\
	)
	$(foreach target,$(WINDOWS_TARGETS),\
		$(foreach pyver,$(PYTHON_VERSIONS),\
			$(call create_venv_py, $(pyver)) ;\
			$(VENV_BIN)/uv run --python $(pyver) python -m maturin build -m iban_validation_polars/Cargo.toml --release -i python$(pyver) --strip --target $(target) --out $(DIST_WHL_DIR) ;\
		)\
	)

.PHONY: publish_iban_validation_rs
publish_iban_validation_rs: test
	cargo doc
	cargo publish -p iban_validation_rs 

.PHONY: test
test:	clippy iban_validation_preprocess iban_validation_wasm
	cargo test
	cargo test -p iban_validation_c
	$(call create_venv)
	$(VENV_BIN)/maturin develop -m iban_validation_polars/Cargo.toml
	$(VENV_BIN)/maturin develop -m iban_validation_py/Cargo.toml
	$(VENV_BIN)/pytest --ignore=iban_validation_bench_py

.PHONY: coverage
coverage:
	cargo llvm-cov --html

.PHONY: clippy
clippy:
	cargo update
	cargo fmt -p iban_validation_rs
	cargo fmt -p iban_validation_c
	cargo fmt -p iban_validation_py
	cargo fmt -p iban_validation_polars
	cargo fmt -p iban_validation_bench_rs
	cargo clippy -p iban_validation_rs
	cargo clippy -p iban_validation_c
	cargo clippy -p iban_validation_py
	cargo clippy -p iban_validation_polars
	cargo clippy -p iban_validation_bench_rs

# only manual when local dist is filled with the artifacts
.PHONY: publishing_pipy
publishing_pipy:
	$(VENV_BIN)/python3 -m twine upload $(DIST_WHL_DIR)/* --verbose

# only manual when local dist is filled with the artifacts
.PHONY: publishing_testpipy
publishing_testpipy:
	$(VENV_BIN)/python3 -m twine upload --repository-url https://test.pypi.org/legacy/ $(DIST_WHL_DIR)/* --verbose

# to execute the bench against other libraries
.PHONY: iban_validation_bench_rs
iban_validation_bench_rs: iban_validation_rs_release
	cargo bench -p iban_validation_bench_rs
	$(RMRF) iban_validation_bench_rs/criterion
	$(MVF) target/criterion iban_validation_bench_rs/criterion
