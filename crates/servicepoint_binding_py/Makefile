.phony:
build:
	cargo build
	pip uninstall -y servicepoint
	pip install --editable .
	python -c "import servicepoint as s; print(dir(s)); print(dir(s.rust_bindings))"
