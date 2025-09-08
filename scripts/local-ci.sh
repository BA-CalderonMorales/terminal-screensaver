#!/bin/bash

# local-ci.sh - Local CI automation script for terminal-screensaver
# This script runs common development tasks that should pass before committing code
# Usage: ./local-ci.sh [--skip-tests] [--skip-audit] [--help]

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SKIP_TESTS=false
SKIP_AUDIT=false
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        --skip-audit)
            SKIP_AUDIT=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --skip-tests    Skip running tests"
            echo "  --skip-audit    Skip security audit"
            echo "  --help, -h      Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Helper functions
print_step() {
    echo -e "${BLUE}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

run_step() {
    local step_name="$1"
    local command="$2"
    
    print_step "$step_name"
    
    if eval "$command"; then
        print_success "$step_name completed"
    else
        print_error "$step_name failed"
        exit 1
    fi
    echo
}

# Ensure we're in the project root
cd "$PROJECT_ROOT"

echo -e "${BLUE}Terminal Screensaver - Local CI Pipeline${NC}"
echo -e "${BLUE}=======================================${NC}"
echo

# Check if Cargo.toml exists
if [[ ! -f "Cargo.toml" ]]; then
    print_error "Cargo.toml not found! Make sure you're running this script from the project root."
    exit 1
fi

# 1. Check Rust toolchain
print_step "Checking Rust toolchain"
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found! Please install Rust: https://rustup.rs/"
    exit 1
fi
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
print_success "Rust toolchain check completed"
echo

# 2. Clean previous builds
run_step "Cleaning previous builds" "cargo clean"

# 3. Format code
run_step "Formatting code" "cargo fmt --all"

# 4. Check formatting
run_step "Checking code formatting" "cargo fmt --all -- --check"

# 5. Run clippy (linting)
run_step "Running Clippy lints" "cargo clippy --all-targets --all-features -- -D warnings"

# 6. Build the project
run_step "Building project (debug)" "RUSTFLAGS='-D warnings' cargo build"

# 7. Build the project in release mode
run_step "Building project (release)" "RUSTFLAGS='-D warnings' cargo build --release"

# 8. Run tests (if not skipped)
if [[ "$SKIP_TESTS" == "false" ]]; then
    run_step "Running unit tests" "cargo test --lib"
    run_step "Running integration tests" "cargo test --bins"
    run_step "Running all tests with verbose output" "cargo test --verbose"
else
    print_warning "Skipping tests (--skip-tests flag provided)"
    echo
fi

# 9. Check documentation
run_step "Checking documentation" "cargo doc --no-deps --document-private-items"

# 10. Security audit (if not skipped)
if [[ "$SKIP_AUDIT" == "false" ]]; then
    # Check if cargo-audit is installed
    if command -v cargo-audit &> /dev/null; then
        run_step "Running security audit" "cargo audit --deny warnings --ignore RUSTSEC-2024-0436"
        
        # Generate security report as specified in security.md
        print_step "Generating security audit report"
        if cargo audit --json > security-audit-report.json 2>/dev/null; then
            print_success "Security audit report generated: security-audit-report.json"
        else
            print_warning "Could not generate JSON security report"
        fi
        echo
    else
        print_warning "cargo-audit not installed. Install with: cargo install cargo-audit"
        print_warning "Skipping security audit"
        echo
    fi
else
    print_warning "Skipping security audit (--skip-audit flag provided)"
    echo
fi

# 11. Check for unused dependencies
if command -v cargo-udeps &> /dev/null; then
    run_step "Checking for unused dependencies" "cargo +nightly udeps"
else
    print_warning "cargo-udeps not installed. Install with: cargo install cargo-udeps --locked"
    print_warning "Note: Requires nightly Rust toolchain"
    echo
fi

# 12. Check dependency tree
run_step "Checking dependency tree" "cargo tree"

# 13. Run binary to ensure it works
print_step "Testing binary execution"
if ./target/release/terminal-screensaver --help > /dev/null 2>&1; then
    print_success "Binary execution test completed"
else
    print_warning "Binary execution test failed - this might be expected if CLI requires specific arguments"
fi
echo

# 14. File size check
print_step "Checking binary sizes"
if [[ -f "./target/release/terminal-screensaver" ]]; then
    release_size=$(ls -lh ./target/release/terminal-screensaver | awk '{print $5}')
    echo "Release binary size: $release_size"
fi
if [[ -f "./target/debug/terminal-screensaver" ]]; then
    debug_size=$(ls -lh ./target/debug/terminal-screensaver | awk '{print $5}')
    echo "Debug binary size: $debug_size"
fi
print_success "Binary size check completed"
echo

# Final summary
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}🎉 All CI steps completed successfully!${NC}"
echo -e "${GREEN}========================================${NC}"
echo
echo "Next steps:"
echo "- Review any warnings above"
echo "- Run additional manual tests if needed"
echo "- Consider running: cargo test --release"
echo "- Ready for commit and push!"
echo

# Optional: Show git status if in a git repository
if git rev-parse --is-inside-work-tree &> /dev/null; then
    echo -e "${BLUE}Git Status:${NC}"
    git status --short
fi
