# AGENTS.md - Terminal Screensaver

## Quick Reference

- **Purpose**: Rust-based dynamic terminal screensavers
- **Type**: Cargo package with plugin architecture
- **Config**: `terminal-screensaver.toml`

## Core Objectives

- Dynamic screensavers that auto-resize with any dimensions
- Cargo package installable into any Rust crate
- Plugin architecture with configurable options
- Minimal user configuration required

## Development Workflow

1. Implement features in dedicated feature directories
2. Maintain test coverage with co-located test files
3. Update documentation for any changes
4. Run `./local-ci.sh` before committing
5. Ensure adherence to project principles

## Key Dependencies

- **crossterm**: Cross-platform terminal manipulation
- **serde**: Serialization with derive features
- **toml**: TOML configuration parsing
- **log**: Logging facade
- **simplelog**: Logger implementation
- **clap**: CLI argument parsing

## Core Development Rules

- **VERIFY BEFORE ASSUMING**: Check actual test results and error messages
- **USE EVIDENCE-BASED DEBUGGING**: Use `gh cli` to inspect workflow runs
- **NO EMOJIS** in documentation or code comments
- Follow Rust naming conventions and idioms
- Keep logging centralized in `src/logger/`
- Test all changes with local CI script

## Security

- No secrets in documentation or code
- Configuration files sanitized before logging
- Input validation on all user interactions
- Secure defaults for all configuration options

## Working Rules

- Stop and explain before major architectural changes
- One change per commit, commit before starting next
- Do not bundle unrelated work into the same commit
