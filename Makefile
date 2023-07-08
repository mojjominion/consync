install:
	cargo build --release
	bash scripts/install.sh

uninstall:
	bash scripts/uninstall.sh
