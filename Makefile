install:
	cargo build --release
	bash scripts/install.sh ${PWD}/target/release

uninstall:
	bash scripts/uninstall.sh
