.SILENT:

# Search for the first line in Cargo.toml that contains 'version' and return its
# value, stripping the quotes.
VERSION := $(shell awk '/version/{gsub(/"/, "", $$3); print $$3; exit}' ./Cargo.toml)

windows:
	rustup update
	cross build --target x86_64-pc-windows-gnu --release

release:
	mkdir -p dist
	rsync -av target/x86_64-pc-windows-gnu/release/magicutil.exe ./dist/MagicUtil.exe
	gh release create v$(VERSION) --title "MagicUtil $(VERSION)" ./dist/MagicUtil.exe

deploy: windows release
