# Contributing to Terminal Screensaver

Thank you for your interest in contributing to the Terminal Screensaver project! This document provides guidelines and requirements for contributors to ensure consistency and quality across the project.

## Code of Conduct

This project adheres to a professional, inclusive environment focused on technical excellence. All contributors are expected to maintain respectful communication and follow project standards.

## Getting Started

### Prerequisites

- Rust 1.70.0 or later
- Git version control
- GitHub account
- Basic understanding of terminal applications and Rust development

### Development Setup

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/yourusername/terminal-screensaver.git
   cd terminal-screensaver
   ```
3. Add the upstream remote:
   ```bash
   git remote add upstream https://github.com/yourusername/terminal-screensaver.git
   ```
4. Install dependencies and test the build:
   ```bash
   cargo build
   ./scripts/local-ci.sh
   ```

## Project Standards and Requirements

### CRITICAL: No Emojis Policy

**IMPORTANT**: This project has a strict **NO EMOJIS** policy that applies to ALL contributions.

#### What This Means:
- **NO emojis in commit messages**: Use descriptive text only
- **NO emojis in code comments**: Professional documentation standards
- **NO emojis in documentation**: Text-only communication
- **NO emojis in pull request descriptions**: Clear, professional descriptions
- **NO emojis in issue reports**: Technical descriptions without emojis
- **NO emojis in code**: This includes string literals, constants, and output

#### Examples of INCORRECT Usage:
```
❌ "Add cool feature 🚀"
❌ "Fix bug 🐛 in screensaver"
❌ "Update docs 📚"
❌ println!("Welcome! 👋");
❌ // TODO: Make this faster ⚡
```

#### Examples of CORRECT Usage:
```
✓ "Add dynamic screen resizing feature"
✓ "Fix rendering bug in alpha feature"
✓ "Update documentation for CLI interface"
✓ println!("Welcome to Terminal Screensaver");
✓ // TODO: Optimize rendering performance
```

#### Pull Request Review Process:
- PRs containing emojis will be **automatically rejected**
- Contributors will be asked to remove ALL emojis before review
- This policy is non-negotiable and applies to all contributors

### Code Quality Standards

#### Formatting and Style
- **Rust Code**: Must pass `cargo fmt --check`
- **Documentation**: Follow project documentation standards
- **Commit Messages**: Use conventional commit format without emojis
- **Line Length**: Maximum 100 characters for code, 80 for documentation

#### Testing Requirements
- **Unit Tests**: All new functionality must include unit tests
- **Integration Tests**: CLI and feature changes require integration tests
- **Test Coverage**: Maintain minimum 90% coverage for new code
- **Documentation Tests**: All code examples in docs must be tested

#### Security Requirements
- **No Secrets**: Never commit sensitive information
- **Input Validation**: All user input must be validated and sanitized
- **Error Handling**: Proper error handling with user-friendly messages
- **Dependency Audit**: New dependencies must pass security audit

## Development Workflow

### Branch Naming Convention
- **Feature branches**: `feature/descriptive-name`
- **Bug fixes**: `fix/issue-description`
- **Documentation**: `docs/section-update`
- **Refactoring**: `refactor/component-name`

### Commit Message Format
```
type(scope): brief description

Detailed explanation of changes if needed.

Closes #123
```

**Types**: feat, fix, docs, style, refactor, test, chore
**Scope**: feature name, component, or area affected

### Pull Request Process

#### Before Submitting
1. **Run Local CI**: Execute `./scripts/local-ci.sh` successfully
2. **Test Changes**: Verify all tests pass locally
3. **Update Documentation**: Update relevant documentation
4. **Self-Review**: Review your own changes thoroughly
5. **Emoji Check**: Ensure NO emojis anywhere in your changes

#### Pull Request Template
```markdown
## Description
Brief description of changes (no emojis)

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Refactoring

## Testing
- [ ] Local CI pipeline passes
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manual testing completed

## Documentation
- [ ] Code comments updated
- [ ] Documentation files updated
- [ ] API documentation updated
- [ ] Examples updated if needed

## Checklist
- [ ] No emojis in any files (commit messages, code, docs, comments)
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Changes are backward compatible (or breaking change documented)
```

#### Review Process
1. **Automated Checks**: CI pipeline must pass
2. **Emoji Validation**: Manual check for emoji compliance
3. **Code Review**: At least one maintainer approval required
4. **Testing Validation**: All tests must pass
5. **Documentation Review**: Documentation changes reviewed for clarity

## Project Architecture Guidelines

### Screaming Architecture
Follow the established project structure where the codebase immediately reveals business intent:

```
src/
├── feature_*/           # Feature-specific implementations
├── shared/             # Cross-feature utilities (not tightly coupled)
├── styles/             # UI styling and theming
├── cli/                # Command-line interface logic
├── logger/             # Centralized logging infrastructure
└── main.rs            # Application entry point
```

### Feature Development
- **New Features**: Create in dedicated `feature_*` directories
- **Shared Code**: Add to `src/shared/` only if truly reusable
- **Testing**: Co-locate tests with implementation
- **Documentation**: Update relevant docs in `docs/` directory

### Plugin Architecture
- **Host Integration**: Maintain clean separation from host applications
- **Configuration**: Use TOML configuration for customization
- **Minimal Defaults**: ESC key always available, additional options configurable

## Common Contribution Areas

### Documentation Improvements
- **API Documentation**: Rust doc comments for public APIs
- **User Guides**: Clear, step-by-step instructions
- **Examples**: Working code examples that demonstrate features
- **Architecture Docs**: Updates to `docs/` directory files

### Bug Fixes
- **Issue Reproduction**: Clear steps to reproduce the bug
- **Root Cause Analysis**: Understanding of why the bug occurred
- **Testing**: Comprehensive tests to prevent regression
- **Documentation**: Update docs if behavior changes

### New Features
- **Design Discussion**: Open an issue for design discussion first
- **Implementation Plan**: Break down into smaller, reviewable changes
- **Backward Compatibility**: Maintain existing API compatibility
- **Configuration**: Add TOML configuration options as needed

### Performance Improvements
- **Benchmarking**: Include before/after performance measurements
- **Testing**: Ensure no functionality regressions
- **Documentation**: Document any API changes
- **Compatibility**: Maintain terminal compatibility

## Issue Reporting

### Bug Reports
```markdown
## Description
Clear description of the bug (no emojis)

## Steps to Reproduce
1. Step one
2. Step two
3. Step three

## Expected Behavior
What should happen

## Actual Behavior
What actually happens

## Environment
- OS: 
- Terminal: 
- Rust version: 
- Crate version: 

## Additional Context
Any other relevant information
```

### Feature Requests
```markdown
## Feature Description
Clear description of the proposed feature (no emojis)

## Use Case
Why is this feature needed?

## Proposed Implementation
How should this feature work?

## Alternatives Considered
What other approaches were considered?

## Additional Context
Any other relevant information
```

## Release Process

Contributors should be aware of the release process:

1. **Version Increment**: Follows semantic versioning
2. **Local CD**: Uses `./scripts/local-cd.sh` for releases
3. **Documentation**: Updates documentation with new features
4. **Testing**: Full test suite must pass before release

## Communication Guidelines

### Professional Communication
- **Clear and Direct**: Use precise technical language
- **Respectful**: Maintain professional tone in all interactions
- **Constructive**: Provide actionable feedback
- **No Emojis**: Text-only communication in all contexts

### Getting Help
- **Issues**: Use GitHub issues for questions and bug reports
- **Discussions**: Use GitHub discussions for general questions
- **Documentation**: Check existing documentation first
- **Code Review**: Participate in code review discussions

## Recognition

Contributors who follow these guidelines and make valuable contributions will be:
- Acknowledged in release notes
- Added to contributor recognition
- Invited to participate in project direction discussions

## Questions?

If you have questions about contributing that aren't covered here:
1. Check existing documentation in the `docs/` directory
2. Search existing GitHub issues and discussions
3. Open a new issue with the "question" label
4. Remember: no emojis in your communication

Thank you for contributing to Terminal Screensaver and helping maintain our professional, high-quality codebase!
