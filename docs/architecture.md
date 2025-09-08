# Terminal Screensaver Architecture

## Overview

The Terminal Screensaver project follows **Screaming Architecture** principles where the codebase structure immediately communicates the business domain and intent. The architecture prioritizes clean separation of concerns, feature-based organization, and maintainable components.

## Core Principles

- **Clear separation of concerns**: Each module has a single, well-defined responsibility
- **Feature-based organization**: Code is organized around business features rather than technical layers
- **Testable components**: All components are designed to be easily testable in isolation
- **Shared utilities**: Common functionality is extracted without creating tight coupling
- **Maintainable logging**: Centralized logging infrastructure across all components

## Project Structure

```
src/
├── cli/                     # Command-line interface and argument parsing
│   ├── cli_logic.rs        # Core CLI functionality and configuration parsing
│   ├── cli.rs              # Command-line interface entry point
│   └── mod.rs              # Module declarations
├── features/               # Feature-based screensaver implementations
│   ├── text_display/       # Text display and script integration feature
│   ├── matrix_rain/        # Matrix rain animation (planned)
│   ├── clock_display/      # Clock display feature (planned)
│   └── ...                 # Additional features
├── shared/                 # Cross-feature utilities and common components
│   ├── shared_logic.rs     # Shared business logic
│   ├── simple_renderer.rs  # Terminal rendering abstraction
│   └── mod.rs              # Module declarations
├── styles/                 # UI styling and theming system
│   ├── style_logic.rs      # Styling implementation
│   └── mod.rs              # Module declarations
├── logger/                 # Centralized logging infrastructure
│   ├── logger_logic.rs     # Logging configuration and setup
│   └── mod.rs              # Module declarations
├── lib.rs                  # Library entry point and public API
└── main.rs                 # Binary entry point
```

## Component Responsibilities

### CLI Module (`src/cli/`)
- **Purpose**: Handle command-line arguments, configuration parsing, and user input validation
- **Key Components**:
  - `Config` struct: Central configuration management
  - `ActionConfig` struct: Script integration action definitions
  - TOML configuration parsing with serde
  - Command-line argument processing with clap

### Features Module (`src/features/`)
- **Purpose**: Implement individual screensaver features with plugin architecture
- **Current Implementation**:
  - `text_display`: Core text display with script integration capabilities
  - Extensible design for adding new screensaver types
- **Plugin Architecture**: Each feature is self-contained and can be enabled/disabled

### Shared Module (`src/shared/`)
- **Purpose**: Provide common utilities and abstractions used across features
- **Key Components**:
  - `SimpleRenderer`: Cross-platform terminal rendering abstraction
  - `TextLine`: Text rendering with color support
  - Terminal utilities and screen management
  - Cross-feature helper functions

### Styles Module (`src/styles/`)
- **Purpose**: Handle visual styling, theming, and UI consistency
- **Capabilities**:
  - Color scheme management
  - Unicode border rendering
  - Layout calculations and positioning
  - Theme-based customization

### Logger Module (`src/logger/`)
- **Purpose**: Provide centralized, structured logging across all components
- **Features**:
  - Configurable log levels
  - File-based logging with rotation
  - Performance monitoring capabilities
  - Debug and troubleshooting support

## Design Patterns

### Plugin Architecture
- **Extensible Actions**: Script integration system allows unlimited custom actions
- **Configuration-Driven**: All behavior controlled via TOML configuration
- **Hot-Pluggable**: Features can be enabled/disabled without code changes

### Event-Driven Design
- **Keyboard Event Handling**: Clean separation between input handling and action execution
- **Script Execution Engine**: Isolated subprocess management with proper terminal handling
- **State Management**: Minimal state with clear ownership and lifecycle

### Cross-Platform Abstraction
- **Terminal Abstraction**: `SimpleRenderer` provides consistent interface across platforms
- **Process Management**: Safe script execution with platform-specific optimizations
- **Configuration**: Platform-agnostic configuration system

## Data Flow

1. **Initialization**: CLI parses arguments and loads TOML configuration
2. **Setup**: Logger initializes, terminal enters raw mode, renderer setup
3. **Main Loop**: Event loop handles keyboard input and screen updates
4. **Action Execution**: Script actions execute with proper terminal mode management
5. **Cleanup**: Terminal state restored, resources properly released

## Testing Strategy

- **Unit Tests**: Each module has co-located test files (`*_tests.rs`)
- **Integration Tests**: End-to-end testing of complete workflows
- **Configuration Testing**: TOML parsing and validation tests
- **Cross-Platform Testing**: Platform-specific behavior verification

## Performance Considerations

- **Minimal Allocations**: Efficient string handling and memory management
- **Terminal Optimization**: Optimized rendering with minimal screen updates
- **Script Isolation**: Subprocess execution doesn't block main event loop
- **Resource Management**: Proper cleanup and resource lifecycle management

## Security Model

- **Input Validation**: All user inputs are validated and sanitized
- **Script Execution**: Safe subprocess execution with error handling
- **Configuration Security**: TOML parsing with safe defaults and validation
- **No Privilege Escalation**: All operations run with user-level permissions

## Extensibility

### Adding New Features
1. Create feature directory under `src/features/`
2. Implement feature-specific logic and tests
3. Add configuration options to CLI module
4. Update documentation and examples

### Custom Actions
- Add action definitions to TOML configuration
- Script actions automatically integrate with existing infrastructure
- No code changes required for new script integrations

### Styling and Themes
- Extend styling system with new color schemes
- Add custom border styles and layout options
- Theme system designed for easy extensibility

This architecture ensures the Terminal Screensaver remains maintainable, testable, and extensible while providing a professional user experience across all supported platforms.
