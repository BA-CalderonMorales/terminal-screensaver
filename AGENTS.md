# Terminal Screensaver Project - Agent Documentation

## Project Overview

The Terminal Screensaver project is a Rust-based cargo package designed to provide dynamic, resizable terminal screensavers that can be easily integrated into any Rust project. This project follows a plugin architecture approach, enabling seamless integration while maintaining separation of concerns.

## Core Objectives

- Create dynamic terminal screensavers that automatically resize with any screen dimensions
- Provide a cargo package installable into any Rust crate project
- Implement a plugin architecture with configurable options
- Maintain minimal user configuration requirements
- Ensure seamless integration through `terminal-screensaver.toml` configuration

### Plugin Architecture
- Host applications configure additional actions through `terminal-screensaver.toml`
- Screensaver delegates action execution to host application
- No knowledge required of what additional actions perform
- Clean separation between screensaver logic and host application logic

## Development Workflow

1. **Feature Development**: Implement new features in dedicated feature directories
2. **Testing**: Maintain comprehensive test coverage with co-located test files
3. **Documentation**: Update relevant documentation files for any changes
4. **Local CI**: Run `./local-ci.sh` before committing changes
5. **Code Review**: Ensure adherence to project principles and standards

## Key Dependencies

- **crossterm**: Cross-platform terminal manipulation library
- **serde**: Serialization framework with derive features for configuration parsing
- **toml**: TOML configuration file format parsing
- **log**: Logging facade for structured logging
- **simplelog**: Simple logger implementation for log output
- **clap**: Command-line argument parsing and CLI interface generation

## Security Considerations

- No secrets or sensitive information in documentation or code
- Configuration files sanitized before logging
- Input validation on all user interactions
- Secure defaults for all configuration options

## Getting Started for Agents

When working on this project:

1. Review the specific discipline documentation in `docs/`
2. Understand the current feature structure
3. Follow the established patterns for new features
4. Maintain test coverage for all changes
5. Update documentation for any architectural changes

## Agent Guidelines

### Core Development Rules

- **VERIFY BEFORE ASSUMING**: Always check actual test results, error messages, and pipeline outputs before proposing solutions. Never assume fixes will work without validation.
- **USE EVIDENCE-BASED DEBUGGING**: Utilize tools like `gh cli` to inspect actual workflow runs, error logs, and system states rather than making theoretical assumptions.
- **CHECK PERMISSIONS AND CONFIGURATION**: Validate that all required permissions, tokens, and configurations are properly set before assuming functionality will work.
- **NO EMOJIS** in any documentation or code comments
- **Evidence-Based Solutions**: Base all recommendations on actual observed behavior, not theoretical assumptions
- Follow Rust naming conventions and idioms
- Maintain the established directory structure
- Keep logging centralized in `src/logger/`
- Test all changes with the local CI script
- Document architectural decisions in appropriate docs files