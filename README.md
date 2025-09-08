# Terminal Screensaver

A dynamic terminal screen saver crate with plugin architecture that automatically resizes with any screen size.

## Features

- **Dynamic Sizing**: Automatically adapts to any terminal size
- **Plugin Architecture**: Extensible event handling for custom actions
- **TOML Configuration**: Easy customization without code changes
- **Cross-platform**: Works on Windows, Linux, and macOS
- **Library & Binary**: Use as a library or standalone application

## Quick Start

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
terminal-screensaver = "0.0.1"
```

Use in your code:

```rust
use terminal_screensaver::{Config, run_screensaver};

let config = Config {
    text: "Welcome to My App".to_string(),
    style: "default".to_string(),
};

run_screensaver(config);
```

### As a Binary

```bash
cargo install terminal-screensaver
terminal-screensaver -c my-config.toml
```

## Configuration

Create a `terminal-screensaver.toml` file:

```toml
text = "Welcome to Terminal Screensaver v0.0.1"
style = "default"
```

Available styles: `default`, `red`, `blue`, `green`

## Controls

- **ESC**: Exit the screen saver
- **Enter**: Trigger additional options (extensible)

## Architecture

The project follows a screaming architecture with clear separation of concerns:

- `src/cli/`: Command-line interface and configuration
- `src/feature_alpha/`: Core screen saver implementation
- `src/logger/`: Centralized logging
- `src/shared/`: Shared utilities
- `src/styles/`: UI styling system

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
