# Projekt-ZAKYX Makefile
# Cross-Platform Build System

.PHONY: help build build-release test clean install deps-linux deps-windows check format lint

# Default target
help:
	@echo "Projekt-ZAKYX Build System"
	@echo "========================"
	@echo ""
	@echo "Available targets:"
	@echo "  build         - Build debug version"
	@echo "  build-release - Build release version"
	@echo "  test          - Run all tests"
	@echo "  clean         - Clean build artifacts"
	@echo "  install       - Install dependencies for current platform"
	@echo "  check         - Check code without building"
	@echo "  format        - Format code with rustfmt"
	@echo "  lint          - Run clippy linter"
	@echo ""
	@echo "Platform-specific targets:"
	@echo "  deps-linux    - Install Linux dependencies"
	@echo "  deps-windows  - Install Windows dependencies"
	@echo "  build-linux   - Build for Linux"
	@echo "  build-windows - Build for Windows"
	@echo "  build-macos   - Build for macOS"

# Build targets
build:
	cargo build --verbose

build-release:
	cargo build --release --verbose

# Test targets
test:
	cargo test --verbose

test-release:
	cargo test --release --verbose

# Clean target
clean:
	cargo clean

# Platform-specific builds
build-linux:
	cargo build --target x86_64-unknown-linux-gnu --verbose

build-windows:
	cargo build --target x86_64-pc-windows-msvc --verbose

build-macos:
	cargo build --target x86_64-apple-darwin --verbose

# Cross-compilation builds
build-all-platforms: build-linux build-windows build-macos

# Development tools
check:
	cargo check --all-targets --all-features

format:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Dependency installation
install: 
ifeq ($(OS),Windows_NT)
	@echo "Windows detected - please ensure WebView2 Runtime is installed"
	@echo "Run 'make deps-windows' for more information"
else
	@echo "Unix-like system detected"
	@if [ -f /etc/os-release ]; then \
		. /etc/os-release; \
		if [ "$$ID" = "ubuntu" ] || [ "$$ID" = "debian" ]; then \
			make deps-linux-debian; \
		elif [ "$$ID" = "fedora" ] || [ "$$ID" = "rhel" ] || [ "$$ID" = "centos" ]; then \
			make deps-linux-fedora; \
		elif [ "$$ID" = "arch" ] || [ "$$ID" = "manjaro" ]; then \
			make deps-linux-arch; \
		else \
			echo "Unsupported Linux distribution: $$ID"; \
			echo "Please install dependencies manually"; \
		fi; \
	else \
		echo "Cannot detect Linux distribution"; \
	fi
endif

deps-linux-debian:
	@echo "Installing Ubuntu/Debian dependencies..."
	sudo apt update
	sudo apt install -y \
		build-essential \
		pkg-config \
		libgtk-3-dev \
		libwebkit2gtk-4.0-dev \
		libglib2.0-dev \
		libcairo-gobject2 \
		libgtk-3-0 \
		libwebkit2gtk-4.0-37 \
		libgdk-pixbuf2.0-dev \
		libpango1.0-dev \
		libatk1.0-dev \
		libcairo-dev

deps-linux-fedora:
	@echo "Installing Fedora/RHEL dependencies..."
	sudo dnf groupinstall -y "Development Tools"
	sudo dnf install -y \
		pkg-config \
		gtk3-devel \
		webkit2gtk3-devel \
		glib2-devel \
		cairo-gobject-devel \
		gdk-pixbuf2-devel \
		pango-devel \
		atk-devel \
		cairo-devel

deps-linux-arch:
	@echo "Installing Arch Linux dependencies..."
	sudo pacman -S --needed \
		base-devel \
		pkg-config \
		gtk3 \
		webkit2gtk \
		glib2 \
		cairo \
		gdk-pixbuf2 \
		pango \
		atk

deps-windows:
	@echo "Windows Dependencies:"
	@echo "1. Rust (install via rustup.rs)"
	@echo "2. Visual Studio Build Tools 2019 or newer"
	@echo "3. WebView2 Runtime (usually pre-installed on Windows 11)"
	@echo ""
	@echo "To install WebView2 Runtime manually:"
	@echo "https://developer.microsoft.com/en-us/microsoft-edge/webview2/"

# Development workflow
dev: format lint check test

# Release workflow
release: clean format lint test build-release

# CI/CD simulation
ci: format lint check test build

# Documentation
docs:
	cargo doc --open

# Benchmarks (if available)
bench:
	cargo bench

# Security audit
audit:
	cargo audit

# Update dependencies
update:
	cargo update

# Show project information
info:
	@echo "Project: Projekt-ZAKYX"
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "Target: $$(rustc -vV | grep host | cut -d' ' -f2)"
	@echo ""
	@echo "Available targets:"
	@rustc --print target-list | grep -E "(linux|windows|darwin)" | head -10 