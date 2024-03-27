.SILENT:

# Search for the first line in Cargo.toml that contains 'version' and return its
# value, stripping the quotes.
VERSION := $(shell awk '/version/{gsub(/"/, "", $$3); print $$3; exit}' ./Cargo.toml)

.PHONY: help
help: ## Show this help message
	printf '\033[32mUsage: make [target] ...\033[0m\n\nAvailable targets:\n\n'
	egrep -h '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

windows: ## Build Windows version
	rustup update
	cross build --target x86_64-pc-windows-gnu --release

release: ## Send the release to github
	mkdir -p dist
	rsync -av target/x86_64-pc-windows-gnu/release/magicutil.exe ./dist/MagicUtil.exe
	gh release create v$(VERSION) --title "MagicUtil $(VERSION)" ./dist/MagicUtil.exe

deploy: ## Deploy the package
deploy: windows release
