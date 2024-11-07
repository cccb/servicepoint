.phony:
build:
	maturin develop
	python -c "import servicepoint as s; print(dir(s)); print(dir(s.rust_bindings))"

.phony:
install:
	uv pip install maturin
	uv pip install -e .
