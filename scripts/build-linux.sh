#!/bin/bash

# Projekt-ZAKYX Linux Build Script
# Automatisiert den Build-Prozess für Linux-Systeme

set -e  # Exit on any error

echo "🚀 Projekt-ZAKYX Linux Build Script"
echo "=================================="

# Farben für Output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Funktionen
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# System-Detection
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$ID
        VERSION=$VERSION_ID
    else
        print_error "Cannot detect Linux distribution"
        exit 1
    fi
    print_status "Detected: $PRETTY_NAME"
}

# Dependency Check
check_dependencies() {
    print_status "Checking dependencies..."
    
    # Check Rust
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed. Please install Rust first:"
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    # Check Cargo
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed"
        exit 1
    fi
    
    # Check pkg-config
    if ! command -v pkg-config &> /dev/null; then
        print_error "pkg-config is not installed"
        install_dependencies
        return
    fi
    
    # Check GTK3
    if ! pkg-config --exists gtk+-3.0; then
        print_error "GTK3 development libraries not found"
        install_dependencies
        return
    fi
    
    # Check WebKit2GTK
    if ! pkg-config --exists webkit2gtk-4.1; then
        print_error "WebKit2GTK 4.1 development libraries not found"
        install_dependencies
        return
    fi
    
    # Check JavaScriptCore GTK
    if ! pkg-config --exists javascriptcoregtk-4.1; then
        print_error "JavaScriptCore GTK 4.1 development libraries not found"
        install_dependencies
        return
    fi
    
    # Check libsoup-3.0
    if ! pkg-config --exists libsoup-3.0; then
        print_error "libsoup-3.0 development libraries not found"
        install_dependencies
        return
    fi
    
    print_success "All dependencies are satisfied"
}

# Install Dependencies
install_dependencies() {
    print_status "Installing dependencies for $DISTRO..."
    
    case $DISTRO in
        ubuntu|debian)
            print_status "Installing Ubuntu/Debian dependencies..."
            sudo apt update
            sudo apt install -y \
                build-essential \
                pkg-config \
                libgtk-3-dev \
                libwebkit2gtk-4.1-dev \
                libjavascriptcoregtk-4.1-dev \
                libsoup-3.0-dev \
                libglib2.0-dev \
                libcairo-gobject2 \
                libgtk-3-0 \
                libwebkit2gtk-4.1-0 \
                libjavascriptcoregtk-4.1-0 \
                libsoup-3.0-0 \
                libgdk-pixbuf2.0-dev \
                libpango1.0-dev \
                libatk1.0-dev \
                libcairo-dev \
                libappindicator3-dev \
                librsvg2-dev
            ;;
        fedora|rhel|centos)
            print_status "Installing Fedora/RHEL dependencies..."
            sudo dnf groupinstall -y "Development Tools"
            sudo dnf install -y \
                pkg-config \
                gtk3-devel \
                webkit2gtk4.1-devel \
                libsoup3-devel \
                glib2-devel \
                cairo-gobject-devel \
                gdk-pixbuf2-devel \
                pango-devel \
                atk-devel \
                cairo-devel \
                libappindicator-gtk3-devel \
                librsvg2-devel
            ;;
        arch|manjaro)
            print_status "Installing Arch Linux dependencies..."
            sudo pacman -S --needed \
                base-devel \
                pkg-config \
                gtk3 \
                webkit2gtk-4.1 \
                libsoup3 \
                glib2 \
                cairo \
                gdk-pixbuf2 \
                pango \
                atk \
                libappindicator-gtk3 \
                librsvg
            ;;
        opensuse*)
            print_status "Installing openSUSE dependencies..."
            sudo zypper install -y \
                -t pattern devel_basis \
                pkg-config \
                gtk3-devel \
                webkit2gtk4_1-devel \
                libsoup3-devel \
                glib2-devel \
                cairo-devel \
                gdk-pixbuf-devel \
                pango-devel \
                atk-devel \
                libappindicator3-devel \
                librsvg-devel
            ;;
        *)
            print_warning "Unknown distribution: $DISTRO"
            print_warning "Please install dependencies manually:"
            echo "- build-essential/development tools"
            echo "- pkg-config"
            echo "- GTK3 development libraries"
            echo "- WebKit2GTK 4.1 development libraries"
            echo "- JavaScriptCore GTK 4.1 development libraries"
            echo "- libsoup-3.0 development libraries"
            read -p "Continue anyway? (y/N): " -n 1 -r
            echo
            if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                exit 1
            fi
            ;;
    esac
}

# Build Project
build_project() {
    print_status "Building Projekt-ZAKYX..."
    
    # Clean previous builds
    if [ "$1" = "--clean" ]; then
        print_status "Cleaning previous builds..."
        cargo clean
    fi
    
    # Build
    if [ "$1" = "--release" ] || [ "$2" = "--release" ]; then
        print_status "Building release version..."
        cargo build --release --verbose
        BINARY_PATH="target/release/projekt-zakyx"
    else
        print_status "Building debug version..."
        cargo build --verbose
        BINARY_PATH="target/debug/projekt-zakyx"
    fi
    
    if [ -f "$BINARY_PATH" ]; then
        print_success "Build completed successfully!"
        print_status "Binary location: $BINARY_PATH"
        
        # Show binary info
        ls -lh "$BINARY_PATH"
        file "$BINARY_PATH"
    else
        print_error "Build failed - binary not found"
        exit 1
    fi
}

# Run Tests
run_tests() {
    print_status "Running tests..."
    cargo test --verbose
    print_success "All tests passed!"
}

# Main execution
main() {
    detect_distro
    check_dependencies
    
    # Parse arguments
    CLEAN=false
    RELEASE=false
    RUN_TESTS=false
    
    for arg in "$@"; do
        case $arg in
            --clean)
                CLEAN=true
                ;;
            --release)
                RELEASE=true
                ;;
            --test)
                RUN_TESTS=true
                ;;
            --help)
                echo "Usage: $0 [OPTIONS]"
                echo "Options:"
                echo "  --clean     Clean previous builds"
                echo "  --release   Build release version"
                echo "  --test      Run tests after build"
                echo "  --help      Show this help"
                exit 0
                ;;
        esac
    done
    
    # Build
    if [ "$CLEAN" = true ] && [ "$RELEASE" = true ]; then
        build_project --clean --release
    elif [ "$CLEAN" = true ]; then
        build_project --clean
    elif [ "$RELEASE" = true ]; then
        build_project --release
    else
        build_project
    fi
    
    # Run tests if requested
    if [ "$RUN_TESTS" = true ]; then
        run_tests
    fi
    
    print_success "Build script completed successfully!"
    echo ""
    echo "To run the application:"
    if [ "$RELEASE" = true ]; then
        echo "  ./target/release/projekt-zakyx"
    else
        echo "  ./target/debug/projekt-zakyx"
    fi
}

# Run main function with all arguments
main "$@" 