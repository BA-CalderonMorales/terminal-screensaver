# Terminal Screensaver Project - Agent Documentation

## Project Overview

The Terminal Screensaver project is a Rust-based cargo package designed to provide dynamic, resizable terminal screensavers that can be easily integrated into any Rust project. This project follows a plugin architecture approach, enabling seamless integration while maintaining separation of concerns.

## Core Objectives

- Create dynamic terminal screensavers that automatically resize with any screen dimensions
- Provide a cargo package installable into any Rust crate project
- Implement a plugin architecture with configurable options
- Maintain minimal user configuration requirements
- Ensure seamless integration through `terminal-screensaver.toml` configuration

## Architecture Principles

This project adheres to **Screaming Architecture** principles where the codebase structure immediately communicates the business domain and intent. The architecture prioritizes:

- Clear separation of concerns
- Feature-based organization
- Testable components
- Shared utilities without tight coupling
- Maintainable logging infrastructure

## Project Structure

```
src/
├── feature_alpha/           # Primary screensaver feature implementation
│   ├── feature_alpha_logic.rs
│   ├── feature_alpha_logic_tests.rs
│   └── mod.rs
├── feature_n/               # Additional screensaver features
├── shared/                  # Cross-feature utilities
│   ├── shared_logic.rs
│   └── mod.rs
├── styles/                  # UI styling and theming
│   ├── style_logic.rs
│   └── mod.rs
├── cli/                     # Command-line interface
│   ├── cli_logic.rs
│   ├── cli.rs
│   └── mod.rs
├── logger/                  # Centralized logging infrastructure
│   ├── logger_logic.rs
│   └── mod.rs
├── lib.rs                   # Library entry point
└── main.rs                  # Binary entry point
```

## Core Functionality

### Default Behavior
- **ESC key**: Exit screensaver (always available)
- **ENTER key**: Navigate to next screen (when additional options configured)
- **Minimal by default**: Only ESC option shown unless configured otherwise

### Plugin Architecture
- Host applications configure additional actions through `terminal-screensaver.toml`
- Screensaver delegates action execution to host application
- No knowledge required of what additional actions perform
- Clean separation between screensaver logic and host application logic

## Documentation Structure

This project maintains comprehensive documentation organized by SDLC discipline:

| Document | Focus Area | Description |
|----------|------------|-------------|
| [backend.md](docs/backend.md) | Backend Architecture | Core logic, data flow, and system design |
| [frontend.md](docs/frontend.md) | UI/Terminal Interface | User interface, styling, and interaction patterns |
| [cli.md](docs/cli.md) | Command Line Interface | CLI design, argument parsing, and user experience |
| [testing.md](docs/testing.md) | Testing Strategy | Unit tests, integration tests, and quality assurance |
| [security.md](docs/security.md) | Security Guidelines | Security considerations and best practices |
| [cicd.md](docs/cicd.md) | CI/CD Pipeline | Build processes, automation, and deployment |
| [documentation.md](docs/documentation.md) | Documentation Standards | Writing guidelines and maintenance procedures |

## Development Workflow

1. **Feature Development**: Implement new features in dedicated feature directories
2. **Testing**: Maintain comprehensive test coverage with co-located test files
3. **Documentation**: Update relevant documentation files for any changes
4. **Local CI**: Run `./local-ci.sh` before committing changes
5. **Code Review**: Ensure adherence to project principles and standards

## Key Dependencies

- **crossterm**: Cross-platform terminal manipulation
- **ratatui**: Terminal UI framework for dynamic layouts
- **serde/toml**: Configuration file parsing
- **log/simplelog**: Structured logging infrastructure
- **clap**: Command-line argument parsing

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

- **NO EMOJIS** in any documentation or code comments
- Follow Rust naming conventions and idioms
- Maintain the established directory structure
- Keep logging centralized in `src/logger/`
- Test all changes with the local CI script
- Document architectural decisions in appropriate docs files