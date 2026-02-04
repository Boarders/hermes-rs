.PHONY: help install-trunk install-wasm-bindgen install-target setup build serve clean

help:
	@echo "Hermes - Available targets:"
	@echo "  make setup           - Install trunk, wasm-bindgen, and WASM target"
	@echo "  make install-trunk   - Install trunk via cargo"
	@echo "  make install-wasm-bindgen - Install wasm-bindgen-cli via cargo"
	@echo "  make install-target  - Install wasm32-unknown-unknown target"
	@echo "  make build           - Build application"
	@echo "  make serve           - Start development server"
	@echo "  make clean           - Remove build artifacts"

install-trunk:
	@echo "Installing trunk..."
	@command -v trunk &> /dev/null || cargo install trunk
	@echo "Trunk installed successfully!"

install-wasm-bindgen:
	@echo "Installing wasm-bindgen-cli..."
	@command -v wasm-bindgen &> /dev/null || cargo install wasm-bindgen-cli
	@echo "wasm-bindgen-cli installed successfully!"

install-target:
	@echo "Checking wasm32-unknown-unknown target..."
	@rustup target list | grep -q "wasm32-unknown-unknown (installed)" || rustup target add wasm32-unknown-unknown
	@echo "WASM target added!"

setup: install-trunk install-wasm-bindgen install-target
	@echo "Setup complete! You can now run 'make build' or 'make serve'"

build: install-target
	@echo "Building Hermes WASM application..."
	@command -v trunk &> /dev/null || (echo "Error: trunk not installed. Run 'make install-trunk'." && exit 1)
	trunk build --release
	@echo "Build complete! Find output in ./dist/"

serve: install-target
	@echo "Starting Hermes development server..."
	@command -v trunk &> /dev/null || (echo "Error: trunk not installed. Run 'make install-trunk'." && exit 1)
	trunk serve --open

clean:
	@echo "Cleaning build artifacts..."
	rm -rf dist target
	@echo "Clean complete!"
