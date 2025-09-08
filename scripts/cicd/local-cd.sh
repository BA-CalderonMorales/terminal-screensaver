#!/bin/bash

# local-cd.sh - Local Continuous Deployment script for terminal-screensaver
# This script handles the release process for publishing to crates.io
# Usage: ./scripts/local-cd.sh [--version VERSION] [--dry-run] [--help]

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Configuration
DRY_RUN=false
VERSION=""
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --version)
            VERSION="$2"
            shift 2
            ;;
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  --version VERSION  Specify the version to release (e.g., 0.0.1)"
            echo "  --dry-run         Perform all steps except actual publishing"
            echo "  --help, -h        Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0 --version 0.0.1 --dry-run    # Test release process for v0.0.1"
            echo "  $0 --version 0.0.1              # Release v0.0.1 to crates.io"
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

print_info() {
    echo -e "${PURPLE}ℹ${NC} $1"
}

run_step() {
    local step_name="$1"
    local command="$2"
    
    print_step "$step_name"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        print_info "DRY RUN: Would execute: $command"
        print_success "$step_name completed (dry run)"
    else
        if eval "$command"; then
            print_success "$step_name completed"
        else
            print_error "$step_name failed"
            exit 1
        fi
    fi
    echo
}

# Ensure we're in the project root
cd "$PROJECT_ROOT"

echo -e "${PURPLE}Terminal Screensaver - Local Continuous Deployment${NC}"
echo -e "${PURPLE}=================================================${NC}"
echo

# Check if version is provided
if [[ -z "$VERSION" ]]; then
    print_error "Version not specified. Use --version to specify the version to release."
    echo "Example: $0 --version 0.0.1"
    exit 1
fi

# Validate version format (semantic versioning)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "Invalid version format. Please use semantic versioning (e.g., 0.0.1)"
    exit 1
fi

print_info "Preparing to release version: $VERSION"
if [[ "$DRY_RUN" == "true" ]]; then
    print_warning "Running in DRY RUN mode - no actual publishing will occur"
fi
echo

# 1. Check if we're in a git repository
print_step "Verifying git repository"
if ! git rev-parse --is-inside-work-tree &> /dev/null; then
    print_error "Not in a git repository"
    exit 1
fi
print_success "Git repository verification completed"
echo

# 2. Check for uncommitted changes
print_step "Checking for uncommitted changes"
if ! git diff-index --quiet HEAD --; then
    print_error "Uncommitted changes found. Please commit or stash changes before releasing."
    git status --short
    exit 1
fi
print_success "Working directory is clean"
echo

# 3. Check if we're on main branch
print_step "Verifying branch"
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "main" ]]; then
    print_warning "Currently on branch '$CURRENT_BRANCH', not 'main'"
    read -p "Continue with release from this branch? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "Release cancelled"
        exit 0
    fi
fi
print_success "Branch verification completed"
echo

# 4. Run local CI to ensure everything passes
print_step "Running local CI pipeline"
if [[ -f "$SCRIPT_DIR/local-ci.sh" ]]; then
    "$SCRIPT_DIR/local-ci.sh"
    print_success "Local CI pipeline passed"
else
    print_warning "Local CI script not found, skipping CI checks"
fi
echo

# 5. Update version in Cargo.toml
print_step "Updating version in Cargo.toml"
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\([^"]*\)"/\1/')
print_info "Current version: $CURRENT_VERSION"
print_info "New version: $VERSION"

if [[ "$DRY_RUN" == "false" ]]; then
    # Create backup
    cp Cargo.toml Cargo.toml.backup
    
    # Update version
    sed -i.tmp "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
    rm Cargo.toml.tmp
    
    # Verify the change
    NEW_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\([^"]*\)"/\1/')
    if [[ "$NEW_VERSION" != "$VERSION" ]]; then
        print_error "Failed to update version in Cargo.toml"
        mv Cargo.toml.backup Cargo.toml
        exit 1
    fi
    rm Cargo.toml.backup
fi
print_success "Version update completed"
echo

# 6. Update version in lib.rs (if version constant exists)
print_step "Checking for version constant in lib.rs"
if grep -q "pub const VERSION:" src/lib.rs 2>/dev/null; then
    if [[ "$DRY_RUN" == "false" ]]; then
        sed -i.tmp "s/pub const VERSION: &str = \".*\";/pub const VERSION: \&str = \"$VERSION\";/" src/lib.rs
        rm src/lib.rs.tmp
    fi
    print_success "Version constant updated in lib.rs"
else
    print_info "No version constant found in lib.rs, skipping"
fi
echo

# 7. Run cargo check to ensure everything compiles
run_step "Running cargo check" "cargo check --all-targets --all-features"

# 8. Run tests one more time
run_step "Running final test suite" "cargo test --all-features"

# 9. Build release version
run_step "Building release version" "cargo build --release"

# 10. Generate documentation
run_step "Generating documentation" "cargo doc --no-deps --all-features"

# 11. Package for publishing (dry run first)
run_step "Packaging crate (dry run)" "cargo package --allow-dirty"

# 12. Check package contents
print_step "Reviewing package contents"
if [[ "$DRY_RUN" == "false" ]]; then
    echo "Package contents:"
    tar -tzf "target/package/terminal-screensaver-$VERSION.crate" | head -20
    if [[ $(tar -tzf "target/package/terminal-screensaver-$VERSION.crate" | wc -l) -gt 20 ]]; then
        echo "... (and $(( $(tar -tzf "target/package/terminal-screensaver-$VERSION.crate" | wc -l) - 20 )) more files)"
    fi
else
    print_info "DRY RUN: Would review package contents"
fi
print_success "Package review completed"
echo

# 13. Check if version already exists on crates.io
print_step "Checking if version exists on crates.io"
if curl -s "https://crates.io/api/v1/crates/terminal-screensaver" | grep -q "\"num\":\"$VERSION\""; then
    print_error "Version $VERSION already exists on crates.io"
    exit 1
fi
print_success "Version $VERSION is available"
echo

# 14. Commit version changes
if [[ "$DRY_RUN" == "false" ]]; then
    print_step "Committing version changes"
    git add Cargo.toml
    if grep -q "pub const VERSION:" src/lib.rs 2>/dev/null; then
        git add src/lib.rs
    fi
    git commit -m "Bump version to $VERSION"
    print_success "Version changes committed"
    echo
fi

# 15. Create git tag
run_step "Creating git tag" "git tag -a v$VERSION -m 'Release version $VERSION'"

# 16. Final confirmation before publishing
if [[ "$DRY_RUN" == "false" ]]; then
    print_warning "FINAL CONFIRMATION REQUIRED"
    echo "About to publish terminal-screensaver v$VERSION to crates.io"
    echo "This action cannot be undone (crates cannot be deleted, only yanked)"
    echo ""
    read -p "Are you absolutely sure you want to proceed? (yes/no): " -r
    if [[ $REPLY != "yes" ]]; then
        print_info "Publication cancelled"
        # Clean up the tag if created
        git tag -d "v$VERSION" 2>/dev/null || true
        exit 0
    fi
fi

# 17. Publish to crates.io
run_step "Publishing to crates.io" "cargo publish"

# 18. Push to git repository
run_step "Pushing to git repository" "git push origin HEAD && git push origin v$VERSION"

# 19. Verify publication
print_step "Verifying publication"
if [[ "$DRY_RUN" == "false" ]]; then
    sleep 10  # Give crates.io time to process
    if curl -s "https://crates.io/api/v1/crates/terminal-screensaver" | grep -q "\"num\":\"$VERSION\""; then
        print_success "Publication verified on crates.io"
    else
        print_warning "Publication not yet visible on crates.io (may take a few minutes)"
    fi
else
    print_info "DRY RUN: Would verify publication"
fi
echo

# 20. Generate release notes
print_step "Generating release notes"
if [[ "$DRY_RUN" == "false" ]]; then
    RELEASE_NOTES_FILE="release-notes-$VERSION.md"
    cat > "$RELEASE_NOTES_FILE" << EOF
# Terminal Screensaver v$VERSION

## Installation

\`\`\`bash
cargo install terminal-screensaver
\`\`\`

## What's New

- Initial release of terminal screensaver
- Dynamic screen size adaptation
- Plugin architecture for host applications
- Configurable through \`terminal-screensaver.toml\`
- Multiple screensaver features

## Documentation

- [README](https://github.com/BA-CalderonMorales/terminal-screensaver/blob/v$VERSION/README.md)
- [Documentation](https://github.com/BA-CalderonMorales/terminal-screensaver/tree/v$VERSION/docs)
- [Examples](https://github.com/BA-CalderonMorales/terminal-screensaver/tree/v$VERSION/examples)

## Changes

$(git log --oneline --pretty=format:"- %s" $(git describe --tags --abbrev=0 HEAD^)..HEAD 2>/dev/null || echo "- Initial release")

---

**Full Changelog**: https://github.com/BA-CalderonMorales/terminal-screensaver/compare/$(git describe --tags --abbrev=0 HEAD^ 2>/dev/null || echo "initial")...v$VERSION
EOF
    print_success "Release notes generated: $RELEASE_NOTES_FILE"
else
    print_info "DRY RUN: Would generate release notes"
fi
echo

# Final summary
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}🎉 Release v$VERSION completed successfully!${NC}"
echo -e "${GREEN}========================================${NC}"
echo
echo "Next steps:"
echo "- Monitor crates.io for download statistics"
echo "- Update documentation if needed"
echo "- Consider creating a GitHub release"
echo "- Update project README with new version info"
echo
echo "Release artifacts:"
echo "- Crate published: https://crates.io/crates/terminal-screensaver"
echo "- Documentation: https://docs.rs/terminal-screensaver/$VERSION/"
echo "- Git tag: v$VERSION"

if [[ "$DRY_RUN" == "false" && -f "release-notes-$VERSION.md" ]]; then
    echo "- Release notes: release-notes-$VERSION.md"
fi

echo
print_success "Terminal Screensaver v$VERSION is now live!"
